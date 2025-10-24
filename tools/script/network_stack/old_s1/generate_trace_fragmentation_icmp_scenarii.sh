#!/bin/bash

set -xve

os_directory_path=$1
mac_src=$2
mac_dst=$3
ip_src=$4
ip_dst=$5
ip_version=$6

if [ "${ip_version}" -ne 4 ] && [ "${ip_version}" -ne 6 ]; then
    echo "Bad ip version provided"
    exit -1
fi


# pep

test_case_directory_path="${os_directory_path}/tc_ipv${ip_version}_pep"

mkdir -p "${test_case_directory_path}"

nb_pcap=$(ls -A "${test_case_directory_path}" | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ${test_case_directory_path}"
    rm "${test_case_directory_path}/"*
fi

RUST_LOG=debug \
"${PYROLYSE_PATH}/rust_code/reassembly_test_pipeline/target/debug/generate_trace_ip_fragmentation_icmp_generic" \
-i "${PYROLYSE_PATH}/test_data/byte_time_sequence.json" \
-o "${test_case_directory_path}" \
-p p \
--mac-src "${mac_src}" \
--mac-dst "${mac_dst}" \
--ip-version "${ip_version}" \
--ipv${ip_version}-src "${ip_src}" \
--ipv${ip_version}-dst "${ip_dst}"



# peos

test_case_directory_path="${os_directory_path}/tc_ipv${ip_version}_peos"

mkdir -p "${test_case_directory_path}"

nb_pcap=$(ls -A "${test_case_directory_path}" | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ${test_case_directory_path}"
    rm "${test_case_directory_path}/"*
fi

RUST_LOG=debug \
"${PYROLYSE_PATH}/rust_code/reassembly_test_pipeline/target/debug/generate_trace_ip_fragmentation_icmp_generic" \
-i "${PYROLYSE_PATH}/test_data/byte_time_sequence.json" \
-o "${test_case_directory_path}" \
-p os \
--mac-src "${mac_src}" \
--mac-dst "${mac_dst}" \
--ip-version "${ip_version}" \
--ipv${ip_version}-src "${ip_src}" \
--ipv${ip_version}-dst "${ip_dst}"



# peoe

test_case_directory_path="${os_directory_path}/tc_ipv${ip_version}_peoe"

mkdir -p "${test_case_directory_path}"

nb_pcap=$(ls -A "${test_case_directory_path}" | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ${test_case_directory_path}"
    rm "${test_case_directory_path}/"*
fi

RUST_LOG=debug \
"${PYROLYSE_PATH}/rust_code/reassembly_test_pipeline/target/debug/generate_trace_ip_fragmentation_icmp_generic" \
-i "${PYROLYSE_PATH}/test_data/byte_time_sequence.json" \
-o "${test_case_directory_path}" \
-p oe \
--mac-src "${mac_src}" \
--mac-dst "${mac_dst}" \
--ip-version "${ip_version}" \
--ipv${ip_version}-src "${ip_src}" \
--ipv${ip_version}-dst "${ip_dst}"



# peose

test_case_directory_path="${os_directory_path}/tc_ipv${ip_version}_peose"

mkdir -p "${test_case_directory_path}"

nb_pcap=$(ls -A "${test_case_directory_path}" | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ${test_case_directory_path}"
    rm "${test_case_directory_path}/"*
fi

RUST_LOG=debug \
"${PYROLYSE_PATH}/rust_code/reassembly_test_pipeline/target/debug/generate_trace_ip_fragmentation_icmp_generic" \
-i "${PYROLYSE_PATH}/test_data/byte_time_sequence.json" \
-o "${test_case_directory_path}" \
-p ose \
--mac-src "${mac_src}" \
--mac-dst "${mac_dst}" \
--ip-version "${ip_version}" \
--ipv${ip_version}-src "${ip_src}" \
--ipv${ip_version}-dst "${ip_dst}"



# peoes

test_case_directory_path="${os_directory_path}/tc_ipv${ip_version}_peoes"

mkdir -p "${test_case_directory_path}"

nb_pcap=$(ls -A "${test_case_directory_path}" | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ${test_case_directory_path}"
    rm "${test_case_directory_path}/"*
fi

RUST_LOG=debug \
"${PYROLYSE_PATH}/rust_code/reassembly_test_pipeline/target/debug/generate_trace_ip_fragmentation_icmp_generic" \
-i "${PYROLYSE_PATH}/test_data/byte_time_sequence.json" \
-o "${test_case_directory_path}" \
-p oes \
--mac-src "${mac_src}" \
--mac-dst "${mac_dst}" \
--ip-version "${ip_version}" \
--ipv${ip_version}-src "${ip_src}" \
--ipv${ip_version}-dst "${ip_dst}"
