#!/usr/bin/env bats

load './helpers.bash'

@test "test \`-h\` prints help" {
  run_keepsorted -h
  [ "$status" -eq "$EXIT_SUCCESS" ]
  diff -u "$FILES_DIR/help.txt" <(printf '%s\n' "$output")
}
