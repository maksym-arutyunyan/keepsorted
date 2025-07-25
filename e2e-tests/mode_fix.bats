#!/usr/bin/env bats

load './helpers.bash'

@test "--mode fix rewrites generic file" {
  local file expected
  file=$(prepare_file generic/1.txt)
  expected="$EXPECTED_DIR/generic/1.txt"

  run_keepsorted --mode fix "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

@test "--mode fix handles bazel files" {
  local file expected
  file=$(prepare_file bazel/1.bazel)
  expected="$EXPECTED_DIR/bazel/1.bazel"

  run_keepsorted --mode fix "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

@test "--mode fix works with Cargo.toml" {
  local file expected
  file=$(prepare_file cargo_toml/1/Cargo.toml)
  expected="$EXPECTED_DIR/cargo_toml/1/Cargo.toml"

  run_keepsorted --mode fix "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

