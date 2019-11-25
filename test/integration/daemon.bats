#!/usr/bin/env bats

load "helpers"

setup() {
  start_daemon
}

teardown() {
  stop_daemon
}

@test "returns version" {

  retry 10 2 ${cli} get-version

}
