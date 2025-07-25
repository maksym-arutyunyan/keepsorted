#!/usr/bin/env bats

load './helpers.bash'

@test "gitignore feature sorts .gitignore" {
  local file expected
  file=$(prepare_file gitignore/.gitignore)
  expected="$FILES_DIR/gitignore/.gitignore_out"

  run_keepsorted --mode fix --features gitignore "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

