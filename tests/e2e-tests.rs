use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

fn run_test(input_file_path: &str, expected_file_path: &str, features: &str) {
    // Read the input and expected output files
    let input_content = fs::read_to_string(input_file_path).expect("Failed to read input file");
    let expected_content = fs::read_to_string(expected_file_path)
        .unwrap_or_else(|_| panic!("Failed to read expected file: {}", expected_file_path));

    // Create a temporary directory
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let temp_input_file_path = temp_dir
        .path()
        .join(Path::new(input_file_path).file_name().unwrap());

    // Write the input content to a temporary file
    fs::write(&temp_input_file_path, &input_content).expect("Failed to write to temporary file");

    // Determine the path to the keepsorted binary based on the build mode
    let keepsorted_binary = if cfg!(debug_assertions) {
        "./target/debug/keepsorted"
    } else {
        "./target/release/keepsorted"
    };
    // Create the command and conditionally add the --features argument if the string is not empty
    let mut command = Command::new(keepsorted_binary);
    command.arg(temp_input_file_path.to_str().unwrap());
    if !features.is_empty() {
        command.arg("--features").arg(features);
    }

    // Run the keepsorted binary on the temporary file
    let output = command.output().expect("Failed to execute keepsorted");

    // Check if the command was successful
    assert!(output.status.success(), "keepsorted command failed");

    // Read the content of the temporary file after running keepsorted
    let output_content =
        fs::read_to_string(&temp_input_file_path).expect("Failed to read output file");

    // Compare the output with the expected content
    assert_eq!(
        output_content, expected_content,
        "The output content does not match the expected content"
    );

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
    run_test(&dir("bazel/1_in.bazel"), &dir("bazel/1_out.bazel"), "");
}

#[test]
fn test_e2e_bazel_2() {
    run_test(&dir("bazel/2_in.bazel"), &dir("bazel/2_out.bazel"), "");
}

#[test]
fn test_e2e_generic_1() {
    run_test(&dir("generic/1_in.txt"), &dir("generic/1_out.txt"), "");
}

#[test]
fn test_e2e_generic_2() {
    run_test(&dir("generic/2_in.txt"), &dir("generic/2_out.txt"), "");
}

#[test]
fn test_e2e_generic_3() {
    run_test(&dir("generic/3_in.txt"), &dir("generic/3_out.txt"), "");
}

#[test]
fn test_e2e_cargo_toml_1() {
    run_test(
        &dir("cargo_toml/1/Cargo.toml"),
        &dir("cargo_toml/1/Cargo_out.toml"),
        "",
    );
}

#[test]
fn test_e2e_cargo_toml_2() {
    run_test(
        &dir("cargo_toml/2/Cargo.toml"),
        &dir("cargo_toml/2/Cargo_out.toml"),
        "",
    );
}

#[test]
fn test_e2e_gitignore_1() {
    run_test(
        &dir("gitignore/.gitignore"),
        &dir("gitignore/.gitignore_out"),
        "gitignore",
    );
}

#[test]
fn test_e2e_codeowners_1() {
    run_test(
        &dir("codeowners/.github/CODEOWNERS"),
        &dir("codeowners/.github/CODEOWNERS_out"),
        "codeowners",
    );
}

#[test]
fn test_e2e_rust_derive_1() {
    run_test(
        &dir("rust_derive/1_in.rs"),
        &dir("rust_derive/1_out.rs"),
        "rust_derive_alphabetical",
    );
}

#[test]
fn test_e2e_rust_derive_2() {
    run_test(
        &dir("rust_derive/2_in.rs"),
        &dir("rust_derive/2_out.rs"),
        "rust_derive_canonical",
    );
}

#[test]
fn test_e2e_rust_derive_3() {
    run_test(
        &dir("rust_derive/3_in.rs"),
        &dir("rust_derive/3_out.rs"),
        "rust_derive_alphabetical",
    );
}
