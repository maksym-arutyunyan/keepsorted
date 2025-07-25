#!/usr/bin/env bats

load './helpers.bash'

@test "process directory recursively" {
  mkdir "$TEST_TMPDIR/dir"
  cp "$FILES_DIR/generic/1_in.txt" "$TEST_TMPDIR/dir/file.txt"

  run_keepsorted --mode fix --recursive "$TEST_TMPDIR/dir"
  [ "$status" -eq "$EXIT_SUCCESS" ]
  diff "$FILES_DIR/generic/1_out.txt" "$TEST_TMPDIR/dir/file.txt"
}

