#!/bin/bash

set -eu

python_script_path=$1
os_directory_path=$2
pattern=$3
nb_character_to_remove=$4
ip_version=$5

export python_script_path
export os_directory_path
export pattern
export nb_character_to_remove
export ip_version

echo "extract_os_icmp_payload_scenario: python_script_path: "$python_script_path
echo "extract_os_icmp_payload_scenario: os_directory_path: "$os_directory_path
echo "extract_os_icmp_payload_scenario: pattern: "$pattern
echo "extract_os_icmp_payload_scenario: nb_character_to_remove: "$nb_character_to_remove
echo "extract_os_icmp_payload_scenario: ip_version: "$ip_version

printf -v date '%(%Y%m%d_%H%M%S)T' -1

function extract_payload() {
    echo ""
    echo ""
    echo ""
    echo "extract_os_icmp_payload_scenario: extract_payload: start"
    directory_path=$1
    echo "extract_os_icmp_payload_scenario: extract_payload: directory_path: ${directory_path}"
    
    pcap_path_wo_ext="${directory_path%.*}"
    
    directory_name=$(basename $directory_path)
    echo "extract_os_icmp_payload_scenario: directory_name: ${directory_name}"

    scenario=`echo $directory_name | rev | cut -d_ -f3 | rev`
    date_s=`echo $directory_name | rev | cut -d_ -f2 | rev`
    time_s=`echo $directory_name | rev | cut -d_ -f1 | rev`
    echo "extract_os_icmp_payload_scenario: extract_payload: scenario: "$scenario
    echo "extract_os_icmp_payload_scenario: extract_payload: date_s: "$date_s
    echo "extract_os_icmp_payload_scenario: extract_payload: time_s: "$time_s
    
    json_path="${os_directory_path}/ipv${ip_version}_${scenario}_payload_${date_s}_${time_s}.json"
    echo "extract_os_icmp_payload_scenario: extract_payload: json_path: $json_path"
    
    log_path="${os_directory_path}/log_ipv${ip_version}_${scenario}_payload_extraction_${date_s}_${time_s}.log"
    
    set -x
    python3 "${python_script_path}" \
    -p "${directory_path}" \
    -v "${ip_version}" \
    -j "${json_path}" \
    -r "${nb_character_to_remove}" &> "${log_path}"
    set +x
    
    echo "extract_os_icmp_payload_scenario: extract_payload: end"
}
export -f extract_payload

find "${os_directory_path}" -maxdepth 1 -name "ipv${ip_version}_*" | grep "${pattern}" | grep -v latest | sort | xargs -I {} bash -c "extract_payload {}"











