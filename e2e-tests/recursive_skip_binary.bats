#!/usr/bin/env bats

load './helpers.bash'

@test "recursive skips binary files" {
  mkdir "$TEST_TMPDIR/dir"
  cp "$FILES_DIR/generic/1_in.txt" "$TEST_TMPDIR/dir/file.txt"
  printf '\x00\xff\x00\xff' > "$TEST_TMPDIR/dir/binary.bin"

  run_keepsorted --mode fix --recursive "$TEST_TMPDIR/dir"
  [ "$status" -eq "$EXIT_SUCCESS" ]
  diff "$FILES_DIR/generic/1_out.txt" "$TEST_TMPDIR/dir/file.txt"
}
