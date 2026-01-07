# FTS5 Query Sanitization - Implementation Plan

**Issue**: Search fails for queries containing special characters (IP addresses, URLs, etc.)  
**ADR**: [ADR-001: FTS5 Query Sanitization](../adr/ADR-001-fts5-query-sanitization.md)  
**Status**: Ready for Implementation  
**Date**: 2026-01-07

## Overview

This document outlines the step-by-step implementation plan for fixing the FTS5 query sanitization issue that prevents users from searching for commands containing special characters like periods (in IP addresses), asterisks, and other FTS5 syntax elements.

## Problem Statement

Users cannot search for commands containing:
- IP addresses: `10.104.113.39`
- URLs: `https://example.com`
- File paths: `./path/to/file.txt`
- Email addresses: `user@domain.com`

Error received:
```
Error: Storage(SqliteFailure(Error { code: Unknown, extended_code: 1 }, 
Some("fts5: syntax error near \".\"")))
```

## Solution Summary

Implement a multi-layered approach:
1. **Primary**: Quote-wrap all queries for literal phrase matching
2. **Secondary**: Fall back to SQL LIKE if FTS5 fails

## Implementation Steps

### Step 1: Add Query Sanitization Helper Function

**File**: `src/storage.rs`

**Location**: Add new helper function in the `impl Storage` block

**Code to Add**:

```rust
/// Sanitizes a query string for FTS5 search by wrapping it in quotes
/// This treats the query as a literal phrase, preventing FTS5 syntax errors
/// for special characters like dots, asterisks, etc.
///
/// # Arguments
/// * `query` - The raw search query from the user
///
/// # Returns
/// A sanitized query string safe for FTS5 MATCH clause
///
/// # Examples
/// ```
/// let sanitized = sanitize_fts5_query("10.104.113.39");
/// assert_eq!(sanitized, "\"10.104.113.39\"");
///
/// let with_quotes = sanitize_fts5_query("grep \"pattern\"");
/// assert_eq!(with_quotes, "\"grep \"\"pattern\"\"\"");
/// ```
fn sanitize_fts5_query(query: &str) -> String {
    // Escape existing double quotes by doubling them (FTS5 standard)
    let escaped = query.replace("\"", "\"\"");
    
    // Wrap entire query in quotes for literal phrase search
    // This makes FTS5 treat all special characters as literals
    format!("\"{}\"", escaped)
}
```

**Rationale**:
- Wrapping in quotes makes FTS5 treat the query as a literal phrase
- Double-quote escaping follows FTS5 conventions
- Simple, fast O(n) string operation

### Step 2: Add LIKE Fallback Search Method

**File**: `src/storage.rs`

**Location**: Add new method in the `impl Storage` block

**Code to Add**:

```rust
/// Fallback search using SQL LIKE when FTS5 fails
/// This is slower but handles any character combination
///
/// # Arguments
/// * `text` - The search text
/// * `base_sql` - The base SQL query (without FTS5 clause)
/// * `base_params` - The base parameters for the query
///
/// # Returns
/// Vector of matching command records
fn search_with_like(
    &self,
    text: &str,
    category: &Option<String>,
    success_only: &Option<bool>,
    limit: usize,
) -> Result<Vec<CommandRecord>> {
    let mut sql = String::from(
        "SELECT id, command, timestamp, exit_code, duration_ms, working_dir,
                category, usage_count, last_used
         FROM commands
         WHERE command LIKE ?",
    );

    let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![
        Box::new(format!("%{}%", text))
    ];

    // Add category filter
    if let Some(ref cat) = category {
        sql.push_str(" AND category = ?");
        params.push(Box::new(cat.clone()));
    }

    // Add success filter
    if let Some(success) = success_only {
        if *success {
            sql.push_str(" AND exit_code = 0");
        } else {
            sql.push_str(" AND exit_code != 0");
        }
    }

    // Order by usage count and timestamp (no relevance ranking available)
    sql.push_str(" ORDER BY usage_count DESC, timestamp DESC");
    sql.push_str(&format!(" LIMIT {}", limit));

    let mut stmt = self.conn.prepare(&sql)?;
    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    
    let records = stmt
        .query_map(param_refs.as_slice(), |row| {
            Ok(CommandRecord {
                id: Some(row.get(0)?),
                command: row.get(1)?,
                timestamp: row.get::<_, String>(2)?.parse().unwrap(),
                exit_code: row.get(3)?,
                duration_ms: row.get(4)?,
                working_dir: row.get(5)?,
                category: row.get(6)?,
                usage_count: row.get(7)?,
                last_used: row.get::<_, String>(8)?.parse().unwrap(),
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(records)
}
```

**Rationale**:
- Provides guaranteed fallback when FTS5 fails
- Uses standard SQL LIKE which handles any characters
- Maintains same filtering logic (category, success_only)
- Accepts slight performance trade-off for reliability

### Step 3: Modify Main Search Method

**File**: `src/storage.rs`

**Location**: Modify existing `pub fn search()` method (around line 150)

**Current Code**:
```rust
// Add text search if provided
if let Some(ref text) = query.text {
    sql.push_str(" AND id IN (SELECT rowid FROM commands_fts WHERE command MATCH ?)");
    params.push(Box::new(text.clone()));
}
```

**Replace With**:
```rust
// Add text search if provided
if let Some(ref text) = query.text {
    // Sanitize query for FTS5 to handle special characters
    let sanitized = Self::sanitize_fts5_query(text);
    sql.push_str(" AND id IN (SELECT rowid FROM commands_fts WHERE command MATCH ?)");
    params.push(Box::new(sanitized));
}
```

**Additional Error Handling** (wrap the entire query execution):

```rust
// Execute the search with FTS5
let search_result = {
    let mut stmt = self.conn.prepare(&sql)?;
    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    
    stmt.query_map(param_refs.as_slice(), |row| {
        Ok(CommandRecord {
            id: Some(row.get(0)?),
            command: row.get(1)?,
            timestamp: row.get::<_, String>(2)?.parse().unwrap(),
            exit_code: row.get(3)?,
            duration_ms: row.get(4)?,
            working_dir: row.get(5)?,
            category: row.get(6)?,
            usage_count: row.get(7)?,
            last_used: row.get::<_, String>(8)?.parse().unwrap(),
        })
    })
};

// If FTS5 fails, fall back to LIKE search
let records = match search_result {
    Ok(rows) => rows.collect::<std::result::Result<Vec<_>, _>>()?,
    Err(_) if query.text.is_some() => {
        // FTS5 failed, fall back to LIKE search
        self.search_with_like(
            query.text.as_ref().unwrap(),
            &query.category,
            &query.success_only,
            query.limit,
        )?
    }
    Err(e) => return Err(e.into()),
};

Ok(records)
```

**Rationale**:
- Primary solution uses sanitized FTS5 (fast, indexed)
- Automatic fallback to LIKE if FTS5 still fails (rare edge cases)
- Transparent to caller - same interface

### Step 4: Add Unit Tests

**File**: `src/storage.rs` (in `#[cfg(test)]` module at end of file)

**Tests to Add**:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_test_db() -> (Storage, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let storage = Storage::new(&db_path).unwrap();
        (storage, temp_dir)
    }

    #[test]
    fn test_sanitize_fts5_query_simple() {
        let result = Storage::sanitize_fts5_query("hello world");
        assert_eq!(result, "\"hello world\"");
    }

    #[test]
    fn test_sanitize_fts5_query_with_dots() {
        let result = Storage::sanitize_fts5_query("10.104.113.39");
        assert_eq!(result, "\"10.104.113.39\"");
    }

    #[test]
    fn test_sanitize_fts5_query_with_quotes() {
        let result = Storage::sanitize_fts5_query("grep \"pattern\"");
        assert_eq!(result, "\"grep \"\"pattern\"\"\"");
    }

    #[test]
    fn test_sanitize_fts5_query_with_asterisk() {
        let result = Storage::sanitize_fts5_query("ls *.txt");
        assert_eq!(result, "\"ls *.txt\"");
    }

    #[test]
    fn test_sanitize_fts5_query_url() {
        let result = Storage::sanitize_fts5_query("https://example.com");
        assert_eq!(result, "\"https://example.com\"");
    }

    #[test]
    fn test_search_with_ip_address() {
        let (storage, _temp) = setup_test_db();
        
        // Insert a command with an IP address
        let record = CommandRecord {
            id: None,
            command: "ssh user@10.104.113.39".to_string(),
            timestamp: Utc::now(),
            exit_code: 0,
            duration_ms: 100,
            working_dir: "/home/user".to_string(),
            category: "network".to_string(),
            usage_count: 1,
            last_used: Utc::now(),
        };
        storage.save_command(&record).unwrap();

        // Search for the IP address
        let query = SearchQuery {
            text: Some("10.104.113.39".to_string()),
            category: None,
            success_only: None,
            limit: 10,
            order_by: OrderBy::Relevance,
        };

        let results = storage.search(&query).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].command.contains("10.104.113.39"));
    }

    #[test]
    fn test_search_with_url() {
        let (storage, _temp) = setup_test_db();
        
        let record = CommandRecord {
            id: None,
            command: "curl https://api.github.com/users/daneb".to_string(),
            timestamp: Utc::now(),
            exit_code: 0,
            duration_ms: 200,
            working_dir: "/home/user".to_string(),
            category: "network".to_string(),
            usage_count: 1,
            last_used: Utc::now(),
        };
        storage.save_command(&record).unwrap();

        let query = SearchQuery {
            text: Some("api.github.com".to_string()),
            category: None,
            success_only: None,
            limit: 10,
            order_by: OrderBy::Relevance,
        };

        let results = storage.search(&query).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_with_file_path() {
        let (storage, _temp) = setup_test_db();
        
        let record = CommandRecord {
            id: None,
            command: "cat ./config/settings.yaml".to_string(),
            timestamp: Utc::now(),
            exit_code: 0,
            duration_ms: 50,
            working_dir: "/home/user".to_string(),
            category: "file".to_string(),
            usage_count: 1,
            last_used: Utc::now(),
        };
        storage.save_command(&record).unwrap();

        let query = SearchQuery {
            text: Some("./config/settings.yaml".to_string()),
            category: None,
            success_only: None,
            limit: 10,
            order_by: OrderBy::Relevance,
        };

        let results = storage.search(&query).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_with_multiple_special_chars() {
        let (storage, _temp) = setup_test_db();
        
        let record = CommandRecord {
            id: None,
            command: "scp file.txt user@host.com:/path/to/dest".to_string(),
            timestamp: Utc::now(),
            exit_code: 0,
            duration_ms: 1500,
            working_dir: "/home/user".to_string(),
            category: "network".to_string(),
            usage_count: 1,
            last_used: Utc::now(),
        };
        storage.save_command(&record).unwrap();

        let query = SearchQuery {
            text: Some("user@host.com".to_string()),
            category: None,
            success_only: None,
            limit: 10,
            order_by: OrderBy::Relevance,
        };

        let results = storage.search(&query).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_empty_query_still_works() {
        let (storage, _temp) = setup_test_db();
        
        let record = CommandRecord {
            id: None,
            command: "ls -la".to_string(),
            timestamp: Utc::now(),
            exit_code: 0,
            duration_ms: 10,
            working_dir: "/home/user".to_string(),
            category: "file".to_string(),
            usage_count: 1,
            last_used: Utc::now(),
        };
        storage.save_command(&record).unwrap();

        // Search without text (should use other filters)
        let query = SearchQuery {
            text: None,
            category: Some("file".to_string()),
            success_only: None,
            limit: 10,
            order_by: OrderBy::Timestamp,
        };

        let results = storage.search(&query).unwrap();
        assert_eq!(results.len(), 1);
    }
}
```

### Step 5: Integration Testing

**Manual Test Cases**:

```bash
# Test 1: IP Address Search
omniscient search "10.104.113.39"
omniscient search 10.104.113.39

# Test 2: URL Search
omniscient search "https://api.github.com"
omniscient search api.github.com

# Test 3: File Path Search
omniscient search "./config/settings.yaml"
omniscient search ../parent/file.txt

# Test 4: Email Search
omniscient search "user@domain.com"

# Test 5: Version Number
omniscient search "v1.2.3"

# Test 6: Complex Command
omniscient search "grep \"pattern\" file.txt"

# Test 7: Normal Search (regression test)
omniscient search "git commit"
omniscient search "docker"

# Test 8: Asterisk
omniscient search "*.txt"

# Test 9: Multiple dots
omniscient search "file.name.with.dots.txt"

# Test 10: Colons
omniscient search "localhost:8080"
```

### Step 6: Performance Testing

**Benchmark Script** (add to `benches/` directory):

```rust
// benches/search_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use omniscient::{Storage, SearchQuery, OrderBy, CommandRecord};
use chrono::Utc;

fn benchmark_search(c: &mut Criterion) {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let db_path = temp_dir.path().join("bench.db");
    let storage = Storage::new(&db_path).unwrap();

    // Insert test data
    for i in 0..1000 {
        let record = CommandRecord {
            id: None,
            command: format!("test command {} with 10.104.113.{}", i, i % 255),
            timestamp: Utc::now(),
            exit_code: 0,
            duration_ms: 100,
            working_dir: "/home/user".to_string(),
            category: "test".to_string(),
            usage_count: i,
            last_used: Utc::now(),
        };
        storage.save_command(&record).unwrap();
    }

    c.bench_function("search_with_ip", |b| {
        b.iter(|| {
            let query = SearchQuery {
                text: Some(black_box("10.104.113.42".to_string())),
                category: None,
                success_only: None,
                limit: 10,
                order_by: OrderBy::Relevance,
            };
            storage.search(&query).unwrap()
        })
    });

    c.bench_function("search_normal", |b| {
        b.iter(|| {
            let query = SearchQuery {
                text: Some(black_box("command".to_string())),
                category: None,
                success_only: None,
                limit: 10,
                order_by: OrderBy::Relevance,
            };
            storage.search(&query).unwrap()
        })
    });
}

criterion_group!(benches, benchmark_search);
criterion_main!(benches);
```

**Performance Acceptance Criteria**:
- Search with sanitization should be < 5% slower than before
- Typical search should complete in < 100ms for 1000 records
- No memory leaks or excessive allocations

### Step 7: Documentation Updates

**File**: `README.md`

**Add Section** (after "Usage" section):

```markdown
### Search Tips

Omniscient's search is optimized for exact phrase matching. Here are some tips:

**Searching for commands with special characters:**
```bash
# IP addresses work perfectly
omniscient search "10.104.113.39"

# URLs are fully supported
omniscient search "https://api.github.com"

# File paths with dots
omniscient search "./config/settings.yaml"

# Email addresses
omniscient search "user@domain.com"
```

**Note**: All searches are treated as literal phrases, so searching for `git commit` 
will find commands containing that exact phrase, not commands with "git" OR "commit".
```

**File**: `docs/INDEX.md`

Add entry for ADR:
```markdown
- [ADR-001: FTS5 Query Sanitization](adr/ADR-001-fts5-query-sanitization.md)
```

### Step 8: Update CHANGELOG

**File**: `CHANGELOG.md`

**Add Entry**:

```markdown
## [1.0.1] - 2026-01-XX

### Fixed
- Fixed search failure when querying for commands containing special characters 
  (IP addresses, URLs, file paths with dots, etc.)
- Added automatic query sanitization for FTS5 full-text search
- Added LIKE fallback for edge cases where FTS5 still fails

### Changed
- All searches now use exact phrase matching for better accuracy
- Search queries are automatically quoted to handle special characters

### Internal
- Added comprehensive test suite for special character searches
- Implemented ADR-001 for FTS5 query sanitization strategy
```

## Validation Checklist

Before marking this task complete, verify:

- [ ] All unit tests pass (`cargo test`)
- [ ] New test cases for special characters pass
- [ ] Integration tests pass (manual testing with real commands)
- [ ] Performance benchmarks show acceptable performance (< 5% regression)
- [ ] No compiler warnings (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Documentation updated (README, CHANGELOG, ADR)
- [ ] Can search for IP addresses without errors
- [ ] Can search for URLs without errors
- [ ] Can search for file paths without errors
- [ ] Existing searches still work (regression test)
- [ ] LIKE fallback works when tested manually

## Rollback Plan

If issues arise post-implementation:

1. **Immediate**: Revert the changes to `src/storage.rs`
2. **Database**: No schema changes, so no migration needed
3. **Communication**: Notify users via GitHub issue
4. **Investigation**: Collect failed query examples
5. **Fix**: Adjust sanitization logic based on new edge cases

## Timeline Estimate

- **Step 1-3** (Core Implementation): 2-3 hours
- **Step 4** (Unit Tests): 2 hours
- **Step 5** (Integration Testing): 1 hour
- **Step 6** (Performance Testing): 1 hour
- **Step 7-8** (Documentation): 1 hour

**Total**: ~7-8 hours for complete implementation and testing

## Success Metrics

- ✅ Zero search failures for IP addresses
- ✅ Zero search failures for URLs
- ✅ Zero search failures for file paths
- ✅ < 5% performance regression
- ✅ All existing tests pass
- ✅ 10+ new test cases added and passing

## Next Steps

1. Create a feature branch: `git checkout -b fix/fts5-query-sanitization`
2. Follow implementation steps in order
3. Run tests after each step
4. Commit with descriptive messages
5. Create PR with reference to ADR-001
6. Request code review
7. Merge after approval and CI passes

## Questions & Concerns

**Q: What if the quoted query is too restrictive?**  
A: Future enhancement: Add `--exact` flag to control phrase matching

**Q: What about performance with large histories?**  
A: FTS5 index still used, quote wrapping is O(n) on query length (tiny)

**Q: Will this break existing user queries?**  
A: No - quoting makes searches more accurate, not less. Users benefit.

**Q: What if someone needs FTS5 advanced syntax?**  
A: Future enhancement: Detect FTS5 syntax patterns, skip sanitization

---

**Document Owner**: Engineering Team  
**Last Updated**: 2026-01-07  
**Status**: Ready for Implementation