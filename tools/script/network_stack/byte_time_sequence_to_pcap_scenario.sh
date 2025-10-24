#!/bin/bash

set -eu
set -o pipefail


chunk_pattern_json_path=$1
byte_time_sequence_directory_path=$2
pcap_directory_path=$3
mac_src=$4
mac_dst=$5
ip_version=$6
ip_src=$7
ip_dst=$8
test_index_offset=$9
output_protocol=${10}
policy_evaluation=${11}
payload_mode=${12}
generate_only_unique_test_cases=${13}

echo "byte_time_sequence_to_pcap: chunk_pattern_json_path: ${chunk_pattern_json_path}"
echo "byte_time_sequence_to_pcap: byte_time_sequence_directory_path: ${byte_time_sequence_directory_path}"
echo "byte_time_sequence_to_pcap: pcap_directory_path: ${pcap_directory_path}"
echo "byte_time_sequence_to_pcap: mac_src: ${mac_src}"
echo "byte_time_sequence_to_pcap: mac_dst: ${mac_dst}"
echo "byte_time_sequence_to_pcap: ip_version: ${ip_version}"
echo "byte_time_sequence_to_pcap: ip_src: ${ip_src}"
echo "byte_time_sequence_to_pcap: ip_dst: ${ip_dst}"
echo "byte_time_sequence_to_pcap: test_index_offset: ${test_index_offset}"
echo "byte_time_sequence_to_pcap: output_protocol: ${output_protocol}"
echo "byte_time_sequence_to_pcap: policy_evaluation: ${policy_evaluation}"
echo "byte_time_sequence_to_pcap: payload_mode: ${payload_mode}"
echo "byte_time_sequence_to_pcap: generate_only_unique_test_cases: ${generate_only_unique_test_cases}"


mkdir -p "${pcap_directory_path}"


"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_for_type.sh" \
"${chunk_pattern_json_path}" \
"${byte_time_sequence_directory_path}" \
"${pcap_directory_path}" \
"${mac_src}" \
"${mac_dst}" \
"${ip_version}" \
"${ip_src}" \
"${ip_dst}" \
"${test_index_offset}" \
"${output_protocol}" \
"${policy_evaluation}" \
"${payload_mode}" \
"${generate_only_unique_test_cases}" \
pair


"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_for_type.sh" \
"${chunk_pattern_json_path}" \
"${byte_time_sequence_directory_path}" \
"${pcap_directory_path}" \
"${mac_src}" \
"${mac_dst}" \
"${ip_version}" \
"${ip_src}" \
"${ip_dst}" \
"${test_index_offset}" \
"${output_protocol}" \
"${policy_evaluation}" \
"${payload_mode}" \
"${generate_only_unique_test_cases}" \
triplet