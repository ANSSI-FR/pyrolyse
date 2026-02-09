#!/bin/bash

set -eu
set -o pipefail

echo "detect_scenarii: start"

target_directory_path=$1
ids_tool_command=$2
ids_argument=$3
ids_log_name=$4
reassembly_options=$5
protocol=$6
payload_mode=$7
nb_processes=$8

echo "detect_scenarii: target_directory_path: ${target_directory_path}"
echo "detect_scenarii: ids_tool_command: ${ids_tool_command}"
echo "detect_scenarii: ids_argument: ${ids_argument}"
echo "detect_scenarii: ids_log_name: ${ids_log_name}"
echo "detect_scenarii: reassembly_options: ${reassembly_options}"
echo "detect_scenarii: payload_mode: ${payload_mode}"
echo "detect_scenarii: protocol: ${protocol}"
echo "detect_scenarii: nb_processes: ${nb_processes}"

# Execution time measurement
# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
SECONDS=0

function detect() {
    scenario=$1
    
    "${PYROLYSE_PATH}/tools/script/midpoint/detect_scenario_generic.sh" \
    "${target_directory_path}" \
    "tc/tc_${protocol}_${scenario}" \
    "${ids_tool_command}" \
    "${ids_argument}" \
    "${ids_log_name}" \
    "${reassembly_options}" \
    "${payload_mode}" \
    "${protocol}" \
    "${nb_processes}"

}
export -f detect

if [[ "${protocol}" == "tcp" ]]; then
    detect pep-ap
    detect pep-ao
    detect peosp-ap
    detect peosp-ao
    detect peoef-ap
    detect peoef-ao
    detect peoep-ap
    detect peoep-ao
    detect peospef-ap
    detect peospef-ao
    detect peospep-ap
    detect peospep-ao
    detect peoepsp-ap
    detect peoepsp-ao

    detect peosf
    detect peosfef
    detect peoefsf
    detect peoepsf
else
    detect pep-of
    detect pep-mf
    detect pep-nf
    detect pep-os
    detect pep-ms
    detect pep-ns
    
    detect peosp-af
    detect peosp-of
    detect peosp-mf
    detect peosp-nf
    detect peosp-onf
    detect peosp-omf
    detect peosp-mnf
    detect peosp-as
    detect peosp-os
    detect peosp-ms
    detect peosp-ns
    detect peosp-ons
    detect peosp-oms
    detect peosp-mns
    
    detect peosf-af
    detect peosf-of
    detect peosf-mf
    detect peosf-nf
    detect peosf-onf
    detect peosf-omf
    detect peosf-mnf
    detect peosf-as
    detect peosf-os
    detect peosf-ms
    detect peosf-ns
    detect peosf-ons
    detect peosf-oms
    detect peosf-mns
    
    detect peoef
    detect peoep
    detect peosfef
    detect peoefsf
    detect peospef
    detect peoepsf
    detect peospep
    detect peoepsp
fi




# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
elapsed_seconds=$SECONDS

printf -v date '%(%Y/%m/%d %H-%M-%S)T' -1

echo "detect_scenarii: elapsed: $((elapsed_seconds / 3600))hrs $(((elapsed_seconds / 60) % 60))min $((elapsed_seconds % 60))sec at ${date}"

echo "detect_scenarii: end"
