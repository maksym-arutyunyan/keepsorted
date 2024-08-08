# keepsorted

`keepsorted` is a command-line tool for sorting blocks of lines in your code files.

It sorts lines following the comment `# Keep sorted`.

For some files, like `Cargo.toml`, `.gitignore`, and `CODEOWNERS`, it sorts automatically without needing the `# Keep sorted` comment. It handles sections and blocks separated by newlines while preserving comments.

See examples in `./tests/e2e-tests/`.

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
```

### .gitignore & CODEOWNERS

*NOTE: These features are experimental and require feature flags:*

```shell
$ keepsorted <path> --features gitignore,codeowners
```

Sort blocks separated by empty lines, keeping comments in place (except the opening block comment).
