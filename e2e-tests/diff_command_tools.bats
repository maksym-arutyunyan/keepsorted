#!/usr/bin/env bats

load './helpers.bash'

setup_file() {
  # Ensure tools used by these tests are installed
  ensure_tool colordiff colordiff
  ensure_tool delta git-delta
}

@test "test \`--diff-command\` with git diff --no-index" {
  run_keepsorted --mode diff --diff-command "git diff --no-index" "e2e-tests/files/bazel/1_in.bazel"
  [ "$status" -eq "$EXIT_SYNTAX_ERROR" ]
  [[ "$output" == diff\ --git* ]]
}

@test "test \`--diff-command\` with colordiff" {
  run_keepsorted --mode diff --diff-command "colordiff -u" "e2e-tests/files/bazel/1_in.bazel"
  [ "$status" -eq "$EXIT_SYNTAX_ERROR" ]
  [[ "$output" == *$'\e[0;31m'* ]]
}

@test "test \`--diff-command\` with delta" {
  run_keepsorted --mode diff --diff-command "delta --paging=never" "e2e-tests/files/bazel/1_in.bazel"
  [ "$status" -eq "$EXIT_SYNTAX_ERROR" ]
  [[ "$output" == *"│"* ]]
}

