#!/usr/bin/env bats

load './helpers.bash'

@test "codeowners feature sorts file" {
  local file expected
  file=$(prepare_file codeowners/CODEOWNERS)
  expected="$EXPECTED_DIR/codeowners/CODEOWNERS"

  run_keepsorted --mode fix --features codeowners "$file"
  [ "$status" -eq 0 ]
  diff -u "$expected" "$file"
}

