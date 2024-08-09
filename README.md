# keepsorted

`keepsorted` is a command-line tool for sorting blocks of lines in your code files.

It sorts a block of lines following the activation comment `# Keep sorted` or `// Keep sorted`.
For some files, like `Cargo.toml`, it sorts automatically without needing the activation comment. 

It also recognizes comments attached to a non-comment lines, for example:

```py
# Before:
dependencies = [
    # Keep sorted.
    # TODO: remove this dependency.
    'aaa',
    'bbb',
    'ccc',
]

# After:
dependencies = [
    # Keep sorted.
    # TODO: remove this dependency.
    'aaa',
    'bbb',
    'ccc',
]
```

See examples in `./tests/e2e-tests/`.

## Keywords

- `# Keep sorted` or `// Keep sorted` or `# keepsorted: keep sorted` for sorting the next block of lines
- `# keepsorted: ignore file` can be placed anywhere in the file
- `# keepsorted: ignore block` can be placed anywhere within the block

## Supported Files

### Generic Text Files

Sort blocks starting with `# Keep sorted` or `// Keep sorted` and ending with a newline.

```txt
# Names
# Keep sorted
Alice
Bob
Conrad

// Colors
// Keep sorted
Blue
Green
Red
```

### Bazel

Sort blocks inside `[...]` starting with `# Keep sorted`.

```bazel
DEPENDENCIES = [
    # Keep sorted
    "a",
    "b",
]
```

### Cargo.toml

Sort blocks starting with `[dependencies]`, `[dev-dependencies]`, etc., and ending with an empty line.

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

*NOTE: These features are experimental and require feature flags:*

```shell
$ keepsorted <path> --features gitignore,codeowners
```

Sort blocks separated by empty lines, keeping comments in place (except the opening block comment).

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
