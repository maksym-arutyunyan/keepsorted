use regex::Regex;
use std::cmp::Ordering;
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

fn sort(block: Vec<String>) -> Vec<String> {
    let n = block.len();
    let mut items = Vec::with_capacity(n);
    let mut current_item = Item::default();
    for line in block.into_iter() {
        if is_single_line_comment(&line) {
            current_item.comment.push(line);
        } else {
            current_item.item = line;
            items.push(current_item);
            current_item = Item::default();
        }
    }
    let trailing_comment = current_item.comment;

    items.sort();

    let mut sorted_block = Vec::with_capacity(n);
    for group in items {
        sorted_block.extend(group.comment);
        sorted_block.push(group.item);
    }
    sorted_block.extend(trailing_comment);

    sorted_block
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Item {
    comment: Vec<String>,
    item: String,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.item.cmp(&other.item)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_single_line_comment(line: &String) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('#') || trimmed.starts_with("//")
}
