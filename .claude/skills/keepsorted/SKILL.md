---
name: keepsorted
description: Work with keepsorted — a tool that sorts annotated blocks in code files while keeping comments attached to their lines. Use when the user wants to check sorting, fix sorting, add sort markers, or set up keepsorted in a project.
# keepsorted: ignore file — this file contains example markers that must not be sorted
---

# keepsorted

`keepsorted` sorts lists of lines in code files while keeping nearby comments
attached to the lines that follow them.

## When the user invokes this skill

The user most likely wants to run keepsorted on a specific folder. Follow this
flow:

### 1. Ensure keepsorted is installed and up to date

```bash
which keepsorted && keepsorted --version || echo "NOT INSTALLED"
cargo install keepsorted  # install or update to latest
```

### 2. Scan the target folder for supported file types

Before running anything, quickly count what's in the folder so you can tell the
user what keepsorted can do for them specifically:

```bash
# Count supported file types in the target folder
find <DIR> -name 'Cargo.toml' | wc -l          # Cargo deps (sorted by default)
find <DIR> -name 'BUILD.bazel' -o -name '*.bzl' | wc -l  # Bazel (sorted by default with markers)
find <DIR> -name '*.rs' | wc -l                 # Rust (derive sorting via feature flag)
find <DIR> -name '.gitignore' | wc -l           # gitignore (via feature flag)
find <DIR> -name 'CODEOWNERS' | wc -l           # CODEOWNERS (via feature flag)
```

### 3. Present what keepsorted can do, grouped by default vs opt-in

Report the file counts and explain:

**Sorted by default (no flags needed):**
- **Cargo.toml** — `[dependencies]`, `[dev-dependencies]`, `[build-dependencies]`
  and their workspace variants.
- **Bazel files** (BUILD.bazel, *.bzl) — blocks marked with `# Keep sorted.`
- **Any file** with `# Keep sorted.` / `// Keep sorted.` / `-- Keep sorted.`
  markers.

**Requires `--features` flag:**
- **Rust `#[derive(...)]` attributes** — needs `--features rust_derive_canonical`
  (or `rust_derive_alphabetical`). Neither is on by default. Prefer canonical
  unless the user asks for alphabetical. See "Rust derive sorting" below.
- **.gitignore** — needs `--features gitignore`. Use with care: order can affect
  semantics.
- **CODEOWNERS** — needs `--features codeowners`. Use with care: last match wins.

Ask the user if they want to enable any feature flags, mentioning which ones
are relevant based on the file types found.

### 4. Run keepsorted

Always use `--recursive` for directories. keepsorted takes a single PATH
argument (not multiple files).

```bash
# Check what needs sorting (dry run)
keepsorted --recursive --check <DIR>

# Check with feature flags
keepsorted --recursive --check --features rust_derive_canonical <DIR>

# Show diffs without modifying
keepsorted --recursive --diff <DIR>
keepsorted --recursive --diff --features rust_derive_canonical <DIR>

# Fix in place
keepsorted --recursive <DIR>
keepsorted --recursive --features rust_derive_canonical <DIR>
```

Exit codes: 0 = all sorted, 4 = needs sorting, 1 = syntax error, 2 = usage
error, 3 = runtime error.

When showing results, present the list of files that need sorting first. Then
ask the user if they want to see diffs or just fix everything.

## Rust derive sorting

By default, keepsorted does NOT touch `#[derive(...)]` attributes. Two feature
flags enable this:

**`rust_derive_canonical`** (preferred) — std traits first in a fixed order,
third-party derives alphabetically at the end:
```
#[derive(Clone, Copy, PartialEq, Deserialize)]  // before
#[derive(Copy, Clone, PartialEq, Deserialize)]  // after: Copy before Clone
```
Canonical order: Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
Default, ... then third-party alphabetically.

**`rust_derive_alphabetical`** — sorts everything A-Z regardless of origin:
```
#[derive(Debug, Clone, Serialize)]  // before
#[derive(Clone, Debug, Serialize)]  // after
```

## Markers reference

Add a comment before a block to opt in to sorting:

```
# Keep sorted.
```

Works with `#`, `//`, or `--` comment styles. Case-insensitive, trailing period
optional. Also accepted: `# keepsorted: keep sorted`.

Skip directives:
- `# keepsorted: ignore file` — skip the entire file.
- `# keepsorted: ignore block` — skip only the next block.

Comments directly above a line stay attached to that line during sorting.

### Good candidates for markers
- Import/use statements
- Dependency lists
- Constant arrays, enum variants (when order is not semantic)
- Feature lists, module declarations, export lists

### Do NOT sort
Pipeline stages, migrations, priority-ordered configs, .gitignore patterns with
overrides. Use `# keepsorted: ignore block` to opt out.

Comment style by file type:
- `#` — Python, YAML, TOML, shell, Bazel, Makefile, .gitignore
- `//` — Rust, C, C++, Java, JavaScript, TypeScript, Go, Protobuf
- `--` — SQL, Lua, Haskell

## CI integration

### GitHub Actions

```yaml
- uses: dtolnay/rust-toolchain@stable
- run: cargo install keepsorted
- run: keepsorted --recursive --check .
```

### Pre-commit hook

```bash
#!/bin/sh
git diff --cached --name-only --diff-filter=ACM | xargs -I{} keepsorted --check {}
```
