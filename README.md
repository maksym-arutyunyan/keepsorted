# keepsorted

[gh-image]: https://github.com/maksym-arutyunyan/keepsorted/workflows/CI/badge.svg
[gh-checks]: https://github.com/maksym-arutyunyan/keepsorted/actions/workflows/workflow.yaml
[cratesio-image]: https://img.shields.io/crates/v/keepsorted.svg
[cratesio]: https://crates.io/crates/keepsorted
[docsrs-image]: https://docs.rs/keepsorted/badge.svg
[docsrs]: https://docs.rs/keepsorted

[![keepsorted GitHub Actions][gh-image]][gh-checks]
[![keepsorted on crates.io][cratesio-image]][cratesio]
[![keepsorted on docs.rs][docsrs-image]][docsrs]

```shell
cargo install keepsorted
```

- [Documentation](https://docs.rs/keepsorted)
- [Source code](https://github.com/maksym-arutyunyan/keepsorted)
- [Issue tracker](https://github.com/maksym-arutyunyan/keepsorted/issues)
- [Specs](docs/specs/index.md)

## Overview

`keepsorted` sorts lists of lines while keeping nearby comments with the lines
that follow them. Add a comment like **`Keep sorted`** and the tool will reorder
the next block. Some file types are sorted automatically.

## Usage

```shell
keepsorted --check <path>   # verify without changes
keepsorted --diff <path>    # preview changes as a diff
keepsorted --fix <path>     # rewrite files in place (default)
```

Use `--recursive` (`-r`) to process directories. Combine with `git ls-files` in
CI to check only tracked files. Binary files are skipped automatically.

### Keywords

- `Keep sorted` or `keepsorted: keep sorted` – sort the next block.
- `keepsorted: ignore file` – skip the whole file.
- `keepsorted: ignore block` – skip a single block.

Markers work with `#`, `//`, or `--` comments. Generic files and Bazel require
one of these comments. `Cargo.toml`, `.gitignore`, and `CODEOWNERS` are sorted
automatically when the matching feature flag is enabled.

### Examples

#### Generic text (Python)

```py
# Keep sorted
# comment B
b
# comment A
a
```

becomes

```py
# Keep sorted
# comment A
a
# comment B
b
```

#### Generic text (C++)

```cpp
// Keep sorted
// comment two
second
// comment one
first
```

becomes

```cpp
// Keep sorted
// comment one
first
// comment two
second
```

#### Generic text (SQL/Lua)

```sql
-- Keep sorted
-- c comment
c
-- a comment
a
```

becomes

```sql
-- Keep sorted
-- a comment
a
-- c comment
c
```

#### Bazel

```bazel
srcs = [
    # Keep sorted
    "b",
    # note for a
    "a",
]
```

becomes

```bazel
srcs = [
    # Keep sorted
    # note for a
    "a",
    "b",
]
```

#### Cargo.toml

```toml
[dependencies]
b = "2"
a = "1"

# keepsorted: ignore block
[dev-dependencies]
z = "1"
y = "2"
```

becomes

```toml
[dependencies]
a = "1"
b = "2"

# keepsorted: ignore block
[dev-dependencies]
z = "1"
y = "2"
```

#### .gitignore

```gitignore
# Build
/b
/a
```

becomes

```gitignore
# Build
/a
/b
```

#### CODEOWNERS

```codeowners
# Team
b
# Lead
a
```

becomes

```codeowners
# Team
a
# Lead
b
```

### Experimental features

The following features are behind flags because sorting might change behaviour:

- `gitignore` and `codeowners` – order matters, so enable with care.
- `rust_derive_alphabetical` and `rust_derive_canonical` – temporary helpers to
  reorder `#[derive(...)]` attributes. `cargo fmt` does not yet sort derives
  ([rust-lang/rustfmt#6574](https://github.com/rust-lang/rustfmt/issues/6574)).
  These features are hidden behind flags and only perform alphabetical or
  canonical ordering. Users have been requesting derive sorting since 2017, so
  `keepsorted` fills the gap for now. Consider upvoting the issue if you want
  built-in support.

Enable features with `--features`:

```shell
keepsorted file --features gitignore,rust_derive_canonical
```

### Limitations

`keepsorted` intentionally does **not**:

- Handle advanced directory traversal or ignore rules automatically.
- Act as a full-fledged parser for every file type.
- Handle ignore files or exclude paths automatically.
- Automatically detect project structure or configuration files.
- Replace formatting tools like `rustfmt` or `prettier`.

