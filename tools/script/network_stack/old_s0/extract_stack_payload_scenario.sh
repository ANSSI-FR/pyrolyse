#!/bin/bash

set -eu

echo "extract_stack_payload_scenario: start"

python_script_path=$1
os_directory_path=$2
pattern=$3
nb_character_to_remove=$4
protocol=$5

echo "extract_stack_payload_scenario: python_script_path: "$python_script_path
echo "extract_stack_payload_scenario: os_directory_path: "$os_directory_path
echo "extract_stack_payload_scenario: pattern: "$pattern
echo "extract_stack_payload_scenario: nb_character_to_remove: "$nb_character_to_remove
echo "extract_stack_payload_scenario: protocol: "$protocol

printf -v date '%(%Y%m%d_%H%M%S)T' -1

export python_script_path
export os_directory_path
export pattern
export nb_character_to_remove
export protocol

function extract_payload() {
    echo ""
    echo ""
    echo ""
    echo "extract_stack_payload_scenario: extract_payload: start"
    directory_path=$1
    echo "extract_stack_payload_scenario: extract_payload: directory_path: ${directory_path}"
    
    pcap_path_wo_ext="${directory_path%.*}"
    
    directory_name=$(basename $directory_path)
    echo "extract_stack_payload_scenario: directory_name: ${directory_name}"

    scenario=`echo $directory_name | rev | cut -d_ -f3 | rev`
    date_s=`echo $directory_name | rev | cut -d_ -f2 | rev`
    time_s=`echo $directory_name | rev | cut -d_ -f1 | rev`
    echo "extract_stack_payload_scenario: extract_payload: scenario: "$scenario
    echo "extract_stack_payload_scenario: extract_payload: date_s: "$date_s
    echo "extract_stack_payload_scenario: extract_payload: time_s: "$time_s
    
    json_path="${os_directory_path}/${protocol}_${scenario}_payload_${date_s}_${time_s}.json"
    echo "extract_stack_payload_scenario: extract_payload: json_path: $json_path"
    
    log_path="${os_directory_path}/log_${protocol}_${scenario}_payload_extraction_${date_s}_${time_s}.log"
    
    if [[ "${protocol}" = "tcp" ]]; then
        set -x
        python3 "${python_script_path}" \
        -p "${directory_path}" \
        -j "${json_path}" &> "${log_path}"
        set +x
    else
        set -x
        python3 "${python_script_path}" \
        -i "${directory_path}" \
        -p "${protocol}" \
        -j "${json_path}" \
        -r "${nb_character_to_remove}" &> "${log_path}"
        set +x
    fi
    
    echo "extract_stack_payload_scenario: extract_payload: end"
}
export -f extract_payload

find "${os_directory_path}" -maxdepth 1 -name "${protocol}_*" -type d | grep "${pattern}" | grep -v latest | sort | xargs -I {} bash -c "extract_payload {}"

echo "extract_stack_payload_scenario: end"










