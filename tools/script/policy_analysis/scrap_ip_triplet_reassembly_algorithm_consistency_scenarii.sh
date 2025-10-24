#!/bin/bash

set -eu
set -o pipefail

echo "scrap_ip_triplet_reassembly_algorithm_consistency_scenarii: start"

target_directory_path=$1
protocol=$2

echo "scrap_ip_triplet_reassembly_algorithm_consistency_scenarii: target_directory_path: ${target_directory_path}"
echo "scrap_ip_triplet_reassembly_algorithm_consistency_scenarii: protocol: ${protocol}"

target_protocol_directory_path="${target_directory_path}/${protocol}/output_ids_json"
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

    complicated_file="${target_protocol_directory_path}/${protocol}_${scenario_name}_policy_full.json"
    echo "scrap: complicated_file: ${complicated_file}"

    output_file_path="${target_protocol_directory_path}/${protocol}_${scenario_name}_triplet_reassembly_algorithm_inconsistency_BIS.txt"
    echo "scrap: output_file_path: ${output_file_path}"

    echo "qoaimnipa: "$(cat "${complicated_file}" | grep -o '"qoaimnipa": "NotConsistent"' | wc -l) > "$output_file_path"
    echo "qoaimnipn: "$(cat "${complicated_file}" | grep -o '"qoaimnipn": "NotConsistent"' | wc -l) >> "$output_file_path"
    echo "qoaimnita: "$(cat "${complicated_file}" | grep -o '"qoaimnita": "NotConsistent"' | wc -l) >> "$output_file_path"
    echo "qoadmnita: "$(cat "${complicated_file}" | grep -o '"qoadmnita": "NotConsistent"' | wc -l) >> "$output_file_path"
    echo "qoaimmipa: "$(cat "${complicated_file}" | grep -o '"qoaimmipa": "NotConsistent"' | wc -l) >> "$output_file_path"
    echo "qoaimmipn: "$(cat "${complicated_file}" | grep -o '"qoaimmipn": "NotConsistent"' | wc -l) >> "$output_file_path"
    echo "qoaimmita: "$(cat "${complicated_file}" | grep -o '"qoaimmita": "NotConsistent"' | wc -l) >> "$output_file_path"
    echo "qoadmmita: "$(cat "${complicated_file}" | grep -o '"qoadmmita": "NotConsistent"' | wc -l) >> "$output_file_path"
    echo "qoaimaipa: "$(cat "${complicated_file}" | grep -o '"qoaimaipa": "NotConsistent"' | wc -l) >> "$output_file_path"
    echo "qoaimaipn: "$(cat "${complicated_file}" | grep -o '"qoaimaipn": "NotConsistent"' | wc -l) >> "$output_file_path"
    echo "qoaimaita: "$(cat "${complicated_file}" | grep -o '"qoaimaita": "NotConsistent"' | wc -l) >> "$output_file_path"
    echo "qoadmaita: "$(cat "${complicated_file}" | grep -o '"qoadmaita": "NotConsistent"' | wc -l) >> "$output_file_path"

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

#scrap pep-of
#scrap pep-nf
#scrap pep-os
#scrap pep-ns
#
#scrap peosp-af
#scrap peosp-of
#scrap peosp-nf
#scrap peosp-as
#scrap peosp-os
#scrap peosp-ns
#
#scrap peosf-af
#scrap peosf-of
#scrap peosf-nf
#scrap peosf-as
#scrap peosf-os
#scrap peosf-ns

echo "replay_capture_test_sequence_scenario: end"


