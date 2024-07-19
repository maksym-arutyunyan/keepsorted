use crate::block::{sort, SortStrategy};
use std::io::{self};

const STRATEGY: SortStrategy = SortStrategy::CargoToml;

pub(crate) fn process_lines_cargo_toml(lines: Vec<&str>) -> io::Result<Vec<&str>> {
    let mut output_lines = Vec::new();
    let mut block = Vec::new();
    let mut is_sorting_block = false;

    for line in lines {
        let trimmed = line.trim();
        let line_without_comment = trimmed.split('#').next().unwrap_or("").trim();

        if line == "[dependencies]" || line == "[dev-dependencies]" {
            is_sorting_block = true;
            output_lines.push(line);
        } else if is_sorting_block
            && (line.trim().is_empty() || line_without_comment.starts_with('['))
        {
            is_sorting_block = false;
            sort(&mut block, STRATEGY);
            output_lines.append(&mut block);
            output_lines.push(line);
        } else if is_sorting_block {
            block.push(line);
        } else {
            output_lines.push(line);
        }
    }

    if is_sorting_block {
        sort(&mut block, STRATEGY);
        output_lines.append(&mut block);
    }

    Ok(output_lines)
}
