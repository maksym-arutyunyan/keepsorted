use regex::Regex;
use std::io;

use crate::is_ignore_block;

pub(crate) fn process(lines: Vec<String>) -> io::Result<Vec<String>> {
    let re = Regex::new(r"^\s*#\s*Keep\s*sorted\.\s*$")
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let mut output_lines: Vec<String> = Vec::new();
    let mut block = Vec::new();
    let mut is_sorting_block = false;
    let mut is_ignore_block_prev_line = false;

    for line in lines {
        if re.is_match(&line) {
            if let Some(prev_line) = output_lines.last() {
                is_ignore_block_prev_line = is_ignore_block(&[prev_line.clone()]);
            }
            is_sorting_block = true;
            output_lines.push(line);
        } else if is_sorting_block && line.trim().is_empty() {
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

#[derive(Default)]
struct Item {
    comment: Vec<String>,
    code: String,
}

/// Sorts a block of lines, keeping associated comments with their items.
fn sort(block: Vec<String>, is_ignore_block_prev_line: bool) -> Vec<String> {
    if is_ignore_block_prev_line || is_ignore_block(&block) {
        return block;
    }
    let n = block.len();
    let mut items = Vec::with_capacity(n);
    let mut current_item = Item::default();
    for line in block {
        if is_single_line_comment(&line) {
            current_item.comment.push(line);
        } else {
            items.push(Item {
                comment: std::mem::take(&mut current_item.comment),
                code: line,
            });
        }
    }
    let trailing_comments = std::mem::take(&mut current_item.comment);

    items.sort_by(|a, b| a.code.cmp(&b.code));

    let mut result = Vec::with_capacity(n);
    for item in items {
        result.extend(item.comment);
        result.push(item.code);
    }
    result.extend(trailing_comments);

    result
}

fn is_single_line_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('#') || trimmed.starts_with("//")
}
