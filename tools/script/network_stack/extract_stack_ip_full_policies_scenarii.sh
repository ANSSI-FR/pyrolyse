#!/bin/bash

set -eu
set -o pipefail

echo "extract_os_full_policies_scenarii: start"

target_directory_path=$1
payload_mode=$2
ip_version=$3

export payload_mode
export ip_version

function extract_complicated_policies() {
  target_directory_path=$1
  scenario=$2

  json_output_directory="${target_directory_path}/ipv${ip_version}/output_ids_json"
  echo "extract_simple_policies: json_output_directory: $json_output_directory"
  mkdir -p "$json_output_directory"
    
  set -x
  RUST_LOG=debug \
  "${PYROLYSE_PATH}/tools/pyrolyse-rs/target/debug/extract_ip_full_policies" \
  --all-chunk-json-path "${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
  --payload-json-path "$target_directory_path/ipv${ip_version}/output/ipv${ip_version}_pe${scenario}_payload.json" \
  --full-policy-json-path "${json_output_directory}/ipv${ip_version}_pe${scenario}_policy_full.json" \
  --payload-mode "$payload_mode" \
  -s
  set +x
}

export -f extract_complicated_policies

pyrolyse_debug=${PYROLYSE_DEBUG:-0}

if [ "${pyrolyse_debug}" -eq 1 ]; then
    extract_complicated_policies "${target_directory_path}" p-of
else
  extract_complicated_policies "${target_directory_path}" oep
  extract_complicated_policies "${target_directory_path}" oef
  extract_complicated_policies "${target_directory_path}" osfef
  extract_complicated_policies "${target_directory_path}" oefsf
  extract_complicated_policies "${target_directory_path}" oepsf
  extract_complicated_policies "${target_directory_path}" ospef
  extract_complicated_policies "${target_directory_path}" ospep
  extract_complicated_policies "${target_directory_path}" oepsp

fi

echo "extract_os_full_policies_scenarii: end"

