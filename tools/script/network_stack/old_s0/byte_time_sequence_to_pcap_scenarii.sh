#!/bin/bash

set -eu
set -o pipefail


chunk_pattern_path=$1
byte_time_sequence_directory_path=$2
target_directory_path=$3
mac_src=$4
mac_dst=$5
ip_version=$6
ip_src=$7
ip_dst=$8
output_protocol=$9





if [ "${ip_version}" -ne 4 ] && [ "${ip_version}" -ne 6 ]; then
    echo "Bad ip version provided"
    exit 1
fi


# pep

test_case_directory_path="${target_directory_path}/tc/tc_ipv${ip_version}_pep"

mkdir -p "${test_case_directory_path}"

nb_pcap=$(ls -A "${test_case_directory_path}" | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ${test_case_directory_path}"
    rm "${test_case_directory_path}/"*
fi

"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenario.sh" \
"${byte_time_sequence_directory_path}" \
"${byte_time_sequence_directory_path}" \
"${test_case_directory_path}" \
"${mac_src}" \
"${mac_dst}" \
"${ip_version}" \
"${ip_src}" \
"${ip_dst}" \
"${output_protocol}" \
"p"



# peos

test_case_directory_path="${target_directory_path}/tc/tc_ipv${ip_version}_peos"

mkdir -p "${test_case_directory_path}"

nb_pcap=$(ls -A "${test_case_directory_path}" | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ${test_case_directory_path}"
    rm "${test_case_directory_path}/"*
fi

"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenario.sh" \
"${byte_time_sequence_directory_path}" \
"${test_case_directory_path}" \
"${mac_src}" \
"${mac_dst}" \
"${ip_version}" \
"${ip_src}" \
"${ip_dst}" \
"${output_protocol}" \
"os"



# peoe

test_case_directory_path="${target_directory_path}/tc/tc_ipv${ip_version}_peoe"

mkdir -p "${test_case_directory_path}"

nb_pcap=$(ls -A "${test_case_directory_path}" | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ${test_case_directory_path}"
    rm "${test_case_directory_path}/"*
fi

"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenario.sh" \
"${byte_time_sequence_directory_path}" \
"${test_case_directory_path}" \
"${mac_src}" \
"${mac_dst}" \
"${ip_version}" \
"${ip_src}" \
"${ip_dst}" \
"${output_protocol}" \
"oe"



# peose

test_case_directory_path="${target_directory_path}/tc/tc_ipv${ip_version}_peose"

mkdir -p "${test_case_directory_path}"

nb_pcap=$(ls -A "${test_case_directory_path}" | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ${test_case_directory_path}"
    rm "${test_case_directory_path}/"*
fi

"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenario.sh" \
"${byte_time_sequence_directory_path}" \
"${test_case_directory_path}" \
"${mac_src}" \
"${mac_dst}" \
"${ip_version}" \
"${ip_src}" \
"${ip_dst}" \
"${output_protocol}" \
"ose"



# peoes

test_case_directory_path="${target_directory_path}/tc/tc_ipv${ip_version}_peoes"

mkdir -p "${test_case_directory_path}"

nb_pcap=$(ls -A "${test_case_directory_path}" | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ${test_case_directory_path}"
    rm "${test_case_directory_path}/"*
fi

"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenario.sh" \
"${byte_time_sequence_directory_path}" \
"${test_case_directory_path}" \
"${mac_src}" \
"${mac_dst}" \
"${ip_version}" \
"${ip_src}" \
"${ip_dst}" \
"${output_protocol}" \
"oes"


