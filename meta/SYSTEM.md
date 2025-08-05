# SYSTEM

Explains how `keepsorted` is structured and how its components interact.

## Overview

`keepsorted` consists of a command-line interface backed by a library of
sorting strategies.

## Components

- **CLI (`src/main.rs`)** – parses arguments with `clap`, iterates over files,
  supports `--recursive` and `--diff-command`, and maps library results to exit
  codes.
- **Library (`src/lib.rs`)** – exposes `process_file` and `process_lines`,
  selects a strategy for each file, and applies the requested mode.
- **Strategies (`src/strategies/*`)** – implement format-specific sorting:
  - `generic` – handles text blocks marked with `# Keep sorted`.
  - `bazel` – sorts lists in Bazel `BUILD`/`.bzl` files.
  - `cargo_toml` – sorts dependency tables in `Cargo.toml`.
  - `gitignore` – sorts `.gitignore` and `CODEOWNERS` files when enabled.
  - `rust_derive` – reorders `#[derive(...)]` attributes.
- **Utilities** – helpers for comment binding, diff generation, and feature
  gating.

## Data flow

1. The CLI gathers file paths and options from the user.
1. `process_file` classifies each path and selects a strategy.
1. The strategy reorders lines and preserves associated comments.
1. The library applies `check`, `diff`, or `fix` and returns a result.
1. The CLI writes diffs or files and exits with success or failure.

## Responsibilities

- **CLI** – user interface, argument parsing, and delegating traversal to
  callers.
- **Library** – orchestration, error handling, and feature flags.
- **Strategies** – canonical ordering rules and comment preservation.
- **Tests** – unit tests in `tests/` and end-to-end cases in
  `e2e-tests/`.

## Inputs and outputs

- **Inputs:** file paths or directories and flags such as `--mode`,
  `--features`, `--recursive`, and `--diff-command`.
- **Outputs:** rewritten files, diff output, and exit codes.

## Environment and layout

- Source under `src/`.
- Unit tests under `tests/`.
- End-to-end tests using Bats in `e2e-tests/`.
- Documentation in `docs/` and contributor guides in `meta/`.
- Rust version pinned in `rust-toolchain.toml`; Cargo manages dependencies.

## Design notes

- Strategy modules isolate format-specific logic so new formats are
  self-contained.
- Experimental behaviour is gated behind features.
- Comments bind to the following line so annotations stay with their items
  after sorting.
- Directory traversal is delegated to the caller for predictability.
