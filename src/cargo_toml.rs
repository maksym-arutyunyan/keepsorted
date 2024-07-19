use crate::block::{sort, SortStrategy};
use std::io::{self};
use std::path::Path;

const STRATEGY: SortStrategy = SortStrategy::CargoToml;

pub(crate) fn is_cargo_toml(path: &Path) -> bool {
    // Check if the path is a file and its file name is "Cargo.toml"
    path.is_file() && path.file_name() == Some(std::ffi::OsStr::new("Cargo.toml"))
}

pub(crate) fn process_lines_cargo_toml(lines: Vec<&str>) -> io::Result<Vec<&str>> {
    let mut output_lines = Vec::new();
    let mut block = Vec::new();
    let mut is_sorting_block = false;

    for line in lines {
        let trimmed = line.trim();
        let line_without_comment = trimmed.split('#').next().unwrap_or("").trim();

        if line.starts_with("[dependencies]") || line.starts_with("[dev-dependencies]") {
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
