#!/bin/bash

set -eu
set -o pipefail

echo "extract_scenarii: start"

python_script_path=$1
merged_byte_time_sequence_json_path=$2
target_directory_path=$3
protocol=$4
payload_mode=$5
extraction_mode=$6
nb_processes=$7

echo "extract_scenarii: python_script_path: ${python_script_path}"
echo "extract_scenarii: merged_byte_time_sequence_json_path: ${merged_byte_time_sequence_json_path}"
echo "extract_scenarii: target_directory_path: ${target_directory_path}"
echo "extract_scenarii: protocol: ${protocol}"
echo "extract_scenarii: payload_mode: ${payload_mode}"
echo "extract_scenarii: extraction_mode: ${extraction_mode}"
echo "extract_scenarii: nb_processes: ${nb_processes}"

export python_script_path
export merged_byte_time_sequence_json_path
export target_directory_path
export protocol
export payload_mode
export extraction_mode
export nb_processes

# Execution time measurement
# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
SECONDS=0

function extract_payload() {
    scenario=$1
    nb_starting_character_to_remove=$2
    nb_final_character_to_remove=$3
    
    "${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenario.sh" \
    "${python_script_path}" \
    "${merged_byte_time_sequence_json_path}" \
    "${target_directory_path}" \
    "${scenario}" \
    "${protocol}" \
    "${nb_starting_character_to_remove}" \
    "${nb_final_character_to_remove}" \
    "${payload_mode}" \
    "${extraction_mode}" \
    "${nb_processes}"
}
export -f extract_payload

if [[ "${protocol}" == "tcp" ]]; then
    extract_payload pep-ap 0 0
    extract_payload pep-ao 0 0
    extract_payload peosp-ap 1 0
    extract_payload peosp-ao 1 0
    extract_payload peoef-ap 0 1
    extract_payload peoef-ao 0 1
    extract_payload peoep-ap 0 1
    extract_payload peoep-ao 0 1
    extract_payload peospef-ap 1 1
    extract_payload peospef-ao 1 1
    extract_payload peospep-ap 1 1
    extract_payload peospep-ao 1 1
    extract_payload peoepsp-ap 1 1
    extract_payload peoepsp-ao 1 1

    extract_payload peosf 1 0
    extract_payload peosfef 1 1
    extract_payload peoefsf 1 1
    extract_payload peoepsf 1 1
else
    
    extract_payload pep-of 0 0
    extract_payload pep-mf 0 0
    extract_payload pep-nf 0 0
    extract_payload pep-os 0 0
    extract_payload pep-ms 0 0
    extract_payload pep-ns 0 0
    extract_payload peosp-af 0 0
    extract_payload peosp-of 0 0
    extract_payload peosp-mf 0 0
    extract_payload peosp-nf 0 0
    extract_payload peosp-onf 0 0
    extract_payload peosp-omf 0 0
    extract_payload peosp-mnf 0 0
    extract_payload peosp-as 0 0
    extract_payload peosp-os 0 0
    extract_payload peosp-ms 0 0
    extract_payload peosp-ns 0 0
    extract_payload peosp-ons 0 0
    extract_payload peosp-oms 0 0
    extract_payload peosp-mns 0 0
    extract_payload peosf-af 0 0
    extract_payload peosf-of 0 0
    extract_payload peosf-mf 0 0
    extract_payload peosf-nf 0 0
    extract_payload peosf-onf 0 0
    extract_payload peosf-omf 0 0
    extract_payload peosf-mnf 0 0
    extract_payload peosf-as 0 0
    extract_payload peosf-os 0 0
    extract_payload peosf-ms 0 0
    extract_payload peosf-ns 0 0
    extract_payload peosf-ons 0 0
    extract_payload peosf-oms 0 0
    extract_payload peosf-mns 0 0
    
    extract_payload peoef 0 8
    extract_payload peoep 0 8
    extract_payload peosfef 0 8
    extract_payload peoefsf 0 8
    extract_payload peospef 0 8
    extract_payload peoepsf 0 8
    extract_payload peospep 0 8
    extract_payload peoepsp 0 8
fi

# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
elapsed_seconds=$SECONDS

printf -v date '%(%Y/%m/%d %H-%M-%S)T' -1

echo "extract_scenarii: elapsed: $((elapsed_seconds / 3600))hrs $(((elapsed_seconds / 60) % 60))min $((elapsed_seconds % 60))sec at ${date}"

echo "extract_scenarii: end"






