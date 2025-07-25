#!/usr/bin/env bats

load './helpers.bash'

@test "--check shorthand" {
  local file
  file=$(prepare_file generic/1.txt)

  run_keepsorted --check "$file"
  [ "$status" -ne 0 ]
}

@test "--diff shorthand" {
  run_keepsorted --diff "e2e-tests/input/bazel/1.bazel"
  [ "$status" -ne 0 ]
  diff -u "$EXPECTED_DIR/bazel/1_diff.txt" - <<<"$output"
}

@test "--fix shorthand" {
  local file expected
  file=$(prepare_file bazel/1.bazel)
  expected="$EXPECTED_DIR/bazel/1.bazel"

  run_keepsorted --fix "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

