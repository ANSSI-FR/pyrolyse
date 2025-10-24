#!/bin/bash

set -eu
set -o pipefail

python_script_path=$1
os_directory_path=$2
ip_version=$3

export os_directory_path
export python_script_path
export ip_version

echo "extract_os_icmp_payload_scenarii_latest: os_directory_path: ${os_directory_path}"
echo "extract_os_icmp_payload_scenarii_latest: python_script_path: ${python_script_path}"
echo "extract_os_icmp_payload_scenarii_latest: ip_version: ${ip_version}"

printf -v date '%(%Y%m%d_%H%M%S)T' -1

function extract_payload() {
    echo ""
    echo ""
    echo ""
    echo "extract_os_icmp_payload_scenarii_latest: extract_payload: start"
    scenario_name=$1
    nb_character_to_remove=$2

    echo "extract_os_icmp_payload_scenarii_latest: extract_payload: scenario_name: $scenario_name"

    latest_pcap_path="$os_directory_path/ipv${ip_version}_${scenario_name}_latest"
    json_path="$os_directory_path/ipv${ip_version}_${scenario_name}_payload.json"

    set -x
    python3 "${python_script_path}" \
    -p "${latest_pcap_path}" \
    -v "${ip_version}" \
    -j "${json_path}" \
    -r "${nb_character_to_remove}"
    set +x
    
    echo "extract_os_icmp_payload_scenarii_latest: extract_payload: end"
}
export -f extract_payload

extract_payload pep 0

extract_payload peos 0

extract_payload peoe 8

extract_payload peose 8

extract_payload peoes 8










