#!/bin/bash

set -eu
set -o pipefail

echo "replay_capture_test_sequence_scenarii: start"

network_interface=$1
tcpdump_filter_global=$2
tcpdump_filter_single=$3
target_directory_path=$4
ip_version=$5
nb_processes=$6

echo "replay_capture_test_sequence_scenarii: network_interface: ${network_interface}"
echo "replay_capture_test_sequence_scenarii: tcpdump_filter_global: ${tcpdump_filter_global}"
echo "replay_capture_test_sequence_scenarii: tcpdump_filter_single: ${tcpdump_filter_single}"
echo "replay_capture_test_sequence_scenarii: target_directory_path: ${target_directory_path}"
echo "replay_capture_test_sequence_scenarii: ip_version: ${ip_version}"
echo "replay_capture_test_sequence_scenarii: nb_processes: ${nb_processes}"

# Execution time measurement
# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
SECONDS=0

"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenario.sh" \
"${network_interface}" \
"${tcpdump_filter_global}" \
"${tcpdump_filter_single}" \
"${target_directory_path}" \
"tc/tc_ipv${ip_version}_pep" \
"${nb_processes}"

"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenario.sh" \
"${network_interface}" \
"${tcpdump_filter_global}" \
"${tcpdump_filter_single}" \
"${target_directory_path}" \
"tc/tc_ipv${ip_version}_peos" \
"${nb_processes}"

"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenario.sh" \
"${network_interface}" \
"${tcpdump_filter_global}" \
"${tcpdump_filter_single}" \
"${target_directory_path}" \
"tc/tc_ipv${ip_version}_peoe" \
"${nb_processes}"

"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenario.sh" \
"${network_interface}" \
"${tcpdump_filter_global}" \
"${tcpdump_filter_single}" \
"${target_directory_path}" \
"tc/tc_ipv${ip_version}_peose" \
"${nb_processes}"

"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenario.sh" \
"${network_interface}" \
"${tcpdump_filter_global}" \
"${tcpdump_filter_single}" \
"${target_directory_path}" \
"tc/tc_ipv${ip_version}_peoes" \
"${nb_processes}"

# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
elapsed_seconds=$SECONDS

printf -v date '%(%Y/%m/%d %H-%M-%S)T' -1

echo "replay_capture_test_sequence_scenarii: elapsed: $((elapsed_seconds / 3600))hrs $(((elapsed_seconds / 60) % 60))min $((elapsed_seconds % 60))sec at ${date}"

echo "replay_capture_test_sequence_scenarii: end"









