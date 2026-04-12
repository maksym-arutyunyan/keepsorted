# keepsorted

[![keepsorted GitHub Actions][gh-image]][gh-checks]
[![keepsorted on crates.io][cratesio-image]][cratesio]

```shell
cargo install keepsorted
```

- [Source code](https://github.com/maksym-arutyunyan/keepsorted)
- [Issue tracker](https://github.com/maksym-arutyunyan/keepsorted/issues)
- [Specs](docs/SPECS.md)

## Overview

`keepsorted` sorts lists of lines while keeping nearby comments with the lines
that follow them. Add a comment like **`Keep sorted`** and the tool will reorder
the next block. Some file types are sorted automatically.

## Usage

Check a single file:

```shell
keepsorted --check file.txt
```

Check a directory recursively:

```shell
keepsorted --check --recursive .
```

Check only git-tracked files:

```shell
git ls-files | xargs -n1 keepsorted --check
```

Fix files in place (default mode):

```shell
keepsorted file.txt
```

### Modes

- `--fix` (default): Rewrites files in place.
- `--check`: Verifies files are sorted. Returns exit code 4 if not.
- `--diff`: Prints a diff of changes without modifying files.

### Keywords

- `Keep sorted` or `keepsorted: keep sorted` – sort the next block.
- `keepsorted: ignore file` – skip the whole file.
- `keepsorted: ignore block` – skip a single block.

Markers work with `#`, `//`, or `--` comments. Generic files support any of these. Bazel files require `#`. `Cargo.toml`, `.gitignore`, and `CODEOWNERS` are sorted
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
# Lead
a
# Team
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

[cratesio]: https://crates.io/crates/keepsorted
[cratesio-image]: https://img.shields.io/crates/v/keepsorted.svg
[gh-checks]: https://github.com/maksym-arutyunyan/keepsorted/actions/workflows/workflow.yaml
[gh-image]: https://github.com/maksym-arutyunyan/keepsorted/workflows/CI/badge.svg
