#!/bin/bash

# Define color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Run cargo build and capture its exit status
cargo build --release --all-targets
build_status=$?

# Run cargo test and capture its exit status
cargo test
test_status=$?

# Run cargo test in release and capture its exit status
cargo test --release
test_release_status=$?

# Run clippy and capture its exit status
cargo clippy --all-targets -- -D warnings
clippy_status=$?

# Run fmt and capture its exit status
cargo fmt --all -- --check
fmt_status=$?

# Run `keepsorted` only on files that are not ignored by `.gitignore`.
# Also ignore `./misc/` and `./tests/`.
git ls-files -co --exclude-standard \
    | grep -vE "^tests/|^misc/" \
    | xargs -I {} bash -c './target/release/keepsorted "$0" --features gitignore' {}
keepsorted_status=$?

# Check the status of each command and print the final status
echo ""
if [ $build_status -eq 0 ] &&\
   [ $test_status -eq 0 ] &&\
   [ $test_release_status -eq 0 ] &&\
   [ $clippy_status -eq 0 ] &&\
   [ $fmt_status -eq 0 ] &&\
   [ $keepsorted_status -eq 0 ]; then
    echo -e "All checks passed ${GREEN}ok${NC}."
else
    echo -e "Some checks ${RED}FAILED${NC}:"
    if [ $build_status -ne 0 ]; then
        echo -e " - cargo build ${RED}FAILED${NC}"
    fi
    if [ $test_status -ne 0 ]; then
        echo -e " - cargo test ${RED}FAILED${NC}"
    fi
    if [ $test_release_status -ne 0 ]; then
        echo -e " - cargo test --release ${RED}FAILED${NC}"
    fi
    if [ $clippy_status -ne 0 ]; then
        echo -e " - clippy ${RED}FAILED${NC}"
    fi
    if [ $fmt_status -ne 0 ]; then
        echo -e " - fmt ${RED}FAILED${NC}"
    fi
    if [ $keepsorted_status -ne 0 ]; then
        echo -e " - keepsorted ${RED}FAILED${NC}"
    fi
fi

# Exit with a status of 1 if any of the steps failed
if [ $build_status -ne 0 ] ||\
   [ $test_status -ne 0 ] ||\
   [ $test_release_status -ne 0 ] ||\
   [ $clippy_status -ne 0 ] ||\
   [ $fmt_status -ne 0 ] ||\
   [ $keepsorted_status -ne 0 ]; then
    exit 1
fi
