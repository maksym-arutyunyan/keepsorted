# WORKFLOW

## Environment Setup
- Install the Rust toolchain from `rust-toolchain.toml` using `rustup`.
- Use isolated environments so tool versions do not bleed into other projects.

## Dependencies and Tools
- Manage libraries with Cargo and enable optional behaviour with feature flags.
- End-to-end tests require the `bats` runner.
- `verify.sh` sequences build, test, lint, format, sorting, and diff checks.

## Local Checks
For Rust or manifest changes run:
```bash
cargo build --release --all-targets
cargo test
cargo test --release
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
./target/release/keepsorted --features gitignore,rust_derive_canonical $(git ls-files -z | grep -vzE '^tests/|^e2e-tests/|^README.md$' | xargs -0 -n1)
bats e2e-tests
./verify.sh
```
Markdown-only edits may skip these steps.

## Continuous Integration
GitHub Actions runs separate jobs for build, test, clippy, rustfmt, keepsorted, end-to-end tests, and ShellCheck across Linux and macOS.

## Versioning and Releases
- Follow Semantic Versioning and maintain `CHANGELOG.md` in Keep a Changelog format.
- Add entries to the "Unreleased" section for user-facing changes.
- Release flow: bump version in `Cargo.toml`, create a GitHub release tag with notes, then publish to crates.io (see `development-guide.md`).
