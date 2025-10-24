#!/bin/bash

set -eu
set -o pipefail

echo "extract_midpoint_os_reassembly_consistency_scenarii: start"

target_midpoint_directory_path=$1
target_os_directory_path=$2
merged_byte_time_sequence_json_path=$3
target_os=$4
ending_file_regex=$5
protocol=$6
without_test_case_with_hole=$7

echo "extract_midpoint_os_reassembly_consistency_scenarii: target_midpoint_directory_path: ${target_midpoint_directory_path}"
echo "extract_midpoint_os_reassembly_consistency_scenarii: target_os_directory_path: ${target_os_directory_path}"
echo "extract_midpoint_os_reassembly_consistency_scenarii: merged_byte_time_sequence_json_path: ${merged_byte_time_sequence_json_path}"
echo "extract_midpoint_os_reassembly_consistency_scenarii: ending_file_regex: ${ending_file_regex}"
echo "extract_midpoint_os_reassembly_consistency_scenarii: protocol: ${protocol}"
echo "extract_midpoint_os_reassembly_consistency_scenarii: without_test_case_with_hole: ${without_test_case_with_hole}"
echo "extract_midpoint_os_reassembly_consistency_scenarii: target_os: ${target_os}"

export target_midpoint_directory_path
export target_os_directory_path
export merged_byte_time_sequence_json_path
export ending_file_regex
export protocol
export outfilepath
export without_test_case_with_hole
export target_os

# Execution time measurement
# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
SECONDS=0

function extract_reassembly_consistency() {
    echo ""
    echo ""
    echo ""
    echo "extract_reassembly_consistency: start"
    scenario=$1

    outfilepath="${target_midpoint_directory_path}/${protocol}_${scenario}_${target_os}_reassembly_consistency.json"
    echo "extract_reassembly_consistency: outfilepath: ${outfilepath}"
    echo "" > "${outfilepath}"

    python3 "${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_os_reassembly_consistency_scenario.py" \
    -tm "${target_midpoint_directory_path}" \
    -to "${target_os_directory_path}" \
    -m "${merged_byte_time_sequence_json_path}" \
    -r "${ending_file_regex}" \
    -s "${scenario}" \
    -o "${outfilepath}" \
    -wh "${without_test_case_with_hole}"
    
    echo "extract_reassembly_consistency: end"
}
export -f extract_reassembly_consistency

if [[ "${protocol}" == "tcp" ]]; then
    extract_reassembly_consistency pep-ap
    extract_reassembly_consistency pep-ao
    extract_reassembly_consistency peosp-ap
    extract_reassembly_consistency peosp-ao
    extract_reassembly_consistency peoef-ap
    extract_reassembly_consistency peoef-ao
    extract_reassembly_consistency peoep-ap
    extract_reassembly_consistency peoep-ao
    extract_reassembly_consistency peospef-ap
    extract_reassembly_consistency peospef-ao
    extract_reassembly_consistency peospep-ap
    extract_reassembly_consistency peospep-ao
    extract_reassembly_consistency peoepsp-ap
    extract_reassembly_consistency peoepsp-ao

    extract_reassembly_consistency peosf 
    extract_reassembly_consistency peosfef 
    extract_reassembly_consistency peoefsf 
    extract_reassembly_consistency peoepsf 
else
    extract_reassembly_consistency pep-of
    extract_reassembly_consistency pep-mf
    extract_reassembly_consistency pep-nf
    extract_reassembly_consistency pep-os
    extract_reassembly_consistency pep-ms
    extract_reassembly_consistency pep-ns
    extract_reassembly_consistency peosp-af
    extract_reassembly_consistency peosp-of
    extract_reassembly_consistency peosp-mf
    extract_reassembly_consistency peosp-nf
    extract_reassembly_consistency peosp-onf
    extract_reassembly_consistency peosp-omf
    extract_reassembly_consistency peosp-mnf
    extract_reassembly_consistency peosp-as
    extract_reassembly_consistency peosp-os
    extract_reassembly_consistency peosp-ms
    extract_reassembly_consistency peosp-ns
    extract_reassembly_consistency peosp-ons
    extract_reassembly_consistency peosp-oms
    extract_reassembly_consistency peosp-mns
    extract_reassembly_consistency peosf-af
    extract_reassembly_consistency peosf-of
    extract_reassembly_consistency peosf-mf
    extract_reassembly_consistency peosf-nf
    extract_reassembly_consistency peosf-onf
    extract_reassembly_consistency peosf-omf
    extract_reassembly_consistency peosf-mnf
    extract_reassembly_consistency peosf-as
    extract_reassembly_consistency peosf-os
    extract_reassembly_consistency peosf-ms
    extract_reassembly_consistency peosf-ns
    extract_reassembly_consistency peosf-ons
    extract_reassembly_consistency peosf-oms
    extract_reassembly_consistency peosf-mns
    
    extract_reassembly_consistency peoef
    extract_reassembly_consistency peoep
    extract_reassembly_consistency peosfef
    extract_reassembly_consistency peoefsf
    extract_reassembly_consistency peospef
    extract_reassembly_consistency peoepsf
    extract_reassembly_consistency peospep
    extract_reassembly_consistency peoepsp
fi

# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
elapsed_seconds=$SECONDS

printf -v date '%(%Y/%m/%d %H-%M-%S)T' -1

echo "extract_midpoint_os_reassembly_consistency_scenarii: elapsed: $((elapsed_seconds / 3600))hrs $(((elapsed_seconds / 60) % 60))min $((elapsed_seconds % 60))sec at ${date}"

echo "extract_midpoint_os_reassembly_consistency_scenarii: end"






