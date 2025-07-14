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

`keepsorted` sorts blocks of lines in your files. Add `# Keep sorted` above a block to keep it alphabetized. In files like `Cargo.toml` sorting works automatically, and comments stay with the lines they describe.

```py
# Before
dependencies = [
    # Keep sorted
    'ddd',
    'ccc',
    # comment about this dep
    'bbb',
]

# After
dependencies = [
    # Keep sorted
    'bbb',
    # comment about this dep
    'ccc',
    'ddd',
]
```

## Usage

```shell
$ keepsorted <path>
```

### Keywords

- `# Keep sorted` or `# keepsorted: keep sorted` – sort the following block
- `# keepsorted: ignore block` – skip a block
- `# keepsorted: ignore file` – skip the entire file

## Supported files

### Generic text

```
# Names
# Keep sorted
Alice
Bob
Conrad
```

### Bazel

```
DEPENDENCIES = [
    # Keep sorted
    "b",
    "a",
]
```

### Cargo.toml

```
[dependencies]
a = "0.1.0"
b = { workspace = true }

# keepsorted: ignore block
[dev-dependencies]
y = { workspace = true }
x = "0.3.0"
```

### .gitignore & CODEOWNERS *(experimental)*

These features require `gitignore` and `codeowners` flags. Be careful: the order of patterns matters.

```shell
$ keepsorted <path> --features gitignore,codeowners
```

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

### Rust derive *(experimental)*

Enable `rust_derive_alphabetical` or `rust_derive_canonical` to sort traits inside `#[derive(...)]`.
Canonical ordering follows the style suggested by the Rust style team.

```shell
$ keepsorted <path> --features rust_derive_alphabetical
# or
$ keepsorted <path> --features rust_derive_canonical
```

More examples are available in `tests/e2e-tests`.
