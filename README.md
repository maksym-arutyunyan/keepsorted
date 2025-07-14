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

`keepsorted` is a command-line tool that helps you sort blocks of lines in your code files.

It works by sorting lines within a block that starts with the activation comment `# Keep sorted`.
In some files, like `Cargo.toml`, it sorts automatically without needing an activation comment.

The tool can also recognize comments attached to non-comment lines, like this:

```py
# Before:
dependencies = [
    # Keep sorted.
    'ddd',
    'ccc',
    # TODO: remove this dependency.
    'bbb',
    'aaa',
]

# After:
dependencies = [
    # Keep sorted.
    'aaa',
    # TODO: remove this dependency.
    'bbb',
    'ccc',
    'ddd',
]
```

You can see more examples in the `./tests/e2e-tests/` directory.

## Keywords

Comments can begin with `#`, `//`, or `--`. The following examples use `#`.

- `# Keep sorted` or `# keepsorted: keep sorted` sorts the next block of lines
- `# keepsorted: ignore file` anywhere in the file skips sorting
- `# keepsorted: ignore block` within a block skips sorting that block

## Supported Files

### Generic Text Files

For generic text files, the tool sorts blocks that start with `# Keep sorted` and end with a newline.

```txt
# Names
# Keep sorted
Alice
Bob
Conrad

# Colors
# Keep sorted
Blue
Green
Red
```

### Bazel

In Bazel files, keepsorted sorts lines within `[...]` blocks that start with `# Keep sorted`.

```bazel
DEPENDENCIES = [
    # Keep sorted
    "a",
    "b",
]
```

### Cargo.toml

In `Cargo.toml` files, the tool sorts lines within blocks that start with `[dependencies]`, `[dev-dependencies]`, etc., and end with an empty line.

```toml
[dependencies]
a = "0.1.0"
b = { workspace = true }

# keepsorted: ignore block
[dev-dependencies]
y = { workspace = true }
x = "0.3.0"
```

### .gitignore & CODEOWNERS

*NOTE: These features are experimental and require feature flags.*

```shell
$ keepsorted <path> --features gitignore,codeowners
```

In `.gitignore` and `CODEOWNERS` files, the tool sorts blocks separated by empty lines while keeping comments in place, except for the opening block comment.

**(!) IMPORTANT**: the order of patterns can be important because it gets executed from top to bottom from more generic to more specific rules, therefore use this feature with extra care.

```.gitignore
# Various build artifacts
**/build
**/build-out
**/build-tmp
artifacts

# Bazel outdir dirs
# keepsorted: ignore block
bazel-c.pb
user.bazelrc
bazel-b.txt
/bazel-*
bazel-a.txt
```

### Rust Derive

*NOTE: These features are experimental and require feature flags.*

```shell
$ keepsorted <path> --features rust_derive_alphabetical
# or
$ keepsorted <path> --features rust_derive_canonical
```

The feature is inspired by a closed ticket to update rust style, [link](https://github.com/rust-lang/style-team/issues/154).


