#!/bin/bash

set -eu
set -o pipefail

python_script_path=$1
merged_byte_time_sequence_json_path=$2
target_directory_path=$3
protocol=$4
payload_mode=$5
extraction_mode=$6

echo "extract_ip_scenarii: python_script_path: ${python_script_path}"
echo "extract_ip_scenarii: merged_byte_time_sequence_json_path: ${merged_byte_time_sequence_json_path}"
echo "extract_ip_scenarii: target_directory_path: ${target_directory_path}"
echo "extract_ip_scenarii: protocol: ${protocol}"
echo "extract_ip_scenarii: payload_mode: ${payload_mode}"
echo "extract_ip_scenarii: extraction_mode: ${extraction_mode}"

reassembly_options=$(basename "${target_directory_path}")
echo "extract_midpoint_payload_scenario: reassembly_options: ${reassembly_options}"

export python_script_path
export merged_byte_time_sequence_json_path
export target_directory_path
export protocol
export payload_mode
export extraction_mode
export reassembly_options

function extract_payload() {
    echo ""
    echo ""
    echo ""
    echo "extract_payload: start"
    scenario_name=$1
    nb_starting_character_to_remove=$2
    nb_final_character_to_remove=$3

    echo "extract_payload: scenario_name: ${scenario_name}"
    echo "extract_payload: nb_starting_character_to_remove: ${nb_starting_character_to_remove}"
    echo "extract_payload: nb_final_character_to_remove: ${nb_final_character_to_remove}"

    latest_log_path="${target_directory_path}/${protocol}_${scenario_name}_latest"
    json_path="${target_directory_path}/${protocol}_${scenario_name}_payload.json"

    log="${target_directory_path}/log_payload_extraction_${scenario_name}.log"
    echo "extract_payload: log: ${log}"

    python3 "${python_script_path}" \
    -i "${latest_log_path}" \
    -j "${json_path}" \
    -m "${merged_byte_time_sequence_json_path}" \
    -p "${protocol}" \
    -pm "${payload_mode}" \
    -em "${extraction_mode}" \
    -s "${nb_starting_character_to_remove}" \
    -f "${nb_final_character_to_remove}" &> "${log}"

    echo "extract_payload: end"
}
export -f extract_payload

if [[ "${protocol}" == "tcp" ]]; then
    #extract_payload pep-ap 0 0
    #extract_payload pep-ao 0 0
    #extract_payload peosp-ap 1 0
    #extract_payload peosp-ao 1 0
    #extract_payload peoef-ap 0 1
    #extract_payload peoef-ao 0 1
    #extract_payload peoep-ap 0 1
    #extract_payload peoep-ao 0 1
    #extract_payload peospef-ap 1 1
    #extract_payload peospef-ao 1 1
    #extract_payload peospep-ap 1 1
    #extract_payload peospep-ao 1 1
    #extract_payload peoepsp-ap 1 1
    #extract_payload peoepsp-ao 1 1

    extract_payload peosf 1 0
    #extract_payload peosfef 1 1
    #extract_payload peoefsf 1 1
    #extract_payload peoepsf 1 1
else
    
    #extract_payload pep-af 0 0
    #extract_payload pep-of 0 0
    #extract_payload pep-mf 0 0
    extract_payload pep-nf 0 0
    #extract_payload pep-onf 0 0
    #extract_payload pep-omf 0 0
    #extract_payload pep-mnf 0 0
    #extract_payload pep-as 0 0
    #extract_payload pep-os 0 0
    #extract_payload pep-ms 0 0
    #extract_payload pep-ns 0 0
    #extract_payload pep-ons 0 0
    #extract_payload pep-oms 0 0
    #extract_payload pep-mns 0 0
    #extract_payload peosp-af 0 0
    #extract_payload peosp-of 0 0
    #extract_payload peosp-mf 0 0
    #extract_payload peosp-nf 0 0
    #extract_payload peosp-onf 0 0
    #extract_payload peosp-omf 0 0
    #extract_payload peosp-mnf 0 0
    #extract_payload peosp-as 0 0
    #extract_payload peosp-os 0 0
    #extract_payload peosp-ms 0 0
    #extract_payload peosp-ns 0 0
    #extract_payload peosp-ons 0 0
    #extract_payload peosp-oms 0 0
    #extract_payload peosp-mns 0 0
    #extract_payload peosf-af 0 0
    #extract_payload peosf-of 0 0
    #extract_payload peosf-mf 0 0
    #extract_payload peosf-nf 0 0
    #extract_payload peosf-onf 0 0
    #extract_payload peosf-omf 0 0
    #extract_payload peosf-mnf 0 0
    #extract_payload peosf-as 0 0
    #extract_payload peosf-os 0 0
    #extract_payload peosf-ms 0 0
    #extract_payload peosf-ns 0 0
    #extract_payload peosf-ons 0 0
    #extract_payload peosf-oms 0 0
    #extract_payload peosf-mns 0 0
    
    #extract_payload peoef 0 8
    #extract_payload peoep 0 8
    #extract_payload peosfef 0 8
    #extract_payload peoefsf 0 8
    #extract_payload peospef 0 8
    #extract_payload peoepsf 0 8
    #extract_payload peospep 0 8
    #extract_payload peoepsp 0 8
fi












