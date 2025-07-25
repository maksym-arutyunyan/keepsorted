use clap::{arg, command, Parser, ValueEnum};
use keepsorted::process_file;
use std::io;
use std::path::Path;
use std::process::{self, Command};

/// Exit code used when the command is invoked incorrectly.
///
/// clap also exits with this code on usage errors such as conflicting flags.
const EXIT_USAGE_ERROR: i32 = 2;
/// Exit code used for I/O problems or internal bugs.
const EXIT_RUNTIME_ERROR: i32 = 3;
/// Exit code used when `--mode check` or `--mode diff` detects unsorted files.
const EXIT_CHECK_FAILED: i32 = 4;

fn about() -> String {
    format!(
        "{}\nSort lines inside '# Keep sorted' blocks. Use --check to verify, --diff to preview, or --fix to apply changes.",
        env!("CARGO_PKG_DESCRIPTION")
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

/// Experimental features controlled via `--features`.
#[derive(Copy, Clone, Debug, ValueEnum)]
#[clap(rename_all = "snake")]
enum Feature {
    /// Enable sorting for `.gitignore` files.
    Gitignore,
    /// Enable sorting for `CODEOWNERS` files.
    Codeowners,
    /// Alphabetical ordering for `#[derive(...)]` attributes.
    RustDeriveAlphabetical,
    /// Canonical ordering for `#[derive(...)]` attributes.
    RustDeriveCanonical,
}

impl Feature {
    fn as_str(self) -> &'static str {
        match self {
            Feature::Gitignore => "gitignore",
            Feature::Codeowners => "codeowners",
            Feature::RustDeriveAlphabetical => "rust_derive_alphabetical",
            Feature::RustDeriveCanonical => "rust_derive_canonical",
        }
    }
}

use std::fmt;

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Parser)]
#[command(
    version,
    about = about(),
    long_about = None,
    after_help = "For more info, visit: https://github.com/maksym-arutyunyan/keepsorted"
)]
struct Args {
    #[arg(
        short = 'p',
        long,
        value_name = "PATH",
        conflicts_with = "positional_path",
        help = "File to process (conflicts with positional path)"
    )]
    path: Option<String>,

    #[arg(
        value_name = "PATH",
        required_unless_present = "path",
        help = "File to process (required if '--path' is not used)"
    )]
    positional_path: Option<String>,

    #[arg(
        short = 'f',
        long,
        value_name = "FEATURE",
        value_enum,
        use_value_delimiter = true,
        help = "Enable experimental features"
    )]
    features: Option<Vec<Feature>>,

    /// Verify that the file is already sorted
    #[arg(
        long,
        conflicts_with_all = ["diff", "fix", "mode"],
        help = "alias for '--mode check'",
    )]
    check: bool,

    /// Print a diff of the required changes
    #[arg(
        long,
        conflicts_with_all = ["check", "fix", "mode"],
        help = "alias for '--mode diff'",
    )]
    diff: bool,

    /// Rewrite the file with sorted content
    #[arg(
        long,
        conflicts_with_all = ["check", "diff", "mode"],
        help = "alias for '--mode fix'",
    )]
    fix: bool,

    /// Formatting mode controlling how files are processed
    #[arg(
        short = 'm',
        long,
        value_enum,
        default_value_t = Mode::Fix,
        conflicts_with_all = ["check", "diff", "fix"],
        help = "Formatting mode"
    )]
    mode: Mode,

    /// Command to run for '--mode diff'
    #[arg(
        long,
        value_name = "COMMAND",
        help = "Custom command for '--mode diff'"
    )]
    diff_command: Option<String>,
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

    // Check for experimental features
    let features = args.features.unwrap_or_default();
    let mut exit_code = 0;

    if path.is_dir() {
        eprintln!(
            "{}: read {}: is a directory",
            env!("CARGO_PKG_NAME"),
            path.display()
        );
        process::exit(EXIT_USAGE_ERROR);
    } else if !handle_file(path, &features, mode, args.diff_command.as_deref()) {
        exit_code = EXIT_CHECK_FAILED;
    }

    if exit_code != 0 {
        process::exit(exit_code);
    }
}

fn handle_file(path: &Path, features: &[Feature], mode: Mode, diff_command: Option<&str>) -> bool {
    let feature_names: Vec<String> = features.iter().map(|f| f.as_str().to_string()).collect();
    let sorted = match process_file(path, feature_names) {
        Ok(s) => s,
        Err(e) => {
            if e.kind() == io::ErrorKind::InvalidInput {
                eprintln!("{}: {}", env!("CARGO_PKG_NAME"), e);
                process::exit(EXIT_USAGE_ERROR);
            } else {
                eprintln!(
                    "{}: failed to process file {}: {}",
                    env!("CARGO_PKG_NAME"),
                    path.display(),
                    e
                );
                process::exit(EXIT_RUNTIME_ERROR);
            }
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
            original == sorted
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
                if let Some(cmd) = diff_command {
                    use tempfile::NamedTempFile;
                    let tmp = NamedTempFile::new().expect("create temp file");
                    if let Err(e) = std::fs::write(tmp.path(), &sorted) {
                        eprintln!(
                            "{}: failed to write temp file {}: {}",
                            env!("CARGO_PKG_NAME"),
                            tmp.path().display(),
                            e
                        );
                        process::exit(EXIT_RUNTIME_ERROR);
                    }
                    let parts = match shell_words::split(cmd) {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!(
                                "{}: failed to parse diff command '{}': {}",
                                env!("CARGO_PKG_NAME"),
                                cmd,
                                e
                            );
                            process::exit(EXIT_RUNTIME_ERROR);
                        }
                    };
                    let (prog, args) = parts.split_first().unwrap_or_else(|| {
                        eprintln!("{}: diff command is empty", env!("CARGO_PKG_NAME"));
                        process::exit(EXIT_RUNTIME_ERROR);
                    });
                    let output = Command::new(prog)
                        .args(args)
                        .arg(path)
                        .arg(tmp.path())
                        .output();
                    match output {
                        Ok(out) => {
                            print!("{}", String::from_utf8_lossy(&out.stdout));
                            if !out.status.success() {
                                process::exit(out.status.code().unwrap_or(EXIT_RUNTIME_ERROR));
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "{}: failed to run diff command '{}': {}",
                                env!("CARGO_PKG_NAME"),
                                cmd,
                                e
                            );
                            process::exit(EXIT_RUNTIME_ERROR);
                        }
                    }
                    return false;
                } else {
                    use similar::TextDiff;
                    let diff = TextDiff::from_lines(&original, &sorted);
                    let old = format!("a/{}", path.display());
                    let new = format!("b/{}", path.display());
                    print!("{}", diff.unified_diff().header(&old, &new));
                    return false;
                }
            }
            true
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
            true
        }
    }
}
