# Changelog
All notable changes to this project will be documented in this file.

This changelog follows the [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) 
format and adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Added `--mode` option with `check`, `diff`, and `fix` variants and added `--check`, `--diff`, and `--fix` convenience flags
- Added `--diff-command` option to run a custom program when showing diffs
- Added project website link at the end of `--help` output
- Simplified help text and clarified `--diff-command`
- Added documentation link to Cargo.toml

### Documentation
- Document installation command and provide docs.rs, source code, and issue tracker links near the top of README

## [0.1.5] - 2025-07-15

- Added `--check` mode to lint command for CI and pre-commit use
- Improved crate-level documentation
- Updated README with badges

## [0.1.4] - 2025-07-14

### Fixed
- Correct Bazel file detection
- Support Lua-style `--` comments for ignore directives

### Documentation
- Fix link in the development guide
- Fix numbering in the publish guide

### Misc
- Added license file
- Corrected alphabetical sort function typo

## [0.1.3] - 2025-07-10

### Added
- Support for Lua files

### Fixed
- Improved `rust_derive` and generic sorting interaction

### CI
- Simplified workflow configuration

## [0.1.2] - 2025-06-03

### Fixed
- fix multi-line arrays in Cargo.toml, [issue 34](https://github.com/maksym-arutyunyan/keepsorted/issues/34)

## [0.1.1] - 2024-10-01

### Added

- (Experimental) Keep Rust derive tokens together, eg. `Serialize` and `serde::Serialize`

## [0.1.0] - 2024-09-12

### Added
- Generic keyword sorting functionality
- Support for Bazel files
- Support for `Cargo.toml` files
- (Experimental) Support for `.gitignore` files
- (Experimental) Support for `CODEOWNERS` files
- (Experimental) Sorting of Rust `#[derive(...)]` traits
