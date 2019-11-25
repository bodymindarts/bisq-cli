#!/usr/bin/env bats

load "helpers"

setup() {
  start_daemon
}

teardown() {
  stop_daemon
}

@test "returns version" {

  retry 5 1 ${cli} get-version

}
