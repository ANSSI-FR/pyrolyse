#!/bin/bash

set -eu

python_script_path=$1
os_directory_path=$2
ip_version=$3


"${PYROLYSE_PATH}/script/network_stack/extract_os_icmp_payload_scenario.sh" \
"${python_script_path}" \
"${os_directory_path}" \
pep_ \
0 \
"${ip_version}" \

"${PYROLYSE_PATH}/script/network_stack/extract_os_icmp_payload_scenario.sh" \
"${python_script_path}" \
"${os_directory_path}" \
peos_ \
0 \
"${ip_version}" \

"${PYROLYSE_PATH}/script/network_stack/extract_os_icmp_payload_scenario.sh" \
"${python_script_path}" \
"${os_directory_path}" \
peoe_ \
8 \
"${ip_version}" \

"${PYROLYSE_PATH}/script/network_stack/extract_os_icmp_payload_scenario.sh" \
"${python_script_path}" \
"${os_directory_path}" \
peose_ \
8 \
"${ip_version}" \

"${PYROLYSE_PATH}/script/network_stack/extract_os_icmp_payload_scenario.sh" \
"${python_script_path}" \
"${os_directory_path}" \
peoes_ \
8 \
"${ip_version}" \










