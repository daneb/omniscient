---
id: PROD-0001
scope: repo
owner: human
verified_at: 2026-08-23
---

# Product

## Purpose

Omniscient captures every shell command you run, categorises it, and makes it
searchable — so a command you worked out once is never lost to a scrollback
buffer or a machine migration.

## Users and stakes

One developer, across many terminals and machines, over years. The failure that
matters is **silent loss**: a command that was never recorded cannot be missed
until it is needed, and by then the context that would have let you reconstruct
it is gone. Capture fidelity outranks every other property, including speed.

Second-order: capture runs on every prompt, so a slow or noisy capture makes the
shell unpleasant and gets the tool uninstalled.

## Out of scope

- Syncing as a service. Export/import and git are the sync story.
- Being a shell. Omniscient observes; it does not replace history, completion
  or the prompt.
- Recording anything the shell itself did not run.
