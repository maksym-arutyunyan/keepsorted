#!/usr/bin/env bats

load './helpers.bash'

@test "test conflicting modes fail" {
  run_keepsorted --check --diff e2e-tests/files/generic/1_in.txt
  [ "$status" -eq 2 ]
}

@test "test --path with positional arg fails" {
  run_keepsorted -p e2e-tests/files/generic/1_in.txt e2e-tests/files/generic/1_in.txt
  [ "$status" -eq 2 ]
}

@test "test unknown feature fails" {
  run_keepsorted --features nope e2e-tests/files/generic/1_in.txt
  [ "$status" -eq 2 ]
}
