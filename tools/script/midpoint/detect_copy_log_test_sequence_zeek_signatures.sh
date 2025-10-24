#!/bin/bash

set -x
set -e
set -o pipefail

target_directory_path=$1
input_directory=$2
midpoint_install_directory_path=$3
midpoint_configuration_directory=$4
reassembly_options=$5
payload_mode=$6
nb_processes=$7


echo "detect_copy_log_test_sequence_zeek: target_directory_path: ${target_directory_path}"
echo "detect_copy_log_test_sequence_zeek: input_directory: ${input_directory}"
echo "detect_copy_log_test_sequence_zeek: midpoint_install_directory_path: ${midpoint_install_directory_path}"
echo "detect_copy_log_test_sequence_zeek: midpoint_configuration_directory: ${midpoint_configuration_directory}"
echo "detect_copy_log_test_sequence_zeek: reassembly_options: ${reassembly_options}"
echo "detect_copy_log_test_sequence_zeek: payload_mode: ${payload_mode}"
echo "detect_copy_log_test_sequence_zeek: nb_processes: ${nb_processes}"

test_case_path="${target_directory_path}/${input_directory}"
echo "detect_copy_log_test_sequence_zeek: test_case_path: '${test_case_path}'"

protocol_s=$(echo "${input_directory}" | cut -d'_' -f2)
scenario_s=$(echo "${input_directory}" | cut -d'_' -f3)

truncated_protocol_s=$(echo "${protocol_s}" | cut -d'v' -f1)

output_log_directory_latest="${target_directory_path}/output/${reassembly_options}/${protocol_s}_${scenario_s}_latest"
output_log_directory_current="${target_directory_path}/output/${reassembly_options}/${protocol_s}_${scenario_s}_${date}"
mkdir -p "${output_log_directory_current}"
mkdir -p "${output_log_directory_latest}"

zeek_log_tmp_directory_path="${target_directory_path}/output/${reassembly_options}/zeek_tmp_${date}"
mkdir -p "${zeek_log_tmp_directory_path}"

export target_directory_path
export input_directory
export protocol_s
export truncated_protocol_s
export scenario_s
export output_log_directory_current
export zeek_log_tmp_directory_path
export midpoint_install_directory_path
export midpoint_configuration_directory
export reassembly_options
export payload_mode

function detect_copy_log_test_sequence_zeek_signatures() {
    echo ""
    echo ""
    echo ""
    echo "detect_copy_log_test_sequence_zeek_signatures: start"
    pcap_name=$1
    pcap_path="${target_directory_path}/${input_directory}/${pcap_name}"
    pcap_path_wo_ext="${pcap_path%.*}"
    echo "detect_copy_log_test_sequence_zeek_signatures: pcap_path: ${pcap_path}"
    echo "detect_copy_log_test_sequence_zeek_signatures: pcap_path_wo_ext: ${pcap_path_wo_ext}"
    
    test_index=$(basename "${pcap_path_wo_ext}" | cut -d'_' -f2)
    echo "detect_copy_log_test_sequence_zeek_signatures: test_index: ${test_index}"
    log_path="${output_log_directory_current}/base_sequence_${test_index}.log"
    echo "detect_copy_log_test_sequence_zeek_signatures: log_path: ${log_path}"
    zeek_log_tmp_directory_path_current="${zeek_log_tmp_directory_path}_${test_index}"
    echo "detect_copy_log_test_sequence_suricata: zeek_log_tmp_directory_path_current: ${zeek_log_tmp_directory_path_current}"
    mkdir "${zeek_log_tmp_directory_path_current}"


    signature_file="${midpoint_configuration_directory}/zeek_${protocol_s}_${payload_mode}_${reassembly_options}.sig"
    #signature_file="${midpoint_configuration_directory}/zeek_tcp_extended.sig"
    echo "detect_copy_log_test_sequence_zeek_signatures: signature_file: ${signature_file}"
    
    "${midpoint_install_directory_path}/bin/zeek" \
    -r "${pcap_path}" \
    -s "${signature_file}" \
    "Log::default_logdir=${zeek_log_tmp_directory_path_current}"
    
    echo "detect_copy_log_test_sequence_zeek_signatures: handling signatures.log"
    if [ -f "${zeek_log_tmp_directory_path_current}/signatures.log" ]; then
        mv "${zeek_log_tmp_directory_path_current}/signatures.log" "${log_path}"
    else
        # If notice.log does not exist, we create an empty file.
        touch "${log_path}"
    fi

    rm -r "${zeek_log_tmp_directory_path_current}"
    
    echo "detect_copy_log_test_sequence_zeek_signatures: end"
}
export -f detect_copy_log_test_sequence_zeek_signatures

    
find "${test_case_path}" -name "*.pcap" > pcap_files_log
find "${test_case_path}" -name "*.pcap" | \
xargs -I {} bash -c "basename {}" | \
sort -t_ -k2 -n | \
parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "detect_copy_log_test_sequence_zeek_signatures {};" 

#xargs -I {} bash -c "detect_copy_log_test_sequence_zeek_signatures {}"

# Copying as the latest
rm -r "${output_log_directory_latest}"
cp -r "${output_log_directory_current}" "${output_log_directory_latest}"

echo "Finished!!!!"
date



