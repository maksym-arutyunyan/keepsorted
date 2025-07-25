#!/usr/bin/env bats

load './helpers.bash'

@test "test \`-h\` prints help" {
  run_keepsorted -h
  [ "$status" -eq 0 ]
  diff -u "$FILES_DIR/help.txt" - <<<"$output"
}
