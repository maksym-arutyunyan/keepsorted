#!/usr/bin/env bats

load './helpers.bash'

@test "test \`rust_derive_alphabetical\` feature" {
  local file expected
  file=$(prepare_file rust_derive/1_in.rs)
  expected="$FILES_DIR/rust_derive/1_out.rs"

  run_keepsorted --mode fix --features rust_derive_alphabetical "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

@test "test \`rust_derive_canonical\` feature" {
  local file expected
  file=$(prepare_file rust_derive/2_in.rs)
  expected="$FILES_DIR/rust_derive/2_out.rs"

  run_keepsorted --mode fix --features rust_derive_canonical "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

