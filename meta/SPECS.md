# SPECS

Defines the expected behavior of `keepsorted`.

## Problem

Unsorted lists in source and config files are tedious to maintain and can detach comments from the items they describe.

## Audience

Developers and CI systems that need deterministic ordering while keeping human commentary intact.

## Goals

- Detect annotated blocks or supported file types that require sorting.
- Reorder items deterministically while preserving associated comments.
- Offer check, diff, and fix modes so projects can enforce or apply sorting.

## Out of scope

Custom sort orders for Rust derive macros.
Derive attributes are either alphabetized or use canonical order.
