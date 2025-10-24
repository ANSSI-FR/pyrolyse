#!/bin/bash

set -eu
set -o pipefail

echo "plot_scenarii_run: start"

test_case_directory=$1
output_directory=$2
#protocol=$3
with_payload_option=$3
nb_processes=$4

echo "plot_scenarii_run: test_case_directory: ${test_case_directory}"
echo "plot_scenarii_run: output_directory: ${output_directory}"
echo "plot_scenarii_run: with_payload_option: ${with_payload_option}"
#echo "plot_scenarii_run: protocol: ${protocol}"
echo "plot_scenarii_run: nb_processes: ${nb_processes}"

export test_case_directory
export output_directory
#export protocol
export with_payload_option

function plot_test_cases_for_single_scenario() {
  echo ""
  echo ""
  echo ""
  echo "plot_test_cases_for_single_scenario: start"
  test_case_directory_scenario=$1
  echo "plot_test_cases_for_single_scenario: test_case_directory_scenario: ${test_case_directory_scenario}"

  python3 "${PYROLYSE_PATH}/tools/script/network_stack/plot_ip_scenario_from_pcap.py" \
  -i "$test_case_directory_scenario" \
  -o "$output_directory" \
  -p "$with_payload_option"
  #-p "${protocol}"
  
  echo "plot_test_cases_for_single_scenario: end"
} 
export -f plot_test_cases_for_single_scenario


find "${test_case_directory}/" -type d -name "tc_*"  | parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "plot_test_cases_for_single_scenario {};"
#find "${test_case_directory}/" -type d -name "tc_*_peoef"  | parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "plot_test_cases_for_single_scenario {};"

echo "replay_capture_test_sequence_scenario: date: $(date)"


echo "plot_scenarii_run: end"
