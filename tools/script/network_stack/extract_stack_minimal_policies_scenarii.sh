#!/bin/bash

#set -eu
#set -o pipefail

echo "extract_os_complicated_policies_scenarii: start"

target_directory_path=$1
protocol=$2
payload_mode=$3

export payload_mode

function extract_simple_policies() {
  target_directory_path=$1
  protocol=$2
  scenario=$3

  json_output_directory="${target_directory_path}/${protocol}/output_ids_json"
  echo "extract_simple_policies: json_output_directory: $json_output_directory"
  mkdir -p "$json_output_directory"
    
  set -x
  RUST_LOG=debug \
  "${PYROLYSE_PATH}/tools/pyrolyse-rs/target/debug/extract_minimal_policies" \
  --all-chunk-json-path "${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
  --payload-json-path "$target_directory_path/${protocol}/output/${protocol}_pe${scenario}_payload.json" \
  --policy-json-path "${json_output_directory}/${protocol}_pe${scenario}_policy_minimal.json" \
  --payload-mode "$payload_mode" \
  -s
  set +x
}

export -f extract_simple_policies

pyrolyse_debug=${PYROLYSE_DEBUG:-0}

if [[ "${protocol}" == "tcp" ]]; then
    if [ "${pyrolyse_debug}" -eq 1 ]; then
        extract_simple_policies "${target_directory_path}" "tcp" oef-ap
    else 
        extract_simple_policies "${target_directory_path}" "tcp" p-ap
        extract_simple_policies "${target_directory_path}" "tcp" p-ao
        extract_simple_policies "${target_directory_path}" "tcp" osp-ap
        extract_simple_policies "${target_directory_path}" "tcp" osp-ao
        extract_simple_policies "${target_directory_path}" "tcp" oef-ap
        extract_simple_policies "${target_directory_path}" "tcp" oef-ao
        extract_simple_policies "${target_directory_path}" "tcp" oep-ap
        extract_simple_policies "${target_directory_path}" "tcp" oep-ao
        extract_simple_policies "${target_directory_path}" "tcp" ospef-ap
        extract_simple_policies "${target_directory_path}" "tcp" ospef-ao
        extract_simple_policies "${target_directory_path}" "tcp" ospep-ap
        extract_simple_policies "${target_directory_path}" "tcp" ospep-ao
        extract_simple_policies "${target_directory_path}" "tcp" oepsp-ap
        extract_simple_policies "${target_directory_path}" "tcp" oepsp-ao

        extract_simple_policies "${target_directory_path}" "tcp" osf
        extract_simple_policies "${target_directory_path}" "tcp" osfef
        extract_simple_policies "${target_directory_path}" "tcp" oefsf
        extract_simple_policies "${target_directory_path}" "tcp" oepsf
    fi
else
    if [ "${pyrolyse_debug}" -eq 1 ]; then
        extract_simple_policies "${target_directory_path}" "${protocol}" p-of
    else
        extract_simple_policies "${target_directory_path}" "${protocol}" p-of
        extract_simple_policies "${target_directory_path}" "${protocol}" p-mf
        extract_simple_policies "${target_directory_path}" "${protocol}" p-nf
        extract_simple_policies "${target_directory_path}" "${protocol}" p-os
        extract_simple_policies "${target_directory_path}" "${protocol}" p-ms
        extract_simple_policies "${target_directory_path}" "${protocol}" p-ns

        extract_simple_policies "${target_directory_path}" "${protocol}" osp-af
        extract_simple_policies "${target_directory_path}" "${protocol}" osp-of
        extract_simple_policies "${target_directory_path}" "${protocol}" osp-mf
        extract_simple_policies "${target_directory_path}" "${protocol}" osp-nf
        extract_simple_policies "${target_directory_path}" "${protocol}" osp-onf
        extract_simple_policies "${target_directory_path}" "${protocol}" osp-omf
        extract_simple_policies "${target_directory_path}" "${protocol}" osp-mnf
        extract_simple_policies "${target_directory_path}" "${protocol}" osp-as
        extract_simple_policies "${target_directory_path}" "${protocol}" osp-os
        extract_simple_policies "${target_directory_path}" "${protocol}" osp-ms
        extract_simple_policies "${target_directory_path}" "${protocol}" osp-ns
        extract_simple_policies "${target_directory_path}" "${protocol}" osp-ons
        extract_simple_policies "${target_directory_path}" "${protocol}" osp-oms
        extract_simple_policies "${target_directory_path}" "${protocol}" osp-mns

        extract_simple_policies "${target_directory_path}" "${protocol}" osf-af
        extract_simple_policies "${target_directory_path}" "${protocol}" osf-of
        extract_simple_policies "${target_directory_path}" "${protocol}" osf-mf
        extract_simple_policies "${target_directory_path}" "${protocol}" osf-nf
        extract_simple_policies "${target_directory_path}" "${protocol}" osf-onf
        extract_simple_policies "${target_directory_path}" "${protocol}" osf-omf
        extract_simple_policies "${target_directory_path}" "${protocol}" osf-mnf
        extract_simple_policies "${target_directory_path}" "${protocol}" osf-as
        extract_simple_policies "${target_directory_path}" "${protocol}" osf-os
        extract_simple_policies "${target_directory_path}" "${protocol}" osf-ms
        extract_simple_policies "${target_directory_path}" "${protocol}" osf-ns
        extract_simple_policies "${target_directory_path}" "${protocol}" osf-ons
        extract_simple_policies "${target_directory_path}" "${protocol}" osf-oms
        extract_simple_policies "${target_directory_path}" "${protocol}" osf-mns

        extract_simple_policies "${target_directory_path}" "${protocol}" oef 
        extract_simple_policies "${target_directory_path}" "${protocol}" oep 
        extract_simple_policies "${target_directory_path}" "${protocol}" osfef 
        extract_simple_policies "${target_directory_path}" "${protocol}" oefsf 
        extract_simple_policies "${target_directory_path}" "${protocol}" ospef 
        extract_simple_policies "${target_directory_path}" "${protocol}" oepsf 
        extract_simple_policies "${target_directory_path}" "${protocol}" ospep 
        extract_simple_policies "${target_directory_path}" "${protocol}" oepsp
    fi
fi

echo "extract_os_complicated_policies_scenarii: end"

