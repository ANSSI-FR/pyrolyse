#!/bin/bash

set -eu
set -o pipefail

sparq_triplet_consistent_directory_path=$1
json_triplet_consistent_directory_path=$2

mkdir -p "${json_triplet_consistent_directory_path}"


nb_processes=10


export json_triplet_consistent_directory_path

function convert() {
    echo ""
    echo ""
    echo ""
    echo "convert: start"
    file_path=$1
    echo "convert: file_path: ${file_path}"

    sparq_string=$(cat "${file_path}")
    echo "convert: sparq_string: ${sparq_string}"

    relation_0=$(echo "${sparq_string}" | cut -d' ' -f2)
    relation_1=$(echo "${sparq_string}" | cut -d' ' -f5)
    relation_2=$(echo "${sparq_string}" | cut -d' ' -f8)

    # NB: we use "sed 's/.*/\u&/'" to replace the first letter of each realtion by its capital.
    # https://stackoverflow.com/questions/12487424/uppercase-first-character-in-a-variable-with-bash
    relation_0_mod=$(echo "${relation_0}" | sed 's/.*/\u&/')
    relation_1_mod=$(echo "${relation_1}" | sed 's/.*/\u&/')
    relation_2_mod=$(echo "${relation_2}" | sed 's/.*/\u&/')

    file_name=$(basename "${file_path}")

    json_triplet_file_path="${json_triplet_consistent_directory_path}/triplet_${relation_0}_${relation_1}_${relation_2}.json"
    echo "{ \"relation_01\": \"${relation_0_mod}\", \"relation_02\": \"${relation_1_mod}\", \"relation_12\": \"${relation_2_mod}\" }" > "${json_triplet_file_path}"

    echo "convert: end"
}
export -f convert

find "${sparq_triplet_consistent_directory_path}" -name "sparq_triplet*" -type f | parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "convert {};"

