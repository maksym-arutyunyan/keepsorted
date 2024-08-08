# keepsorted

`keepsorted` is a simple command-line tool to sort blocks of lines in your code files. 

It looks for the comment `# Keep sorted` and sorts the lines that follow it.

For some files, like `Cargo.toml`, `.gitignore`, and `CODEOWNERS`, it works without needing the `# Keep sorted` comment. It automatically sorts sections like dependencies in `Cargo.toml` and blocks separated by newlines in the other files, keeping comments in place.

For some examples look at `./tests/e2e-tests/`.
