---
name: keepsorted
description: Work with keepsorted — a tool that sorts annotated blocks in code files while keeping comments attached to their lines. Use when the user wants to check sorting, fix sorting, add sort markers, or set up keepsorted in a project.
# keepsorted: ignore file — this file contains example markers that must not be sorted
---

# keepsorted

`keepsorted` sorts lists of lines in code files while keeping nearby comments
attached to the lines that follow them. Install with `cargo install keepsorted`.

## Modes

- `keepsorted <files...>` — fix files in place (default).
- `keepsorted --check <files...>` — verify sorting, exit code 4 if unsorted.
- `keepsorted --diff <files...>` — print a unified diff without modifying files.
- `keepsorted --recursive --check .` — check a directory recursively.
- `git ls-files | xargs keepsorted --check` — check only git-tracked files.

## Markers

Add a comment before a block to opt in to sorting:

```
# Keep sorted.
```

Works with `#`, `//`, or `--` comment styles. Case-insensitive, trailing period
optional. Also accepted: `# keepsorted: keep sorted`.

Skip directives:
- `# keepsorted: ignore file` — skip the entire file.
- `# keepsorted: ignore block` — skip only the next block.

## Automatic sorting (no marker needed)

- **Cargo.toml**: `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`
  and their workspace variants are sorted automatically.
- **.gitignore**: requires `--features gitignore` (order can affect semantics).
- **CODEOWNERS**: requires `--features codeowners` (last match wins).

## Feature flags

Some file types need explicit opt-in via `--features`:

```
keepsorted --features gitignore,codeowners,rust_derive_canonical <files...>
```

Available features: `gitignore`, `codeowners`, `rust_derive_alphabetical`,
`rust_derive_canonical`.

## Bazel files

BUILD, WORKSPACE, *.bazel, *.bzl files use `#` comments only. Sorting groups
items by prefix (`:`, `//`, `@`) then compares segments split by `.`, `:`, `"`.

## Comment attachment

Comments directly above a line stay attached to that line during sorting:

```python
# Keep sorted.
```
# comment A
a
# comment B
b

becomes:

```python
# Keep sorted.
```
# comment A
a
# comment B
b

## Adding markers to code

Good candidates for `Keep sorted.` markers:
- Import/use statements
- Dependency lists
- Constant arrays, enum variants (when order is not semantic)
- Feature lists, module declarations, export lists

Do NOT sort blocks where order matters: pipeline stages, migrations,
priority-ordered configs, .gitignore patterns with overrides. Use
`# keepsorted: ignore block` to explicitly opt out.

Use the comment style matching the file type:
- `#` — Python, YAML, TOML, shell, Bazel, Makefile, .gitignore
- `//` — Rust, C, C++, Java, JavaScript, TypeScript, Go, Protobuf
- `--` — SQL, Lua, Haskell

## CI integration

### GitHub Actions

```yaml
- uses: dtolnay/rust-toolchain@stable
- run: cargo install keepsorted
- run: git ls-files | xargs keepsorted --check
```

### Pre-commit hook

```bash
#!/bin/sh
git diff --cached --name-only --diff-filter=ACM | xargs keepsorted --check
```
