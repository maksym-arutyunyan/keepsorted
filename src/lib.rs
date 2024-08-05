use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub mod strategies;

pub fn process_file(path: &Path) -> io::Result<()> {
    let mut content = std::fs::read_to_string(path)?;
    let ends_with_newline = content.ends_with('\n');
    if !ends_with_newline {
        // Add trailing '\n' so all the lines have it.
        content.push('\n');
    }
    let lines: Vec<&str> = content.split_inclusive('\n').collect();

    let output_lines = process_lines(classify(path), lines)?;

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

pub enum Strategy {
    Generic,
    Bazel,
    CargoToml,
}

pub fn process_lines<'a>(strategy: Strategy, lines: Vec<&'a str>) -> io::Result<Vec<&'a str>> {
    match strategy {
        Strategy::Generic => crate::strategies::generic::process(lines),
        Strategy::Bazel => crate::strategies::bazel::process(lines),
        Strategy::CargoToml => crate::strategies::cargo_toml::process(lines),
    }
}

fn classify(path: &Path) -> Strategy {
    if is_bazel(path) {
        Strategy::Bazel
    } else if is_cargo_toml(path) {
        Strategy::CargoToml
    } else {
        Strategy::Generic
    }
}

fn is_bazel(_path: &Path) -> bool {
    false
}

fn is_cargo_toml(_path: &Path) -> bool {
    false
}

