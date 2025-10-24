#!/bin/bash

set -eu
set -o pipefail

echo "scrap_tcp_triplet_reassembly_algorithm_consistency_scenarii: start"

target_directory_path=$1

echo "scrap_tcp_triplet_reassembly_algorithm_consistency_scenarii: target_directory_path: ${target_directory_path}"

target_protocol_directory_path="${target_directory_path}/tcp/output"
echo "scrap_tcp_triplet_reassembly_algorithm_consistency_scenarii: target_protocol_directory_path: ${target_protocol_directory_path}"


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

    complicated_file="${target_protocol_directory_path}/tcp_${scenario_name}_tcp_policy_complicated.json"
    echo "scrap: complicated_file: ${complicated_file}"

    output_file_path="${target_protocol_directory_path}/tcp_${scenario_name}_triplet_individual_inconsistency_with_isolated_pairs.txt"
    echo "scrap: output_file_path: ${output_file_path}"

    echo $(cat "${complicated_file}" | grep -o '"triplet_individual_consistency_with_isolated_pairs": "NotConsistent"' | wc -l) > "$output_file_path"

    echo "scrap: end"
}
export -f scrap

scrap pep-ap
scrap pep-ao
scrap peosp-ap
scrap peosp-ao
scrap peoef-ap
scrap peoef-ao
scrap peoep-ap
scrap peoep-ao
scrap peospef-ap
scrap peospef-ao
scrap peospep-ap
scrap peospep-ao
scrap peoepsp-ap
scrap peoepsp-ao
scrap peosf
scrap peosfef
scrap peoefsf
scrap peoepsf


echo "replay_capture_test_sequence_scenario: end"


