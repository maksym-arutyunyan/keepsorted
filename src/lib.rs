use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Component, Path};

pub mod strategies;

pub fn process_file(path: &Path, features: Vec<String>) -> io::Result<()> {
    let mut content = fs::read_to_string(path)?;
    let ends_with_newline = content.ends_with('\n');
    if !ends_with_newline {
        // Ensure content ends with a newline to support line reordering.
        content.push('\n');
    }

    let lines: Vec<_> = content.split_inclusive('\n').map(String::from).collect();
    let output_lines = process_lines(classify(path, features), lines)?;

    let mut writer = BufWriter::new(File::create(path)?);
    for (i, line) in output_lines.iter().enumerate() {
        write!(
            writer,
            "{}",
            if i + 1 == output_lines.len() && !ends_with_newline {
                // Remove the newline if it wasn’t in the original.
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
    Gitignore,
}

pub fn process_lines(strategy: Strategy, lines: Vec<String>) -> io::Result<Vec<String>> {
    match strategy {
        Strategy::Generic => crate::strategies::generic::process(lines),
        Strategy::Bazel => crate::strategies::bazel::process(lines),
        Strategy::CargoToml => crate::strategies::cargo_toml::process(lines),
        Strategy::Gitignore => crate::strategies::gitignore::process(lines),
    }
}

fn classify(path: &Path, features: Vec<String>) -> Strategy {
    if is_bazel(path) {
        return Strategy::Bazel;
    }
    if features.contains(&"cargo_toml".to_string()) && is_cargo_toml(path) {
        return Strategy::CargoToml;
    }
    if features.contains(&"gitignore".to_string()) && is_gitignore(path) {
        return Strategy::Gitignore;
    }
    if features.contains(&"codeowners".to_string()) && is_codeowners(path) {
        return Strategy::Gitignore;
    }
    Strategy::Generic
}

fn is_bazel(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(ext, "bazel" | "bzl" | "BUILD" | "WORKSPACE"),
        None => false,
    }
}

fn is_cargo_toml(path: &Path) -> bool {
    path.is_file() && path.file_name() == Some(std::ffi::OsStr::new("Cargo.toml"))
}

fn is_gitignore(path: &Path) -> bool {
    path.is_file() && path.file_name() == Some(std::ffi::OsStr::new(".gitignore"))
}

fn is_codeowners(path: &Path) -> bool {
    // Check if the path is a file and has the name 'CODEOWNERS'
    if !path.is_file() || path.file_name().map_or(true, |name| name != "CODEOWNERS") {
        return false;
    }

    // Check if any of the parent directories is '.github' or '.gitlab'
    let mut components = path.components().rev(); // Reverse to process from the end (file -> directories)

    // Skip the file component, process only directories
    if components.next().is_none() {
        // No components found, path is just a file name
        return false;
    }

    for component in components {
        if let Component::Normal(name) = component {
            if name == ".github" || name == ".gitlab" {
                return true; // Found one of the target directories
            }
        }
    }

    false // None of the directories matched
}
