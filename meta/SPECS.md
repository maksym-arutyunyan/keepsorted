# SPECS

Describes the product scope of `keepsorted` and what "done" means.

## Problem

Unsorted lists in source and config files are tedious to maintain and can
detach comments from the items they describe.

## Audience

Developers and CI systems that need deterministic ordering while keeping human
commentary intact.

## Goals

- Detect annotated blocks or supported file types that require sorting.
- Reorder items deterministically while preserving the comments bound to each
  item.
- Offer check, diff, and fix modes so projects can enforce or apply sorting.

## Non-goals

- Custom sort orders for individual projects or derive macros.
- Automatic discovery of files beyond paths explicitly provided.
- Modifying unrelated formatting or comments.

## Acceptance criteria

A format or feature is complete when:

- Annotated blocks are detected and sorted idempotently.
- Comments remain attached to the intended line.
- Output is stable across runs and platforms.
- `--mode=check` exits with a non-zero status for unsorted input.
- `--mode=fix` rewrites files in place; `--mode=diff` prints a unified diff.
- Tests cover typical usage and edge cases.
- User-facing behaviour and limitations are documented.
