#!/bin/bash


set -eu
set -o pipefail

echo "byte_sequence_to_byte_time_sequence: start"

relation_type=$1
test_index_offset=$2
byte_sequence_directory_path=$3
simple_chunk_pattern_json_path=$4
internet_checksum_chunk_pattern_json_path=$5
internet_invariant_ipv4_checksum_chunk_pattern_json_path=$6
internet_invariant_ipv6_checksum_chunk_pattern_json_path=$7
byte_time_sequence_directory_path=$8

echo "byte_sequence_to_byte_time_sequence: relation_type: ${relation_type}"
echo "byte_sequence_to_byte_time_sequence: test_index_offset: ${test_index_offset}"
echo "byte_sequence_to_byte_time_sequence: byte_sequence_directory_path: ${byte_sequence_directory_path}"
echo "byte_sequence_to_byte_time_sequence: simple_chunk_pattern_json_path: ${simple_chunk_pattern_json_path}"
echo "byte_sequence_to_byte_time_sequence: internet_checksum_chunk_pattern_json_path: ${internet_checksum_chunk_pattern_json_path}"
echo "byte_sequence_to_byte_time_sequence: internet_invariant_ipv4_checksum_chunk_pattern_json_path: ${internet_invariant_ipv4_checksum_chunk_pattern_json_path}"
echo "byte_sequence_to_byte_time_sequence: internet_invariant_ipv6_checksum_chunk_pattern_json_path: ${internet_invariant_ipv6_checksum_chunk_pattern_json_path}"
echo "byte_sequence_to_byte_time_sequence: byte_time_sequence_directory_path: ${byte_time_sequence_directory_path}"

mkdir -p "${byte_time_sequence_directory_path}"

# nb_processes=1

export relation_type
export test_index_offset
export simple_chunk_pattern_json_path
export internet_checksum_chunk_pattern_json_path
export internet_invariant_ipv4_checksum_chunk_pattern_json_path
export internet_invariant_ipv6_checksum_chunk_pattern_json_path
export byte_time_sequence_directory_path

function convert() {
    echo ""
    echo ""
    echo ""
    echo "convert: start"
    index=$1
    echo "convert: index: ${index}"
    file_path=$2
    echo "convert: file_path: ${file_path}"

    file_name=$(basename "${file_path}")
    file_name_wo_ext="${file_name%.*}"

    output_file_path="${byte_time_sequence_directory_path}/${file_name_wo_ext}.json"
    echo "convert: output_file_path: ${output_file_path}"

    test_index=$((test_index_offset + index - 1))

    RUST_LOG=debug \
    "${PYROLYSE_PATH}/tools/pyrolyse-rs/target/debug/generate_byte_time_sequence_single" \
    -i "${file_path}" \
    --vc1b "${simple_chunk_pattern_json_path}" \
    --icfl8b "${internet_checksum_chunk_pattern_json_path}" \
    --icvl8i4 "${internet_invariant_ipv4_checksum_chunk_pattern_json_path}" \
    --icvl8i6 "${internet_invariant_ipv6_checksum_chunk_pattern_json_path}" \
    -o "${output_file_path}" \
    --rt "${relation_type}" \
    --ti "${test_index}" 

    echo "convert: end"
}
export -f convert

# find "${byte_sequence_directory_path}" -name "${relation_type}_*.json" -type f | sort | parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "convert_byte_sequence_to_byte_time_sequence {};"


while read -r idx file_path; do 
    convert "${idx}" "${file_path}"
done <<<$(find "${byte_sequence_directory_path}" -name "${relation_type}_*.json" -type f | sort | nl)

