#!/bin/bash

set -eu
set -o pipefail

echo "plot_scenarii_run: start"

pcap_directory=$1
plot_directory=$2
#protocol=$3
with_payload_option=$3
nb_processes=$4

echo "plot_scenarii_run: pcap_directory: ${pcap_directory}"
echo "plot_scenarii_run: plot_directory: ${plot_directory}"
echo "plot_scenarii_run: with_payload_option: ${with_payload_option}"
#echo "plot_scenarii_run: protocol: ${protocol}"
echo "plot_scenarii_run: nb_processes: ${nb_processes}"

export pcap_directory
export plot_directory
#export protocol
export with_payload_option

function plot_test_cases_for_single_scenario() {
  echo ""
  echo ""
  echo ""
  echo "plot_test_cases_for_single_scenario: start"
  pcap_directory_scenario=$1
  echo "plot_test_cases_for_single_scenario: pcap_directory_scenario: ${pcap_directory_scenario}"

  python3 "${PYROLYSE_PATH}/tools/script/network_stack/plot_tcp_scenario_from_pcap.py" \
  -i "$pcap_directory_scenario" \
  -o "$plot_directory" \
  -p "$with_payload_option"
  
  echo "plot_test_cases_for_single_scenario: end"
} 
export -f plot_test_cases_for_single_scenario


find "${pcap_directory}/" -type d -name "tcp_*_latest"  | parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "plot_test_cases_for_single_scenario {};"
#find "${pcap_directory}/" -type d -name "tcp_peoefsf_latest"  | parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "plot_test_cases_for_single_scenario {};"

echo "replay_capture_test_sequence_scenario: date: $(date)"


echo "plot_scenarii_run: end"
