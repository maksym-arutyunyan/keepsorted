use crate::block::{sort, SortStrategy};
use regex::Regex;
use std::io::{self};

const STRATEGY: SortStrategy = SortStrategy::CargoToml;

pub(crate) fn process_lines_cargo_toml(lines: Vec<&str>) -> io::Result<Vec<&str>> {
    let re = Regex::new(r"^\[dependencies\]$")
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let mut output_lines = Vec::new();
    let mut block = Vec::new();
    let mut is_sorting_block = false;

    for line in lines {
        if re.is_match(line) {
            is_sorting_block = true;
            output_lines.push(line);
        } else if is_sorting_block && line.trim().is_empty() {
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
