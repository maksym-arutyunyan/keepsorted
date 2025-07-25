#!/usr/bin/env bats

load './helpers.bash'

@test "test \`codeowners\` feature sorts file" {
  local file expected
  file=$(prepare_file codeowners/CODEOWNERS)
  expected="$FILES_DIR/codeowners/CODEOWNERS_out"

  run_keepsorted --mode fix --features codeowners "$file"
  [ "$status" -eq "$EXIT_SUCCESS" ]
  diff -u "$expected" "$file"
}

