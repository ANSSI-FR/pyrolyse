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


function replay() {
    scenario=$1
    test_index_offset=$2
    
    "${PYROLYSE_PATH}/tools/script/network_stack/replay_capture_test_sequence_scenario.sh" \
    "${network_interface}" \
    "${tcpdump_filter_global}" \
    "${tcpdump_filter_single}" \
    "${test_index_offset}" \
    "${target_directory_path}" \
    "tc/tc_ipv${ip_version}_${scenario}" \
    "${nb_processes}"
}
export -f replay

pyrolyse_debug=${PYROLYSE_DEBUG:-0}

if [ "${pyrolyse_debug}" -eq 1 ]; then
    replay pep-mf 12000
    # replay pep-nf 13000
    # replay pep-onf 14000
    # replay pep-omf 15000
    # replay pep-mnf 16000
    # replay pep-as 17000

    # replay pep-os 18000
    # replay pep-ms 19000
    # replay pep-ns 20000

    # replay pep-ons 21000
    # replay pep-oms 22000
    # replay pep-mns 23000

    # replay pep-af 10000
else
    replay pep-of 11000
    replay pep-mf 12000
    replay pep-nf 13000
    replay pep-os 18000
    replay pep-ms 19000
    replay pep-ns 20000
    
    replay peosp-af 24000
    replay peosp-of 25000
    replay peosp-mf 26000
    replay peosp-nf 27000
    replay peosp-onf 28000
    replay peosp-omf 29000
    replay peosp-mnf 30000
    replay peosp-as 31000
    replay peosp-os 32000
    replay peosp-ms 33000
    replay peosp-ns 34000
    replay peosp-ons 35000
    replay peosp-oms 36000
    replay peosp-mns 37000

    replay peosf-af 38000
    replay peosf-of 39000
    replay peosf-mf 40000
    replay peosf-nf 41000
    replay peosf-onf 42000
    replay peosf-omf 43000
    replay peosf-mnf 44000
    replay peosf-as 45000
    replay peosf-os 46000
    replay peosf-ms 47000
    replay peosf-ns 48000
    replay peosf-ons 49000
    replay peosf-oms 50000
    replay peosf-mns 51000

    replay peoef 52000
    replay peoep 53000
    replay peosfef 54000
    replay peoefsf 55000
    replay peospef 56000
    replay peoepsf 57000
    replay peospep 58000
    replay peoepsp 59000
fi

# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
elapsed_seconds=$SECONDS

printf -v date '%(%Y/%m/%d %H:%M:%S)T' -1

echo "replay_capture_test_sequence_scenarii: elapsed: $((elapsed_seconds / 3600))hrs $(((elapsed_seconds / 60) % 60))min $((elapsed_seconds % 60))sec at ${date}"

echo "replay_capture_test_sequence_scenarii: end"


