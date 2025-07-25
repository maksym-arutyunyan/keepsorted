#!/usr/bin/env bats

load './helpers.bash'

@test "--check shorthand" {
  local file
  file=$(prepare_file generic/1_in.txt)

  run_keepsorted --check "$file"
  [ "$status" -ne 0 ]
}

@test "--diff shorthand" {
  run_keepsorted --diff "e2e-tests/files/bazel/1_in.bazel"
  [ "$status" -ne 0 ]
  diff -u "$FILES_DIR/bazel/1_diff.txt" - <<<"$output"
}

@test "--fix shorthand" {
  local file expected
  file=$(prepare_file bazel/1_in.bazel)
  expected="$FILES_DIR/bazel/1_out.bazel"

  run_keepsorted --fix "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

