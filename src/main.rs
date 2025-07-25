use clap::{arg, command, Parser, ValueEnum};
use keepsorted::process_file;
use std::path::Path;
use std::process;

/// Exit code used for I/O problems or internal bugs.
const EXIT_RUNTIME_ERROR: i32 = 3;
/// Exit code used when `--mode check` or `--mode diff` detects unsorted files.
const EXIT_CHECK_FAILED: i32 = 4;

fn about() -> String {
    format!(
        "{}\nThis tool sorts lines in blocks marked with '# Keep sorted'. Use --mode check (or --check), --mode diff (or --diff), or --mode fix (or --fix) and enable extra features with flags. {}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_REPOSITORY")
    )
}

/// Formatting mode controlling whether the file is overwritten or only verified.
#[derive(Copy, Clone, Debug, ValueEnum)]
enum Mode {
    /// Verify that the file is already sorted.
    Check,
    /// Print a diff of the required changes.
    Diff,
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

    /// Verify that the file is already sorted
    #[arg(
        long,
        conflicts_with_all = ["diff", "fix", "mode"],
        help = "alias for `--mode check`",
    )]
    check: bool,

    /// Print a diff of the required changes
    #[arg(
        long,
        conflicts_with_all = ["check", "fix", "mode"],
        help = "alias for `--mode diff`",
    )]
    diff: bool,

    /// Rewrite the file with sorted content
    #[arg(
        long,
        conflicts_with_all = ["check", "diff", "mode"],
        help = "alias for `--mode fix`",
    )]
    fix: bool,

    /// Formatting mode: check, diff, or fix (default fix)
    #[arg(
        short = 'm',
        long,
        value_enum,
        default_value_t = Mode::Fix,
        conflicts_with_all = ["check", "diff", "fix"],
        help = "formatting mode: check, diff, or fix (default fix)"
    )]
    mode: Mode,
}

fn main() {
    let args = Args::parse();

    let mode = if args.check {
        Mode::Check
    } else if args.diff {
        Mode::Diff
    } else if args.fix {
        Mode::Fix
    } else {
        args.mode
    };

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
        process::exit(EXIT_RUNTIME_ERROR);
    }

    // Check for experimental features
    let features = args.features.unwrap_or_default();
    let sorted = match process_file(path, features) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "{}: failed to process file {}: {}",
                env!("CARGO_PKG_NAME"),
                path.display(),
                e
            );
            process::exit(EXIT_RUNTIME_ERROR);
        }
    };

    match mode {
        Mode::Check => {
            let original = match std::fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!(
                        "{}: failed to read file {}: {}",
                        env!("CARGO_PKG_NAME"),
                        path.display(),
                        e
                    );
                    process::exit(EXIT_RUNTIME_ERROR);
                }
            };
            if original != sorted {
                process::exit(EXIT_CHECK_FAILED);
            }
        }
        Mode::Diff => {
            let original = match std::fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!(
                        "{}: failed to read file {}: {}",
                        env!("CARGO_PKG_NAME"),
                        path.display(),
                        e
                    );
                    process::exit(EXIT_RUNTIME_ERROR);
                }
            };
            if original != sorted {
                use similar::TextDiff;
                let diff = TextDiff::from_lines(&original, &sorted);
                let old = format!("a/{}", path.display());
                let new = format!("b/{}", path.display());
                print!("{}", diff.unified_diff().header(&old, &new));
                process::exit(EXIT_CHECK_FAILED);
            }
        }
        Mode::Fix => {
            if let Err(e) = std::fs::write(path, sorted) {
                eprintln!(
                    "{}: failed to write file {}: {}",
                    env!("CARGO_PKG_NAME"),
                    path.display(),
                    e
                );
                process::exit(EXIT_RUNTIME_ERROR);
            }
        }
    }
}
