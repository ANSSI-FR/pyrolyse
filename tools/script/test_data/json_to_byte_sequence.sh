#!/bin/bash

set -eu
set -o pipefail


relation_type=$1
json_directory_path=$2
byte_sequence_directory_path=$3

mkdir -p "${byte_sequence_directory_path}"

# TODO: clean this
# sparq_command_path="${PYROLYSE_PATH}/sparq_dir/SparQ_${HOSTNAME}/sparq"
sparq_command_path="${PYROLYSE_PATH}/sparq/SparQ/sparq"

nb_processes=10


export relation_type
export byte_sequence_directory_path
export sparq_command_path

function convert() {
    echo ""
    echo ""
    echo ""
    echo "convert: start"
    file_path=$1
    echo "convert: file_path: ${file_path}"

    # file_name=$(basename "${file_path}" | cut -c 6-)
    file_name=$(basename "${file_path}")
    file_name_wo_ext="${file_name%.*}"

    output_file_path="${byte_sequence_directory_path}/${file_name_wo_ext}.json"
    echo "convert: output_file_path: ${output_file_path}"

    RUST_LOG=debug \
    "${PYROLYSE_PATH}/tools/pyrolyse-rs/target/debug/generate_byte_sequence_single" \
    -s "${sparq_command_path}" \
    --rt "${relation_type}" \
    -i "${file_path}" \
    -o "${output_file_path}"

    echo "convert: end"
}
export -f convert

find "${json_directory_path}" -name "${relation_type}_*.json" -type f | parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "convert {};"

