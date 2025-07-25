# Architecture

This document explains how the main pieces of the `keepsorted` crate fit together.

## Modules

### `strategies` directory

Each file under `src/strategies/` provides a `process` function that sorts lines for a particular file type:

- **`generic.rs`** – handles text blocks marked with `# Keep sorted`.
- **`bazel.rs`** – sorts lists in Bazel `BUILD`/`.bzl` files.
- **`cargo_toml.rs`** – sorts dependency tables in `Cargo.toml` files.
- **`gitignore.rs`** – sorts `.gitignore` or `CODEOWNERS` files when the feature is enabled.
- **`rust_derive.rs`** – reorders `#[derive(...)]` attributes; may also trigger a generic sort.

### CLI (`src/main.rs`)

`main.rs` implements the command‑line interface using `clap`. It parses arguments, selects the formatting mode (check, diff or fix) and passes files to `handle_file`. The helper `handle_file` runs the crate API on each file and applies the chosen mode.

### Crate API (`src/lib.rs`)

Library code exposes two key functions:

- `process_file` – reads a file, determines the correct strategy and returns the sorted content.
- `process_lines` – sorts an in‑memory list of lines using a chosen strategy.

Both functions rely on private helpers such as `classify` for strategy selection.

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

