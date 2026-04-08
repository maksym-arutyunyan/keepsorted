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
    // Check if the line starts and ends with brackets.
    let trimmed = line.trim();
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return false;
    }
    for case in ["dependencies", "dev-dependencies", "build-dependencies"] {
        let patterns = [
            format!("[{case}]"),           // E.g. [dependencies]
            format!(".{case}]"),           // E.g. [xxx.dev-dependencies]
            format!("[{case}."),           // E.g. [dev-dependencies.xxx]
            format!("[workspace.{case}."), // E.g. [workspace.dependencies.xxx]
        ];
        if patterns.iter().any(|pattern| trimmed.contains(pattern)) {
            return true;
        }
    }

    false
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
    let mut is_multiline_code = false;
    for line in block {
        if !is_multiline_code && is_single_line_comment(&line) {
            current_item.comment.push(line);
        } else {
            let is_multi = is_multi_line_code(&line);
            let is_completed = is_code_section_completed(&line);
            current_item.code.push(line);
            if is_multi {
                is_multiline_code = true;
            }
            if !is_multiline_code || is_completed {
                items.push(std::mem::take(&mut current_item));
                is_multiline_code = false;
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

fn is_multi_line_code(line: &str) -> bool {
    let (code, _comment) = split_code_and_comment(line.trim());
    code.contains('{') || code.contains('[')
}

fn is_code_section_completed(line: &str) -> bool {
    // Split the line at the '#' character, take the first part, trim it,
    // and check if it ends with '}' or ']'.
    let (code, _comment) = split_code_and_comment(line.trim());
    let x = code.trim();
    x.ends_with('}') || x.ends_with(']')
}
