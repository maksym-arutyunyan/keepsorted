#!/bin/bash

# Define color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Run cargo test and capture its exit status
cargo test
test_status=$?

# Run clippy and capture its exit status
./run-clippy.sh
clippy_status=$?

# Run fmt and capture its exit status
./run-fmt.sh
fmt_status=$?

# Run e2e tests and capture its exit status
./run-e2e-tests.sh
e2e_status=$?

# Check the status of each command and print the final status
echo ""
if [ $test_status -eq 0 ] && [ $clippy_status -eq 0 ] && [ $fmt_status -eq 0 ] && [ $e2e_status -eq 0 ]; then
    echo -e "All checks passed ${GREEN}ok${NC}."
else
    echo -e "Some checks ${RED}FAILED${NC}:"
    if [ $test_status -ne 0 ]; then
        echo -e " - cargo test ${RED}FAILED${NC}"
    fi
    if [ $clippy_status -ne 0 ]; then
        echo -e " - clippy ${RED}FAILED${NC}"
    fi
    if [ $fmt_status -ne 0 ]; then
        echo -e " - fmt ${RED}FAILED${NC}"
    fi
    if [ $e2e_status -ne 0 ]; then
        echo -e " - e2e tests ${RED}FAILED${NC}"
    fi
fi

# Exit with a status of 1 if any of the steps failed
if [ $test_status -ne 0 ] || [ $clippy_status -ne 0 ] || [ $fmt_status -ne 0 ] || [ $e2e_status -ne 0 ]; then
    exit 1
fi
