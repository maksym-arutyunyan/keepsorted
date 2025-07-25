# Architecture

This document explains how the main pieces of the `keepsorted` crate fit together.

## Inspiration

`keepsorted` was inspired by [Buildifier](https://github.com/bazelbuild/buildtools/tree/master/buildifier), which sorts items in Bazel `BUILD` files. The command-line flags and exit codes follow a similar design so that tooling can integrate either tool with minimal changes.

The CLI returns these codes:
1. `0` — success
2. `1` — syntax errors in input
3. `2` — incorrect command usage
4. `3` — unexpected runtime failures
5. `4` — check mode detected unsorted files

## Sorting Behaviour

The core feature is sorting lines while preserving any comments associated with each item. Multi-line items (for example in `Cargo.toml`) are kept intact when possible so that sections remain readable.

Some experimental features exist:
- **Rust derive sorting** temporarily supports alphabetical or canonical ordering of `#[derive(...)]` attributes because `cargo fmt` does not yet implement this. The functionality is intentionally basic and may be removed once rustfmt provides a stable implementation.
- **Gitignore and CODEOWNERS sorting** helps maintain consistent ordering but should be used carefully since pattern order can affect semantics.

## Modules

### `strategies` directory

Each file under `src/strategies/` provides a `process` function that sorts lines for a particular file type:

- **`generic.rs`** – handles text blocks marked with `# Keep sorted`.
- **`bazel.rs`** – sorts lists in Bazel `BUILD`/`.bzl` files.
- **`cargo_toml.rs`** – sorts dependency tables in `Cargo.toml` files.
- **`gitignore.rs`** – sorts `.gitignore` or `CODEOWNERS` files when the feature is enabled.
- **`rust_derive.rs`** – reorders `#[derive(...)]` attributes; may also trigger a generic sort.

### CLI (`src/main.rs`)

`main.rs` implements the command-line interface using `clap`. It parses arguments, selects the formatting mode (check, diff or fix) and passes a single file to `handle_file`. Directory traversal is intentionally left to external scripts so that the binary stays simple and composable. The helper `handle_file` runs the crate API on each file and applies the chosen mode.

### Crate API (`src/lib.rs`)

Library code exposes two key functions:

- `process_file` – reads a file, determines the correct strategy and returns the sorted content.
- `process_lines` – sorts an in-memory list of lines using a chosen strategy.

Both functions rely on private helpers such as `classify` for strategy selection.

## Testing Strategy

End-to-end tests in `e2e-tests` invoke the CLI using a command-line test framework to simulate real usage. Each flag or parameter has its own test file with descriptive names following the Arrange–Act–Assert style. This keeps tests short and focused while covering many combinations.

Rust unit tests inside `tests/` verify the behaviour of individual strategies for different file and data types.

## Key functions

| Function | Location | Responsibility |
| -------- | -------- | -------------- |
| `process_file` | `src/lib.rs` | Load a file and return its sorted contents. |
| `process_lines` | `src/lib.rs` | Apply a sorting strategy to lines. |
| `classify` | `src/lib.rs` | Identify the strategy for a path and feature set. |
| `handle_file` | `src/main.rs` | CLI helper that checks, diffs or rewrites a file. |
| `generic::process` | `src/strategies/generic.rs` | Sort `# Keep sorted` blocks in text files. |
| `bazel::process` | `src/strategies/bazel.rs` | Sort string lists in Bazel files. |
| `cargo_toml::process` | `src/strategies/cargo_toml.rs` | Sort dependency sections in `Cargo.toml`. |
| `gitignore::process` | `src/strategies/gitignore.rs` | Sort `.gitignore` and `CODEOWNERS` files. |
| `rust_derive::process` | `src/strategies/rust_derive.rs` | Reorder traits inside `#[derive]` attributes. |
