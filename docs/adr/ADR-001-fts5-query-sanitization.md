# ADR-001: FTS5 Query Sanitization for Special Characters

## Status
Accepted

## Date
2026-01-07

## Context

Omniscient uses SQLite's FTS5 (Full-Text Search 5) virtual table for fast command searching. Users have reported search failures when querying for commands containing special characters, specifically IP addresses.

### The Problem

When users attempt to search for commands containing IP addresses or other special characters:

```bash
omniscient search "10.104.113.39"
# Error: Storage(SqliteFailure(Error { code: Unknown, extended_code: 1 }, 
# Some("fts5: syntax error near \".\"")))

omniscient search 10.104.113.39
# Error: Storage(SqliteFailure(Error { code: Unknown, extended_code: 1 }, 
# Some("fts5: syntax error near \".\"")))
```

### Root Cause

FTS5 has its own query syntax where certain characters have special meaning:
- `.` (period) - Used for column specification in multi-column searches
- `*` (asterisk) - Prefix matching wildcard
- `"` (quotes) - Phrase search delimiter
- `-` (minus) - Exclusion operator
- `+` (plus) - Required term operator
- `(` `)` - Grouping operators
- `:` (colon) - Column prefix operator

When these characters appear in search queries without proper escaping, FTS5 interprets them as syntax elements rather than literal characters, causing syntax errors.

### Current Implementation

In `src/storage.rs`, the search query is passed directly to FTS5 MATCH without sanitization:

```rust
if let Some(ref text) = query.text {
    sql.push_str(" AND id IN (SELECT rowid FROM commands_fts WHERE command MATCH ?)");
    params.push(Box::new(text.clone()));
}
```

### Impact

This affects searches for:
- IP addresses: `10.104.113.39`, `192.168.1.1`
- URLs: `https://example.com`, `api.github.com`
- File paths: `./path/to/file.txt`, `../config.yml`
- Email addresses: `user@domain.com`
- Commands with special syntax: `grep "pattern" file.txt`
- Version numbers: `v1.2.3`

## Decision

We will implement a **multi-layered query sanitization strategy** that balances search accuracy, performance, and robustness:

### Primary Solution: Quote-Based Phrase Search

Wrap user queries in double quotes to treat them as literal phrase searches. This is the simplest and most effective solution for most cases:

```rust
fn sanitize_fts5_query(query: &str) -> String {
    // Escape any existing double quotes by doubling them
    let escaped = query.replace("\"", "\"\"");
    // Wrap in quotes for literal phrase search
    format!("\"{}\"", escaped)
}
```

**Why this works:**
- FTS5 treats quoted strings as literal phrases
- Special characters inside quotes are interpreted literally
- Maintains search accuracy for exact matches
- Simple implementation with minimal overhead

### Secondary Solution: LIKE Fallback

If the FTS5 query still fails (edge cases), fall back to a standard SQL LIKE query:

```rust
// In search() method
let results = if let Some(ref text) = query.text {
    match self.search_with_fts5(text, &sql, &params) {
        Ok(results) => results,
        Err(_) => {
            // Fallback to LIKE query for problematic patterns
            self.search_with_like(text, &sql, &params)?
        }
    }
} else {
    // No text search, proceed with regular query
    self.execute_search(&sql, &params)?
};
```

**Why we need this:**
- Handles extreme edge cases where even quoted searches might fail
- Ensures search always succeeds, even if slower
- Better user experience than returning an error

### Implementation Details

1. **Add helper function** `sanitize_fts5_query()` to escape and quote queries
2. **Modify search method** to use sanitized queries
3. **Add fallback mechanism** for LIKE-based search when FTS5 fails
4. **Preserve existing behavior** for category, success_only filters
5. **Maintain performance** - quote wrapping is O(n) string operation

## Consequences

### Positive

✅ **Fixes the immediate issue**: Users can search for IP addresses and special characters
✅ **Maintains search accuracy**: Quoted phrases match exactly as expected
✅ **Minimal performance impact**: String escaping is fast, FTS5 still used
✅ **Robust fallback**: LIKE ensures search never fails completely
✅ **Simple implementation**: ~50 lines of code, easy to understand and maintain
✅ **No schema changes**: Works with existing database structure
✅ **Backward compatible**: Existing searches continue to work

### Negative

⚠️ **Different search semantics**: All searches become phrase searches
  - **Mitigation**: This is actually more intuitive for most users
  - Users can still search for partial matches, just as exact substrings

⚠️ **No advanced FTS5 features**: Users can't use `-term` or `term*` syntax
  - **Mitigation**: Most users don't need advanced syntax
  - Future enhancement: Detect if query uses FTS5 syntax, skip sanitization

⚠️ **LIKE fallback is slower**: O(n) scan vs FTS5 index
  - **Mitigation**: Only used when FTS5 fails (rare)
  - Still acceptable for command history sizes (<100k records)

### Trade-offs

| Aspect | Before | After |
|--------|--------|-------|
| IP addresses | ❌ Fails | ✅ Works |
| URLs | ❌ Fails | ✅ Works |
| Simple queries | ✅ Works | ✅ Works |
| Advanced FTS5 syntax | ✅ Works | ⚠️ Limited |
| Performance | ⚡ Fast | ⚡ Fast (slight overhead) |

## Alternatives Considered

### Alternative 1: Character-by-Character Escaping

**Approach**: Escape each special character individually
```rust
fn escape_fts5(query: &str) -> String {
    query.replace(".", "\\.")
         .replace("*", "\\*")
         .replace("\"", "\\\"")
    // ... etc
}
```

**Why rejected**:
- ❌ FTS5 doesn't support backslash escaping consistently
- ❌ More complex to implement and maintain
- ❌ Easy to miss special characters
- ❌ Doesn't handle all edge cases

### Alternative 2: Disable FTS5, Use LIKE Only

**Approach**: Remove FTS5 entirely, use SQL LIKE for all searches

**Why rejected**:
- ❌ Significant performance degradation
- ❌ No relevance ranking
- ❌ Loses major feature of the project
- ❌ Poor user experience for large histories

### Alternative 3: Parse and Validate FTS5 Syntax

**Approach**: Build a full FTS5 query parser to validate and fix queries

**Why rejected**:
- ❌ Complex implementation (would need a lexer/parser)
- ❌ Duplicates FTS5's internal parser logic
- ❌ Maintenance burden
- ❌ Overkill for the problem

### Alternative 4: Allow Raw Queries with Flag

**Approach**: Add `--raw` flag to allow advanced users to use FTS5 syntax

**Why rejected for now**:
- ⚠️ Adds complexity for minimal benefit
- ⚠️ Most users don't need advanced syntax
- ✅ **Possible future enhancement** if demand exists

## Implementation Plan

### Phase 1: Core Implementation
1. Add `sanitize_fts5_query()` helper function
2. Modify `search()` method to use sanitization
3. Add comprehensive error handling

### Phase 2: Fallback Mechanism
1. Implement `search_with_fts5()` method
2. Implement `search_with_like()` fallback method
3. Add retry logic in main `search()` method

### Phase 3: Testing
1. Unit tests for `sanitize_fts5_query()`
   - IP addresses
   - URLs
   - File paths
   - Email addresses
   - Quoted strings
   - Mixed special characters
2. Integration tests for search functionality
3. Performance benchmarks (ensure no regression)

### Phase 4: Documentation
1. Update README with search behavior notes
2. Add troubleshooting section
3. Document limitations (no advanced FTS5 syntax)

## Testing Strategy

### Test Cases

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_search_ip_address() {
        // Search for "10.104.113.39"
    }
    
    #[test]
    fn test_search_url() {
        // Search for "https://example.com"
    }
    
    #[test]
    fn test_search_file_path() {
        // Search for "./path/to/file.txt"
    }
    
    #[test]
    fn test_search_with_quotes() {
        // Search for 'grep "pattern" file.txt'
    }
    
    #[test]
    fn test_sanitization_escapes_quotes() {
        // Verify quote doubling
    }
}
```

### Validation Criteria

- ✅ All existing tests pass
- ✅ New tests for special characters pass
- ✅ Performance benchmarks show <5% regression
- ✅ Manual testing with real-world queries succeeds

## References

- **Issue**: User reported search failure with IP addresses
- **SQLite FTS5 Documentation**: https://www.sqlite.org/fts5.html
- **FTS5 Query Syntax**: https://www.sqlite.org/fts5.html#full_text_query_syntax
- **Related Code**: `src/storage.rs` lines 150-220

## Future Enhancements

1. **Advanced Query Mode**: Add `--advanced` flag for users who want FTS5 syntax
2. **Query Suggestions**: Detect failed queries and suggest corrections
3. **Smart Detection**: Detect if query uses FTS5 syntax, skip sanitization
4. **Fuzzy Matching**: Add optional fuzzy/approximate search
5. **Search History**: Track failed searches to improve sanitization

## Notes

This ADR represents a pragmatic solution that prioritizes:
1. **User experience**: Search should "just work" for common cases
2. **Simplicity**: Easy to implement, test, and maintain
3. **Robustness**: Graceful degradation with fallback
4. **Performance**: Minimal impact on search speed

The solution can be enhanced in the future based on user feedback and usage patterns.