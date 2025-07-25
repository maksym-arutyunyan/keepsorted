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

run_keepsorted() {
  pushd "$REPO_ROOT" >/dev/null
  run "$BIN" "$@"
  popd >/dev/null
}

