use clap::{arg, command, Parser, ValueEnum};
use keepsorted::process_file;
use std::fmt;
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

fn about() -> &'static str {
    env!("CARGO_PKG_DESCRIPTION")
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

/// A fatal error that should terminate the process with the given exit code.
struct AppError {
    code: i32,
    message: String,
}

impl AppError {
    fn usage(message: impl Into<String>) -> Self {
        Self {
            code: EXIT_USAGE_ERROR,
            message: message.into(),
        }
    }

    fn runtime(message: impl Into<String>) -> Self {
        Self {
            code: EXIT_RUNTIME_ERROR,
            message: message.into(),
        }
    }

    fn with_code(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
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
    let feature_names: Vec<String> = features.iter().map(|f| f.as_str().to_string()).collect();
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
            match handle_file(&file, &feature_names, mode, args.diff_command.as_deref()) {
                Ok(true) => {}
                Ok(false) => exit_code = EXIT_CHECK_FAILED,
                Err(e) => {
                    if !e.message.is_empty() {
                        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), e.message);
                    }
                    process::exit(e.code);
                }
            }
        }
    } else {
        match handle_file(path, &feature_names, mode, args.diff_command.as_deref()) {
            Ok(true) => {}
            Ok(false) => exit_code = EXIT_CHECK_FAILED,
            Err(e) => {
                eprintln!("{}: {}", env!("CARGO_PKG_NAME"), e.message);
                process::exit(e.code);
            }
        }
    }

    if exit_code != 0 {
        process::exit(exit_code);
    }
}

/// Returns `Ok(true)` if the file is sorted, `Ok(false)` if it needs sorting,
/// or `Err` with an exit code and message for any fatal condition.
fn handle_file(
    path: &Path,
    feature_names: &[String],
    mode: Mode,
    diff_command: Option<&str>,
) -> Result<bool, AppError> {
    match is_text_file(path) {
        Ok(true) => {}
        Ok(false) => return Ok(true),
        Err(e) => {
            return Err(AppError::runtime(format!(
                "failed to read file {}: {}",
                path.display(),
                e
            )));
        }
    }

    let (original, sorted) = match process_file(path, feature_names) {
        Ok(pair) => pair,
        Err(e) if e.kind() == io::ErrorKind::InvalidInput => {
            return Err(AppError::usage(e.to_string()));
        }
        Err(e) => {
            return Err(AppError::runtime(format!(
                "failed to process file {}: {}",
                path.display(),
                e
            )));
        }
    };

    match mode {
        Mode::Check => {
            if original == sorted {
                Ok(true)
            } else {
                println!("{}: needs sorting", path.display());
                Ok(false)
            }
        }
        Mode::Diff => {
            if original != sorted {
                if let Some(cmd) = diff_command {
                    use tempfile::NamedTempFile;
                    let tmp = NamedTempFile::new().map_err(|e| {
                        AppError::runtime(format!("failed to create temp file: {}", e))
                    })?;
                    std::fs::write(tmp.path(), &sorted).map_err(|e| {
                        AppError::runtime(format!(
                            "failed to write temp file {}: {}",
                            tmp.path().display(),
                            e
                        ))
                    })?;
                    let parts = shell_words::split(cmd).map_err(|e| {
                        AppError::runtime(format!("failed to parse diff command '{}': {}", cmd, e))
                    })?;
                    let (prog, args) = parts
                        .split_first()
                        .ok_or_else(|| AppError::runtime("diff command is empty"))?;
                    let output = Command::new(prog)
                        .args(args)
                        .arg(path)
                        .arg(tmp.path())
                        .output()
                        .map_err(|e| {
                            AppError::runtime(format!(
                                "failed to run diff command '{}': {}",
                                cmd, e
                            ))
                        })?;
                    if !output.stderr.is_empty() {
                        eprint!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                    if !output.status.success() {
                        let code = output.status.code().unwrap_or(EXIT_RUNTIME_ERROR);
                        let exit_code = if code == 1 { EXIT_SYNTAX_ERROR } else { code };
                        return Err(AppError::with_code(exit_code, ""));
                    }
                    return Ok(false);
                } else {
                    use similar::TextDiff;
                    let diff = TextDiff::from_lines(&original, &sorted);
                    let old = format!("a/{}", path.display());
                    let new = format!("b/{}", path.display());
                    print!("{}", diff.unified_diff().header(&old, &new));
                    return Ok(false);
                }
            }
            Ok(true)
        }
        Mode::Fix => {
            let dir = path.parent().unwrap_or(Path::new("."));
            let tmp = tempfile::NamedTempFile::new_in(dir)
                .map_err(|e| AppError::runtime(format!("failed to create temp file: {}", e)))?;
            std::fs::write(tmp.path(), &sorted).map_err(|e| {
                AppError::runtime(format!("failed to write file {}: {}", path.display(), e))
            })?;
            if let Ok(metadata) = std::fs::metadata(path) {
                let _ = std::fs::set_permissions(tmp.path(), metadata.permissions());
            }
            tmp.persist(path).map_err(|e| {
                AppError::runtime(format!("failed to write file {}: {}", path.display(), e))
            })?;
            Ok(true)
        }
    }
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    let mut entries = std::fs::read_dir(dir)?.collect::<io::Result<Vec<_>>>()?;
    entries.sort_by_key(|e| e.path());
    for entry in entries {
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_files(&path, out)?;
        } else if file_type.is_file() {
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
