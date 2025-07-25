#!/usr/bin/env bats

load './helpers.bash'

@test "--mode diff fails on unsorted file" {
  run_diff bazel/1.bazel bazel/1_diff.txt --mode diff
}

@test "--diff-command custom command" {
  run_diff bazel/1.bazel bazel/1_diff_cmd.txt --mode diff --diff-command "sh -c 'echo executed'"
}

