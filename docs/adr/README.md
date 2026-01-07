# Architecture Decision Records (ADR)

This directory contains Architecture Decision Records for the Omniscient project.

## What is an ADR?

An Architecture Decision Record (ADR) captures an important architectural decision made along with its context and consequences.

## Format

Each ADR follows this structure:

```markdown
# ADR-XXX: [Title]

## Status
[Proposed | Accepted | Deprecated | Superseded]

## Context
What is the issue we're seeing that is motivating this decision or change?

## Decision
What is the change that we're proposing and/or doing?

## Consequences
What becomes easier or more difficult to do because of this change?

## Alternatives Considered
What other options were considered and why were they not chosen?

## References
Links to related issues, PRs, documentation, etc.
```

## Naming Convention

ADRs are numbered sequentially and use the following naming pattern:
- `ADR-001-title-in-kebab-case.md`
- `ADR-002-another-decision.md`

## List of ADRs

| Number | Title | Status | Date |
|--------|-------|--------|------|
| [001](ADR-001-fts5-query-sanitization.md) | FTS5 Query Sanitization for Special Characters | Accepted | 2026-01-07 |

## When to Create an ADR

Create an ADR when:
- Making significant architectural or design decisions
- Choosing between multiple technical approaches
- Changing existing architectural decisions
- Establishing new patterns or conventions
- Making decisions that will impact future development

## Process

1. Copy the template above
2. Fill in the sections
3. Create a PR for review
4. Update the status when decision is made
5. Add to the table above