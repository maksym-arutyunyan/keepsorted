#!/usr/bin/env bats

load './helpers.bash'

@test "test \`--mode fix\` rewrites generic file" {
  local file expected
  file=$(prepare_file generic/1_in.txt)
  expected="$FILES_DIR/generic/1_out.txt"

  run_keepsorted --mode fix "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

@test "test \`--mode fix\` handles bazel files" {
  local file expected
  file=$(prepare_file bazel/1_in.bazel)
  expected="$FILES_DIR/bazel/1_out.bazel"

  run_keepsorted --mode fix "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

@test "test \`--mode fix\` works with Cargo.toml" {
  local file expected
  file=$(prepare_file cargo_toml/1/Cargo.toml)
  expected="$FILES_DIR/cargo_toml/1/Cargo_out.toml"

  run_keepsorted --mode fix "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

@test "test \`--fix\` shorthand" {
  local file expected
  file=$(prepare_file bazel/1_in.bazel)
  expected="$FILES_DIR/bazel/1_out.bazel"

  run_keepsorted --fix "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

