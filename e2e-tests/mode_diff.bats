#!/usr/bin/env bats

load './helpers.bash'

@test "--mode diff fails on unsorted file" {
  run_keepsorted --mode diff "e2e-tests/files/bazel/1_in.bazel"
  [ "$status" -ne 0 ]
  diff -u "$FILES_DIR/bazel/1_diff.txt" - <<<"$output"
}

@test "--diff-command custom command" {
  run_keepsorted --mode diff --diff-command "sh -c 'echo executed'" "e2e-tests/files/bazel/1_in.bazel"
  [ "$status" -ne 0 ]
  diff -u "$FILES_DIR/bazel/1_diff_cmd.txt" - <<<"$output"
}

