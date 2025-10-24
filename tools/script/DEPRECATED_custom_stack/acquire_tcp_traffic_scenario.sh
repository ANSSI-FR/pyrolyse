#!/bin/bash

set -eu
set -o pipefail

echo "acquire_tcp_traffic_scenario: start"

target_directory_path=$1
byte_time_sequence_path=$2
scenario=$3
ip_src=$4
ip_dst=$5
mac_src=$6
mac_dst=$7
payload_mode=$8
nb_processes=$9


echo "acquire_tcp_traffic_scenario: target_directory_path: ${target_directory_path}"
echo "acquire_tcp_traffic_scenario: byte_time_sequence_path: ${byte_time_sequence_path}"
echo "acquire_tcp_traffic_scenario: scenario: ${scenario}"
echo "acquire_tcp_traffic_scenario: ip_src: ${ip_src}"
echo "acquire_tcp_traffic_scenario: ip_dst: ${ip_dst}"
echo "acquire_tcp_traffic_scenario: mac_src: ${mac_src}"
echo "acquire_tcp_traffic_scenario: mac_dst: ${mac_dst}"
echo "acquire_tcp_traffic_scenario: payload_mode: ${payload_mode}"
echo "acquire_tcp_traffic_scenario: nb_processes: ${nb_processes}"

printf -v date '%(%Y%m%d_%H%M%S)T' -1

starting_date=${date}
output_pcap_directory_latest="${target_directory_path}/tcp_${scenario}_latest"
output_pcap_directory_current="${target_directory_path}/tcp_${scenario}_${starting_date}"
log_dir="${target_directory_path}/log_tcp_${scenario}_${starting_date}"
echo "acquire_tcp_traffic_scenario: output_pcap_directory_latest: ${output_pcap_directory_latest}"
echo "acquire_tcp_traffic_scenario: output_pcap_directory_current: ${output_pcap_directory_current}"
echo "acquire_tcp_traffic_scenario: log_dir: ${log_dir}"
mkdir -p "${output_pcap_directory_current}"
mkdir -p "${output_pcap_directory_latest}"
mkdir -p "${log_dir}"

export target_directory_path
export output_pcap_directory_current
export ip_src
export ip_dst
export mac_src
export mac_dst
export byte_time_sequence_path
export scenario
export log_dir
export payload_mode


function acquire_single_test_trace() {
    set -e

    echo ""
    echo ""
    echo ""
    echo "acquire_single_test_trace: start"
    byte_time_sequence_json_path=$1
    echo "acquire_single_test_trace: byte_time_sequence_json_path: ${byte_time_sequence_json_path}"
    
    test_index=$(cat "${byte_time_sequence_json_path}" | jq ".byte_sequence_index")
    echo "acquire_single_test_trace: test_index: ${test_index}"
    
    pcap_path="${output_pcap_directory_current}/test_${test_index}.pcap"
    echo "acquire_single_test_trace: pcap_path: ${pcap_path}"
    log_path="${log_dir}/test_${test_index}.txt"
    echo "acquire_single_test_trace: log_path: ${log_path}"

    
    set -x
    sudo RUST_LOG=DEBUG "${PYROLYSE_PATH}/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk" \
    -j "${byte_time_sequence_json_path}" \
    -o "${pcap_path}" \
    -s "${ip_src}" \
    -d "${ip_dst}" \
    --mac-src "${mac_src}" \
    --mac-dst "${mac_dst}" \
    -c "${scenario}" \
    --tio 10000 \
    --payload-mode "${payload_mode}" \
    -i "${test_index}" \
    --input-mode "sbts" &> "${log_path}"
    set +x
    
    # if [[ "pair" == *"${json_path}"* ]]; then
    #     
    # else
    #     cat "${byte_time_sequence_path}" | \
    #     jq ".byte_time_triplet_sequence_c.hm.\"${test_index}\".chunk_c" \
    #     > "${chunk_json_path}"
    # fi

    echo "acquire_single_test_trace: end"
}
export -f acquire_single_test_trace


sudo iptables -I OUTPUT 1 -d "${ip_dst}" -p tcp --tcp-flags RST RST -j DROP

find "${byte_time_sequence_path}" -name "*.json" | parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "acquire_single_test_trace {};"

sudo iptables -D OUTPUT 1

echo "replay_capture_test_sequence_scenario: copying latest data"
rm -r "${output_pcap_directory_latest}"
cp -r "${output_pcap_directory_current}" "${output_pcap_directory_latest}"

echo "replay_capture_test_sequence_scenario: date: $(date)"

echo "replay_capture_test_sequence_scenario: end"


