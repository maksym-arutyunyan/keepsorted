#!/usr/bin/env bats

load './helpers.bash'

@test "test directory argument fails" {
  mkdir "$TEST_TMPDIR/dir"
  cp "$FILES_DIR/generic/1_in.txt" "$TEST_TMPDIR/dir/file.txt"

  run_keepsorted --mode fix "$TEST_TMPDIR/dir"
  [ "$status" -eq "$EXIT_USAGE_ERROR" ]
}

