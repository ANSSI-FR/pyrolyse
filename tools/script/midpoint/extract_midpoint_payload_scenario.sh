#!/bin/bash

set -eu
set -o pipefail

echo "extract_midpoint_payload_scenario: start"

python_script_path=$1
merged_byte_time_sequence_json_path=$2
target_directory_path=$3
scenario=$4
protocol=$5
nb_starting_character_to_remove=$6
nb_final_character_to_remove=$7
payload_mode=$8
extraction_mode=$9
nb_processes=${10}

echo "extract_midpoint_payload_scenario: python_script_path: ${python_script_path}"
echo "extract_midpoint_payload_scenario: merged_byte_time_sequence_json_path: ${merged_byte_time_sequence_json_path}"
echo "extract_midpoint_payload_scenario: target_directory_path: ${target_directory_path}"
echo "extract_midpoint_payload_scenario: scenario: ${scenario}"
echo "extract_midpoint_payload_scenario: protocol: ${protocol}"
echo "extract_midpoint_payload_scenario: nb_starting_character_to_remove: ${nb_starting_character_to_remove}"
echo "extract_midpoint_payload_scenario: nb_final_character_to_remove: ${nb_final_character_to_remove}"
echo "extract_midpoint_payload_scenario: payload_mode: ${payload_mode}"
echo "extract_midpoint_payload_scenario: extraction_mode: ${extraction_mode}"
echo "extract_midpoint_payload_scenario: nb_processes: ${nb_processes}"

reassembly_options=$(basename "${target_directory_path}")
echo "extract_midpoint_payload_scenario: reassembly_options: ${reassembly_options}"

export python_script_path
export merged_byte_time_sequence_json_path
export target_directory_path
export scenario
export protocol
export nb_starting_character_to_remove
export nb_final_character_to_remove
export reassembly_options
export payload_mode
export extraction_mode

function extract_payload() {
    echo ""
    echo ""
    echo ""
    echo "extract_payload: start"
    directory_path=$1
    echo "extract_payload: directory_path: ${directory_path}"
        
    directory_name=$(basename "${directory_path}")
    echo "extract_payload: directory_name: ${directory_name}"

    scenario=$(echo "${directory_name}" | rev | cut -d_ -f3 | rev)
    date_s=$(echo "${directory_name}" | rev | cut -d_ -f2 | rev)
    time_s=$(echo "${directory_name}" | rev | cut -d_ -f1 | rev)
    echo "extract_payload: scenario: ${scenario}"
    echo "extract_payload: date_s: ${date_s}"
    echo "extract_payload: time_s: ${time_s}"
    
    json_path="${target_directory_path}/${protocol}_${scenario}_payload_${date_s}_${time_s}.json"
    echo "extract_payload: json_path: ${json_path}"
    
    log="${target_directory_path}/log_payload_extraction_${scenario}_${date_s}_${time_s}.log"
    echo "extract_payload: log: ${log}"
    
    python3 "${python_script_path}" \
    -i "${directory_path}" \
    -j "${json_path}" \
    -p "${protocol}" \
    -m "${merged_byte_time_sequence_json_path}" \
    -pm "${payload_mode}" \
    -em "${extraction_mode}" \
    -s "${nb_starting_character_to_remove}" \
    -f "${nb_final_character_to_remove}" &> "${log}"
    
    echo "extract_payload: end"
}
export -f extract_payload

find "${target_directory_path}" -maxdepth 1 -name "${protocol}_*" -type d | \
grep "_${scenario}" | \
grep -v latest | \
sort | \
parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "extract_payload {}"




