#!/bin/bash

# set -xve

os_directory_path=$1
byte_time_sequence_json_path=$2

export os_directory_path
export byte_time_sequence_json_path

echo "extract_icmp_payload: os_directory_path: ${os_directory_path}"
echo "extract_icmp_payload: byte_time_sequence_json_path: ${byte_time_sequence_json_path}"

printf -v date '%(%Y%m%d_%H%M%S)T' -1

function check_reassembly_consistency() {
    echo ""
    echo ""
    echo ""
    echo "check_reassembly_consistency: start"
    scenario_name=$1

    echo "check_reassembly_consistency: scenario_name: ${scenario_name}"

    json_path="${os_directory_path}/tcp_${scenario_name}_payload.json"
    result_file="${os_directory_path}/tcp_${scenario_name}_reassembly_consistency.json"

    log="${os_directory_path}/log_check_reassembly_consistency_${scenario_name}.log"
    echo "check_reassembly_consistency: log: ${log}"

    python3 "${PYROLYSE_PATH}/tools/script/midpoint/check_reassembly_consistency.py" \
    -p "${json_path}" \
    -s "${byte_time_sequence_json_path}" \
    -o "${result_file}" &> "${log}"
    
    echo "check_reassembly_consistency: end"
}
export -f check_reassembly_consistency

check_reassembly_consistency peos

check_reassembly_consistency pep1

check_reassembly_consistency pep2










