#!/bin/bash

set -eu
set -o pipefail

triplet_all_directory_path=$1
sparq_tmp_directory_path=$2
triplet_consistent_directory_path=$3

mkdir -p "${sparq_tmp_directory_path}"
mkdir -p "${triplet_consistent_directory_path}"

nb_processes=10

# TODO: clean this
# sparq_command_path="${PYROLYSE_PATH}/sparq_dir/SparQ_${HOSTNAME}/sparq"
sparq_command_path="${PYROLYSE_PATH}/sparq/SparQ/sparq"

export sparq_tmp_directory_path
export triplet_consistent_directory_path
export sparq_command_path

function check_sparq_string() {
    echo ""
    echo ""
    echo ""
    echo "check_sparq_string: start"
    file_path=$1
    echo "check_sparq_string: file_path: ${file_path}"
    sparq_string=$(cat "${file_path}")
    echo "check_sparq_string: sparq_string: ${sparq_string}"

    echo "check_sparq_string: sparq_command_path: ${sparq_command_path}"

    file_name=$(basename "${file_path}")

    sparq_tmp_path="${sparq_tmp_directory_path}/${file_name}"
    echo "check_sparq_string: sparq_tmp_path: ${sparq_tmp_path}"
    "${sparq_command_path}" constraint-reasoning allen check-consistency "${sparq_string}" > "${sparq_tmp_path}"

    nb_line_consistent=$(cat "${sparq_tmp_path}" | grep Consistent | wc -l)
    echo "check_sparq_string: nb_line_consistent: ${nb_line_consistent}"

    if [[ "${nb_line_consistent}" = "1" ]]; then
        sparq_triplet_file_path="${triplet_consistent_directory_path}/${file_name}"
        cat "${file_path}" > "${sparq_triplet_file_path}"
    fi

    echo "check_sparq_string: end"
}
export -f check_sparq_string

find "${triplet_all_directory_path}" -name "sparq_triplet*" -type f | parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "check_sparq_string {};"

