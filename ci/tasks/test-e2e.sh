#!/bin/bash

set -eu

export CARGO_HOME="$(pwd)/cargo-home"
export CARGO_TARGET_DIR="$(pwd)/cargo-target-dir"
export BISQ_CLI_BIN_DIR=${CARGO_TARGET_DIR}/debug
export BISQ_SRC_DIR="$(pwd)/bisq-repo"

pushd ${BISQ_SRC_DIR}
./gradlew daemon:build
popd

pushd repo

make integration
