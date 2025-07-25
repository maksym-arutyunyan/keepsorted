# keepsorted

[![CI](https://github.com/maksym-arutyunyan/keepsorted/workflows/CI/badge.svg)](https://github.com/maksym-arutyunyan/keepsorted/actions/workflows/workflow.yaml)
[![crates.io](https://img.shields.io/crates/v/keepsorted.svg)](https://crates.io/crates/keepsorted)
[![docs.rs](https://docs.rs/keepsorted/badge.svg)](https://docs.rs/keepsorted)

**keepsorted** sorts selected blocks of text in code files while keeping comments in place. It works on many file types, such as `Cargo.toml`, Bazel files and general text.  
See [Architecture](docs/architecture.md) for a deeper explanation.

```shell
cargo install keepsorted
```

- [Documentation](https://docs.rs/keepsorted)
- [Source](https://github.com/maksym-arutyunyan/keepsorted)
- [Issues](https://github.com/maksym-arutyunyan/keepsorted/issues)

## Quick Start

Run `keepsorted` on a single file:

```shell
# verify sorting without editing
keepsorted --check <path>

# show differences
keepsorted --diff <path>

# apply changes in place (default)
keepsorted --fix <path>
```

The tool accepts one explicit file path at a time. Use shell commands like `git ls-files` or `find` to generate a list of files for CI or pre-commit hooks.

### Pre-commit Example

```shell
#!/bin/sh
git ls-files -z \
  | grep -vzE '^tests/|^e2e-tests/|^README.md$' \
  | xargs -0 -n1 keepsorted --check || {
    echo 'Run keepsorted --fix' >&2
    exit 1
}
```

## Using as a Library

`keepsorted` can be embedded in Rust projects. Two main functions are provided:

```rust
use keepsorted::{process_lines, Strategy};
use std::io;

fn main() -> io::Result<()> {
    // Sort an in-memory list of lines
    let lines = vec!["# Keep sorted".into(), "b".into(), "a".into()];
    let sorted = process_lines(Strategy::Generic, lines)?;

    Ok(())
}
```

See [Architecture](docs/architecture.md#crate-api-srclibrs) for more details.

## Experimental Features

Several optional behaviours are disabled by default. Enable them with the comma-separated `--features` flag:

```shell
keepsorted <path> --features gitignore,codeowners
```

Available flags:

- `gitignore` – sort `.gitignore` and `CODEOWNERS`
- `codeowners` – sort `CODEOWNERS` when combined with `gitignore`
- `rust_derive_alphabetical` – alphabetize `#[derive(...)]`
- `rust_derive_canonical` – canonical order of `#[derive(...)]`

## Limitations

`keepsorted` intentionally does not:

- Walk directories recursively (use tools like `find` or `git ls-files`)
- Parse every file type in depth
- Respect ignore files automatically
- Detect project configuration files or layout
- Replace formatters such as `rustfmt` or `prettier`

Refer to [Architecture – Out of Scope](docs/architecture.md#out-of-scope) for the full list.
