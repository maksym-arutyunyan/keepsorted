use clap::{arg, command, Parser};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use crate::block::{SortStrategy, sort};

mod block;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "PATH")]
    path: Option<String>,

    #[arg(value_name = "PATH", required_unless_present = "path")]
    positional_path: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Get the path from either the option or the positional argument
    let path = args
        .path
        .or(args.positional_path)
        .expect("Path must be provided");

    let path = Path::new(&path);

    if path.is_dir() {
        eprintln!(
            "{}: read {}: is a directory",
            env!("CARGO_PKG_NAME"),
            path.display()
        );
        std::process::exit(1);
    }

    process_file(path)
}

fn is_bazel_related(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(ext, "bazel" | "bzl" | "BUILD" | "WORKSPACE"),
        None => false,
    }
}

fn process_file(path: &Path) -> io::Result<()> {
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

fn process_lines(lines: Vec<&str>) -> io::Result<Vec<&str>> {
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

fn process_lines_bazel(lines: Vec<&str>) -> io::Result<Vec<&str>> {
    let re = Regex::new(r"^\s*#\s*Keep\s*sorted\.\s*$")
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let mut output_lines = Vec::new();
    let mut block = Vec::<&str>::new();
    let mut is_scope = false;
    let mut is_sorting_block = false;

    fn is_sorting_block_end(line: &str) -> bool {
        line.contains(']') || line.trim().is_empty()
    }

    for line in lines {
        if line.contains('[') {
            is_scope = true;
            output_lines.push(line);
        } else if is_scope {
            if re.is_match(line) {
                is_sorting_block = true;
                output_lines.push(line);
            } else if is_sorting_block && is_sorting_block_end(line) {
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

#[cfg(test)]
mod main_tests {
    use super::*;

    // Helper function to hide text-lines conversion.
    fn process_text(text: &str) -> io::Result<String> {
        let lines: Vec<&str> = text.lines().collect();
        let processed_lines = process_lines(lines)?;
        Ok(processed_lines.join("\n"))
    }

    fn process_text_bazel(text: &str) -> io::Result<String> {
        let lines: Vec<&str> = text.lines().collect();
        let processed_lines = process_lines_bazel(lines)?;
        Ok(processed_lines.join("\n"))
    }

    #[test]
    fn empty() {
        let input = "";
        let expected = "";
        let result = process_text(input).unwrap();
        assert!(result == expected, "Expected: {expected}\nActual: {result}");
    }

    #[test]
    fn single_letter() {
        let input = "
            a
        ";
        let expected = "
            a
        ";
        let result = process_text(input).unwrap();
        assert!(result == expected, "Expected: {expected}\nActual: {result}");
    }

    #[test]
    fn no_comment() {
        let input = "
            b
            a
        ";
        let expected = "
            b
            a
        ";
        let result = process_text(input).unwrap();
        assert!(result == expected, "Expected: {expected}\nActual: {result}");
    }

    #[test]
    fn simple_block() {
        let input = "
            # Keep sorted.
            b
            a
        ";
        let expected = "
            # Keep sorted.
            a
            b
        ";
        let result = process_text(input).unwrap();
        assert!(result == expected, "Expected: {expected}\nActual: {result}");
    }

    #[test]
    fn blocks_divided_by_newline() {
        let input = "
            # Keep sorted.
            d
            c

            b
            a
        ";
        let expected = "
            # Keep sorted.
            c
            d

            b
            a
        ";
        let result = process_text(input).unwrap();
        assert!(result == expected, "Expected: {expected}\nActual: {result}");
    }

    #[test]
    fn bazel_block() {
        let input = r#"
            block = [
                # Keep sorted.
                "b",
                "a",
            ]
        "#;
        let expected = r#"
            block = [
                # Keep sorted.
                "a",
                "b",
            ]
        "#;
        let result = process_text_bazel(input).unwrap();
        assert!(result == expected, "Expected: {expected}\nActual: {result}");
    }

    #[test]
    #[ignore]
    fn bazel_block_with_comment() {
        let input = r#"
            block = [
                # Keep sorted.
                "d",
                # Some comment about the line below.
                "c",
                "b",
                "a",
            ]
        "#;
        let expected = r#"
            block = [
                # Keep sorted.
                "a",
                "b",
                # Some comment about the line below.
                "c",
                "d",
            ]
        "#;
        let result = process_text_bazel(input).unwrap();
        assert!(result == expected, "Expected: {expected}\nActual: {result}");
    }

    #[test]
    fn bazel_blocks() {
        let input = r#"
            block_1 = [
                # Keep sorted.
                "b",
                "a",
            ],
            block_2 = [
                "y",
                "x",
            ],
        "#;
        let expected = r#"
            block_1 = [
                # Keep sorted.
                "a",
                "b",
            ],
            block_2 = [
                "y",
                "x",
            ],
        "#;
        let result = process_text_bazel(input).unwrap();
        assert!(result == expected, "Expected: {expected}\nActual: {result}");
    }

    #[test]
    fn bazel_blocks_select() {
        let input = r#"
            deps = [
                # Keep sorted.
                "b",
                "a",
            ] + select({
                "@platforms//os:osx": [
                    # Keep sorted.
                    "y",
                    "x",
                ],
                "//conditions:default": [
                    # Keep sorted.
                    "m",
                    "k",
                ],
            })
        "#;
        let expected = r#"
            deps = [
                # Keep sorted.
                "a",
                "b",
            ] + select({
                "@platforms//os:osx": [
                    # Keep sorted.
                    "x",
                    "y",
                ],
                "//conditions:default": [
                    # Keep sorted.
                    "k",
                    "m",
                ],
            })
        "#;
        let result = process_text_bazel(input).unwrap();
        assert!(result == expected, "Expected: {expected}\nActual: {result}");
    }

    #[test]
    fn bazel_order() {
        let input = r#"
            block = [
                # Keep sorted.
                ":b",
                ":a",
                "//path/b",
                "//path/a",
                "@crate_index//:b",
                "@crate_index//:a",
            ]
        "#;
        let expected = r#"
            block = [
                # Keep sorted.
                ":a",
                ":b",
                "//path/a",
                "//path/b",
                "@crate_index//:a",
                "@crate_index//:b",
            ]
        "#;
        let result = process_text_bazel(input).unwrap();
        assert!(result == expected, "Expected: {expected}\nActual: {result}");
    }
}
