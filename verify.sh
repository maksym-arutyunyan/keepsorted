#!/usr/bin/env bash
set -euo pipefail

# Define color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Default to 'all' if no args
args=("$@")
[ ${#args[@]} -eq 0 ] && args=("all")

print_help() {
  cat <<EOF
Usage: $0 [task ...]

Tasks:
  build         Run \`cargo build --release --all-targets\`
  test          Run \`cargo test\`
  test-release  Run \`cargo test --release\`
  clippy        Run \`cargo clippy\` with -D warnings
  fmt           Run \`cargo fmt --check\`
  keepsorted    Run keepsorted on tracked files
  diff          Check that keepsorted made no changes
  e2e           Run Bats end-to-end tests
  all           Run all checks (default)
  -h, --help    Show this help message
EOF
}

# shellcheck disable=SC2329
print_header() {
  echo
  echo "============================================================"
  echo ">>> $1"
  echo "============================================================"
}

# Status tracking
status_ok=true
failures=()

# shellcheck disable=SC2329
run_build() {
  print_header "cargo build"
  if ! cargo build --release --all-targets; then
    failures+=("build")
    status_ok=false
  fi
}

# shellcheck disable=SC2329
run_test() {
  print_header "cargo test"
  if ! cargo test; then
    failures+=("test")
    status_ok=false
  fi
}

# shellcheck disable=SC2329
run_test_release() {
  print_header "cargo test --release"
  if ! cargo test --release; then
    failures+=("test-release")
    status_ok=false
  fi
}

# shellcheck disable=SC2329
run_clippy() {
  print_header "cargo clippy"
  if ! cargo clippy --all-targets -- -D warnings; then
    failures+=("clippy")
    status_ok=false
  fi
}

# shellcheck disable=SC2329
run_fmt() {
  print_header "cargo fmt"
  if ! cargo fmt --all -- --check; then
    failures+=("fmt")
    status_ok=false
  fi
}

# shellcheck disable=SC2329
run_keepsorted() {
  print_header "keepsorted"
  if ! git ls-files -z \
      | grep -vzE '^tests/|^e2e-tests/|^README.md$' \
      | xargs -0 -n1 ./target/release/keepsorted \
          --features gitignore,rust_derive_canonical; then
    failures+=("keepsorted")
    status_ok=false
  fi
}

# shellcheck disable=SC2329
run_diff() {
  print_header "git diff"
  if ! git diff --exit-code; then
    failures+=("diff")
    status_ok=false
  fi
}

# shellcheck disable=SC2329
run_e2e() {
  print_header "bats e2e-tests"
  if ! bats e2e-tests; then
    failures+=("e2e")
    status_ok=false
  fi
}

# Expand 'all' into all tasks
expanded_tasks=()
for task in "${args[@]}"; do
  case "$task" in
    -h|--help)
      print_help
      exit 0
      ;;
    all)
      expanded_tasks+=(build test test-release clippy fmt keepsorted diff e2e)
      ;;
    build|test|test-release|clippy|fmt|keepsorted|diff|e2e)
      expanded_tasks+=("$task")
      ;;
    *)
      echo -e "${RED}Unknown task: $task${NC}" >&2
      echo "Run with -h for help." >&2
      exit 1
      ;;
  esac
done

# Dispatch
for task in "${expanded_tasks[@]}"; do
  run_"${task//-/_}"
done

# Final status
echo
if $status_ok; then
  echo -e "All checks passed ${GREEN}ok${NC}."
  exit 0
else
  echo -e "Some checks ${RED}FAILED${NC}:"
  for f in "${failures[@]}"; do
    echo -e " - $f ${RED}FAILED${NC}"
  done
  exit 1
fi
