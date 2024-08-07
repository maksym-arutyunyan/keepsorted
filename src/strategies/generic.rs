use regex::Regex;
use std::io;

pub(crate) fn process(lines: Vec<String>) -> io::Result<Vec<String>> {
    let re = Regex::new(r"^\s*#\s*Keep\s*sorted\.\s*$")
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let mut output_lines = Vec::new();
    let mut block = Vec::new();
    let mut is_sorting_block = false;

    for line in lines {
        if re.is_match(&line) {
            is_sorting_block = true;
            output_lines.push(line);
        } else if is_sorting_block && line.trim().is_empty() {
            is_sorting_block = false;
            block = sort(block);
            output_lines.append(&mut block);
            output_lines.push(line);
        } else if is_sorting_block {
            block.push(line);
        } else {
            output_lines.push(line);
        }
    }

    if is_sorting_block {
        block = sort(block);
        output_lines.append(&mut block);
    }

    Ok(output_lines)
}

/// Sorts a block of lines, keeping associated comments with their items.
fn sort(block: Vec<String>) -> Vec<String> {
    let n = block.len();
    let mut items = Vec::with_capacity(n);
    let mut current_item = Item::default();
    for line in block {
        if is_single_line_comment(&line) {
            current_item.comment.push(line);
        } else {
            current_item.code = line;
            items.push(std::mem::take(&mut current_item));
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

#[derive(Default)]
struct Item {
    comment: Vec<String>,
    code: String,
}

fn is_single_line_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('#') || trimmed.starts_with("//")
}
