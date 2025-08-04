# AGENTS

keepsorted is a Rust tool that sorts annotated lists while preserving nearby comments. It helps keep configuration files and code blocks tidy.

## Behaviors
- Follow Conventional Commits for commit messages and PR titles.
- For Rust or manifest changes, run each command in `run-all.sh` and then the script itself. Doc-only changes may skip these checks.
- Use the Rust version pinned in `rust-toolchain.toml`.
- Keep lists and manifest sections sorted and update `CHANGELOG.md` for user-facing changes.
- Do not modify `docs/specs.md` unless a human requests it.

## Reference
| File | Purpose |
| ---- | ------- |
| `SPECS.md` | Product scope and problem definition. |
| `SYSTEM.md` | Architecture overview and module responsibilities. |
| `WORKFLOW.md` | Development workflow, checks, and release process. |

## Conventions
- Markdown files use `#`-style headings and wrap text reasonably.
- File names and commit scopes use `snake_case` and lowercase.
- Paths and examples assume the repository root unless noted.
