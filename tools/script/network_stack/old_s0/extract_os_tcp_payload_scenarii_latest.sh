#!/bin/bash

# set -xve

PYTHON_SCRIPT_PATH=$1
OS_DIRECTORY=$2

export OS_DIRECTORY
export PYTHON_SCRIPT_PATH

echo "extract_icmp_payload: OS_DIRECTORY: "$OS_DIRECTORY
echo "extract_icmp_payload: PYTHON_SCRIPT_PATH: "$PYTHON_SCRIPT_PATH

printf -v date '%(%Y%m%d_%H%M%S)T' -1

function extract_payload() {
    echo ""
    echo ""
    echo ""
    echo "extract_payload: start"
    scenario_name=$1
    nb_character_to_remove=$2

    echo "extract_payload: scenario_name: "$scenario_name

    json_path=$OS_DIRECTORY"/tcp_"$scenario_name"_payload.json"

    latest_pcap_path=$OS_DIRECTORY"/output_tcp_${scenario_name}_latest"
    pcap_path_wo_ext="${directory_path%.*}"

    log=$OS_DIRECTORY"/log_tcp_payload_extraction_"$scenario_name".log"


    python3 $PYTHON_SCRIPT_PATH \
    -p $latest_pcap_path \
    -j $json_path &> $log
    
    echo "extract_payload: end"
}
export -f extract_payload

extract_payload pep1

extract_payload pep2

extract_payload peos










