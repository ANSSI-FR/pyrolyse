#!/bin/bash

set -eu
set -o pipefail

echo "byte_time_sequence_to_pcap_for_type: start"

chunk_pattern_path=$1
byte_time_sequence_directory_path=$2
pcap_directory_path=$3
mac_src=$4
mac_dst=$5
ip_version=$6
ip_src=$7
ip_dst=$8
output_protocol=$9
policy_evaluation=${10}
relation_type=${11}

echo "byte_time_sequence_to_pcap_for_type: chunk_pattern_path: ${chunk_pattern_path}"
echo "byte_time_sequence_to_pcap_for_type: byte_time_sequence_directory_path: ${byte_time_sequence_directory_path}"
echo "byte_time_sequence_to_pcap_for_type: pcap_directory_path: ${pcap_directory_path}"
echo "byte_time_sequence_to_pcap_for_type: mac_src: ${mac_src}"
echo "byte_time_sequence_to_pcap_for_type: mac_dst: ${mac_dst}"
echo "byte_time_sequence_to_pcap_for_type: ip_version: ${ip_version}"
echo "byte_time_sequence_to_pcap_for_type: ip_src: ${ip_src}"
echo "byte_time_sequence_to_pcap_for_type: ip_dst: ${ip_dst}"
echo "byte_time_sequence_to_pcap_for_type: output_protocol: ${output_protocol}"
echo "byte_time_sequence_to_pcap_for_type: policy_evaluation: ${policy_evaluation}"
echo "byte_time_sequence_to_pcap_for_type: relation_type: ${relation_type}"


if [ "${ip_version}" -ne 4 ] && [ "${ip_version}" -ne 6 ]; then
    echo "Bad ip version provided"
    exit 1
fi

mkdir -p "${pcap_directory_path}"


nb_processes=10


export chunk_pattern_path
export pcap_directory_path
export mac_src
export mac_dst
export ip_version
export ip_src
export ip_dst
export test_index_offset
export output_protocol
export policy_evaluation
export relation_type

function convert_byte_sequence_to_byte_time_sequence() {
    echo ""
    echo ""
    echo ""
    echo "convert_byte_sequence_to_byte_time_sequence: start"
    file_path=$1
    echo "convert_byte_sequence_to_byte_time_sequence: file_path: ${file_path}"

    file_name=$(basename "${file_path}")
    file_name_wo_ext="${file_name%.*}"

    output_file_path="${pcap_directory_path}/${file_name_wo_ext}.pcap"
    echo "convert_byte_sequence_to_byte_time_sequence: output_file_path: ${output_file_path}"

    RUST_LOG=debug \
    "${PYROLYSE_PATH}/rust_code/reassembly_test_pipeline/target/debug/generate_trace_ip_fragmentation" \
    --cp "${chunk_pattern_path}" \
    -s "${file_path}" \
    -o "${pcap_directory_path}" \
    --mac-src "${mac_src}" \
    --mac-dst "${mac_dst}" \
    --ip-version "${ip_version}" \
    --ipv"${ip_version}"-src "${ip_src}" \
    --ipv"${ip_version}"-dst "${ip_dst}" \
    --op "${output_protocol}" \
    --pe "${policy_evaluation}" \
    --rt "${relation_type}"

    echo "convert_byte_sequence_to_byte_time_sequence: end"
}
export -f convert_byte_sequence_to_byte_time_sequence

find "${byte_time_sequence_directory_path}" -name "${relation_type}_*.json" -type f | sort | parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "convert_byte_sequence_to_byte_time_sequence {};"

echo "byte_time_sequence_to_pcap_for_type: end"
