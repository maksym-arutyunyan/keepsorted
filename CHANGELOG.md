# Changelog

All notable changes to this project are documented here.  
This changelog follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

Adds quiet mode and skips binary files during recursive checks. Also drafts new meta documentation for contributors and AI models.

- feat: skip binary files during recursive traversal
- feat: add `--quiet` flag to suppress informational output
- fix: correctly parse comments in `rust_derive` when string literals contain `//`
- fix: print stderr output when diff command fails
- docs: draft meta guidelines for AI and contributors

## v0.1.6 – 2025-07-26

Adds recursive traversal, new options, and better CI exit codes. Expands documentation and fixes CLI errors.

- feat: recursive directory traversal via `-r` / `--recursive`
- feat: add `--mode` and `--diff-command` options
- feat: add exit code `4` for failed diff checks
- fix: correct exit codes and usage errors
- docs: expand README and help text

## v0.1.5 – 2025-07-15

Adds `--check` mode for CI and pre-commit use. Improves crate documentation and updates README.

- feat: add `--check` mode to lint command
- docs: improve crate-level documentation
- docs: update README with badges

## v0.1.4 – 2025-07-14

Fixes Bazel file detection and improves comment support. Cleans up documentation and sorting logic.

- fix: correct Bazel file detection
- fix: support Lua-style `--` comments for ignore directives
- docs: fix link in the development guide
- docs: fix numbering in the publish guide
- chore: add license file
- chore: correct alphabetical sort function typo

## v0.1.3 – 2025-07-10

Adds Lua file support and improves interaction with Rust generics. Simplifies CI configuration.

- feat: add support for Lua files
- fix: improve `rust_derive` and generic sorting interaction
- chore: simplify workflow configuration

## v0.1.2 – 2025-06-03

Fixes multi-line array handling in `Cargo.toml`.

- fix: support multi-line arrays in `Cargo.toml` ([#34](https://github.com/maksym-arutyunyan/keepsorted/issues/34))

## v0.1.1 – 2024-10-01

Introduces experimental support for grouped Rust derive tokens.

- feat: (experimental) keep Rust derive tokens together (e.g., `Serialize`, `serde::Serialize`)

## v0.1.0 – 2024-09-12

Initial release with support for common config file types and experimental sorting features.

- feat: add generic keyword sorting
- feat: support Bazel files
- feat: support `Cargo.toml` files
- feat: (experimental) support `.gitignore` files
- feat: (experimental) support `CODEOWNERS` files
- feat: (experimental) sort Rust `#[derive(...)]` traits
