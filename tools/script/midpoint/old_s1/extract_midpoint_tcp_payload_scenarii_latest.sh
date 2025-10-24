#!/bin/bash

# set -xve

PYTHON_SCRIPT_PATH=$1
OS_DIRECTORY=$2
BYTE_TIME_SEQUENCE_JSON_PATH=$3

export PYTHON_SCRIPT_PATH
export OS_DIRECTORY
export BYTE_TIME_SEQUENCE_JSON_PATH

echo "extract_icmp_payload: OS_DIRECTORY: "$OS_DIRECTORY
echo "extract_icmp_payload: PYTHON_SCRIPT_PATH: "$PYTHON_SCRIPT_PATH
echo "extract_icmp_payload: BYTE_TIME_SEQUENCE_JSON_PATH: "$BYTE_TIME_SEQUENCE_JSON_PATH

printf -v date '%(%Y%m%d_%H%M%S)T' -1

function extract_payload() {
    echo ""
    echo ""
    echo ""
    echo "extract_payload: start"
    scenario_name=$1

    echo "extract_payload: scenario_name: "$scenario_name

    latest_pcap_path=$OS_DIRECTORY"/output_tcp_"$scenario_name"_pcap_latest"
    json_path=$OS_DIRECTORY"/tcp_"$scenario_name"_payload.json"

    log=$OS_DIRECTORY"/log_payload_extraction_"$scenario_name".log"
    echo "extract_payload: log: "$log

    python3 $PYTHON_SCRIPT_PATH \
    -p $latest_pcap_path \
    -j $json_path \
    -s $BYTE_TIME_SEQUENCE_JSON_PATH &> $log
    
    echo "extract_payload: end"
}
export -f extract_payload

extract_payload pep1

extract_payload pep2

extract_payload peos

