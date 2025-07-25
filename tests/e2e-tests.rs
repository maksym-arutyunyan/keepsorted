use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

fn run_test(
    input_file_path: &str,
    expected_file_path: &str,
    features: &str,
    args: &[&str],
    expect_success: bool,
) {
    // Read the input and expected output files
    let input_content = fs::read_to_string(input_file_path).expect("Failed to read input file");
    let expected_content = fs::read_to_string(expected_file_path)
        .unwrap_or_else(|_| panic!("Failed to read expected file: {}", expected_file_path));

    let mode = if args.contains(&"--check") {
        "check"
    } else if args.contains(&"--diff") {
        "diff"
    } else if args.contains(&"--fix") {
        "fix"
    } else if let Some(pos) = args.iter().position(|a| *a == "--mode") {
        args.get(pos + 1).copied().expect("--mode value missing")
    } else {
        panic!("no mode flag provided")
    };

    let (run_path, _temp_dir): (std::path::PathBuf, Option<tempfile::TempDir>);
    if mode == "diff" {
        run_path = Path::new(input_file_path).to_path_buf();
        _temp_dir = None;
    } else {
        let td = tempdir().expect("Failed to create temporary directory");
        let tmp = td
            .path()
            .join(Path::new(input_file_path).file_name().unwrap());
        fs::write(&tmp, &input_content).expect("Failed to write to temporary file");
        run_path = tmp;
        _temp_dir = Some(td);
    }

    // Determine the path to the keepsorted binary based on the build mode
    let keepsorted_binary = if cfg!(debug_assertions) {
        "./target/debug/keepsorted"
    } else {
        "./target/release/keepsorted"
    };
    // Create the command and add requested CLI arguments
    let mut command = Command::new(keepsorted_binary);
    command.args(args).arg(run_path.to_str().unwrap());
    if !features.is_empty() {
        command.arg("--features").arg(features);
    }

    // Run the keepsorted binary on the temporary file
    let output = command.output().expect("Failed to execute keepsorted");

    // Check if the command was successful
    if expect_success {
        assert!(output.status.success(), "keepsorted command failed");
    } else {
        assert!(
            !output.status.success(),
            "keepsorted command unexpectedly succeeded"
        );
    }

    // Read the content of the temporary file after running keepsorted
    let output_content = fs::read_to_string(&run_path).expect("Failed to read output file");

    if mode == "fix" {
        assert_eq!(
            output_content, expected_content,
            "The output content does not match the expected content",
        );
    } else {
        assert_eq!(
            output_content, input_content,
            "--mode {} should not modify the file",
            mode
        );
    }

    if mode == "diff" {
        let diff_output = String::from_utf8_lossy(&output.stdout);
        assert_eq!(diff_output, expected_content, "Diff output mismatch");
    }

    // Ensure the input file is not modified
    let original_input_content =
        fs::read_to_string(input_file_path).expect("Failed to read input file");
    assert_eq!(
        input_content, original_input_content,
        "The input file content was modified"
    );
}

fn dir(path: &str) -> String {
    format!("./tests/e2e-tests/{path}")
}

#[test]
fn test_e2e_bazel_1() {
    run_test(
        &dir("bazel/1_in.bazel"),
        &dir("bazel/1_out.bazel"),
        "",
        &["--mode", "fix"],
        true,
    );
}

#[test]
fn test_e2e_bazel_2() {
    run_test(
        &dir("bazel/2_in.bazel"),
        &dir("bazel/2_out.bazel"),
        "",
        &["--mode", "fix"],
        true,
    );
}

#[test]
fn test_e2e_generic_1() {
    run_test(
        &dir("generic/1_in.txt"),
        &dir("generic/1_out.txt"),
        "",
        &["--mode", "fix"],
        true,
    );
}

#[test]
fn test_e2e_generic_2() {
    run_test(
        &dir("generic/2_in.txt"),
        &dir("generic/2_out.txt"),
        "",
        &["--mode", "fix"],
        true,
    );
}

#[test]
fn test_e2e_generic_3() {
    run_test(
        &dir("generic/3_in.txt"),
        &dir("generic/3_out.txt"),
        "",
        &["--mode", "fix"],
        true,
    );
}

#[test]
fn test_e2e_cargo_toml_1() {
    run_test(
        &dir("cargo_toml/1/Cargo.toml"),
        &dir("cargo_toml/1/Cargo_out.toml"),
        "",
        &["--mode", "fix"],
        true,
    );
}

#[test]
fn test_e2e_cargo_toml_2() {
    run_test(
        &dir("cargo_toml/2/Cargo.toml"),
        &dir("cargo_toml/2/Cargo_out.toml"),
        "",
        &["--mode", "fix"],
        true,
    );
}

#[test]
fn test_e2e_gitignore_1() {
    run_test(
        &dir("gitignore/.gitignore"),
        &dir("gitignore/.gitignore_out"),
        "gitignore",
        &["--mode", "fix"],
        true,
    );
}

#[test]
fn test_e2e_codeowners_1() {
    run_test(
        &dir("codeowners/.github/CODEOWNERS"),
        &dir("codeowners/.github/CODEOWNERS_out"),
        "codeowners",
        &["--mode", "fix"],
        true,
    );
}

#[test]
fn test_e2e_rust_derive_1() {
    run_test(
        &dir("rust_derive/1_in.rs"),
        &dir("rust_derive/1_out.rs"),
        "rust_derive_alphabetical",
        &["--mode", "fix"],
        true,
    );
}

#[test]
fn test_e2e_rust_derive_2() {
    run_test(
        &dir("rust_derive/2_in.rs"),
        &dir("rust_derive/2_out.rs"),
        "rust_derive_canonical",
        &["--mode", "fix"],
        true,
    );
}

#[test]
fn test_e2e_rust_derive_3() {
    run_test(
        &dir("rust_derive/3_in.rs"),
        &dir("rust_derive/3_out.rs"),
        "rust_derive_alphabetical",
        &["--mode", "fix"],
        true,
    );
}

#[test]
fn test_check_fails_on_unsorted() {
    run_test(
        &dir("generic/1_in.txt"),
        &dir("generic/1_in.txt"),
        "",
        &["--mode", "check"],
        false,
    );
}

#[test]
fn test_check_succeeds_on_sorted() {
    run_test(
        &dir("generic/1_out.txt"),
        &dir("generic/1_out.txt"),
        "",
        &["--mode", "check"],
        true,
    );
}

#[test]
fn test_diff_fails_on_unsorted() {
    run_test(
        &dir("bazel/1_in.bazel"),
        &dir("bazel/1_out_diff.bazel"),
        "",
        &["--mode", "diff"],
        false,
    );
}

#[test]
fn test_diff_succeeds_on_sorted() {
    run_test(
        &dir("bazel/1_out.bazel"),
        &dir("bazel/1_out_diff_empty.bazel"),
        "",
        &["--mode", "diff"],
        true,
    );
}

#[test]
fn test_shorthand_check() {
    run_test(
        &dir("generic/1_in.txt"),
        &dir("generic/1_in.txt"),
        "",
        &["--check"],
        false,
    );
}

#[test]
fn test_shorthand_diff() {
    run_test(
        &dir("bazel/1_in.bazel"),
        &dir("bazel/1_out_diff.bazel"),
        "",
        &["--diff"],
        false,
    );
}

#[test]
fn test_shorthand_fix() {
    run_test(
        &dir("bazel/1_in.bazel"),
        &dir("bazel/1_out.bazel"),
        "",
        &["--fix"],
        true,
    );
}
