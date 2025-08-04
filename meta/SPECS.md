# SPECS

## Problem
Unsorted lists in source and config files are tedious to maintain and can detach comments from the items they describe.

## Audience
Developers and CI systems that need deterministic ordering while keeping human commentary intact.

## Solution Overview
`keepsorted` scans files, detects blocks marked for sorting or known file types, reorders items, and either rewrites files or reports required changes.

## Key Modules
- **CLI** parses flags and selects mode: check, diff, or fix.
- **Core library** reads files, classifies them, and returns sorted content.
- **Strategies** handle specific formats such as generic text, Bazel, Cargo.toml, gitignore/CODEOWNERS, and Rust `#[derive]` attributes.
- **Optional features** enable additional strategies via command-line flags or crate features.
- **Tests** include Rust unit tests and Bats end-to-end scripts.
