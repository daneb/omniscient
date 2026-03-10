# ADR-003: UX Enhancements — Frecency Scoring, Colorized Output, Search Match Highlighting

## Status
Accepted

## Date
2026-03-10

## Context

Omniscient's search output was functional but plain. Several usability gaps were identified:

1. **Search ranking was suboptimal**: Results ordered by `usage_count DESC` gave high weight to old habits. A command run 50 times two years ago ranked above a command used 5 times this week, even though the recent one is more relevant to current work.

2. **Output was monochrome**: Distinguishing success (`✓`) from failure (`✗`), or quickly scanning command categories, required reading every word. Colour provides instant visual signals at no cognitive cost.

3. **Search results lacked context on the match**: When `omniscient search git` returned 20 results, each line showed the full command but gave no visual anchor for where the match occurred. For longer commands, users had to scan the full string manually.

These issues make the tool slower to use daily, despite its strong underlying data model.

## Decision

### 1. Frecency Scoring

Replace the `OrderBy::Relevance` SQL clause with a frecency formula:

```sql
ORDER BY
  CAST(usage_count AS REAL) / ((julianday('now') - julianday(last_used)) * 24.0 + 1.0) DESC,
  usage_count DESC
```

**Frecency** = frequency × recency. The divisor converts the gap between now and `last_used` into hours, then adds `1.0` to prevent division by zero for commands used within the last hour. A secondary sort on `usage_count` breaks ties in favour of more established commands.

This is a pure SQL expression evaluated at query time — no schema changes, no Rust-side arithmetic, no new columns. `OrderBy::UsageCount` (used by `top`) is deliberately left unchanged: that command exists specifically to rank by raw frequency.

### 2. Colorized Output

Add the `colored` crate (`v2.1`) and apply ANSI styling across all five command-displaying subcommands (Search, Here, Recent, Top, Category) and the Stats category list:

| Element | Style |
|---|---|
| Timestamps | Dimmed (grey) |
| `✓` success symbol | Green |
| `✗` failure symbol | Red |
| Usage counts (Top, Category) | Bold |
| Directories | Dimmed |
| Categories | Per-category colour (see below) |

Category colours are assigned statically by name to be consistent across all commands:

| Category | Colour |
|---|---|
| git | Cyan |
| docker | Blue |
| network | Magenta |
| file | Yellow |
| package | Bright green |
| database | Bright magenta |
| kubernetes | Bright blue |
| cloud | Bright cyan |
| system | Bright yellow |
| editor | White |
| build | Bright red |
| vcs | Bright white |
| other | Normal |

The `colored` crate automatically disables ANSI codes when `NO_COLOR` is set or when output is not a TTY (e.g. when piped to a file), so no manual detection is required.

### 3. Search Match Highlighting

In the `Search` subcommand, the matched portion of each command string is rendered **bold + underlined**:

```rust
fn highlight_match(text: &str, query: &str) -> String {
    let lower_text = text.to_lowercase();
    let lower_query = query.to_lowercase();
    match lower_text.find(&lower_query) {
        Some(start) => {
            let end = start + query.len();
            format!("{}{}{}", &text[..start], &text[start..end].bold().underline(), &text[end..])
        }
        None => text.to_string(),
    }
}
```

Matching is case-insensitive. Only the first occurrence is highlighted — sufficient for command-line history queries where the search term typically appears once. Other subcommands (Here, Recent, Top, Category) do not receive a query term, so highlighting is not applied there.

## Consequences

### Positive

- Search results now surface recently-used commands higher, matching daily workflow patterns
- Visual scan time is reduced: exit code, category, and timestamps are immediately distinguishable
- The matched term is visually anchored in long command strings
- No schema migration required — frecency is computed at query time
- `NO_COLOR` and pipe-detection are handled by the crate with no application code

### Negative

- `colored` adds a compile dependency (lightweight: no transitive deps beyond std)
- Frecency scoring changes the ordering of `omniscient search` results — users accustomed to the old ranking will see different output
- `highlight_match` uses byte-offset slicing, which is unsound for multi-byte Unicode characters that change byte length when lowercased (e.g. `ß` → `SS`). This is acceptable given the domain: shell commands are overwhelmingly ASCII. A robust fix would use `char_indices()` iteration.
- Only the first match occurrence is highlighted; multiple occurrences in a single command are not all marked

## Alternatives Considered

### Frecency: Compute in Rust instead of SQL

The score could be fetched as raw columns and sorted in Rust after the query. Rejected because it would require fetching more rows than the limit to sort correctly, and SQL-side ordering is cleaner and already supported by SQLite's `julianday()` functions.

### Frecency: Add a stored `frecency_score` column

A pre-computed column updated on every insert/increment would remove the runtime formula. Rejected because it requires a schema migration, adds write overhead, and the formula is cheap enough to compute at query time.

### Colors: `termcolor` or `owo-colors`

Both are viable. `colored` was chosen for its ergonomic trait-extension API (`.green()`, `.dimmed()`) which keeps display code readable inline. `owo-colors` has zero-cost no-color modes but adds more complexity for this use case.

### Highlighting: Highlight all occurrences

Splitting the string on all occurrences and wrapping each match adds complexity for marginal benefit. Shell commands rarely repeat the same token multiple times in a way that benefits from multi-highlight.

## References

- `src/storage.rs` — `search()` and `search_with_like()` ORDER BY clauses
- `src/main.rs` — `colorize_status()`, `colorize_category()`, `highlight_match()` helpers
- `Cargo.toml` — `colored = "2.1"` dependency
- [colored crate](https://crates.io/crates/colored)
- [NO_COLOR standard](https://no-color.org)
