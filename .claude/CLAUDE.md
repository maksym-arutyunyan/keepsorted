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
