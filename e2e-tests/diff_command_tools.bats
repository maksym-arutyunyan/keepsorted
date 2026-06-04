#!/usr/bin/env bats

load './helpers.bash'

@test "test \`--diff-command\` with git diff --no-index" {
  run_keepsorted --mode diff --diff-command "git diff --no-index" "e2e-tests/files/bazel/1_in.bazel"
  [ "$status" -eq "$EXIT_CHECK_FAILED" ]
  [[ "$output" == diff\ --git* ]]
}

@test "test \`--diff-command\` with colordiff" {
  command -v colordiff >/dev/null || skip "colordiff not installed"
  run_keepsorted --mode diff --diff-command "colordiff -u" "e2e-tests/files/bazel/1_in.bazel"
  [ "$status" -eq "$EXIT_CHECK_FAILED" ]
  [[ "$output" == *$'\e[0;31m'* ]]
}

@test "test \`--diff-command\` with delta" {
  command -v delta >/dev/null || skip "delta not installed"
  run_keepsorted --mode diff --diff-command "delta --paging=never" "e2e-tests/files/bazel/1_in.bazel"
  [ "$status" -eq "$EXIT_CHECK_FAILED" ]
  [[ "$output" == *"│"* ]]
}

