use std::io;

pub(crate) fn process(lines: Vec<String>) -> io::Result<Vec<String>> {
    let mut output_lines = Vec::new();
    let mut block = Vec::new();
    let mut is_sorting_block = false;

    for line in lines {
        let trimmed = line.trim();
        let line_without_comment = trimmed.split('#').next().unwrap_or("").trim();

        if is_block_start(&line) {
            is_sorting_block = true;
            output_lines.push(line);
        } else if is_sorting_block
            && (line.trim().is_empty() || line_without_comment.starts_with('['))
        {
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

fn is_block_start(line: &str) -> bool {
    line.starts_with("[dependencies]") || line.starts_with("[dev-dependencies]")
}

#[derive(Default)]
struct Item {
    comment: Vec<String>,
    code: Vec<String>,
}

/// Sorts a block of lines, keeping associated comments with their items.
fn sort(block: Vec<String>) -> Vec<String> {
    let n = block.len();
    let mut items = Vec::with_capacity(n);
    let mut current_item = Item::default();
    let mut is_multiline_code = false;
    for line in block {
        if is_single_line_comment(&line) {
            current_item.comment.push(line);
            is_multiline_code = false;
        } else {
            if line.contains('{') {
                is_multiline_code = true;
            }
            if !is_multiline_code {
                current_item.code.push(line);
                items.push(std::mem::take(&mut current_item));
            } else {
                current_item.code.push(line.clone());
                if is_code_section_completed(&line) {
                    items.push(std::mem::take(&mut current_item));
                    is_multiline_code = false;
                }
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

fn is_single_line_comment(line: &str) -> bool {
    line.trim().starts_with('#')
}

fn is_code_section_completed(line: &str) -> bool {
    line.trim().ends_with('}')
}
