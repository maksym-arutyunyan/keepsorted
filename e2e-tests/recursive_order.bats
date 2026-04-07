#!/usr/bin/env bats

load './helpers.bash'

@test "recursive check output is in sorted file order" {
  # Create several unsorted files under a directory.
  # collect_files must return them sorted so that --mode check
  # reports files in a deterministic alphabetical order.
  mkdir "$TEST_TMPDIR/dir"
  cp "$FILES_DIR/generic/1_in.txt" "$TEST_TMPDIR/dir/c.txt"
  cp "$FILES_DIR/generic/1_in.txt" "$TEST_TMPDIR/dir/a.txt"
  cp "$FILES_DIR/generic/1_in.txt" "$TEST_TMPDIR/dir/b.txt"

  run_keepsorted --mode check --recursive "$TEST_TMPDIR/dir"
  [ "$status" -eq "$EXIT_CHECK_FAILED" ]

  # Extract just the file basenames from the output lines.
  local names
  names=$(echo "$output" | grep -o '[abc]\.txt' | tr '\n' ' ' | xargs)
  [ "$names" = "a.txt b.txt c.txt" ]
}
