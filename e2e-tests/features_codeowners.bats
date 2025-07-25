#!/usr/bin/env bats

load './helpers.bash'

@test "codeowners feature sorts file" {
  run_fix codeowners/CODEOWNERS codeowners/CODEOWNERS --mode fix --features codeowners
}

