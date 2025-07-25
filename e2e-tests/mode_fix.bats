#!/usr/bin/env bats

load './helpers.bash'

@test "--mode fix rewrites generic file" {
  run_fix generic/1.txt generic/1.txt --mode fix
}

@test "--mode fix handles bazel files" {
  run_fix bazel/1.bazel bazel/1.bazel --mode fix
}

@test "--mode fix works with Cargo.toml" {
  run_fix cargo_toml/1/Cargo.toml cargo_toml/1/Cargo.toml --mode fix
}

