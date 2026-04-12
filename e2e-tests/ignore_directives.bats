#!/usr/bin/env bats

load './helpers.bash'

@test "\`ignore file\` skips the entire file in check mode" {
  local file
  file=$(prepare_file generic/ignore_file.txt)

  run_keepsorted --mode check "$file"
  [ "$status" -eq "$EXIT_SUCCESS" ]
}

@test "\`ignore block\` skips one block but check passes on sorted blocks" {
  local file
  file=$(prepare_file generic/ignore_block.txt)

  run_keepsorted --mode check "$file"
  [ "$status" -eq "$EXIT_SUCCESS" ]
}
