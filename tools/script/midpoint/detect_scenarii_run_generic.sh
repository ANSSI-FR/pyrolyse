#!/bin/bash

set -eu
set -o pipefail

echo "detect_scenarii_run: start"

target_directory_path=$1
ids_tool_command=$2
ids_argument=$3
ids_log_name=$4
reassembly_options=$5
protocol=$6
payload_mode=$7
nb_processes=$8
nb_run=$9

echo "detect_scenarii_run: target_directory_path: ${target_directory_path}"
echo "detect_scenarii_run: ids_tool_command: ${ids_tool_command}"
echo "detect_scenarii_run: ids_argument: ${ids_argument}"
echo "detect_scenarii_run: ids_log_name: ${ids_log_name}"
echo "detect_scenarii_run: reassembly_options: ${reassembly_options}"
echo "detect_scenarii_run: payload_mode: ${payload_mode}"
echo "detect_scenarii_run: protocol: ${protocol}"
echo "detect_scenarii_run: nb_processes: ${nb_processes}"
echo "detect_scenarii_run: nb_run: ${nb_run}"


# Execution time measurement
# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
SECONDS=0

last_run_index=$(("$nb_run" - 1))
echo "detect_scenarii_run: last_run_index: ${last_run_index}"

for run in $( seq 0 $last_run_index )
do
    echo "detect_scenarii_run: run: ${run}"

    "${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_generic.sh" \
    "${target_directory_path}" \
    "${ids_tool_command}" \
    "${ids_argument}" \
    "${ids_log_name}" \
    "${reassembly_options}" \
    "${protocol}" \
    "${payload_mode}" \
    "${nb_processes}"
done

# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
elapsed_seconds=$SECONDS

printf -v date '%(%Y/%m/%d %H-%M-%S)T' -1

echo "detect_scenarii_run: elapsed: $((elapsed_seconds / 3600))hrs $(((elapsed_seconds / 60) % 60))min $((elapsed_seconds % 60))sec at ${date}"

echo "detect_scenarii_run: end"









