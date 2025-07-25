# Architecture

This document explains how the main pieces of the `keepsorted` crate fit together.

## Inspiration

`keepsorted` was inspired by [Buildifier](https://github.com/bazelbuild/buildtools/tree/master/buildifier), which sorts items in Bazel `BUILD` files. The command-line flags and exit codes follow a similar design so that tooling can integrate either tool with minimal changes.

The CLI returns these codes:
- `0` — success
- `1` — syntax errors in input
- `2` — incorrect command usage
- `3` — unexpected runtime failures
- `4` — check mode detected unsorted files

## Sorting Behaviour

The core feature is sorting lines while preserving any comments associated with each item. Multi-line items (for example in `Cargo.toml`) are kept intact when possible so that sections remain readable.

Sorting can be skipped with two special directives:
- **`# keepsorted: ignore file`** anywhere in a file leaves the entire file unchanged.
- **`# keepsorted: ignore block`** inside a `# Keep sorted` block preserves that block without reordering.

Some experimental features exist:

- **Rust derive sorting** temporarily supports alphabetical or canonical ordering of `#[derive(...)]` attributes because `cargo fmt` does not yet implement this. The functionality is intentionally basic and may be removed once rustfmt provides a stable implementation.
- **Gitignore and CODEOWNERS sorting** helps maintain consistent ordering but should be used carefully since pattern order can affect semantics.

## Experimental Features

The following features are behind flags and must be enabled explicitly using `--features`:

- `rust_derive_alphabetical` — sorts `#[derive(...)]` attributes alphabetically
- `rust_derive_canonical` — sorts `#[derive(...)]` attributes in canonical Rust order
- `gitignore` — enables sorting logic for `.gitignore` files
- `codeowners` — enables sorting logic for `CODEOWNERS` files

## Modules

### `strategies` directory

Each file under `src/strategies/` provides a `process` function that sorts lines for a particular file type:

- **`generic.rs`** – handles text blocks marked with `# Keep sorted`.
- **`bazel.rs`** – sorts lists in Bazel `BUILD`/`.bzl` files.
- **`cargo_toml.rs`** – sorts dependency tables in `Cargo.toml` files.
- **`gitignore.rs`** – sorts `.gitignore` or `CODEOWNERS` files when the feature is enabled.
- **`rust_derive.rs`** – reorders `#[derive(...)]` attributes; may also trigger a generic sort.

### CLI (`src/main.rs`)

`main.rs` implements the command-line interface using `clap`. It parses arguments, selects the formatting mode (check, diff or fix) and passes a single file to `handle_file`. Directory traversal is intentionally left to external scripts so that the binary stays simple and composable. There is deliberately no `-r` or `--recursive` option; use tools like `git ls-files` if you need to process multiple files. The helper `handle_file` runs the crate API on each file and applies the chosen mode.

`keepsorted` focuses on sorting and does not try to walk directories itself. Implementing a fully featured crawler would require handling ignore files, generated sources and other project-specific rules. Existing tools already solve these problems, so the CLI expects callers to provide an explicit list of files. This design keeps the binary small while letting users combine it with powerful shell filters.

### Crate API (`src/lib.rs`)

Library code exposes two key functions:

- `process_file` – reads a file, determines the correct strategy and returns the sorted content.
- `process_lines` – sorts an in-memory list of lines using a chosen strategy.

Both functions rely on private helpers such as `classify` for strategy selection.

## File Processing Flow

1. CLI parses args and opens the file
2. `process_file` calls `classify` to select the strategy
3. The strategy module processes lines via `process_lines`
4. The result is checked, diffed, or written depending on the mode

## Testing Strategy

End-to-end tests in `e2e-tests` invoke the CLI using a command-line test framework to simulate real usage. Each flag or parameter has its own test file with descriptive names following the Arrange–Act–Assert style. This keeps tests short and focused while covering many combinations.

Tests use real input files and golden outputs. Each test case includes input.txt and expected.txt stored in the same subdirectory for clarity and simplicity.

Rust unit tests inside `tests/` verify the behaviour of individual strategies for different file and data types.

## Out of Scope

To stay focused and composable, `keepsorted` does **not** aim to:

- Perform recursive directory traversal (use external tools like `find`, `git ls-files`, or CI filters)
- Act as a full-fledged parser for every supported file type
- Handle ignore files or exclude paths automatically
- Automatically detect project structure or configuration files
- Replace formatting tools like `rustfmt` or `prettier`
