#!/bin/bash

cargo test

./run-clippy.sh
./run-fmt.sh
./run-e2e-tests.sh
