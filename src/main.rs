use clap::{arg, command, Parser, ValueEnum};
use keepsorted::process_file;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

/// Exit code used when the command is invoked incorrectly.
///
/// clap also exits with this code on usage errors such as conflicting flags.
const EXIT_USAGE_ERROR: i32 = 2;
/// Exit code used for syntax errors in the input.
const EXIT_SYNTAX_ERROR: i32 = 1;
/// Exit code used for I/O problems or internal bugs.
const EXIT_RUNTIME_ERROR: i32 = 3;
/// Exit code used when `--mode check` or `--mode diff` detects unsorted files.
const EXIT_CHECK_FAILED: i32 = 4;

fn about() -> String {
    env!("CARGO_PKG_DESCRIPTION").to_string()
}

fn long_about() -> String {
    format!(
        "{}\n\
\n\
Sort lists inside '# Keep sorted' blocks. Generic and Bazel files require the comment. \
Cargo.toml, .gitignore and CODEOWNERS are sorted automatically. \
Skip sorting with '# keepsorted: ignore file' or '# keepsorted: ignore block'. \
Comments starting with '#', '//' or '--' are preserved.\n\
\n\
Return codes:\n\
\t0: success, everything went well\n\
\t1: syntax errors in input\n\
\t2: usage errors: invoked incorrectly\n\
\t3: unexpected runtime errors: file I/O problems or internal bugs\n\
\t4: check mode failed (reformat is needed)",
        env!("CARGO_PKG_DESCRIPTION")
    )
}

fn after_help() -> &'static str {
    "For more info, visit: https://github.com/maksym-arutyunyan/keepsorted"
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
    long_about = long_about(),
    after_help = after_help()
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

    /// Recursively traverse directories for files
    #[arg(short = 'r', long, help = "Process directories recursively")]
    recursive: bool,

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
        if !args.recursive {
            eprintln!(
                "{}: read {}: is a directory",
                env!("CARGO_PKG_NAME"),
                path.display()
            );
            process::exit(EXIT_USAGE_ERROR);
        }
        let mut files = Vec::new();
        if let Err(e) = collect_files(path, &mut files) {
            eprintln!(
                "{}: failed to read directory {}: {}",
                env!("CARGO_PKG_NAME"),
                path.display(),
                e
            );
            process::exit(EXIT_RUNTIME_ERROR);
        }
        for file in files {
            if !handle_file(&file, &features, mode, args.diff_command.as_deref()) {
                exit_code = EXIT_CHECK_FAILED;
            }
        }
    } else if !handle_file(path, &features, mode, args.diff_command.as_deref()) {
        exit_code = EXIT_CHECK_FAILED;
    }

    if exit_code != 0 {
        process::exit(exit_code);
    }
}

fn handle_file(path: &Path, features: &[Feature], mode: Mode, diff_command: Option<&str>) -> bool {
    match is_text_file(path) {
        Ok(true) => {}
        Ok(false) => {
            return true;
        }
        Err(e) => {
            eprintln!(
                "{}: failed to read file {}: {}",
                env!("CARGO_PKG_NAME"),
                path.display(),
                e
            );
            process::exit(EXIT_RUNTIME_ERROR);
        }
    }

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
            if original == sorted {
                true
            } else {
                println!("{}: needs sorting", path.display());
                false
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
                if let Some(cmd) = diff_command {
                    use tempfile::NamedTempFile;
                    let tmp = match NamedTempFile::new() {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!(
                                "{}: failed to create temp file: {}",
                                env!("CARGO_PKG_NAME"),
                                e
                            );
                            process::exit(EXIT_RUNTIME_ERROR);
                        }
                    };
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
                            if !out.stderr.is_empty() {
                                eprint!("{}", String::from_utf8_lossy(&out.stderr));
                            }
                            print!("{}", String::from_utf8_lossy(&out.stdout));
                            if !out.status.success() {
                                let code = out.status.code().unwrap_or(EXIT_RUNTIME_ERROR);
                                if code == 1 {
                                    process::exit(EXIT_SYNTAX_ERROR);
                                }
                                process::exit(code);
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

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, out)?;
        } else if path.is_file() {
            out.push(path);
        }
    }
    Ok(())
}

fn is_text_file(path: &Path) -> io::Result<bool> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut buf = [0u8; 8192];
    let n = file.read(&mut buf)?;
    let slice = &buf[..n];
    if slice.contains(&0) {
        return Ok(false);
    }
    Ok(std::str::from_utf8(slice).is_ok())
}
