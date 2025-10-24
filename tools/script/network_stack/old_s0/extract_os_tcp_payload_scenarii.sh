#!/bin/bash

set -xveu

python_script_path=$1
os_directory_path=$2

"${PYROLYSE_PATH}/script/network_stack/extract_os_tcp_payload_scenario.sh" \
"${python_script_path}" \
"${os_directory_path}" \
pep1 

"${PYROLYSE_PATH}/script/network_stack/extract_os_tcp_payload_scenario.sh" \
"${python_script_path}" \
"${os_directory_path}" \
pep2

"${PYROLYSE_PATH}/script/network_stack/extract_os_tcp_payload_scenario.sh" \
"${python_script_path}" \
"${os_directory_path}" \
peos 
