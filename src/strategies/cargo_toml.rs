use std::io;

use super::common::{is_single_line_comment, split_code_and_comment};
use crate::{is_ignore_block, is_ignore_block_line};

pub(crate) fn process(lines: Vec<String>) -> io::Result<Vec<String>> {
    let mut output_lines: Vec<String> = Vec::new();
    let mut block = Vec::new();
    let mut is_sorting_block = false;
    let mut is_ignore_block_prev_line = false;

    for line in lines {
        let trimmed = line.trim();
        let (line_without_comment, _comment) = split_code_and_comment(trimmed);
        let line_without_comment = line_without_comment.trim();

        if is_block_start(&line) {
            if let Some(prev_line) = output_lines.last() {
                is_ignore_block_prev_line = is_ignore_block_line(prev_line);
            }
            is_sorting_block = true;
            output_lines.push(line);
        } else if is_sorting_block
            && (line.trim().is_empty() || line_without_comment.starts_with('['))
        {
            block = sort(block, is_ignore_block_prev_line);
            is_ignore_block_prev_line = false;
            is_sorting_block = false;
            output_lines.append(&mut block);
            output_lines.push(line);
        } else if is_sorting_block {
            block.push(line);
        } else {
            output_lines.push(line);
        }
    }

    if is_sorting_block {
        block = sort(block, is_ignore_block_prev_line);
        output_lines.append(&mut block);
    }

    Ok(output_lines)
}

fn is_block_start(line: &str) -> bool {
    let inner = match line
        .trim()
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
    {
        Some(s) if !s.is_empty() => s,
        _ => return false,
    };
    ["dependencies", "dev-dependencies", "build-dependencies"]
        .iter()
        .any(|dep| {
            inner == *dep                                                          // [dependencies]
                || inner.starts_with(dep) && inner[dep.len()..].starts_with('.') // [dependencies.xxx]
                || inner.ends_with(dep) && inner[..inner.len() - dep.len()].ends_with('.') // [xxx.dependencies]
                || inner                                                           // [workspace.dependencies.xxx]
                    .strip_prefix("workspace.")
                    .map(|s| s.starts_with(dep) && s[dep.len()..].starts_with('.'))
                    .unwrap_or(false)
        })
}

#[derive(Default)]
struct Item {
    comment: Vec<String>,
    code: Vec<String>,
}

/// Sorts a block of lines, keeping associated comments with their items.
fn sort(block: Vec<String>, is_ignore_block_prev_line: bool) -> Vec<String> {
    if is_ignore_block_prev_line || is_ignore_block(&block) {
        return block;
    }
    let n = block.len();
    let mut items = Vec::with_capacity(n);
    let mut current_item = Item::default();
    let mut depth = 0i32;
    for line in block {
        if depth == 0 && is_single_line_comment(&line) {
            current_item.comment.push(line);
        } else {
            depth += bracket_depth_delta(&line);
            current_item.code.push(line);
            if depth <= 0 {
                items.push(std::mem::take(&mut current_item));
                depth = 0;
            }
        }
    }
    let trailing_comments = std::mem::take(&mut current_item.comment);

    items.sort_by(|a, b| a.code.cmp(&b.code));

    let mut result = Vec::with_capacity(n);
    for item in items {
        result.extend(item.comment);
        result.extend(item.code);
    }
    result.extend(trailing_comments);

    result
}

/// Returns the net change in bracket/brace nesting depth for a line.
///
/// Counts unquoted `{` and `[` as +1, unquoted `}` and `]` as -1.
/// Characters inside quoted strings are ignored.
fn bracket_depth_delta(line: &str) -> i32 {
    let (code, _comment) = split_code_and_comment(line.trim());
    let mut in_string = false;
    let mut escape = false;
    let mut quote_char = b'\0';
    let mut delta = 0i32;
    for &c in code.as_bytes() {
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
        } else if c == b'{' || c == b'[' {
            delta += 1;
        } else if c == b'}' || c == b']' {
            delta -= 1;
        }
    }
    delta
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_block_start_exact() {
        assert!(is_block_start("[dependencies]"));
        assert!(is_block_start("[dev-dependencies]"));
        assert!(is_block_start("[build-dependencies]"));
    }

    #[test]
    fn test_is_block_start_dep_first_segment() {
        assert!(is_block_start("[dependencies.xxx]"));
        assert!(is_block_start("[dev-dependencies.foo]"));
        assert!(is_block_start("[build-dependencies.bar]"));
    }

    #[test]
    fn test_is_block_start_dep_last_segment() {
        assert!(is_block_start("[xxx.dependencies]"));
        assert!(is_block_start("[target.'cfg(unix)'.dev-dependencies]"));
        assert!(is_block_start("[workspace.build-dependencies]"));
    }

    #[test]
    fn test_is_block_start_workspace_prefix() {
        assert!(is_block_start("[workspace.dependencies.xxx]"));
        assert!(is_block_start("[workspace.dev-dependencies.foo]"));
        assert!(is_block_start("[workspace.build-dependencies.bar]"));
    }

    #[test]
    fn test_is_block_start_whitespace() {
        assert!(is_block_start("  [dependencies]  "));
    }

    #[test]
    fn test_is_block_start_non_matching() {
        assert!(!is_block_start("[package]"));
        assert!(!is_block_start("[features]"));
        assert!(!is_block_start("[profile.dev]"));
        assert!(!is_block_start("dependencies = []"));
        assert!(!is_block_start(""));
        assert!(!is_block_start("[]"));
    }
}
