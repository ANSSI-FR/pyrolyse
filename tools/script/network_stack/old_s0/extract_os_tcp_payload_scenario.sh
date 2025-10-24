#!/bin/bash

set -eu

python_script_path=$1
os_directory_path=$2
pattern=$3

export os_directory_path
export python_script_path
export pattern

echo "extract_icmp_payload: python_script_path: ${python_script_path}"
echo "extract_icmp_payload: os_directory_path: ${os_directory_path}"
echo "extract_icmp_payload: pattern: ${pattern}"

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
    
    json_path="${os_directory_path}/tcp_${scenario}_payload_${date_s}_${time_s}.json"
    echo "extract_payload: json_path: ${json_path}"
    
    log_path="${os_directory_path}/log_tcp_payload_extraction_${scenario}_${date_s}_${time_s}.log"    
    echo "extract_payload: log_path: ${log_path}"

    python3 "${python_script_path}" \
    -p "${directory_path}" \
    -j "${json_path}" &> "${log_path}"
    
    echo "extract_payload: end"
}
export -f extract_payload

find "${os_directory_path}" -name "output_tcp_${pattern}_20*" | sort | xargs -I {} bash -c "extract_payload {}"











