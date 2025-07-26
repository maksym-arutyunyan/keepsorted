#!/usr/bin/env bats

load './helpers.bash'

@test "--quiet suppresses skip messages" {
  mkdir "$TEST_TMPDIR/dir"
  cp "$FILES_DIR/generic/1_in.txt" "$TEST_TMPDIR/dir/file.txt"
  printf '\x00' > "$TEST_TMPDIR/dir/binary.bin"

  run_keepsorted --mode diff --recursive --quiet "$TEST_TMPDIR/dir"
  [ "$status" -eq "$EXIT_CHECK_FAILED" ]
  [[ "$output" == ---* ]]
  [[ "$output" == *"@@ -1,4 +1,4 @@"* ]]
  [[ "$stderr" == "" ]]
}
