use crate::bazel::{is_bazel, process_lines_bazel};
use crate::cargo_toml::{is_cargo_toml, process_lines_cargo_toml};
use crate::generic::process_lines_generic;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub use crate::block::SortStrategy;

mod bazel;
mod block;
mod cargo_toml;
mod generic;

pub fn process_file(path: &Path) -> io::Result<()> {
    let mut content = std::fs::read_to_string(path)?;
    let ends_with_newline = content.ends_with('\n');
    if !ends_with_newline {
        // Add trailing '\n' so all the lines have it.
        content.push('\n');
    }
    let lines: Vec<&str> = content.split_inclusive('\n').collect();

    // Check the file extension
    let output_lines = if is_bazel(path) {
        process_lines(SortStrategy::Bazel, lines)?
    } else if is_cargo_toml(path) {
        process_lines(SortStrategy::CargoToml, lines)?
    } else {
        process_lines(SortStrategy::Generic, lines)?
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

pub fn process_lines(strategy: SortStrategy, lines: Vec<&str>) -> io::Result<Vec<&str>> {
    match strategy {
        SortStrategy::Generic => process_lines_generic(lines),
        SortStrategy::Bazel => process_lines_bazel(lines),
        SortStrategy::CargoToml => process_lines_cargo_toml(lines),
    }
}
