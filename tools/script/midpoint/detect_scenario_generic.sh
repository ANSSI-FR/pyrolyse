#!/bin/bash

set -x
set -e

target_directory_path=$1
input_directory=$2
ids_tool_command=$3
ids_argument=$4
ids_log_name=$5
reassembly_options=$6
payload_mode=$7
protocol_s=$8
nb_processes=$9


echo "detect_scenario: target_directory_path: ${target_directory_path}"
echo "detect_scenario: input_directory: ${input_directory}"
echo "detect_scenario: ids_tool_command: ${ids_tool_command}"
echo "detect_scenario: ids_argument: ${ids_argument}"
echo "detect_scenario: ids_log_name: ${ids_log_name}"
echo "detect_scenario: reassembly_options: ${reassembly_options}"
echo "detect_scenario: payload_mode: ${payload_mode}"
echo "detect_scenario: protocol_s: ${protocol_s}"
echo "detect_scenario: nb_processes: ${nb_processes}"

test_case_path="${target_directory_path}/${input_directory}"
echo "detect_scenario: test_case_path: '${test_case_path}'"

#protocol_s=$(echo "${input_directory}" | cut -d'_' -f2)
scenario_s=$(echo "${input_directory}" | cut -d'_' -f3)

truncated_protocol_s=$(echo "${protocol_s}" | cut -d'v' -f1)

output_log_directory_latest="${target_directory_path}/output/${reassembly_options}/${protocol_s}_${scenario_s}_latest"
output_log_directory_current="${target_directory_path}/output/${reassembly_options}/${protocol_s}_${scenario_s}_${date}"
mkdir -p "${output_log_directory_current}"
mkdir -p "${output_log_directory_latest}"

ids_log_tmp_directory_path="${target_directory_path}/output/${reassembly_options}/ids_tmp_${date}"
mkdir -p "${ids_log_tmp_directory_path}"

printf -v date '%(%Y%m%d_%H%M%S)T' -1

export target_directory_path
export input_directory
export protocol_s
export truncated_protocol_s
export scenario_s
export output_log_directory_current
export ids_log_tmp_directory_path
export ids_tool_command
export ids_argument
export ids_log_name
export reassembly_options
export payload_mode

function detect_scenario_signatures() {
    echo ""
    echo ""
    echo ""
    echo "detect: start"
    pcap_name=$1
    pcap_path="${target_directory_path}/${input_directory}/$pcap_name"
    pcap_path_wo_ext="${pcap_path%.*}"
    echo "detect: pcap_path: ${pcap_path}"
    echo "detect: pcap_path_wo_ext: ${pcap_path_wo_ext}"

    pcap_relative_path=$(realpath --relative-to="${target_directory_path}" "${pcap_path}")
    echo "detect: pcap_relative_path: ${pcap_relative_path}"
    
    test_index=$(basename "${pcap_path_wo_ext}" | cut -d'_' -f2)
    echo "detect: test_index: ${test_index}"
    ids_log_tc_output_path="${output_log_directory_current}/base_sequence_${test_index}.log"
    echo "detect: ids_log_tc_output_path: ${ids_log_tc_output_path}"
    
    ids_log_tmp_directory_path_current="${ids_log_tmp_directory_path}_${test_index}"
    echo "detect: ids_log_tmp_directory_path_current: ${ids_log_tmp_directory_path_current}"
    mkdir -p "${ids_log_tmp_directory_path_current}"

    echo "detect: ids_tool_command: ${ids_tool_command}"
    ids_tool_command_expanded=$(eval echo "${ids_tool_command}")
    echo "detect: ids_tool_command_expanded: ${ids_tool_command_expanded}"

    echo "detect: ids_argument: ${ids_argument}"
    ids_argument_expanded=$(eval echo "${ids_argument}")
    echo "detect: ids_argument_expanded: ${ids_argument_expanded}"

    set -x
    env --chdir="${ids_log_tmp_directory_path_current}" -S ${ids_tool_command_expanded} ${ids_argument_expanded}
    local status=$?
    set +x
    echo "detect: status: ${status}"

    echo "detect: will copy/create ${ids_log_name}"
    ids_log_output_path="${ids_log_tmp_directory_path_current}/${ids_log_name}"
    echo "detect: ids_log_output_path: ${ids_log_output_path}"
    echo "detect: ids_log_tc_output_path: ${ids_log_tc_output_path}"
    if [ -f "$ids_log_output_path" ]; then
        mv "$ids_log_output_path" "$ids_log_tc_output_path"
    else
        # If notice.log does not exist, we create an empty file.
        touch "$ids_log_tc_output_path"
    fi

    rm -r "$ids_log_tmp_directory_path_current"
    
    echo "detect: end"
}
export -f detect_scenario_signatures

    
find "${test_case_path}" -name "*.pcap" > pcap_files_log
find "${test_case_path}" -name "*.pcap" | \
xargs -I {} bash -c "basename {}" | \
sort -t_ -k2 -n | \
parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "detect_scenario_signatures {};" 

#xargs -I {} bash -c "detect_scenario_signatures {}"

# Copying as the latest
rm -r "${output_log_directory_latest}"
cp -r "${output_log_directory_current}" "${output_log_directory_latest}"

echo "Finished!!!!"
date


