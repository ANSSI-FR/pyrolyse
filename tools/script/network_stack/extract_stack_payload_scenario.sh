#!/bin/bash

set -eu
set -o pipefail

echo "extract_stack_payload_scenario: start"

python_script_path=$1
target_directory_path=$2
scenario=$3
nb_starting_character_to_remove=$4
nb_finishing_character_to_remove=$5
protocol=$6
test_index_offset=$7

echo "extract_stack_payload_scenario: python_script_path: ${python_script_path}"
echo "extract_stack_payload_scenario: target_directory_path: ${target_directory_path}"
echo "extract_stack_payload_scenario: scenario: ${scenario}"
echo "extract_stack_payload_scenario: nb_starting_character_to_remove: ${nb_starting_character_to_remove}"
echo "extract_stack_payload_scenario: nb_finishing_character_to_remove: ${nb_finishing_character_to_remove}"
echo "extract_stack_payload_scenario: protocol: ${protocol}"
echo "extract_stack_payload_scenario: test_index_offset: ${test_index_offset}"

# printf -v date '%(%Y%m%d_%H%M%S)T' -1

export python_script_path
export target_directory_path
export scenario
export nb_starting_character_to_remove
export nb_finishing_character_to_remove
export protocol
export test_index_offset

function extract_payload() {
    # This should not be needed (cf link below) but we actually do.
    # https://unix.stackexchange.com/questions/405382/set-e-inside-a-bash-function
    set -e

    echo ""
    echo ""
    echo ""
    echo "extract_stack_payload_scenario: extract_payload: start"
    directory_path=$1
    echo "extract_stack_payload_scenario: extract_payload: directory_path: ${directory_path}"
    
    # pcap_path_wo_ext="${directory_path%.*}"
    
    directory_name=$(basename "${directory_path}")
    echo "extract_stack_payload_scenario: extract_payload: directory_name: ${directory_name}"

    scenario=$(echo "${directory_name}" | rev | cut -d_ -f3 | rev)
    date_s=$(echo "${directory_name}" | rev | cut -d_ -f2 | rev)
    time_s=$(echo "${directory_name}" | rev | cut -d_ -f1 | rev)
    echo "extract_stack_payload_scenario: extract_payload: scenario: ${scenario}"
    echo "extract_stack_payload_scenario: extract_payload: date_s: ${date_s}"
    echo "extract_stack_payload_scenario: extract_payload: time_s: ${time_s}"
    
    json_path="${target_directory_path}/${protocol}_${scenario}_payload_${date_s}_${time_s}.json"
    echo "extract_stack_payload_scenario: extract_payload: json_path: $json_path"
    
    log_path="${target_directory_path}/log_${protocol}_${scenario}_payload_extraction_${date_s}_${time_s}.log"
    echo "extract_stack_payload_scenario: extract_payload: log_path: ${log_path}"
    
    set -x
    python3 "${python_script_path}" \
    -i "${directory_path}" \
    -j "${json_path}" \
    -sr "${nb_starting_character_to_remove}" \
    -fr "${nb_finishing_character_to_remove}" \
    -o "${test_index_offset}" &> "${log_path}"
    # -p "${protocol}" \
    # echo "$?"
    set +x
    
    echo "extract_stack_payload_scenario: extract_payload: end"
}
export -f extract_payload

find "${target_directory_path}" -maxdepth 1 -name "${protocol}_*" -type d | grep "_${scenario}" | grep -v latest | sort | xargs -I {} bash -c "extract_payload {}"

echo "extract_stack_payload_scenario: end"










