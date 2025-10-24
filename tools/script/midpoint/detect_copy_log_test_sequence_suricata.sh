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


echo "detect_copy_log_test_sequence_suricata: target_directory_path: ${target_directory_path}"
echo "detect_copy_log_test_sequence_suricata: input_directory: ${input_directory}"
echo "detect_copy_log_test_sequence_suricata: midpoint_install_directory_path: ${midpoint_install_directory_path}"
echo "detect_copy_log_test_sequence_suricata: midpoint_configuration_directory: ${midpoint_configuration_directory}"
echo "detect_copy_log_test_sequence_suricata: reassembly_options: ${reassembly_options}"
echo "detect_copy_log_test_sequence_suricata: payload_mode: ${payload_mode}"
echo "detect_copy_log_test_sequence_suricata: nb_processes: ${nb_processes}"

printf -v date '%(%Y%m%d_%H%M%S)T' -1

test_case_path="${target_directory_path}/${input_directory}"
echo "detect_copy_log_test_sequence_suricata: test_case_path: '${test_case_path}'"

protocol_s=$(echo "${input_directory}" | cut -d'_' -f2)
scenario_s=$(echo "${input_directory}" | cut -d'_' -f3)

output_log_directory_latest="${target_directory_path}/output/${reassembly_options}/${protocol_s}_${scenario_s}_latest"
output_log_directory_current="${target_directory_path}/output/${reassembly_options}/${protocol_s}_${scenario_s}_${date}"
mkdir -p "${output_log_directory_current}"
mkdir -p "${output_log_directory_latest}"

suricata_log_tmp_directory_path="${target_directory_path}/output/${reassembly_options}/suricata_tmp_${date}"
mkdir -p "${suricata_log_tmp_directory_path}"

export target_directory_path
export input_directory
export protocol_s
export scenario_s
export output_log_directory_current
export suricata_log_tmp_directory_path
export midpoint_install_directory_path
export midpoint_configuration_directory
export reassembly_options
export payload_mode

echo "replay_capture_test_sequence: suricata.yaml modification"
sed -i -e "s~^classification-file:.*~classification-file: ${midpoint_install_directory_path}/etc/suricata/classification.config~g" "${midpoint_configuration_directory}/suricata.yaml"
sed -i -e "s~^reference-config-file:.*~reference-config-file: ${midpoint_install_directory_path}/etc/suricata/reference.config~g" "${midpoint_configuration_directory}/suricata.yaml"


function detect_copy_log_test_sequence_suricata() {
    echo ""
    echo ""
    echo ""
    echo "detect_copy_log_test_sequence_suricata: start"
    pcap_name=$1
    pcap_path="${target_directory_path}/${input_directory}/$pcap_name"
    pcap_path_wo_ext="${pcap_path%.*}"
    echo "detect_copy_log_test_sequence_suricata: pcap_path: ${pcap_path}"
    echo "detect_copy_log_test_sequence_suricata: pcap_path_wo_ext: ${pcap_path_wo_ext}"
    
    # TODO: replace with "$(basename $pcap_path_wo_ext | cut -d'_' -f2)"
    test_index=$(basename "${pcap_path_wo_ext}" | cut -d'_' -f2)
    echo "detect_copy_log_test_sequence_suricata: test_index: ${test_index}"
    # TODO: rename to JSON because eve.json will copied into this file ?
    log_path="${output_log_directory_current}/base_sequence_${test_index}.log"
    echo "detect_copy_log_test_sequence_suricata: log_path: ${log_path}"
    suricata_log_tmp_directory_path_current="${suricata_log_tmp_directory_path}_${test_index}"
    echo "detect_copy_log_test_sequence_suricata: suricata_log_tmp_directory_path_current: ${suricata_log_tmp_directory_path_current}"
    mkdir "${suricata_log_tmp_directory_path_current}"


    rules_file="${midpoint_configuration_directory}/suricata_${protocol_s}_${payload_mode}_${reassembly_options}.rules"
    echo "detect_copy_log_test_sequence_suricata: rules_file: ${rules_file}"
    
    "${midpoint_install_directory_path}/bin/suricata" \
    -r "${pcap_path}" \
    -c "${midpoint_configuration_directory}/suricata.yaml" \
    -S "${rules_file}" \
    -l "${suricata_log_tmp_directory_path_current}"

    echo "detect_copy_log_test_sequence_suricata: start"

    mv "${suricata_log_tmp_directory_path_current}/eve.json" "${log_path}"
    rm -r "${suricata_log_tmp_directory_path_current}"
    
    echo "detect_copy_log_test_sequence_suricata: end"
}
export -f detect_copy_log_test_sequence_suricata

# find "${test_case_path}" -name "*.pcap" > pcap_files_log
# find "${test_case_path}" -name "*.pcap" | xargs -I {} bash -c "basename {}" | sort -t_ -k2 -n | xargs -I {} bash -c "detect_copy_log_test_sequence_suricata {}"

find "${test_case_path}" -name "*.pcap" | \
xargs -I {} bash -c "basename {}" | sort -t_ -k2 -n \
| parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "detect_copy_log_test_sequence_suricata {};" 

# Copying as the latest
rm -r "${output_log_directory_latest}"
cp -r "${output_log_directory_current}" "${output_log_directory_latest}"

echo "Finished!!!!"
date










