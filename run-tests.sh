#!/bin/bash

# Define the path to the binary and test directory
BINARY_PATH="./target/release/keepsorted"
TEST_DIR="./tests"

# Build the Rust project
echo "Compiling the Rust project..."
cargo build --release

# Check if the compilation was successful
if [ $? -ne 0 ]; then
  echo "Compilation failed. Exiting."
  exit 1
fi

# Run tests
echo "Running tests..."
for src_file in "$TEST_DIR"/*_src*; do
    # Extract the base name and extension
    base_name=$(basename "$src_file" "_src${src_file##*_src}")
    extension="${src_file##*_src}"
    expected_file="$TEST_DIR/${base_name}_expected${extension}"

    # Check if the expected file exists
    if [ ! -f "$expected_file" ]; then
        echo "Expected file for $base_name not found. Skipping."
        continue
    fi

    # Create a temporary file and copy the source file to it
    TEMP_FILE=$(mktemp)
    cp "$src_file" "$TEMP_FILE"

    # Run the binary on the temporary file
    "$BINARY_PATH" "$TEMP_FILE"

    # Compare the output with the expected file
    if diff -q "$TEMP_FILE" "$expected_file" > /dev/null; then
        echo "$base_name: PASSED"
    else
        echo "$base_name: FAILED"
        echo "Differences:"
        diff "$TEMP_FILE" "$expected_file"
    fi

    # Clean up the temporary file
    rm "$TEMP_FILE"
done

echo "Tests completed."
