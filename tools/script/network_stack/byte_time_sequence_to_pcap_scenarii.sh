#!/bin/bash

set -eu
set -o pipefail

chunk_pattern_json_path=$1
byte_time_sequence_directory_path=$2
target_directory_path=$3
mac_src=$4
mac_dst=$5
ip_version=$6
ip_src=$7
ip_dst=$8
output_protocol=$9
generate_only_unique_test_cases=${10}
payload_mode=${11}


if [ "${ip_version}" -ne 4 ] && [ "${ip_version}" -ne 6 ]; then
    echo "Bad ip version provided"
    exit 1
fi

export chunk_pattern_json_path


function build_pcap() {
    policy_evaluation=$1
    test_index_offset=$2
    
    test_case_directory_path="${target_directory_path}/tc/tc_ipv${ip_version}_pe${policy_evaluation}"

    mkdir -p "${test_case_directory_path}"

    nb_pcap=$(find "${test_case_directory_path}" -type f | wc -l)
    if [ "${nb_pcap}" -gt 0 ]; then
        echo "Files present in ${test_case_directory_path}"
        rm "${test_case_directory_path}/"*
    fi

    "${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_scenario.sh" \
    "${chunk_pattern_json_path}" \
    "${byte_time_sequence_directory_path}" \
    "${test_case_directory_path}" \
    "${mac_src}" \
    "${mac_dst}" \
    "${ip_version}" \
    "${ip_src}" \
    "${ip_dst}" \
    "${test_index_offset}" \
    "${output_protocol}" \
    "${policy_evaluation}" \
    "${payload_mode}" \
    "${generate_only_unique_test_cases}"
}
export -f build_pcap

pyrolyse_debug=${PYROLYSE_DEBUG:-0}

if [ "${pyrolyse_debug}" -eq 1 ]; then
    build_pcap p-of 11000
else
    build_pcap p-of 11000
    build_pcap p-mf 12000
    build_pcap p-nf 13000
    build_pcap p-os 18000
    build_pcap p-ms 19000
    build_pcap p-ns 20000

    build_pcap osp-af 24000
    build_pcap osp-of 25000
    build_pcap osp-mf 26000
    build_pcap osp-nf 27000
    build_pcap osp-onf 28000
    build_pcap osp-omf 29000
    build_pcap osp-mnf 30000
    build_pcap osp-as 31000
    build_pcap osp-os 32000
    build_pcap osp-ms 33000
    build_pcap osp-ns 34000
    build_pcap osp-ons 35000
    build_pcap osp-oms 36000
    build_pcap osp-mns 37000

    build_pcap osf-af 38000
    build_pcap osf-of 39000
    build_pcap osf-mf 40000
    build_pcap osf-nf 41000
    build_pcap osf-onf 42000
    build_pcap osf-omf 43000
    build_pcap osf-mnf 44000
    build_pcap osf-as 45000
    build_pcap osf-os 46000
    build_pcap osf-ms 47000
    build_pcap osf-ns 48000
    build_pcap osf-ons 49000
    build_pcap osf-oms 50000
    build_pcap osf-mns 51000

    build_pcap oef 52000
    build_pcap oep 53000
    build_pcap osfef 54000
    build_pcap oefsf 55000
    build_pcap ospef 56000
    build_pcap oepsf 57000
    build_pcap ospep 58000
    build_pcap oepsp 59000
fi
