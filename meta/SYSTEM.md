# SYSTEM

## Architecture
- `main.rs` provides the CLI, parses arguments, and iterates over files.
- `lib.rs` exposes `process_file` and `process_lines`, classifies each file, and dispatches to a strategy.
- `src/strategies/` modules implement format-specific sorting and return reordered lines.

Data flows from the CLI into the library, through a chosen strategy, and back out as sorted text which is checked, diffed, or written depending on the mode.

## Inputs and Outputs
- Inputs: file paths or directories plus flags like `--mode`, `--features`, `--recursive`, and `--diff-command`.
- Outputs: rewritten files, diff output, or status codes signalling success or failure.

## Layout and Environment
- Source lives under `src/` with `main.rs`, `lib.rs`, and strategy modules.
- Tests reside in `tests/` for Rust units and `e2e-tests/` for Bats scripts.
- Documentation and guides sit in `docs/` and the project root.
- Rust 1.76 is pinned in `rust-toolchain.toml`; Cargo manages dependencies.
- Bats provides shell-based end-to-end testing.

## Design Decisions
- Strategy modules keep file-specific logic isolated.
- Experimental behaviours are gated behind features to avoid surprising users.
- The tool delegates directory traversal to callers to stay small and predictable.
- Comments are bound to following lines so annotations remain with their items after sorting.
