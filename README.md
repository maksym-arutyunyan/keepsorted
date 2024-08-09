# keepsorted

`keepsorted` is a command-line tool that helps you sort blocks of lines in your code files.

It works by sorting lines within a block that starts with the activation comment `# Keep sorted` or `// Keep sorted`. 
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

You can see more examples in the ./tests/e2e-tests/ directory.

## Keywords

- Use `# Keep sorted`, `// Keep sorted`, or `# keepsorted: keep sorted` to sort the next block of lines
- Add `# keepsorted: ignore file` anywhere in the file to skip sorting
- Use `# keepsorted: ignore block` within a block to skip sorting that block

## Supported Files

### Generic Text Files

For generic text files, the tool sorts blocks that start with `# Keep sorted` or `// Keep sorted` and end with a newline.

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
