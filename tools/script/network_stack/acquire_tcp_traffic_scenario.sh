#!/bin/bash

set -eu
set -o pipefail

echo "acquire_tcp_traffic_scenario: start"

target_directory_path=$1
byte_time_sequence_path=$2
scenario=$3
ip_version=$4
ip_src=$5
ip_dst=$6
mac_src=$7
mac_dst=$8
payload_mode=$9
test_index_offset=${10}
nb_processes=${11}


echo "acquire_tcp_traffic_scenario: target_directory_path: ${target_directory_path}"
echo "acquire_tcp_traffic_scenario: byte_time_sequence_path: ${byte_time_sequence_path}"
echo "acquire_tcp_traffic_scenario: scenario: ${scenario}"
echo "acquire_tcp_traffic_scenario: ip_version: ${ip_version}"
echo "acquire_tcp_traffic_scenario: ip_src: ${ip_src}"
echo "acquire_tcp_traffic_scenario: ip_dst: ${ip_dst}"
echo "acquire_tcp_traffic_scenario: mac_src: ${mac_src}"
echo "acquire_tcp_traffic_scenario: mac_dst: ${mac_dst}"
echo "acquire_tcp_traffic_scenario: payload_mode: ${payload_mode}"
echo "acquire_tcp_traffic_scenario: test_index_offset: ${test_index_offset}"
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
sync

export target_directory_path
export output_pcap_directory_current
export ip_version
export ip_src
export ip_dst
export mac_src
export mac_dst
export byte_time_sequence_path
export scenario
export log_dir
export payload_mode
export test_index_offset

function acquire_single_test_trace() {
    set -e

    echo ""
    echo ""
    echo ""
    echo "acquire_single_test_trace: start"
    byte_time_sequence_json_path=$1
    echo "acquire_single_test_trace: byte_time_sequence_json_path: ${byte_time_sequence_json_path}"
    
    test_index=$(jq ".byte_sequence_index" "${byte_time_sequence_json_path}")
    echo "acquire_single_test_trace: test_index: ${test_index}"
    
    pcap_path="${output_pcap_directory_current}/test_${test_index}.pcap"
    echo "acquire_single_test_trace: pcap_path: ${pcap_path}"
    log_path="${log_dir}/test_${test_index}.log"
    echo "acquire_single_test_trace: log_path: ${log_path}"

    # timeout -s SIGTERM 15
    #--connection-end-mode "mrst" \
    #--connection-end-mode "fhs" \
    set -x
    sudo RUST_LOG=DEBUG "${PYROLYSE_PATH}/tools/pyrolyse-rs/target/debug/send_tcp_chunk" \
    -j "${byte_time_sequence_json_path}" \
    -o "${pcap_path}" \
    --ip-version "${ip_version}" \
    --ipv"${ip_version}"-src "${ip_src}" \
    --ipv"${ip_version}"-dst "${ip_dst}" \
    --mac-src "${mac_src}" \
    --mac-dst "${mac_dst}" \
    -c "${scenario}" \
    --tio "${test_index_offset}" \
    --payload-mode "${payload_mode}" \
    -i "${test_index}" \
    --input-mode "sbts" \
    --connection-end-mode "rst" \
    &> "${log_path}"
    set +x
    
    echo "acquire_single_test_trace: end"
}
export -f acquire_single_test_trace


scenario_log_path="${log_dir}/scenario_${scenario}.log"
echo "replay_capture_test_sequence_scenario: scenario_log_path: ${scenario_log_path}"

if [ "${ip_version}" -eq 4 ]; then
    sudo iptables -I OUTPUT 1 -d "${ip_dst}" -p tcp --tcp-flags RST RST -j DROP
    
    find "${byte_time_sequence_path}" -name "*.json" | sort | parallel --no-notice --bar --joblog "${scenario_log_path}" --halt now,fail=1 --timeout 20 --delay 0.3 -j "${nb_processes}" "acquire_single_test_trace {};"

    sudo iptables -D OUTPUT 1
elif [ "${ip_version}" -eq 6 ]; then
    sudo ip6tables -I OUTPUT 1 -d "${ip_dst}" -p tcp --tcp-flags RST RST -j DROP

    find "${byte_time_sequence_path}" -name "*.json" | sort | parallel --no-notice --bar --joblog "${scenario_log_path}" --halt now,fail=1 --timeout 20 --delay 0.3 -j "${nb_processes}" "acquire_single_test_trace {};"
    
    sudo ip6tables -D OUTPUT 1
else 
    echo "Bad IP version provided"
    exit 1
fi



echo "replay_capture_test_sequence_scenario: copying latest data"
rm -r "${output_pcap_directory_latest}"
cp -r "${output_pcap_directory_current}" "${output_pcap_directory_latest}"

echo "replay_capture_test_sequence_scenario: date: $(date)"

echo "replay_capture_test_sequence_scenario: end"


