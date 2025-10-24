#!/bin/bash

set -eu
set -o pipefail

echo "replay_capture_test_sequence_scenarii_run: start"

network_interface=$1
tcpdump_filter_global=$2
tcpdump_filter_single=$3
target_directory_path=$4
ip_version=$5
nb_processes=$6
nb_run=$7

echo "replay_capture_test_sequence_scenarii_run: network_interface: ${network_interface}"
echo "replay_capture_test_sequence_scenarii_run: tcpdump_filter_global: ${tcpdump_filter_global}"
echo "replay_capture_test_sequence_scenarii_run: tcpdump_filter_single: ${tcpdump_filter_single}"
echo "replay_capture_test_sequence_scenarii_run: target_directory_path: ${target_directory_path}"
echo "replay_capture_test_sequence_scenarii_run: ip_version: ${ip_version}"
echo "replay_capture_test_sequence_scenarii_run: nb_processes: ${nb_processes}"
echo "replay_capture_test_sequence_scenarii_run: nb_run: ${nb_run}"

# Execution time measurement
# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
SECONDS=0

last_run_index=$(("$nb_run" - 1))
echo "replay_capture_test_sequence_scenarii_run: last_run_index: ${last_run_index}"

for run in $( seq 0 $last_run_index )
do
    echo "replay_capture_test_sequence_scenarii_run: run: ${run}"

    "${PYROLYSE_PATH}/tools/script/network_stack/replay_capture_test_sequence_scenarii.sh" \
    "${network_interface}" \
    "${tcpdump_filter_global}" \
    "${tcpdump_filter_single}" \
    "${target_directory_path}" \
    "${ip_version}" \
    "${nb_processes}"
done

# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
elapsed_seconds=$SECONDS

printf -v date '%(%Y/%m/%d %H-%M-%S)T' -1

echo "replay_capture_test_sequence_scenarii_run: elapsed: $((elapsed_seconds / 3600))hrs $(((elapsed_seconds / 60) % 60))min $((elapsed_seconds % 60))sec at ${date}"

echo "replay_capture_test_sequence_scenarii_run: end"









