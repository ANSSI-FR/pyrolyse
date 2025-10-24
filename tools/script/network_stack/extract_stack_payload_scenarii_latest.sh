#!/bin/bash

#set -eu
set -u
set -o pipefail

echo "extract_stack_payload_scenarii_latest: start"

python_script_path=$1
target_directory_path=$2
protocol=$3

echo "extract_stack_payload_scenarii_latest: python_script_path: ${python_script_path}"
echo "extract_stack_payload_scenarii_latest: target_directory_path: ${target_directory_path}"
echo "extract_stack_payload_scenarii_latest: protocol: ${protocol}"

export os_directory_path
export python_script_path
export protocol

function extract_payload() {
    echo ""
    echo ""
    echo ""
    echo "extract_stack_payload_scenarii_latest: extract_payload: start"
    scenario_name=$1
    nb_starting_character_to_remove=$2
    nb_finishing_character_to_remove=$3
    test_index_offset=$4

    echo "extract_stack_payload_scenarii_latest: extract_payload: scenario_name: ${scenario_name}"

    latest_pcappath="${target_directory_path}/${protocol}_${scenario_name}_latest"
    json_path="${target_directory_path}/${protocol}_${scenario_name}_payload.json"

    log_path="${target_directory_path}/log_${protocol}_${scenario_name}_payload_extraction_latest.log"
    echo "extract_stack_payload_scenarii_latest: extract_payload: log_path: ${log_path}"
    
    set -x
    python3 "${python_script_path}" \
    -i "${latest_pcappath}" \
    -j "${json_path}" \
    -sr "${nb_starting_character_to_remove}" \
    -fr "${nb_finishing_character_to_remove}" \
    -o "${test_index_offset}" &> "${log_path}"
    # -p "${protocol}" \
    set +x
    
    echo "extract_stack_payload_scenarii_latest: extract_payload: end"
}
export -f extract_payload

pyrolyse_debug=${PYROLYSE_DEBUG:-0}

if [[ "${protocol}" == "tcp" ]]; then
    if [ "${pyrolyse_debug}" -eq 1 ]; then
        extract_payload peoef-ap 0 1 0
    else 
        extract_payload pep-ap 0 0 0
        extract_payload pep-ao 0 0 0
        extract_payload peosp-ap 1 0 0
        extract_payload peosp-ao 1 0 0
        extract_payload peoef-ap 0 1 0
        extract_payload peoef-ao 0 1 0
        extract_payload peoep-ap 0 1 0
        extract_payload peoep-ao 0 1 0
        extract_payload peospef-ap 1 1 0
        extract_payload peospef-ao 1 1 0
        extract_payload peospep-ap 1 1 0
        extract_payload peospep-ao 1 1 0
        extract_payload peoepsp-ap 1 1 0
        extract_payload peoepsp-ao 1 1 0

        extract_payload peosf 1 0 0
        extract_payload peosfef 1 1 0
        extract_payload peoefsf 1 1 0
        extract_payload peoepsf 1 1 0
    fi
else
    if [ "${pyrolyse_debug}" -eq 1 ]; then
        extract_payload pep-of 0 0 11000
    else
        extract_payload pep-of 0 0 11000
        extract_payload pep-mf 0 0 12000
        extract_payload pep-nf 0 0 13000
        extract_payload pep-os 0 0 18000
        extract_payload pep-ms 0 0 19000
        extract_payload pep-ns 0 0 20000

        extract_payload peosp-af 0 0 24000
        extract_payload peosp-of 0 0 25000
        extract_payload peosp-mf 0 0 26000
        extract_payload peosp-nf 0 0 27000
        extract_payload peosp-onf 0 0 28000
        extract_payload peosp-omf 0 0 29000
        extract_payload peosp-mnf 0 0 30000
        extract_payload peosp-as 0 0 31000
        extract_payload peosp-os 0 0 32000
        extract_payload peosp-ms 0 0 33000
        extract_payload peosp-ns 0 0 34000
        extract_payload peosp-ons 0 0 35000
        extract_payload peosp-oms 0 0 36000
        extract_payload peosp-mns 0 0 37000

        extract_payload peosf-af 0 0 38000
        extract_payload peosf-of 0 0 39000
        extract_payload peosf-mf 0 0 40000
        extract_payload peosf-nf 0 0 41000
        extract_payload peosf-onf 0 0 42000
        extract_payload peosf-omf 0 0 43000
        extract_payload peosf-mnf 0 0 44000
        extract_payload peosf-as 0 0 45000
        extract_payload peosf-os 0 0 46000
        extract_payload peosf-ms 0 0 47000
        extract_payload peosf-ns 0 0 48000
        extract_payload peosf-ons 0 0 49000
        extract_payload peosf-oms 0 0 50000
        extract_payload peosf-mns 0 0 51000

        extract_payload peoef 0 8 52000
        extract_payload peoep 0 8 53000
        extract_payload peosfef 0 8 54000
        extract_payload peoefsf 0 8 55000
        extract_payload peospef 0 8 56000
        extract_payload peoepsf 0 8 57000
        extract_payload peospep 0 8 58000
        extract_payload peoepsp 0 8 59000
    fi
fi

echo "extract_stack_payload_scenarii_latest: end"


