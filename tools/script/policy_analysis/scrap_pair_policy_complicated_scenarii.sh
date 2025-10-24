#!/bin/bash

set -eu
set -o pipefail

echo "scrap_pair_policy_complicated_scenarii: start"

target_directory_path=$1
protocol=$2

echo "scrap_pair_policy_complicated_scenarii: target_directory_path: ${target_directory_path}"
echo "scrap_pair_policy_complicated_scenarii: protocol: ${protocol}"

target_protocol_directory_path="${target_directory_path}/${protocol}"
echo "scrap_pair_policy_complicated_scenarii: target_protocol_directory_path: ${target_protocol_directory_path}"

output_file_path="${target_protocol_directory_path}/output/pair_policy_complicated.txt"
echo "scrap_pair_policy_complicated_scenarii: output_file_path: ${output_file_path}"

export target_protocol_directory_path
export output_file_path

echo "deleting $output_file_path"
rm -f "$output_file_path"

function scrap() {
    set -e

    echo ""
    echo ""
    echo ""
    echo "scrap: start"
    complicated_file=$1
    echo "scrap: complicated_file: ${complicated_file}"

    scenario_name=$(basename -- "$complicated_file" | cut -d'_' -f2)
    echo "scrap: scenario_name: ${scenario_name}"
    
    #jq_rule='.pair_position_policy_data_c.hm[] | .pair_position_policy.relation, .pair_position_policy.pair_time_policy'

    jq_output=$(cat "${complicated_file}" | jq '.pair_position_policy_data_c.hm[] | .pair_position_policy.relation + "-" + .pair_position_policy.pair_time_policy' | tr -d '"' | tr '\n' '&')
    echo "scrap: jq_output: ${jq_output}" 
    echo "${scenario_name}:${jq_output}" >> "$output_file_path"

    echo "scrap: end"
}
export -f scrap


find "${target_protocol_directory_path}" -regextype sed -regex ".*/[a-z0-9_]*policy_complicated\.json" | sort | xargs -I {} bash -c "scrap {}"
find "${target_protocol_directory_path}" -regextype sed -regex ".*/[a-z0-9_]*-[a-z0-9_]*policy_complicated\.json" | sort | xargs -I {} bash -c "scrap {}"


echo "replay_capture_test_sequence_scenario: end"


