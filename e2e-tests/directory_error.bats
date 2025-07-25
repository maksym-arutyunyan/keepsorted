#!/usr/bin/env bats

load './helpers.bash'

@test "directory without --recursive fails" {
  mkdir "$TEST_TMPDIR/dir"
  cp "$INPUT_DIR/generic/1.txt" "$TEST_TMPDIR/dir/file.txt"
  run "$BIN" --mode fix "$TEST_TMPDIR/dir"
  [ "$status" -eq 2 ]
}

