#!/bin/bash

# Define the path to the binary and test directory
BINARY_PATH="./target/release/keepsorted"
TEST_DIR="./tests"

# Define color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Build the Rust project
echo "Compiling the Rust project..."
if ! cargo build --release; then
  echo "Compilation failed. Exiting."
  exit 1
fi

# Track if any test fails
all_tests_passed=true

# Run tests
echo ""
echo "Running tests..."
for src_file in "$TEST_DIR"/*_in*; do
    # Extract the base name and extension
    base_name=$(basename "$src_file" "_in${src_file##*_in}")
    extension="${src_file##*_in}"
    expected_file="$TEST_DIR/${base_name}_out${extension}"

    # Check if the test is disabled
    if [[ "$(basename "$src_file")" == _* ]]; then
        echo -e "test $base_name ... ${YELLOW}ignored${NC}"
        continue
    fi

    # Check if the expected file exists
    if [ ! -f "$expected_file" ]; then
        echo "Expected file for $base_name not found. Skipping."
        continue
    fi

    # Create a temporary file and copy the source file to it
    TEMP_FILE=$(mktemp --suffix=".$extension")
    cp "$src_file" "$TEMP_FILE"

    # Run the binary on the temporary file
    "$BINARY_PATH" "$TEMP_FILE"

    # Compare the output with the expected file
    if diff -q "$TEMP_FILE" "$expected_file" > /dev/null; then
        echo -e "test $base_name ... ${GREEN}ok${NC}"
    else
        echo -e "test $base_name ... ${RED}FAILED${NC}"
        echo "Differences:"
        diff "$TEMP_FILE" "$expected_file"
        all_tests_passed=false
    fi

    # Clean up the temporary file
    rm "$TEMP_FILE"
done

echo "Tests completed."

# Exit with non-zero status if any test failed
if [ "$all_tests_passed" = false ]; then
    exit 1
fi
