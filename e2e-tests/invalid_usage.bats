#!/usr/bin/env bats

load './helpers.bash'

@test "test conflicting modes fail" {
  run_keepsorted --check --diff e2e-tests/files/generic/1_in.txt
  [ "$status" -eq 2 ]
}

@test "test \`--path\` with positional arg fails" {
  run_keepsorted -p e2e-tests/files/generic/1_in.txt e2e-tests/files/generic/1_in.txt
  [ "$status" -eq 2 ]
}

@test "test conflicting rust_derive features" {
  run_keepsorted --features rust_derive_alphabetical,rust_derive_canonical e2e-tests/files/rust_derive/1_in.rs
  [ "$status" -eq 2 ]
}

@test "conflicting rust_derive features on non-rust file" {
  run_keepsorted --features rust_derive_alphabetical,rust_derive_canonical e2e-tests/files/generic/1_in.txt
  [ "$status" -eq 2 ]
}
