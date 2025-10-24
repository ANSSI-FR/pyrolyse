#!/bin/bash

set -eu
set -o pipefail

echo "acquire_tcp_traffic_scenarii_w_target_shutdown: start"

target_directory_path=$1
byte_time_sequence_path=$2
ip_src=$3
ip_dst=$4
mac_src=$5
mac_dst=$6
payload_mode=$7
nb_processes=$8

echo "acquire_tcp_traffic_scenarii_w_target_shutdown: target_directory_path: ${target_directory_path}"
echo "acquire_tcp_traffic_scenarii_w_target_shutdown: byte_time_sequence_path: ${byte_time_sequence_path}"
echo "acquire_tcp_traffic_scenarii_w_target_shutdown: ip_src: ${ip_src}"
echo "acquire_tcp_traffic_scenarii_w_target_shutdown: ip_dst: ${ip_dst}"
echo "acquire_tcp_traffic_scenarii_w_target_shutdown: mac_src: ${mac_src}"
echo "acquire_tcp_traffic_scenarii_w_target_shutdown: mac_dst: ${mac_dst}"
echo "acquire_tcp_traffic_scenarii_w_target_shutdown: payload_mode: ${payload_mode}"
echo "acquire_tcp_traffic_scenarii_w_target_shutdown: nb_processes: ${nb_processes}"

export target_directory_path
export byte_time_sequence_path
export ip_src
export ip_dst
export mac_src
export mac_dst
export payload_mode
export nb_processes

# Execution time measurement
# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
SECONDS=0

cd "${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}"
vagrant up base

function acquire_tcp_data() {
    scenario=$1

    cd "${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}"
    vagrant up target 

    #script_arguments='"${target_directory_path}" \
    #"${byte_time_sequence_path}" \
    #"${scenario}" \
    #"${ip_src}" \
    #"${ip_dst}" \
    #"${mac_src}" \
    #"${mac_dst}" \
    #"${payload_mode}" \
    #"${nb_processes}"'
#
    #script_argument_expanded=$(eval echo "${script_arguments}")
    #echo "acquire_tcp_traffic_scenarii_w_target_shutdown: script_argument_expanded: ${script_argument_expanded}"
#
#
    set -x
    vagrant ssh base -c "source /home/vagrant/.profile; \
    /home/vagrant/pyrolyse/script/custom_stack/acquire_tcp_traffic_scenario.sh \
    ${target_directory_path} \
    ${byte_time_sequence_path} \
    ${scenario} \
    ${ip_src} \
    ${ip_dst} \
    ${mac_src} \
    ${mac_dst} \
    ${payload_mode} \
    ${nb_processes}"
    set +x

    vagrant halt target
}
export -f acquire_tcp_data

#acquire_tcp_data "pep-ap"
#acquire_tcp_data "pep-ao"
#acquire_tcp_data "peosp-ap"
#acquire_tcp_data "peosp-ao"
#acquire_tcp_data "peoef-ap"
#acquire_tcp_data "peoef-ao"
#acquire_tcp_data "peoep-ap"
#acquire_tcp_data "peoep-ao"
#acquire_tcp_data "peospef-ap"
#acquire_tcp_data "peospef-ao"
#acquire_tcp_data "peospep-ap"
#acquire_tcp_data "peospep-ao"
#acquire_tcp_data "peoepsp-ap"
#acquire_tcp_data "peoepsp-ao"
#
acquire_tcp_data "peosf"
#acquire_tcp_data "peosfef"
acquire_tcp_data "peoefsf"
acquire_tcp_data "peoepsf"


# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
elapsed_seconds=$SECONDS
echo "acquire_tcp_traffic_scenarii_w_target_shutdown: elapsed: $((elapsed_seconds / 3600))hrs $(((elapsed_seconds / 60) % 60))min $((elapsed_seconds % 60))sec"

echo "acquire_tcp_traffic_scenarii_w_target_shutdown: end"


