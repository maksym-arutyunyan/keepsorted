use clap::{arg, command, Parser, ValueEnum};
use keepsorted::process_file;
use std::io::{self};
use std::path::Path;

fn about() -> String {
    format!(
        "{}\nThis tool sorts lines in blocks marked with '# Keep sorted'. Use --mode fix or --mode check and enable extra features with flags. {}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_REPOSITORY")
    )
}

/// Formatting mode controlling whether the file is overwritten or only verified.
#[derive(Copy, Clone, Debug, ValueEnum)]
enum Mode {
    /// Verify that the file is already sorted.
    Check,
    /// Rewrite the file with sorted content.
    Fix,
}

#[derive(Debug, Parser)]
#[command(
    version,
    about = about(),
    long_about = None
)]
struct Args {
    #[arg(
        short = 'p',
        long,
        value_name = "PATH",
        conflicts_with = "positional_path",
        help = "Path to the file to run on. This option is mutually exclusive with the positional path."
    )]
    path: Option<String>,

    #[arg(
        value_name = "PATH",
        required_unless_present = "path",
        help = "Path to the file to run on. This is required if the -p option is not used."
    )]
    positional_path: Option<String>,

    #[arg(
        short = 'f',
        long,
        value_name = "FEATURE",
        use_value_delimiter = true,
        help = "Experimental feature flags. Provide a list of features to enable."
    )]
    features: Option<Vec<String>>,

    /// Formatting mode: check or fix (default fix)
    #[arg(
        short = 'm',
        long,
        value_enum,
        default_value_t = Mode::Fix,
        help = "formatting mode: check or fix (default fix)"
    )]
    mode: Mode,
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
    let sorted = process_file(path, features).map_err(|e| {
        eprintln!(
            "{}: failed to process file {}: {}",
            env!("CARGO_PKG_NAME"),
            path.display(),
            e
        );
        e
    })?;

    match args.mode {
        Mode::Check => {
            let original = std::fs::read_to_string(path)?;
            if original != sorted {
                std::process::exit(1);
            }
        }
        Mode::Fix => {
            std::fs::write(path, sorted)?;
        }
    }

    Ok(())
}
