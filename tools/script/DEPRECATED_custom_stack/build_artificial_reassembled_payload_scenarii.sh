#!/bin/bash

set -eu
set -o pipefail

echo "build_artificial_reassembled_payload_scenarii: start"

target_directory_path=$1
protocol=$2

echo "build_artificial_reassembled_payload_scenarii: target_directory_path: ${target_directory_path}"
echo "build_artificial_reassembled_payload_scenarii: protocol: ${protocol}"

export target_directory_path
export protocol

# Execution time measurement
# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
SECONDS=0

function build_artificial_scenario() {
    echo "build_artificial_scenario: start"
    scenario=$1

    pattern_json_payload_file="${protocol}_${scenario}_payload_"
    echo "build_artificial_scenario: pattern_json_payload_file: $pattern_json_payload_file"
    output_json_payload_file="${protocol}_${scenario}_payload.json"
    echo "build_artificial_scenario: output_json_payload_file: $output_json_payload_file"
    
    python3 "${PYROLYSE_PATH}/tools/script/custom_stack/build_artificial_reassembled_payload_scenario.py" \
    -t "${target_directory_path}" \
    -p "${pattern_json_payload_file}" \
    -o "${output_json_payload_file}"
    
    echo "build_artificial_scenario: end"
}
export -f build_artificial_scenario


if [[ "${protocol}" == "tcp" ]]; then
    build_artificial_scenario pep-ap
    build_artificial_scenario pep-ao
    build_artificial_scenario peosp-ap
    build_artificial_scenario peosp-ao
    build_artificial_scenario peoef-ap
    build_artificial_scenario peoef-ao
    build_artificial_scenario peoep-ap
    build_artificial_scenario peoep-ao
    build_artificial_scenario peospef-ap
    build_artificial_scenario peospef-ao
    build_artificial_scenario peospep-ap
    build_artificial_scenario peospep-ao
    build_artificial_scenario peoepsp-ap
    build_artificial_scenario peoepsp-ao

    build_artificial_scenario peosf
    build_artificial_scenario peosfef
    build_artificial_scenario peoefsf
    build_artificial_scenario peoepsf
else
    build_artificial_scenario pep-af
    build_artificial_scenario pep-of
    build_artificial_scenario pep-mf
    build_artificial_scenario pep-nf
    build_artificial_scenario pep-onf
    build_artificial_scenario pep-omf
    build_artificial_scenario pep-mnf
    build_artificial_scenario pep-as
    build_artificial_scenario pep-os
    build_artificial_scenario pep-ms
    build_artificial_scenario pep-ns
    build_artificial_scenario pep-ons
    build_artificial_scenario pep-oms
    build_artificial_scenario pep-mns

    build_artificial_scenario peosp-af
    build_artificial_scenario peosp-of
    build_artificial_scenario peosp-mf
    build_artificial_scenario peosp-nf
    build_artificial_scenario peosp-onf
    build_artificial_scenario peosp-omf
    build_artificial_scenario peosp-mnf
    build_artificial_scenario peosp-as
    build_artificial_scenario peosp-os
    build_artificial_scenario peosp-ms
    build_artificial_scenario peosp-ns
    build_artificial_scenario peosp-ons
    build_artificial_scenario peosp-oms
    build_artificial_scenario peosp-mns

    build_artificial_scenario peosf-af
    build_artificial_scenario peosf-of
    build_artificial_scenario peosf-mf
    build_artificial_scenario peosf-nf
    build_artificial_scenario peosf-onf
    build_artificial_scenario peosf-omf
    build_artificial_scenario peosf-mnf
    build_artificial_scenario peosf-as
    build_artificial_scenario peosf-os
    build_artificial_scenario peosf-ms
    build_artificial_scenario peosf-ns
    build_artificial_scenario peosf-ons
    build_artificial_scenario peosf-oms
    build_artificial_scenario peosf-mns

    build_artificial_scenario peoef
    build_artificial_scenario peoep
    build_artificial_scenario peosfef
    build_artificial_scenario peoefsf
    build_artificial_scenario peospef
    build_artificial_scenario peoepsf
    build_artificial_scenario peospep
    build_artificial_scenario peoepsp
fi


# https://unix.stackexchange.com/questions/52313/how-to-get-execution-time-of-a-script-effectively
elapsed_seconds=$SECONDS
echo "build_artificial_reassembled_payload_scenarii: elapsed: $((elapsed_seconds / 3600))hrs $(((elapsed_seconds / 60) % 60))min $((elapsed_seconds % 60))sec"

echo "build_artificial_reassembled_payload_scenarii: end"



