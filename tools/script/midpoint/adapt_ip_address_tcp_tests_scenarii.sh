#!/bin/bash

set -eu
set -o pipefail

midpoint_directory_path=$1
target_directory_path=$2
new_ip_src=$3
new_ip_dst=$4

export midpoint_directory_path
export target_directory_path
export new_ip_dst
export new_ip_src

echo "adapt_tcp_pcap_scenarii: midpoint_directory_path: ${midpoint_directory_path}"
echo "adapt_tcp_pcap_scenarii: target_directory_path: ${target_directory_path}"
echo "adapt_tcp_pcap_scenarii: new_ip_dst: ${new_ip_dst}"
echo "adapt_tcp_pcap_scenarii: new_ip_src: ${new_ip_src}"

function adapt_ip_address() {
    echo ""
    echo ""
    echo ""
    echo "adapt_ip_address: start"
    scenario_name=$1
    echo "adapt_ip_address: target_directory_path: ${target_directory_path}"

    output_directory_path="${midpoint_directory_path}/tc_tcp_${scenario_name}"
    #output_directory_path="${midpoint_directory_path}/tcp_${scenario_name}_latest"
    target_scenario_directory_path="${target_directory_path}/tcp_${scenario_name}_latest"
    mkdir -p "${output_directory_path}"
    cp "${target_scenario_directory_path}/"* "${output_directory_path}"
    
    log="${midpoint_directory_path}/log_adapt_ip_address_${scenario_name}.log"
    echo "adapt_ip_address: log: ${log}"
    
    python3 "${PYROLYSE_PATH}/tools/script/midpoint/adapt_ip_address_from_os_to_midpoint_tcp_tests.py" \
    -i "${output_directory_path}" \
    -s "${new_ip_src}"  \
    -d "${new_ip_dst}" &> "${log}"

    echo "adapt_ip_address: end"
}
export -f adapt_ip_address

adapt_ip_address "pep-ap"
adapt_ip_address "pep-ao"
adapt_ip_address "peosp-ap"
adapt_ip_address "peosp-ao"
adapt_ip_address "peoef-ap"
adapt_ip_address "peoef-ao"
adapt_ip_address "peoep-ap"
adapt_ip_address "peoep-ao"
adapt_ip_address "peospef-ap"
adapt_ip_address "peospef-ao"
adapt_ip_address "peospep-ap"
adapt_ip_address "peospep-ao"
adapt_ip_address "peoepsp-ap"
adapt_ip_address "peoepsp-ao"

adapt_ip_address "peosf"
adapt_ip_address "peosfef"
adapt_ip_address "peoefsf"
adapt_ip_address "peoepsf"
