#!/usr/bin/env bats

# Point to our local risq binary!
if [ -z "${BISQ_CLI_BIN_DIR}" ]; then
    echo "Must set BISQ_CLI_BIN_DIR variable to a location that contains bisq-cli binary!"
    exit 1
fi

if [ -z "${BISQ_SRC_DIR}" ]; then
    echo "Must set BISQ_SRC_DIR variable to the place to find the ./bisq-deamon script!"
    exit 1
fi

cli=${BISQ_CLI_BIN_DIR}/bisq-cli

test_tmp_dir() {
  mkdir -p ${BATS_TMPDIR}/${BATS_TEST_NAME}
  echo ${BATS_TMPDIR}/${BATS_TEST_NAME}
}

start_daemon() {
  background ${BISQ_SRC_DIR}/bisq-daemon --appName=${BATS_TEST_NAME}_deamon --daoActivated=false > $(test_tmp_dir)/bisq_daemon_pid
}

stop_daemon() {
  kill $(cat $(test_tmp_dir)/bisq_daemon_pid) > /dev/null
}

# Run the given command in the background. Useful for starting a
# node and then moving on with commands that exercise it for the
# test.
#
# Ensures that BATS' handling of file handles is taken into account;
# see
# https://github.com/bats-core/bats-core#printing-to-the-terminal
# https://github.com/sstephenson/bats/issues/80#issuecomment-174101686
# for details.
background() {
  "$@" 3>- &
  echo $!
}

# Stolen from
# https://github.com/docker/swarm/blob/master/test/integration/helpers.bash
retry() {
  local attempts=$1
  shift
  local delay=$1
  shift
  local i

  for ((i=0; i < attempts; i++)); do
    run "$@"
    # shellcheck disable=2154
    if [[ "$status" -eq 0 ]] ; then
      return 0
    fi
    sleep "$delay"
  done

    # shellcheck disable=2154
    echo "Command \"$*\" failed $attempts times. Output: $output"
    false
  }
