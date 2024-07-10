use clap::{arg, command, Parser};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

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

fn process_file(path: &Path) -> io::Result<()> {
    let mut content = std::fs::read_to_string(path)?;
    let ends_with_newline = content.ends_with('\n');

    if !ends_with_newline {
        // Add trailing '\n' so all the lines have it.
        content.push('\n');
    }

    let lines: Vec<&str> = content.split_inclusive('\n').collect();

    let re =
        Regex::new(r" Keep sorted").map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let mut output_lines = Vec::new();
    let mut block = Vec::new();
    let mut is_sorting_block = false;

    for line in lines {
        if re.is_match(line) {
            is_sorting_block = true;
            output_lines.push(line);
        } else if is_sorting_block && line.trim().is_empty() {
            is_sorting_block = false;
            block.sort_unstable();
            output_lines.extend(block.drain(..));
            output_lines.push(line);
        } else if is_sorting_block {
            block.push(line);
        } else {
            output_lines.push(line);
        }
    }

    if is_sorting_block {
        block.sort_unstable();
        output_lines.extend(block.drain(..));
    }

    let output_file = File::create(path)?;
    let mut writer = BufWriter::new(output_file);

    let n = output_lines.len();
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
