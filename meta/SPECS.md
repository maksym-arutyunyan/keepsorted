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

## Sorting behaviour

- Groups consecutive comment lines with the following item and preserves that
  association after sorting.
- Directives:
  - `# keepsorted: ignore file` — skip sorting for the entire file.
  - `# keepsorted: ignore block` — skip the annotated block within a sorted
    section.

## Exit codes

- `0` — success.
- `1` — syntax errors in input.
- `2` — incorrect command usage.
- `3` — unexpected runtime failures.
- `4` — check mode detected unsorted files.

## Experimental features

Disabled by default and enabled via `--features` or crate features:

- `rust_derive_alphabetical` — sorts `#[derive(...)]` attributes alphabetically.
- `rust_derive_canonical` — sorts `#[derive(...)]` attributes in canonical
  Rust order.
- `gitignore` — sorts `.gitignore` files.
- `codeowners` — sorts `CODEOWNERS` files.

## Acceptance criteria

A format or feature is complete when:

- Annotated blocks are detected and sorted idempotently.
- Comments remain attached to the intended line.
- Output is stable across runs and platforms.
- `--mode=check` exits with a non-zero status for unsorted input.
- `--mode=fix` rewrites files in place; `--mode=diff` prints a unified diff.
- Tests cover typical usage and edge cases.
- User-facing behaviour and limitations are documented.
