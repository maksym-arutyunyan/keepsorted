use clap::{Parser, arg, command};
use walkdir::WalkDir;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
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
    let path = args.path.or(args.positional_path).expect("Path must be provided");

    if Path::new(&path).is_dir() {
        for entry in WalkDir::new(path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                process_file(entry.path())?;
            }
        }
    } else {
        process_file(Path::new(&path))?;
    }

    Ok(())
}

fn process_file(path: &Path) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

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
            output_lines.extend(block.drain(..));
            output_lines.push(line);
        } else {
            if is_sorting_block {
                block.push(line);
            } else {
                output_lines.push(line);
            }
        }
    }

    if is_sorting_block {
        block.sort();
        output_lines.extend(block.drain(..));
    }

    let mut file = File::create(path)?;
    for line in output_lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
