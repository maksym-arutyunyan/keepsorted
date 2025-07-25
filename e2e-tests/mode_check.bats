#!/usr/bin/env bats

load './helpers.bash'

@test "--mode check fails on unsorted file" {
  run_check generic/1.txt false --mode check
}

@test "--mode check succeeds on sorted file" {
  run_check generic/1_sorted.txt true --mode check
}
