#!/bin/bash

set -eu
set -o pipefail

echo "acquire_tcp_traffic_scenarii: start"

target_directory_path=$1
byte_time_sequence_path=$2
ip_version=$3
ip_src=$4
ip_dst=$5
mac_src=$6
mac_dst=$7
payload_mode=$8
nb_processes=$9

echo "acquire_tcp_traffic_scenarii: target_directory_path: ${target_directory_path}"
echo "acquire_tcp_traffic_scenarii: byte_time_sequence_path: ${byte_time_sequence_path}"
echo "acquire_tcp_traffic_scenarii: ip_version: ${ip_version}"
echo "acquire_tcp_traffic_scenarii: ip_src: ${ip_src}"
echo "acquire_tcp_traffic_scenarii: ip_dst: ${ip_dst}"
echo "acquire_tcp_traffic_scenarii: mac_src: ${mac_src}"
echo "acquire_tcp_traffic_scenarii: mac_dst: ${mac_dst}"
echo "acquire_tcp_traffic_scenarii: payload_mode: ${payload_mode}"
echo "acquire_tcp_traffic_scenarii: nb_processes: ${nb_processes}"

export target_directory_path
export byte_time_sequence_path
export ip_version
export ip_src
export ip_dst
export mac_src
export mac_dst
export payload_mode
export nb_processes

# Execution time measurement
# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
SECONDS=0

function acquire_tcp_data() {
    scenario=$1
    test_index_offset=$2
    
    "${PYROLYSE_PATH}/tools/script/network_stack/acquire_tcp_traffic_scenario.sh" \
    "${target_directory_path}" \
    "${byte_time_sequence_path}" \
    "${scenario}" \
    "${ip_version}" \
    "${ip_src}" \
    "${ip_dst}" \
    "${mac_src}" \
    "${mac_dst}" \
    "${payload_mode}" \
    "${test_index_offset}" \
    "${nb_processes}"
}
export -f acquire_tcp_data

pyrolyse_debug=${PYROLYSE_DEBUG:-0}

if [ "${pyrolyse_debug}" -eq 1 ]; then
    echo "!!!! DEBUG !!!!"
    # acquire_tcp_data "pep-ao" 13000
    # acquire_tcp_data "pep-ap" 18000
    acquire_tcp_data "peoef-ap" 14000
else
    acquire_tcp_data "pep-ap" 10000
    acquire_tcp_data "pep-ao" 11000
    acquire_tcp_data "peosp-ap" 12000
    acquire_tcp_data "peosp-ao" 13000
    acquire_tcp_data "peoef-ap" 14000
    acquire_tcp_data "peoef-ao" 15000
    acquire_tcp_data "peoep-ap" 16000
    acquire_tcp_data "peoep-ao" 17000
    acquire_tcp_data "peospef-ap" 18000
    acquire_tcp_data "peospef-ao" 19000
    acquire_tcp_data "peospep-ap" 20000
    acquire_tcp_data "peospep-ao" 21000
    acquire_tcp_data "peoepsp-ap" 22000
    acquire_tcp_data "peoepsp-ao" 23000

    acquire_tcp_data "peosf" 24000
    acquire_tcp_data "peosfef" 25000
    acquire_tcp_data "peoefsf" 26000
    acquire_tcp_data "peoepsf" 27000
fi

# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
elapsed_seconds=$SECONDS
echo "acquire_tcp_traffic_scenarii: elapsed: $((elapsed_seconds / 3600))hrs $(((elapsed_seconds / 60) % 60))min $((elapsed_seconds % 60))sec"

echo "acquire_tcp_traffic_scenarii: end"



