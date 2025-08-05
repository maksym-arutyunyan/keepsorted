# WORKFLOW

Guides contributors through setup, development, testing, and releases.

## Environment setup

1. Install the Rust toolchain referenced in `rust-toolchain.toml` using
   `rustup`.
1. Install `mdformat` for Markdown formatting and `bats` for end-to-end tests.
1. Use isolated toolchains so dependencies do not leak between projects.

## Project structure

- `src/` – library and CLI sources.
- `tests/` – Rust unit tests.
- `e2e-tests/` – Bats-based end-to-end tests.
- `docs/` – user documentation.
- `meta/` – contributor guides (this folder).
- Temporary build output lives in `target/` and should not be committed.

## Coding conventions

- Rust code is formatted with `cargo fmt`; Clippy warnings are treated as
  errors.
- Lists and manifest sections annotated with `keepsorted` comments must remain
  sorted.
- Markdown is formatted with `mdformat`.
- File and directory names use `snake_case`.
- Commit messages and PR titles follow Conventional Commits.
- Generated files and build artifacts are excluded from commits.

## Development workflow

1. Create a branch and make changes.
1. Keep lists sorted and update tests and docs alongside code.
1. Documentation-only changes:
   - Run `mdformat` on updated files.
1. Rust or manifest changes:
   - Run each task in `verify.sh` manually:
     - `cargo build --release --all-targets` – ensures the code compiles.
     - `cargo test` and `cargo test --release` – verify behaviour.
     - `cargo clippy --all-targets -- -D warnings` – enforce idiomatic Rust.
     - `cargo fmt --all -- --check` – maintain consistent formatting.
     - `keepsorted` – check that annotated lists stay ordered.
     - `bats e2e-tests` – exercise the CLI end to end.
   - Run `./verify.sh` to ensure the tasks succeed and no files were modified.
   - Use `git diff --exit-code` to confirm a clean working tree.
1. Update `docs/` and `CHANGELOG.md` for user-facing changes.
1. Commit with a Conventional Commit message.

## Definition of done

A pull request is ready to merge when:

- Code and docs follow the conventions above.
- Tests cover new behaviour and all verify tasks pass.
- Documentation explains the feature and `SPECS.md` is updated if scope
  changes.
- The worktree is clean after running `./verify.sh`.
- User-visible changes are recorded in the CHANGELOG.

## Continuous integration

GitHub Actions runs `verify.sh` tasks and `mdformat --check` so local runs
match CI.

## Release process

1. Comment `prepare release vX.Y.Z` on the main branch.
1. A bot opens `chore(release): vX.Y.Z`.
1. Merge the PR to tag the commit and publish the release.
