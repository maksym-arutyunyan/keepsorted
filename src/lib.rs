#![doc = include_str!("../README.md")]

use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;
use std::io::{self};
use std::path::Path;

mod strategies;

static RE_KEEP_SORTED: Lazy<Regex> = Lazy::new(re_keyword_keep_sorted);
static RE_IGNORE_FILE: Lazy<Regex> = Lazy::new(re_keyword_ignore_file);
static RE_IGNORE_BLOCK: Lazy<Regex> = Lazy::new(re_keyword_ignore_block);

/// Returns the sorted content of a file using an appropriate strategy.
///
/// The `features` list enables optional experimental strategies.
pub fn process_file(path: &Path, features: Vec<String>) -> io::Result<String> {
    let mut content = fs::read_to_string(path)?;
    let ends_with_newline = content.ends_with('\n');
    if !ends_with_newline {
        // Ensure content ends with a newline to support line reordering.
        content.push('\n');
    }

    let lines: Vec<_> = content.split_inclusive('\n').map(String::from).collect();
    let strategy = classify(path, features)?;
    let output_lines = process_lines(strategy, lines)?;

    let mut result = String::new();
    for (i, line) in output_lines.iter().enumerate() {
        if i + 1 == output_lines.len() && !ends_with_newline {
            result.push_str(line.trim_end_matches('\n'));
        } else {
            result.push_str(line);
        }
    }

    Ok(result)
}

/// Available sorting strategies.
///
/// `Strategy` values describe how `process_lines` will sort a file.
#[derive(Copy, Clone)]
pub enum Strategy {
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
pub fn process_lines(strategy: Strategy, lines: Vec<String>) -> io::Result<Vec<String>> {
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

fn classify(path: &Path, features: Vec<String>) -> io::Result<Strategy> {
    if is_bazel(path) {
        return Ok(Strategy::Bazel);
    }
    if is_cargo_toml(path) {
        return Ok(Strategy::CargoToml);
    }
    if features.contains(&"gitignore".to_string()) && is_gitignore(path) {
        return Ok(Strategy::Gitignore);
    }
    if features.contains(&"codeowners".to_string()) && is_codeowners(path) {
        return Ok(Strategy::Gitignore);
    }
    if is_rust(path) {
        match (
            features.contains(&"rust_derive_alphabetical".to_string()),
            features.contains(&"rust_derive_canonical".to_string()),
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
    path.is_file() && path.file_name() == Some(std::ffi::OsStr::new("Cargo.toml"))
}

fn is_gitignore(path: &Path) -> bool {
    path.is_file() && path.file_name() == Some(std::ffi::OsStr::new(".gitignore"))
}

fn is_codeowners(path: &Path) -> bool {
    path.is_file() && path.file_name() == Some(std::ffi::OsStr::new("CODEOWNERS"))
}

fn is_rust(path: &Path) -> bool {
    path.is_file() && path.extension() == Some(std::ffi::OsStr::new("rs"))
}

fn re_keyword_keep_sorted() -> Regex {
    Regex::new(r"(?i)^\s*(#|\/\/|--)(\s*keepsorted\s*:)?\s*keep\s+sorted\s*\.?\s*$")
        .expect("Failed to build regex for keep sorted")
}

#[test]
fn test_re_keyword_keep_sorted() {
    let re = re_keyword_keep_sorted();
    for line in [
        "  #Keep sorted",
        "  # Keep sorted  ",
        "  # Keep   sorted .  ",
        "  #   keepsorted  : keep   sorted  .  ",
        "  //  Keep sorted   .  ",
        "  //keepsorted: keep sorted",
        "  //   keepsorted  : keep   sorted  .  ",
        "--keepsorted: keep sorted",
        "-- keep sorted",
    ] {
        assert!(
            re.is_match(line),
            "The regex failed to match the valid line: '{}'",
            line
        );
    }
}

fn re_keyword_ignore_file() -> Regex {
    Regex::new(r"(?i)^\s*(#|\/\/|--)\s*keepsorted\s*:\s*ignore\s+file\s*\.?\s*$")
        .expect("Failed to build regex for ignore file")
}

#[test]
fn test_re_keyword_ignore_file() {
    let re = re_keyword_ignore_file();
    for line in [
        "  #   keepsorted  : ignore   file  .  ",
        "#keepsorted:ignore file",
        "  //   keepsorted  : ignore   file  .  ",
        "//keepsorted:ignore file",
        "  --   keepsorted  : ignore   file  .  ",
        "--keepsorted:ignore file",
    ] {
        assert!(
            re.is_match(line),
            "The regex failed to match the valid line: '{}'",
            line
        );
    }
}

fn re_keyword_ignore_block() -> Regex {
    Regex::new(r"(?i)^\s*(#|\/\/|--)\s*keepsorted\s*:\s*ignore\s+block\s*\.?\s*$")
        .expect("Failed to build regex for ignore block")
}

#[test]
fn test_re_keyword_ignore_block() {
    let re = re_keyword_ignore_block();
    for line in [
        "  #   keepsorted  : ignore   block  .  ",
        "#keepsorted:ignore block",
        "  //   keepsorted  : ignore   block  .  ",
        "//keepsorted:ignore block",
        "  --   keepsorted  : ignore   block  .  ",
        "--keepsorted:ignore block",
    ] {
        assert!(
            re.is_match(line),
            "The regex failed to match the valid line: '{}'",
            line
        );
    }
}

#[test]
fn test_classify_bazel_files() {
    assert!(matches!(
        classify(Path::new("BUILD"), vec![]).unwrap(),
        Strategy::Bazel
    ));
    assert!(matches!(
        classify(Path::new("WORKSPACE"), vec![]).unwrap(),
        Strategy::Bazel
    ));
    assert!(matches!(
        classify(Path::new("foo.bazel"), vec![]).unwrap(),
        Strategy::Bazel
    ));
    assert!(matches!(
        classify(Path::new("BUILD.bazel"), vec![]).unwrap(),
        Strategy::Bazel
    ));
    assert!(matches!(
        classify(Path::new("WORKSPACE.bazel"), vec![]).unwrap(),
        Strategy::Bazel
    ));
    assert!(matches!(
        classify(Path::new("foo.bzl"), vec![]).unwrap(),
        Strategy::Bazel
    ));
}
