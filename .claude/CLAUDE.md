## Local checks before pushing

Run these after making changes to catch CI failures early:

```bash
./verify.sh
```

## Allowed commit types

Allowed types: `feat`, `fix`, `docs`, `test`, `ci`, `refactor`, `perf`, `chore`, `revert`.

## Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <short description>
```

Examples:
- `fix: handle empty input without panic`
- `feat(cli): add --verbose flag`
- `chore: bump version to v0.2.0`

Keep the description lowercase and under 72 characters. No period at the end.

## Pull request titles and descriptions

When asked to "write PR description" or similar:
- Look at all commits in the current branch vs default branch
- Title: under 70 characters, use conventional commit prefixes
- Description: a single line summary on top, followed by a short explanation of what was added and why
- Both must be correct, short, clear and informative
- Do not use excessive formatting, but use bullet points or tables if it improves readability
- Don't list commits or files changed, the PR view already shows that

## Acceptance criteria

A format or feature is complete when:

- Annotated blocks are detected and sorted idempotently.
- Comments remain attached to the intended line.
- Output is stable across runs and platforms.
- `--mode check` exits non-zero for unsorted input.
- `--mode fix` rewrites files in place; `--mode diff` prints a unified diff.
- Tests cover typical usage and edge cases.

Do not change the product scope or acceptance criteria without explicit human approval.

## Release procedure

1. Bump the version in `Cargo.toml`.
1. Draft a GitHub release:
   - Identify the merge commit.
   - Go to Releases → **Draft a new release**.
   - Set tag (`vX.Y.Z`), target (merge commit), title (`vX.Y.Z`), previous tag.
   - Click **Generate release notes**, edit if needed.
   - Click **Publish release**.
1. Publish to crates.io:
   ```bash
   cargo login
   git checkout vX.Y.Z
   cargo publish -p keepsorted
   ```
