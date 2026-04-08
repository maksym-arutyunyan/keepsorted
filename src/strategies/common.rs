/// Splits `line` at the first `#` that is not inside a quoted string.
///
/// Returns `(code, comment)` where `comment` begins with `#`.
/// Used by Bazel and Cargo.toml strategies.
pub(crate) fn split_code_and_comment(line: &str) -> (&str, &str) {
    let mut in_string = false;
    let mut escape = false;
    let mut quote_char = b'\0';
    let bytes = line.as_bytes();

    for (i, &c) in bytes.iter().enumerate() {
        if in_string {
            if escape {
                escape = false;
            } else if c == b'\\' {
                escape = true;
            } else if c == quote_char {
                in_string = false;
            }
        } else if c == b'"' || c == b'\'' {
            in_string = true;
            quote_char = c;
        } else if c == b'#' {
            return (&line[..i], &line[i..]);
        }
    }
    (line, "")
}

/// Splits `line` at the first `//` that is not inside a double-quoted string.
///
/// Returns `(code, comment)` where `comment` begins with `//`.
/// Used by the Rust derive strategy.
pub(crate) fn split_code_and_line_comment(line: &str) -> (&str, &str) {
    let mut in_string = false;
    let mut escape = false;
    let bytes = line.as_bytes();

    for (i, &c) in bytes.iter().enumerate() {
        if in_string {
            if escape {
                escape = false;
            } else if c == b'\\' {
                escape = true;
            } else if c == b'"' {
                in_string = false;
            }
        } else if c == b'"' {
            in_string = true;
        } else if c == b'/' && bytes.get(i + 1) == Some(&b'/') {
            return (&line[..i], &line[i..]);
        }
    }
    (line, "")
}

/// Returns true if `line` is a `#`-prefixed comment (after trimming).
///
/// Used by Bazel, Cargo.toml, and Gitignore strategies.
pub(crate) fn is_single_line_comment(line: &str) -> bool {
    line.trim().starts_with('#')
}
