use clap::{arg, command, Parser, ValueEnum};
use keepsorted::process_file;
use shell_words;
use std::io::Write;
use std::path::Path;
use std::process::{self, Command};
use tempfile::NamedTempFile;

/// Exit code used for I/O problems or internal bugs.
const EXIT_RUNTIME_ERROR: i32 = 3;
/// Exit code used when the CLI is invoked incorrectly.
/// Clap also exits with this code when it encounters command-line parsing errors.
const EXIT_USAGE_ERROR: i32 = 2;
/// Exit code used when `--mode check` or `--mode diff` detects unsorted files.
const EXIT_CHECK_FAILED: i32 = 4;

fn about() -> String {
    format!(
        "{}\nThis tool sorts lines in blocks marked with '# Keep sorted'. Use --mode check, --mode diff, or --mode fix and enable extra features with flags. {}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_REPOSITORY")
    )
}

/// Formatting mode controlling whether the file is overwritten or only verified.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq)]
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

    /// Formatting mode: check, diff, or fix (default fix)
    #[arg(
        short = 'm',
        long,
        value_enum,
        default_value_t = Mode::Fix,
        help = "formatting mode: check, diff, or fix (default fix)"
    )]
    mode: Mode,

    #[arg(
        long = "diff-command",
        value_name = "CMD",
        help = "command to run when the formatting mode is diff"
    )]
    diff_command: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Get the path from either the option or the positional argument
    let file_path = args
        .path
        .or(args.positional_path)
        .expect("Path must be provided");

    let path = Path::new(&file_path);

    if args.diff_command.is_some() && args.mode != Mode::Diff {
        eprintln!(
            "{}: --diff-command requires --mode diff",
            env!("CARGO_PKG_NAME")
        );
        process::exit(EXIT_USAGE_ERROR);
    }

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

    match args.mode {
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
                if let Some(cmdline) = args.diff_command.as_deref() {
                    let mut old_file = NamedTempFile::new().expect("tempfile");
                    old_file.write_all(original.as_bytes()).expect("write temp");
                    let mut new_file = NamedTempFile::new().expect("tempfile");
                    new_file.write_all(sorted.as_bytes()).expect("write temp");

                    let parts =
                        shell_words::split(cmdline).unwrap_or_else(|_| vec![cmdline.to_string()]);
                    let (prog, rest) = parts.split_first().expect("empty diff command");
                    let output = Command::new(prog)
                        .args(rest)
                        .arg(old_file.path())
                        .arg(new_file.path())
                        .output();
                    match output {
                        Ok(out) => {
                            print!("{}", String::from_utf8_lossy(&out.stdout));
                        }
                        Err(e) => {
                            eprintln!(
                                "{}: failed to run diff command: {}",
                                env!("CARGO_PKG_NAME"),
                                e
                            );
                            process::exit(EXIT_RUNTIME_ERROR);
                        }
                    }
                } else {
                    use similar::TextDiff;
                    let diff = TextDiff::from_lines(&original, &sorted);
                    let old = format!("a/{}", path.display());
                    let new = format!("b/{}", path.display());
                    print!("{}", diff.unified_diff().header(&old, &new));
                }
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
