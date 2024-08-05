use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub mod strategies;

pub fn process_file(path: &Path) -> io::Result<()> {
    let mut content = fs::read_to_string(path)?;
    let ends_with_newline = content.ends_with('\n');
    if !ends_with_newline {
        content.push('\n');
    }

    let mut lines: Vec<String> = content.split_inclusive('\n').map(String::from).collect();
    process_lines(classify(path), &mut lines)?;

    let mut writer = BufWriter::new(File::create(path)?);
    for (i, line) in lines.iter().enumerate() {
        write!(
            writer,
            "{}",
            if i + 1 == lines.len() && !ends_with_newline {
                line.trim_end_matches('\n')
            } else {
                line
            }
        )?;
    }

    writer.flush()
}

pub enum Strategy {
    Generic,
    Bazel,
    CargoToml,
}

pub fn process_lines(strategy: Strategy, lines: &mut [String]) -> io::Result<()> {
    match strategy {
        Strategy::Generic => crate::strategies::generic::process(lines),
        Strategy::Bazel => crate::strategies::bazel::process(lines),
        Strategy::CargoToml => crate::strategies::cargo_toml::process(lines),
    }
}

fn classify(path: &Path) -> Strategy {
    match path {
        _ if is_bazel(path) => Strategy::Bazel,
        _ if is_cargo_toml(path) => Strategy::CargoToml,
        _ => Strategy::Generic,
    }
}

fn is_bazel(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(ext, "bazel" | "bzl" | "BUILD" | "WORKSPACE"),
        None => false,
    }
}

fn is_cargo_toml(path: &Path) -> bool {
    // Check if the path is a file and its file name is "Cargo.toml"
    path.is_file() && path.file_name() == Some(std::ffi::OsStr::new("Cargo.toml"))
}
