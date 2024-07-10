use clap::{arg, command, Parser};
use regex::Regex;
use std::fs::File;
use std::io::{self, Write};
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

    if Path::new(&path).is_dir() {
        let project_name = env!("CARGO_PKG_NAME");
        eprintln!("{}: read {}: is a directory", project_name, path);
        std::process::exit(1);
    } else {
        process_file(Path::new(&path))?;
    }

    Ok(())
}

fn process_file(path: &Path) -> io::Result<()> {
    let mut content = std::fs::read_to_string(path)?;
    let ends_with_newline = content.ends_with('\n');
    if !ends_with_newline {
        // Add trailing '\n' so all the lines have it.
        content.push_str("\n");
    }
    let lines: Vec<&str> = content.split_inclusive('\n').collect();

    let re = Regex::new(r" Keep sorted").unwrap();
    let mut output_lines = Vec::new();
    let mut is_sorting_block = false;
    let mut block = Vec::new();

    for line in lines {
        if re.is_match(&line) {
            is_sorting_block = true;
            output_lines.push(line);
        } else if is_sorting_block && line.trim().is_empty() {
            is_sorting_block = false;
            block.sort();
            output_lines.append(&mut block);
            output_lines.push(line);
        } else if is_sorting_block {
            block.push(line);
        } else {
            output_lines.push(line);
        }
    }

    if is_sorting_block {
        block.sort();
        output_lines.append(&mut block);
    }

    let mut file = File::create(path)?;
    let n = output_lines.len();
    for (i, line) in output_lines.iter().enumerate() {
        if i+1 == n && !ends_with_newline {
            // Remove trailing '\n' since there were none in the source.
            write!(file, "{}", line.trim_end_matches('\n'))?;
        } else {
            write!(file, "{}", line)?;
        }
    }

    Ok(())
}
