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
- `docs/` – contributor guides (this folder).
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

## Documentation

- Keep `README.md`, especially the "Getting Started" section, in sync with the
  codebase and the specs in `docs/`.

## Code quality

- Evaluate contributions in this order: correctness, robustness, simplicity,
  clarity, performance, readability, maintainability, testability, test
  coverage, reusability.

## Development workflow

1. Create a branch and make changes.
1. Keep lists sorted and update tests and docs alongside code.
1. Documentation-only changes:
   - Run `mdformat` on updated files (recommended).
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

## Testing

- Maintain unit tests in `tests/` and end-to-end tests in `e2e-tests/`.
- Tests verify behaviour described in `docs/SPECS.md`, `docs/SYSTEM.md`, and
  this workflow.
- Bug fixes and new features require accompanying tests.
- CI and local runs must pass all tests before merging.

## Definition of done

A pull request is ready to merge when:

- Code and docs follow the conventions above.
- Tests cover new behaviour and all verify tasks pass.
- Documentation explains the feature and `SPECS.md` is updated if scope
  changes.
- The worktree is clean after running `./verify.sh`.
- User-visible changes are recorded in the CHANGELOG.

## CI

- GitHub Actions invokes `./verify.sh` for build, test, format and other jobs plus a final check.
- A PR title workflow enforces Conventional Commits.
- Using the prompt `prepare release vX.Y.Z` triggers a release PR named `chore(release): vX.Y.Z`; merges create tags and run the release workflow.

## Versioning & releases

- Use Semantic Versioning (`MAJOR.MINOR.PATCH`).
- Track version in `Cargo.toml`, changes in `CHANGELOG.md`.
- Group changelog entries by prefix: `feat`, `fix`, `refactor`, `test`, `docs`, `chore` (in that order). Keep original item order within each group.
- Add a summary at the top of each release:
  - **Patch (`x.y.Z`)**: 1–2 lines on key fixes or internal changes.
  - **Minor (`x.Y.0`)**: Short paragraph on main new features since last minor.
  - **Major (`X.0.0`)**: Summary of major changes and any breaking updates.
- Release PR:
  - Only updates `Cargo.toml` version, `CHANGELOG.md`, and `README.md` if needed.
  - Title: `chore(release): vX.Y.Z`
- After merge:
  - CI tags the release and runs final checks.

### Release procedure

1. Bump the `keepsorted` version in `Cargo.toml`.
1. Draft a GitHub release:
   - Identify the merge commit.
   - Go to Releases → **Draft a new release**.
   - Set:
     - Tag: `vX.Y.Z`
     - Target: the merge commit
     - Title: `vX.Y.Z`
     - Previous tag: last release
     - Notes: click **Generate release notes**, edit if needed
   - Summarize highlights in `CHANGELOG.md`.
   - Click **Publish release**.
1. Publish to crates.io:
   - Get a token at crates.io.
   - Authenticate:

     ```bash
     cargo login
     ```

   - Check out the release tag:

     ```bash
     git checkout vX.Y.Z
     ```

   - Publish the crate:

     ```bash
     cargo publish -p keepsorted
     ```
