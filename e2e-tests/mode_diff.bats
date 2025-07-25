#!/usr/bin/env bats

load './helpers.bash'

@test "test \`--mode diff\` fails on unsorted file" {
  run_keepsorted --mode diff "e2e-tests/files/bazel/1_in.bazel"
  [ "$status" -eq "$EXIT_CHECK_FAILED" ]
  diff -u "$FILES_DIR/bazel/1_diff.bazel" - <<<"$output"
}

@test "test \`--diff-command\` custom command" {
  run_keepsorted --mode diff --diff-command "sh -c 'echo executed'" "e2e-tests/files/bazel/1_in.bazel"
  [ "$status" -eq "$EXIT_CHECK_FAILED" ]
  diff -u "$FILES_DIR/bazel/1_diff_cmd.bazel" - <<<"$output"
}

@test "test \`--diff\` shorthand" {
  run_keepsorted --diff "e2e-tests/files/bazel/1_in.bazel"
  [ "$status" -eq "$EXIT_CHECK_FAILED" ]
  diff -u "$FILES_DIR/bazel/1_diff.bazel" - <<<"$output"
}

