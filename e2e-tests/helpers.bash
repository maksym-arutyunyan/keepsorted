#!/usr/bin/env bash

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
INPUT_DIR="$REPO_ROOT/e2e-tests/input"
EXPECTED_DIR="$REPO_ROOT/e2e-tests/expected"
BIN="$REPO_ROOT/target/release/keepsorted"

setup() {
  TEST_TMPDIR="$(mktemp -d)"
}

teardown() {
  rm -rf "$TEST_TMPDIR"
}

prepare_file() {
  local src="$INPUT_DIR/$1"
  local dest="$TEST_TMPDIR/$(basename "$1")"
  cp "$src" "$dest"
  echo "$dest"
}

# run keepsorted on a temporary copy and compare to expected
run_fix() {
  local input="$1" expected="$2"
  shift 2
  local tmp
  tmp=$(prepare_file "$input")
pushd "$REPO_ROOT" >/dev/null
run "$BIN" "$@" "$tmp"
popd >/dev/null
  local exit=$status
  [ "$exit" -eq 0 ] || { echo "expected success, got $exit" >&2; return 1; }
  diff -u "$EXPECTED_DIR/$expected" "$tmp"
}

run_check() {
  local input="$1" expect_success="$2"
  shift 2
  local tmp
  tmp=$(prepare_file "$input")
  pushd "$REPO_ROOT" >/dev/null
  run "$BIN" "$@" "$tmp"
  popd >/dev/null
  local exit=$status
  if [ "$expect_success" = true ]; then
    [ "$exit" -eq 0 ] || { echo "expected success, got $exit" >&2; return 1; }
  else
    [ "$exit" -ne 0 ] || { echo "expected failure" >&2; return 1; }
  fi
}

run_diff() {
  local input="$1"; shift
  pushd "$REPO_ROOT" >/dev/null
  run "$BIN" "$@" "e2e-tests/input/$input"
  popd >/dev/null
  [ "$status" -ne 0 ]
}

run_diff_success() {
  local input="$1"; shift
  local tmp
  tmp=$(prepare_file "$input")
  pushd "$REPO_ROOT" >/dev/null
  "$BIN" --mode fix "$tmp" >/dev/null
  run "$BIN" "$@" "$tmp"
  popd >/dev/null
  [ "$status" -eq 0 ]
}

