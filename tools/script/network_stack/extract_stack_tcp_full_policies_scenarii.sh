#!/bin/bash

set -eu
set -o pipefail

echo "extract_os_complicated_policies_scenarii: start"

target_directory_path=$1
payload_mode=$2

export payload_mode

function extract_complicated_policies() {
  target_directory_path=$1
  scenario=$2
    
  json_output_directory="${target_directory_path}/tcp/output_ids_json"
  echo "extract_simple_policies: json_output_directory: $json_output_directory"
  mkdir -p "$json_output_directory"

  set -x
  RUST_LOG=debug \
  "${PYROLYSE_PATH}/tools/pyrolyse-rs/target/debug/extract_tcp_full_policies" \
  --all-chunk-json-path "${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
  --payload-json-path "$target_directory_path/tcp/output/tcp_pe${scenario}_payload.json" \
  --complicated-policy-json-path "${json_output_directory}/tcp_pe${scenario}_policy_full.json" \
  --payload-mode "$payload_mode" \
  -s
  set +x
}

export -f extract_complicated_policies



extract_complicated_policies "${target_directory_path}" p-ap
extract_complicated_policies "${target_directory_path}" p-ao
extract_complicated_policies "${target_directory_path}" osp-ap
extract_complicated_policies "${target_directory_path}" osp-ao
extract_complicated_policies "${target_directory_path}" oef-ap
extract_complicated_policies "${target_directory_path}" oef-ao
extract_complicated_policies "${target_directory_path}" oep-ap
extract_complicated_policies "${target_directory_path}" oep-ao
extract_complicated_policies "${target_directory_path}" ospef-ap
extract_complicated_policies "${target_directory_path}" ospef-ao
extract_complicated_policies "${target_directory_path}" ospep-ap
extract_complicated_policies "${target_directory_path}" ospep-ao
extract_complicated_policies "${target_directory_path}" oepsp-ap
extract_complicated_policies "${target_directory_path}" oepsp-ao

extract_complicated_policies "${target_directory_path}" osf
extract_complicated_policies "${target_directory_path}" osfef
extract_complicated_policies "${target_directory_path}" oefsf
extract_complicated_policies "${target_directory_path}" oepsf

echo "extract_os_complicated_policies_scenarii: end"

