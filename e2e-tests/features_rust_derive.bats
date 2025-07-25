#!/usr/bin/env bats

load './helpers.bash'

@test "rust_derive_alphabetical feature" {
  run_fix rust_derive/1.rs rust_derive/1.rs --mode fix --features rust_derive_alphabetical
}

@test "rust_derive_canonical feature" {
  run_fix rust_derive/2.rs rust_derive/2.rs --mode fix --features rust_derive_canonical
}

