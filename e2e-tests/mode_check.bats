#!/usr/bin/env bats

load './helpers.bash'

@test "--mode check fails on unsorted file" {
  local file
  file=$(prepare_file generic/1.txt)

  run_keepsorted --mode check "$file"
  [ "$status" -ne 0 ]
}

@test "--mode check succeeds on sorted file" {
  local file
  file=$(prepare_file generic/1_sorted.txt)

  run_keepsorted --mode check "$file"
  [ "$status" -eq 0 ]
}
