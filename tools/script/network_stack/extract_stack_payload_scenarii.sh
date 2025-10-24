#!/bin/bash

set -eu
#set -u
set -o pipefail

echo "extract_stack_payload_scenarii: start"

python_script_path=$1
target_directory_path=$2
protocol=$3

echo "extract_stack_payload_scenarii: python_script_path: ${python_script_path}"
echo "extract_stack_payload_scenarii: target_directory_path: ${target_directory_path}"
echo "extract_stack_payload_scenarii: protocol: ${protocol}"

export target_directory_path

# NB: test_index_offset is not used for TCP

function extract_payload() {
    scenario=$1
    nb_starting_character_to_remove=$2
    nb_final_character_to_remove=$3
    test_index_offset=$4
    
    "${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenario.sh" \
    "${python_script_path}" \
    "${target_directory_path}" \
    "${scenario}" \
    "${nb_starting_character_to_remove}" \
    "${nb_final_character_to_remove}" \
    "${protocol}" \
    "${test_index_offset}"
}
export -f extract_payload

pyrolyse_debug=${PYROLYSE_DEBUG:-0}

if [[ "${protocol}" == "tcp" ]]; then
    if [ "${pyrolyse_debug}" -eq 1 ]; then
        extract_payload peoef-ap_ 0 1 0
    else 
        extract_payload pep-ap_ 0 0 0
        extract_payload pep-ao_ 0 0 0
        extract_payload peosp-ap_ 1 0 0
        extract_payload peosp-ao_ 1 0 0
        extract_payload peoef-ap_ 0 1 0
        extract_payload peoef-ao_ 0 1 0
        extract_payload peoep-ap_ 0 1 0
        extract_payload peoep-ao_ 0 1 0
        extract_payload peospef-ap_ 1 1 0
        extract_payload peospef-ao_ 1 1 0
        extract_payload peospep-ap_ 1 1 0
        extract_payload peospep-ao_ 1 1 0
        extract_payload peoepsp-ap_ 1 1 0
        extract_payload peoepsp-ao_ 1 1 0

        extract_payload peosf_ 1 0 0
        extract_payload peosfef_ 1 1 0
        extract_payload peoefsf_ 1 1 0
        extract_payload peoepsf_ 1 1 0
    fi
else
    if [ "${pyrolyse_debug}" -eq 1 ]; then
        extract_payload pep-of_ 0 0 11000
    else
        extract_payload pep-of_ 0 0 11000
        extract_payload pep-mf_ 0 0 12000
        extract_payload pep-nf_ 0 0 13000
        extract_payload pep-os_ 0 0 18000
        extract_payload pep-ms_ 0 0 19000
        extract_payload pep-ns_ 0 0 20000

        extract_payload peosp-af_ 0 0 24000
        extract_payload peosp-of_ 0 0 25000
        extract_payload peosp-mf_ 0 0 26000
        extract_payload peosp-nf_ 0 0 27000
        extract_payload peosp-onf_ 0 0 28000
        extract_payload peosp-omf_ 0 0 29000
        extract_payload peosp-mnf_ 0 0 30000
        extract_payload peosp-as_ 0 0 31000
        extract_payload peosp-os_ 0 0 32000
        extract_payload peosp-ms_ 0 0 33000
        extract_payload peosp-ns_ 0 0 34000
        extract_payload peosp-ons_ 0 0 35000
        extract_payload peosp-oms_ 0 0 36000
        extract_payload peosp-mns_ 0 0 37000

        extract_payload peosf-af_ 0 0 38000
        extract_payload peosf-of_ 0 0 39000
        extract_payload peosf-mf_ 0 0 40000
        extract_payload peosf-nf_ 0 0 41000
        extract_payload peosf-onf_ 0 0 42000
        extract_payload peosf-omf_ 0 0 43000
        extract_payload peosf-mnf_ 0 0 44000
        extract_payload peosf-as_ 0 0 45000
        extract_payload peosf-os_ 0 0 46000
        extract_payload peosf-ms_ 0 0 47000
        extract_payload peosf-ns_ 0 0 48000
        extract_payload peosf-ons_ 0 0 49000
        extract_payload peosf-oms_ 0 0 50000
        extract_payload peosf-mns_ 0 0 51000

        extract_payload peoef_ 0 8 52000
        extract_payload peoep_ 0 8 53000
        extract_payload peosfef_ 0 8 54000
        extract_payload peoefsf_ 0 8 55000
        extract_payload peospef_ 0 8 56000
        extract_payload peoepsf_ 0 8 57000
        extract_payload peospep_ 0 8 58000
        extract_payload peoepsp_ 0 8 59000
    fi
fi

echo "extract_stack_payload_scenarii: end"




