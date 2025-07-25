#!/usr/bin/env bash

REPO_ROOT="$(git rev-parse --show-toplevel)"
FILES_DIR="$REPO_ROOT/e2e-tests/files"
BIN="$REPO_ROOT/target/release/keepsorted"

setup() {
  source "$REPO_ROOT/utils/exit_codes.bash"
  TEST_TMPDIR="$(mktemp -d)"
}

teardown() {
  rm -rf "$TEST_TMPDIR"
}

prepare_file() {
  local src="$FILES_DIR/$1"
  local dest
  dest="$TEST_TMPDIR/$(basename "$1")"
  cp "$src" "$dest"
  echo "$dest"
}

run_keepsorted() {
  pushd "$REPO_ROOT" >/dev/null || return
  run "$BIN" "$@"
  popd >/dev/null || return
}

# Ensure that the given command is available. If not, attempt to
# install the provided package name (defaults to the command name).
ensure_tool() {
  local cmd="$1"
  local pkg="${2:-$1}"
  if ! command -v "$cmd" >/dev/null; then
    apt-get update -y >/dev/null
    apt-get install -y "$pkg" >/dev/null
  fi
}

