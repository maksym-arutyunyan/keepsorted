#!/usr/bin/env bats

load './helpers.bash'

@test "process directory recursively" {
  mkdir "$TEST_TMPDIR/dir"
  cp "$FILES_DIR/generic/1_in.txt" "$TEST_TMPDIR/dir/file.txt"

  run_keepsorted --mode fix --recursive "$TEST_TMPDIR/dir"
  [ "$status" -eq "$EXIT_SUCCESS" ]
  diff "$FILES_DIR/generic/1_out.txt" "$TEST_TMPDIR/dir/file.txt"
}

@test "recursive mode skips hidden directories" {
  mkdir -p "$TEST_TMPDIR/project/src" "$TEST_TMPDIR/project/.git/hooks"
  printf '# Keep sorted.\nb\na\n' > "$TEST_TMPDIR/project/src/test.txt"
  printf '# Keep sorted.\nb\na\n' > "$TEST_TMPDIR/project/.git/hooks/test.txt"

  run_keepsorted -r --check "$TEST_TMPDIR/project"
  [ "$status" -eq "$EXIT_CHECK_FAILED" ]
  [[ "$output" == *"src/test.txt"* ]]
  [[ "$output" != *".git"* ]]
}

