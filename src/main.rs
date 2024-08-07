use clap::{arg, command, Parser};
use keepsorted::process_file;
use std::io::{self};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "PATH", conflicts_with = "positional_path")]
    path: Option<String>,

    #[arg(value_name = "PATH", required_unless_present = "path")]
    positional_path: Option<String>,

    #[arg(short, long, value_name = "FEATURE", use_value_delimiter = true)]
    features: Option<Vec<String>>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Get the path from either the option or the positional argument
    let file_path = args
        .path
        .or(args.positional_path)
        .expect("Path must be provided");

    let path = Path::new(&file_path);

    if path.is_dir() {
        eprintln!(
            "{}: read {}: is a directory",
            env!("CARGO_PKG_NAME"),
            path.display()
        );
        std::process::exit(1);
    }

    // Check for experimental features
    let features = args.features.unwrap_or_default();
    let cargo_toml_enabled = features.contains(&"cargo_toml".to_string());

    process_file(path, cargo_toml_enabled).map_err(|e| {
        eprintln!(
            "{}: failed to process file {}: {}",
            env!("CARGO_PKG_NAME"),
            path.display(),
            e
        );
        e
    })
}
