use crate::block::{sort, SortStrategy};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

mod block;

fn is_bazel_related(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(ext, "bazel" | "bzl" | "BUILD" | "WORKSPACE"),
        None => false,
    }
}

pub fn process_file(path: &Path) -> io::Result<()> {
    let mut content = std::fs::read_to_string(path)?;
    let ends_with_newline = content.ends_with('\n');
    if !ends_with_newline {
        // Add trailing '\n' so all the lines have it.
        content.push('\n');
    }
    let lines: Vec<&str> = content.split_inclusive('\n').collect();

    // Check the file extension
    let output_lines = if is_bazel_related(path) {
        process_lines_bazel(lines)?
    } else {
        process_lines(lines)?
    };

    let n = output_lines.len();
    let output_file = File::create(path)?;
    let mut writer = BufWriter::new(output_file);
    for (i, line) in output_lines.iter().enumerate() {
        if i + 1 == n && !ends_with_newline {
            // Remove trailing '\n' since there were none in the source.
            write!(writer, "{}", line.trim_end_matches('\n'))?;
        } else {
            write!(writer, "{}", line)?;
        }
    }

    writer.flush()?;
    Ok(())
}

pub fn process_lines(lines: Vec<&str>) -> io::Result<Vec<&str>> {
    let re = Regex::new(r"^\s*#\s*Keep\s*sorted\.\s*$")
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
            block::sort(&mut block, SortStrategy::Default);
            output_lines.append(&mut block);
            output_lines.push(line);
        } else if is_sorting_block {
            block.push(line);
        } else {
            output_lines.push(line);
        }
    }

    if is_sorting_block {
        block::sort(&mut block, SortStrategy::Default);
        output_lines.append(&mut block);
    }

    Ok(output_lines)
}

pub fn process_lines_bazel(lines: Vec<&str>) -> io::Result<Vec<&str>> {
    let re = Regex::new(r"^\s*#\s*Keep\s*sorted\.\s*$")
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let mut output_lines = Vec::new();
    let mut block = Vec::<&str>::new();
    let mut is_scope = false;
    let mut is_sorting_block = false;

    for line in lines {
        // Trim the input line
        let trimmed = line.trim();

        // Find and remove the portion of the line starting from the '#' character
        let line_without_comment = trimmed.split('#').next().unwrap_or("").trim();

        if line_without_comment.contains('[') {
            is_scope = true;
            output_lines.push(line);
        } else if is_scope {
            if re.is_match(line) {
                is_sorting_block = true;
                output_lines.push(line);
            } else if is_sorting_block
                && (line_without_comment.contains(']') || line.trim().is_empty())
            {
                is_sorting_block = false;
                sort(&mut block, SortStrategy::Bazel);
                output_lines.append(&mut block);
                output_lines.push(line);
            } else if is_sorting_block {
                block.push(line);
            } else {
                output_lines.push(line);
            }
        } else {
            output_lines.push(line);
        }
    }

    if is_sorting_block {
        sort(&mut block, SortStrategy::Bazel);
        output_lines.append(&mut block);
    }

    Ok(output_lines)
}
