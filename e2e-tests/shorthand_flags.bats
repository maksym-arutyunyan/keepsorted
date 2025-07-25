#!/usr/bin/env bats

load './helpers.bash'

@test "--check shorthand" {
  run_check generic/1.txt false --check
}

@test "--diff shorthand" {
  run_diff bazel/1.bazel bazel/1_diff.txt --diff
}

@test "--fix shorthand" {
  run_fix bazel/1.bazel bazel/1.bazel --fix
}

