#![doc = include_str!("../README.md")]

use regex::Regex;
use std::fs;
use std::io;
use std::path::Path;
use std::sync::LazyLock;

mod strategies;

#[cfg(test)]
mod tests;

static RE_KEEP_SORTED: LazyLock<Regex> = LazyLock::new(re_keyword_keep_sorted);
static RE_IGNORE_FILE: LazyLock<Regex> = LazyLock::new(re_keyword_ignore_file);
static RE_IGNORE_BLOCK: LazyLock<Regex> = LazyLock::new(re_keyword_ignore_block);

/// Returns `(original, sorted)` content of a file using an appropriate strategy.
///
/// The `features` list enables optional experimental strategies.
/// Both strings are derived from a single file read.
pub fn process_file(path: &Path, features: &[String]) -> io::Result<(String, String)> {
    let original = fs::read_to_string(path)?;
    let ends_with_newline = original.ends_with('\n');

    // Build lines guaranteed to end with '\n', without cloning the full content.
    let lines: Vec<_> = if ends_with_newline {
        original.split_inclusive('\n').map(String::from).collect()
    } else {
        let mut lines: Vec<_> = original.split_inclusive('\n').map(String::from).collect();
        if let Some(last) = lines.last_mut() {
            last.push('\n');
        } else {
            lines.push("\n".to_string());
        }
        lines
    };
    let strategy = classify(path, features)?;
    let output_lines = process_lines(strategy, lines)?;

    let mut sorted = String::new();
    for (i, line) in output_lines.iter().enumerate() {
        if i + 1 == output_lines.len() && !ends_with_newline {
            sorted.push_str(line.trim_end_matches('\n'));
        } else {
            sorted.push_str(line);
        }
    }

    Ok((original, sorted))
}

/// Available sorting strategies.
///
/// `Strategy` values describe how `process_lines` will sort a file.
#[derive(Copy, Clone, Debug)]
pub(crate) enum Strategy {
    /// Generic text sorting activated by the `# Keep sorted` comment.
    Generic,
    /// Sorting for Bazel `BUILD` and `.bzl` files.
    Bazel,
    /// Sorting for the dependency sections of `Cargo.toml`.
    CargoToml,
    /// Sorting for `.gitignore` and `CODEOWNERS` files.
    Gitignore,
    /// Alphabetical ordering for `#[derive(...)]` attributes in Rust code.
    RustDeriveAlphabetical,
    /// Canonical ordering for `#[derive(...)]` attributes in Rust code.
    RustDeriveCanonical,
}

/// Sorts `lines` according to the chosen [`Strategy`] and returns the reordered lines.
pub(crate) fn process_lines(strategy: Strategy, lines: Vec<String>) -> io::Result<Vec<String>> {
    if is_ignore_file(&lines) {
        return Ok(lines);
    }
    match strategy {
        Strategy::Generic => crate::strategies::generic::process(lines),
        Strategy::Bazel => crate::strategies::bazel::process(lines),
        Strategy::CargoToml => crate::strategies::cargo_toml::process(lines),
        Strategy::Gitignore => crate::strategies::gitignore::process(lines),
        Strategy::RustDeriveAlphabetical | Strategy::RustDeriveCanonical => {
            crate::strategies::rust_derive::process(lines, strategy).and_then(
                |(lines, requires_generic_sort)| {
                    if requires_generic_sort {
                        crate::strategies::generic::process(lines)
                    } else {
                        Ok(lines)
                    }
                },
            )
        }
    }
}

pub(crate) fn classify(path: &Path, features: &[String]) -> io::Result<Strategy> {
    if is_bazel(path) {
        return Ok(Strategy::Bazel);
    }
    if is_cargo_toml(path) {
        return Ok(Strategy::CargoToml);
    }
    if features.iter().any(|f| f == "gitignore") && is_gitignore(path) {
        return Ok(Strategy::Gitignore);
    }
    if features.iter().any(|f| f == "codeowners") && is_codeowners(path) {
        return Ok(Strategy::Gitignore);
    }
    if is_rust(path) {
        match (
            features.iter().any(|f| f == "rust_derive_alphabetical"),
            features.iter().any(|f| f == "rust_derive_canonical"),
        ) {
            (true, true) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Mutually exclusive rust_derive feature flags are not allowed",
                ))
            }
            (true, false) => return Ok(Strategy::RustDeriveAlphabetical),
            (false, true) => return Ok(Strategy::RustDeriveCanonical),
            _ => {}
        }
    }
    Ok(Strategy::Generic)
}

fn is_ignore_file(lines: &[String]) -> bool {
    lines.iter().any(|x| RE_IGNORE_FILE.is_match(x))
}

fn is_ignore_block(lines: &[String]) -> bool {
    lines.iter().any(|x| RE_IGNORE_BLOCK.is_match(x))
}

pub(crate) fn is_ignore_block_line(line: &str) -> bool {
    RE_IGNORE_BLOCK.is_match(line)
}

fn is_bazel(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(ext, "bazel" | "bzl"),
        None => matches!(
            path.file_name().and_then(|s| s.to_str()),
            Some("BUILD") | Some("WORKSPACE")
        ),
    }
}

fn is_cargo_toml(path: &Path) -> bool {
    path.file_name() == Some(std::ffi::OsStr::new("Cargo.toml"))
}

fn is_gitignore(path: &Path) -> bool {
    path.file_name() == Some(std::ffi::OsStr::new(".gitignore"))
}

fn is_codeowners(path: &Path) -> bool {
    path.file_name() == Some(std::ffi::OsStr::new("CODEOWNERS"))
}

fn is_rust(path: &Path) -> bool {
    path.extension() == Some(std::ffi::OsStr::new("rs"))
}

pub(crate) fn re_keyword_keep_sorted() -> Regex {
    Regex::new(r"(?i)^\s*(#|\/\/|--)(\s*keepsorted\s*:)?\s*keep\s+sorted\s*\.?\s*$")
        .expect("Failed to build regex for keep sorted")
}

pub(crate) fn re_keyword_ignore_file() -> Regex {
    Regex::new(r"(?i)^\s*(#|\/\/|--)\s*keepsorted\s*:\s*ignore\s+file\s*\.?\s*$")
        .expect("Failed to build regex for ignore file")
}

pub(crate) fn re_keyword_ignore_block() -> Regex {
    Regex::new(r"(?i)^\s*(#|\/\/|--)\s*keepsorted\s*:\s*ignore\s+block\s*\.?\s*$")
        .expect("Failed to build regex for ignore block")
}
