use std::io;

pub(crate) fn process(lines: Vec<String>) -> io::Result<Vec<String>> {
    let mut output_lines = Vec::new();
    let mut block = Vec::new();
    let mut is_sorting_block = false;

    for line in lines {
        if line.trim().is_empty() {
            if is_sorting_block {
                // End of the current sorting block
                block = sort(block);
                output_lines.append(&mut block);
                output_lines.push(line.clone()); // Include the empty line as well
                block.clear(); // Clear the block for next possible sorting block
                is_sorting_block = false;
            } else {
                output_lines.push(line);
            }
        } else {
            if !is_sorting_block {
                is_sorting_block = true;
            }
            if is_sorting_block {
                block.push(line);
            } else {
                output_lines.push(line);
            }
        }
    }

    if is_sorting_block {
        block = sort(block);
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
fn sort(block: Vec<String>) -> Vec<String> {
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
    line.trim().starts_with('#')
}
