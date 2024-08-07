use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

fn run_test(input_file_path: &str, expected_file_path: &str) {
    // Read the input and expected output files
    let input_content = fs::read_to_string(input_file_path).expect("Failed to read input file");
    let expected_content =
        fs::read_to_string(expected_file_path).expect("Failed to read expected file");

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
    // Run the keepsorted binary on the temporary file
    let output = Command::new(keepsorted_binary)
        .arg(temp_input_file_path.to_str().unwrap())
        .output()
        .expect("Failed to execute keepsorted");

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

#[test]
fn test_e2e_bazel_1() {
    run_test(
        "./tests/e2e-tests/bazel/1_in.bazel",
        "./tests/e2e-tests/bazel/1_out.bazel",
    );
}

#[test]
fn test_e2e_bazel_2() {
    run_test(
        "./tests/e2e-tests/bazel/2_in.bazel",
        "./tests/e2e-tests/bazel/2_out.bazel",
    );
}

#[test]
fn test_e2e_plain_text_1() {
    run_test(
        "./tests/e2e-tests/plain_text/1_in.txt",
        "./tests/e2e-tests/plain_text/1_out.txt",
    );
}

#[test]
fn test_e2e_plain_text_2() {
    run_test(
        "./tests/e2e-tests/plain_text/2_in.txt",
        "./tests/e2e-tests/plain_text/2_out.txt",
    );
}

#[test]
fn test_e2e_plain_text_3() {
    run_test(
        "./tests/e2e-tests/plain_text/3_in.txt",
        "./tests/e2e-tests/plain_text/3_out.txt",
    );
}

#[test]
fn test_e2e_cargo_toml_1() {
    run_test(
        "./tests/e2e-tests/cargo_toml/1/Cargo.toml",
        "./tests/e2e-tests/cargo_toml/1/Cargo_out.toml",
    );
}

#[test]
fn test_e2e_cargo_toml_2() {
    run_test(
        "./tests/e2e-tests/cargo_toml/2/Cargo.toml",
        "./tests/e2e-tests/cargo_toml/2/Cargo_out.toml",
    );
}
