#!/usr/bin/env bats

load './helpers.bash'

@test "rust_derive_alphabetical feature" {
  local file expected
  file=$(prepare_file rust_derive/1.rs)
  expected="$EXPECTED_DIR/rust_derive/1.rs"

  run_keepsorted --mode fix --features rust_derive_alphabetical "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

@test "rust_derive_canonical feature" {
  local file expected
  file=$(prepare_file rust_derive/2.rs)
  expected="$EXPECTED_DIR/rust_derive/2.rs"

  run_keepsorted --mode fix --features rust_derive_canonical "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

