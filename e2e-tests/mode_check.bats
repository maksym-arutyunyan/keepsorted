#!/usr/bin/env bats

load './helpers.bash'

@test "test \`--mode check\` fails on unsorted file" {
  local file
  file=$(prepare_file generic/1_in.txt)

  run_keepsorted --mode check "$file"
  [ "$status" -eq "$EXIT_CHECK_FAILED" ]
}

@test "test \`--mode check\` succeeds on sorted file" {
  local file
  file=$(prepare_file generic/1_out.txt)

  run_keepsorted --mode check "$file"
  [ "$status" -eq "$EXIT_SUCCESS" ]
}

@test "test \`--check\` shorthand" {
  local file
  file=$(prepare_file generic/1_in.txt)

  run_keepsorted --check "$file"
  [ "$status" -eq "$EXIT_CHECK_FAILED" ]
}
