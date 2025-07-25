#!/usr/bin/env bats

load './helpers.bash'

@test "gitignore feature sorts .gitignore" {
  run_fix gitignore/.gitignore gitignore/.gitignore --mode fix --features gitignore
}

