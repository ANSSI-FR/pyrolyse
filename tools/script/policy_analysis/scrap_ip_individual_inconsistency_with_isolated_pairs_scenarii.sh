#!/bin/bash

set -eu
set -o pipefail

echo "scrap_ip_triplet_reassembly_algorithm_consistency_scenarii: start"

target_directory_path=$1
protocol=$2

echo "scrap_ip_triplet_reassembly_algorithm_consistency_scenarii: target_directory_path: ${target_directory_path}"
echo "scrap_ip_triplet_reassembly_algorithm_consistency_scenarii: protocol: ${protocol}"

target_protocol_directory_path="${target_directory_path}/${protocol}/output"
echo "scrap_ip_triplet_reassembly_algorithm_consistency_scenarii: target_protocol_directory_path: ${target_protocol_directory_path}"



export target_protocol_directory_path
export output_file_path
export scenario_name


function scrap() {
    set -e

    echo ""
    echo ""
    echo ""
    echo "scrap: start"
    scenario_name=$1
    echo "scrap: scenario_name: ${scenario_name}"

    complicated_file="${target_protocol_directory_path}/${protocol}_${scenario_name}_${protocol}_policy_complicated.json"
    echo "scrap: complicated_file: ${complicated_file}"

    output_file_path="${target_protocol_directory_path}/${protocol}_${scenario_name}_triplet_individual_inconsistency_with_isolated_pairs.txt"
    echo "scrap: output_file_path: ${output_file_path}"

    echo $(cat "${complicated_file}" | grep -o '"triplet_individual_consistency_with_isolated_pairs": "NotConsistent"' | wc -l) > "$output_file_path"

    echo "scrap: end"
}
export -f scrap

scrap peoep
scrap peoef
scrap peosfef
scrap peoefsf
scrap peoepsf
scrap peospef
scrap peospep
scrap peoepsp

echo "replay_capture_test_sequence_scenario: end"


