#!/bin/bash

set -eu
set -o pipefail

echo "acquire_tcp_traffic_scenarii_run: start"

target_directory_path=$1
byte_time_sequence_path=$2
ip_src=$3
ip_dst=$4
mac_src=$5
mac_dst=$6
payload_mode=$7
nb_processes=$8
nb_run=$9

echo "acquire_tcp_traffic_scenarii_run: target_directory_path: ${target_directory_path}"
echo "acquire_tcp_traffic_scenarii_run: byte_time_sequence_path: ${byte_time_sequence_path}"
echo "acquire_tcp_traffic_scenarii_run: ip_src: ${ip_src}"
echo "acquire_tcp_traffic_scenarii_run: ip_dst: ${ip_dst}"
echo "acquire_tcp_traffic_scenarii_run: mac_src: ${mac_src}"
echo "acquire_tcp_traffic_scenarii_run: mac_dst: ${mac_dst}"
echo "acquire_tcp_traffic_scenarii_run: payload_mode: ${payload_mode}"
echo "acquire_tcp_traffic_scenarii_run: nb_processes: ${nb_processes}"
echo "acquire_tcp_traffic_scenarii_run: nb_run: ${nb_run}"

# Execution time measurement
# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
SECONDS=0

last_run_index=$(("$nb_run" - 1))
echo "acquire_tcp_traffic_scenarii_run: last_run_index: ${last_run_index}"

for run in $( seq 0 $last_run_index )
do
    echo "acquire_tcp_traffic_scenarii_run: run: ${run}"

    "${PYROLYSE_PATH}/tools/script/custom_stack/acquire_tcp_traffic_scenarii.sh" \
    "${target_directory_path}" \
    "${byte_time_sequence_path}" \
    "${ip_src}" \
    "${ip_dst}" \
    "${mac_src}" \
    "${mac_dst}" \
    "${payload_mode}" \
    "${nb_processes}"
done

elapsed_seconds=$SECONDS

printf -v date '%(%Y/%m/%d %H-%M-%S)T' -1

echo "acquire_tcp_traffic_scenarii_run: elapsed: $((elapsed_seconds / 3600))hrs $(((elapsed_seconds / 60) % 60))min $((elapsed_seconds % 60))sec at ${date}"

echo "acquire_tcp_traffic_scenarii_run: end"









