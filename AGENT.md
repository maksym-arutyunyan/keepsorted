# Instructions for AI Agent

## PR Naming
- Name GitHub pull requests using the **Conventional Commits** specification. Example: `feat(parser): support new syntax` or `fix(ci): correct clippy invocation`.

## Validation Steps
- Before submitting a PR, run each command from `./run-all.sh` manually. It may
  include:
  - `cargo build --release --all-targets`
  - `cargo test`
  - `cargo test --release`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo fmt --all -- --check`
  - running `keepsorted` over tracked files
- After running the individual commands, execute `./run-all.sh` to confirm that
  no further changes are introduced and that all steps succeed. Always consult
  the script for updates.
  - The PR fails if any command fails or if `git diff` shows changes

## Best Practices
- Use the Rust version pinned in `rust-toolchain.toml`.
- Always format code using `cargo fmt`.
- Address warnings reported by `cargo clippy` as invoked in `run-all.sh`.
- Keep entries in `Cargo.toml` and other marked blocks sorted using `keepsorted` comments.
- Ensure code changes adhere to `docs/architecture.md`, which is the source of truth. Modify this file only when explicitly instructed by a human.
- Update `CHANGELOG.md` for user‑visible changes following the Keep a Changelog format.
- Commit messages should also follow the Conventional Commits style.
