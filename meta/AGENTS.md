# AGENTS

Guidelines for working with the contributor documentation in this folder.

keepsorted is a Rust tool that sorts annotated lists while preserving nearby
comments so configuration files and code blocks stay tidy.

## Behaviors

- Follow Conventional Commits for commit messages and PR titles.
- For Rust or manifest changes, run each command in `verify.sh` and then the
  script itself. Doc-only changes may skip these checks.
- Use the Rust version pinned in `rust-toolchain.toml`.
- Keep lists and manifest sections sorted and update `CHANGELOG.md` for
  user-facing changes.
- Do not modify `docs/specs.md` unless a human requests it.
- Format Markdown with `mdformat`; CI runs a check to ensure files stay
  formatted.

## Reference

| File | Purpose |
| ---- | ------- |
| `SPECS.md` | Product scope, goals, and acceptance criteria. |
| `SYSTEM.md` | Architecture overview and component responsibilities. |
| `WORKFLOW.md` | Environment setup, workflow, and conventions. |

## Conventions

- Markdown files use `#` headings, wrap text reasonably, and rely on
  `mdformat` for consistent style.
- File names and commit scopes use `snake_case` and lowercase.
- Paths and examples assume the repository root unless noted.
