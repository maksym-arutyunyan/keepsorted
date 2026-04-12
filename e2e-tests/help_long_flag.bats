#!/usr/bin/env bats

load './helpers.bash'

@test "test \`--help\` prints long help" {
  run_keepsorted --help
  [ "$status" -eq "$EXIT_SUCCESS" ]
  diff -u "$FILES_DIR/help_long.txt" <(printf '%s\n' "$output")
}
