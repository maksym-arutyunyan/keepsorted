# AGENTS

Guidelines for working with the contributor documentation in this folder.

`keepsorted` sorts annotated lists while preserving nearby comments so
configuration files and code blocks stay tidy.

## Commit and PR conventions

- Commit messages and PR titles follow Conventional Commits.
- File names and commit scopes use `snake_case` and lowercase.

## Validation

- For Rust source or manifest changes:
  - Run each command from `verify.sh` individually.
  - Then run `./verify.sh` to confirm success and a clean working tree.
  - The PR fails if any command errors or if `git diff` shows changes.
- Documentation-only changes:
  - Run `mdformat` on modified Markdown files.
  - Other verify steps may be skipped.

## Environment and tools

- Use the Rust version pinned in `rust-toolchain.toml`.
- Keep lists and manifest sections sorted; run `keepsorted` where annotated.
- Update `CHANGELOG.md` for user-facing changes.
- Do not modify `meta/SPECS.md` unless a human requests it.
- Markdown is formatted with `mdformat`; CI checks formatting.

## Reference

| File | Purpose |
| ---- | ------- |
| `SPECS.md` | Product scope, goals, and acceptance criteria. |
| `SYSTEM.md` | Architecture overview and component responsibilities. |
| `WORKFLOW.md` | Environment setup, workflow, and conventions. |

## Conventions

- Markdown files use `#` headings, wrap text reasonably, and rely on
  `mdformat` for consistent style.
- Paths and examples assume the repository root unless noted.
