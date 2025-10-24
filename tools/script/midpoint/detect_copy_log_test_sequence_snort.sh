#!/bin/bash

set -x
set -e

target_directory_path=$1
input_directory=$2
midpoint_install_directory_path=$3
midpoint_configuration_directory=$4
reassembly_options=$5
payload_mode=$6
nb_processes=$7

echo "detect_copy_log_test_sequence_snort: target_directory_path: ${target_directory_path}"
echo "detect_copy_log_test_sequence_snort: input_directory: ${input_directory}"
echo "detect_copy_log_test_sequence_snort: midpoint_install_directory_path: ${midpoint_install_directory_path}"
echo "detect_copy_log_test_sequence_snort: midpoint_configuration_directory: ${midpoint_configuration_directory}"
echo "detect_copy_log_test_sequence_snort: reassembly_options: ${reassembly_options}"
echo "detect_copy_log_test_sequence_snort: payload_mode: ${payload_mode}"
echo "detect_copy_log_test_sequence_snort: nb_processes: ${nb_processes}"

printf -v date '%(%Y%m%d_%H%M%S)T' -1

test_case_path="${target_directory_path}/${input_directory}"
echo "detect_copy_log_test_sequence_snort: test_case_path: '${test_case_path}'"

protocol_s=$(echo "${input_directory}" | cut -d'_' -f2)
scenario_s=$(echo "${input_directory}" | cut -d'_' -f3)

truncated_protocol_s=$(echo "${protocol_s}" | cut -d'v' -f1)

output_log_directory_latest="${target_directory_path}/output/${reassembly_options}/${protocol_s}_${scenario_s}_latest"
output_log_directory_current="${target_directory_path}/output/${reassembly_options}/${protocol_s}_${scenario_s}_${date}"
mkdir -p "${output_log_directory_current}"
mkdir -p "${output_log_directory_latest}"
output_log_directory_current_alert_fast="${output_log_directory_current}/alert_fast"
output_log_directory_current_alert_json="${output_log_directory_current}/alert_json"
mkdir -p "${output_log_directory_current_alert_fast}"
mkdir -p "${output_log_directory_current_alert_json}"

snort_log_tmp_directory_path="${target_directory_path}/output/${reassembly_options}/snort_tmp_${date}"
mkdir -p "${snort_log_tmp_directory_path}"

export target_directory_path
export input_directory
export protocol_s
export truncated_protocol_s
export scenario_s
export output_log_directory_current
export output_log_directory_current_alert_fast
export output_log_directory_current_alert_json
export snort_log_tmp_directory_path
export midpoint_install_directory_path
export midpoint_configuration_directory
export reassembly_options
export payload_mode

function detect_copy_log_test_sequence_snort() {
    echo ""
    echo ""
    echo ""
    echo "detect_copy_log_test_sequence_snort: start"
    pcap_name=$1
    pcap_path="${target_directory_path}/${input_directory}/$pcap_name"
    pcap_path_wo_ext="${pcap_path%.*}"
    echo "detect_copy_log_test_sequence_snort: pcap_path: ${pcap_path}"
    echo "detect_copy_log_test_sequence_snort: pcap_path_wo_ext: ${pcap_path_wo_ext}"
    
    test_index=$(basename "${pcap_path_wo_ext}" | cut -d'_' -f2)
    echo "detect_copy_log_test_sequence_snort: test_index: ${test_index}"
    log_fast_path="${output_log_directory_current_alert_fast}/base_sequence_${test_index}.log"
    echo "detect_copy_log_test_sequence_snort: log_fast_path: ${log_fast_path}"
    log_json_path="${output_log_directory_current_alert_json}/base_sequence_${test_index}.log"
    echo "detect_copy_log_test_sequence_snort: log_json_path: ${log_json_path}"
    snort_log_tmp_directory_path_current="${snort_log_tmp_directory_path}_${test_index}"
    echo "detect_copy_log_test_sequence_suricata: suricata_log_tmp_directory_path_current: ${snort_log_tmp_directory_path_current}"
    mkdir -p "${snort_log_tmp_directory_path_current}"

    #rule_file="${midpoint_configuration_directory}/snort_${truncated_protocol_s}.rules"
    #echo "detect_copy_log_test_sequence_snort: rule_file: "$rule_file
    rule_file="${midpoint_configuration_directory}/snort_${protocol_s}_${payload_mode}_${reassembly_options}.rules"
    echo "detect_copy_log_test_sequence_snort: rule_file: ${rule_file}"

    
    LD_LIBRARY_PATH="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/" \
    "${midpoint_install_directory_path}/bin/snort" \
    --daq-dir "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/daq/" \
    -r "${pcap_path}" \
    -c "${midpoint_configuration_directory}/snort.lua" \
    -R "${rule_file}" \
    -l "${snort_log_tmp_directory_path_current}" \
    -d

    echo "detect_copy_log_test_sequence_snort: start"
    mv "${snort_log_tmp_directory_path_current}/alert_fast.txt" "${log_fast_path}"
    mv "${snort_log_tmp_directory_path_current}/alert_json.txt" "${log_json_path}"
    rm -r "${snort_log_tmp_directory_path_current}"
    
    echo "detect_copy_log_test_sequence_snort: end"
}
export -f detect_copy_log_test_sequence_snort

    
find "${test_case_path}" -name "*.pcap" | \
xargs -I {} bash -c "basename {}" | sort -t_ -k2 -n \
| parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "detect_copy_log_test_sequence_snort {};" 

# Copying as the latest
rm -r "${output_log_directory_latest}"
cp -r "${output_log_directory_current}" "${output_log_directory_latest}"

echo "Finished!!!!"
date



