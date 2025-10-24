#!/bin/bash

set -eu
set -o pipefail


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


mkdir -p "${pcap_directory_path}"


"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_for_type.sh" \
"${chunk_pattern_path}" \
"${byte_time_sequence_directory_path}" \
"${pcap_directory_path}" \
"${mac_src}" \
"${mac_dst}" \
"${ip_version}" \
"${ip_src}" \
"${ip_dst}" \
"${output_protocol}" \
"${policy_evaluation}" \
pair


"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_for_type.sh" \
"${chunk_pattern_path}" \
"${byte_time_sequence_directory_path}" \
"${pcap_directory_path}" \
"${mac_src}" \
"${mac_dst}" \
"${ip_version}" \
"${ip_src}" \
"${ip_dst}" \
"${output_protocol}" \
"${policy_evaluation}" \
triplet