# WORKFLOW

## Environment Setup

- Install the Rust toolchain from `rust-toolchain.toml` with `rustup`.
- Use isolated environments so tool versions do not bleed into other projects.

## Local development

- Format Markdown files with `mdformat`.
- Run `./verify.sh` before committing. Without arguments it runs every check, or pass task names to limit the run.

### verify.sh tasks

| Argument | Runs | CI job |
| --- | --- | --- |
| `build` | `cargo build --release --all-targets` | build |
| `test` | `cargo test` | test |
| `test-release` | `cargo test --release` | test-release |
| `clippy` | `cargo clippy --all-targets -- -D warnings` | lint |
| `fmt` | `cargo fmt --all -- --check` | format |
| `keepsorted` | run `keepsorted` on tracked files | sort |
| `diff` | `git diff --exit-code` | diff |
| `e2e` | `bats e2e-tests` | e2e |
| `all` | all tasks above (default) | combined |

Documentation-only changes may run `mdformat` and skip `verify.sh`.

## Continuous Integration

- GitHub Actions invokes `verify.sh` for build, test, lint, format, sort, diff, and end-to-end jobs so local runs match CI.
- A dedicated job executes `mdformat --check` to enforce Markdown formatting.
- PR titles must follow the Conventional Commits specification.

## Release automation

- Comment `prepare release vX.Y.Z` to begin a release.
- The workflow opens a PR titled `chore(release): vX.Y.Z`; merging it tags the commit and runs the release pipeline automatically.

## Training Module

1. Gather historical data and define the parameter ranges to explore.
1. Optionally override the default four-month test window.
1. Execute the training module to backtest each configuration on the train
   split and score the winner on the held-out test data.
1. Inspect the generated summary file to compare metrics like return on
   investment or Sharpe ratio and choose a configuration.

Runs are deterministic, so repeating the process with the same inputs yields
identical summaries.
