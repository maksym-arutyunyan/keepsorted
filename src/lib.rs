use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub mod strategies;

static RE_KEEP_SORTED: Lazy<Regex> = Lazy::new(re_keyword_keep_sorted);
static RE_IGNORE_FILE: Lazy<Regex> = Lazy::new(re_keyword_ignore_file);
static RE_IGNORE_BLOCK: Lazy<Regex> = Lazy::new(re_keyword_ignore_block);

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
    RustDeriveAlphabetical,
    //RustDeriveCanonical,
}

pub fn process_lines(strategy: Strategy, lines: Vec<String>) -> io::Result<Vec<String>> {
    if is_ignore_file(&lines) {
        return Ok(lines);
    }
    match strategy {
        Strategy::Generic => crate::strategies::generic::process(lines),
        Strategy::Bazel => crate::strategies::bazel::process(lines),
        Strategy::CargoToml => crate::strategies::cargo_toml::process(lines),
        Strategy::Gitignore => crate::strategies::gitignore::process(lines),
        Strategy::RustDeriveAlphabetical => crate::strategies::rust_derive::process(lines),
    }
}

fn classify(path: &Path, features: Vec<String>) -> Strategy {
    if is_bazel(path) {
        return Strategy::Bazel;
    }
    if is_cargo_toml(path) {
        return Strategy::CargoToml;
    }
    if features.contains(&"gitignore".to_string()) && is_gitignore(path) {
        return Strategy::Gitignore;
    }
    if features.contains(&"codeowners".to_string()) && is_codeowners(path) {
        return Strategy::Gitignore;
    }
    if features.contains(&"rust_derive_alphabetical".to_string()) && is_rust(path) {
        return Strategy::RustDeriveAlphabetical;
    }
    Strategy::Generic
}

fn is_ignore_file(lines: &[String]) -> bool {
    lines.iter().any(|x| RE_IGNORE_FILE.is_match(x))
}

fn is_ignore_block(lines: &[String]) -> bool {
    lines.iter().any(|x| RE_IGNORE_BLOCK.is_match(x))
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
    path.is_file() && path.file_name() == Some(std::ffi::OsStr::new("CODEOWNERS"))
}

fn is_rust(path: &Path) -> bool {
    path.is_file() && path.extension() == Some(std::ffi::OsStr::new("rs"))
}

fn re_keyword_keep_sorted() -> Regex {
    Regex::new(
        r"(?i)^\s*(#|\/\/|#\s+keepsorted\s*:|\/\/\s+keepsorted\s*:)\s*keep\s+sorted\s*\.?\s*$",
    )
    .expect("Failed to build regex for keep sorted")
}

#[test]
fn test_re_keyword_keep_sorted() {
    let re = re_keyword_keep_sorted();
    for line in [
        "  # Keep sorted  ",
        "  # Keep   sorted .  ",
        "  #   keepsorted  : keep   sorted  .  ",
        "  //  Keep sorted   .  ",
        "  //   keepsorted  : keep   sorted  .  ",
    ] {
        assert!(
            re.is_match(line),
            "The regex failed to match the valid line: '{}'",
            line
        );
    }
}

fn re_keyword_ignore_file() -> Regex {
    Regex::new(r"(?i)^\s*(#|\/\/)\s*keepsorted\s*:\s*ignore\s+file\s*\.?\s*$")
        .expect("Failed to build regex for ignore file")
}

#[test]
fn test_re_keyword_ignore_file() {
    let re = re_keyword_ignore_file();
    for line in [
        "  #   keepsorted  : ignore   file  .  ",
        "  //   keepsorted  : ignore   file  .  ",
    ] {
        assert!(
            re.is_match(line),
            "The regex failed to match the valid line: '{}'",
            line
        );
    }
}

fn re_keyword_ignore_block() -> Regex {
    Regex::new(r"(?i)^\s*(#|\/\/)\s*keepsorted\s*:\s*ignore\s+block\s*\.?\s*$")
        .expect("Failed to build regex for ignore block")
}

#[test]
fn test_re_keyword_ignore_block() {
    let re = re_keyword_ignore_block();
    for line in [
        "  #   keepsorted  : ignore   block  .  ",
        "  //   keepsorted  : ignore   block  .  ",
    ] {
        assert!(
            re.is_match(line),
            "The regex failed to match the valid line: '{}'",
            line
        );
    }
}
