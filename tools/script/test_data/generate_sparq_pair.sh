#!/bin/bash

set -eu
set -o pipefail

allen_relation_file_path=$1
pair_directory_path=$2

mkdir -p "${pair_directory_path}"

pair_index=0

while read -r allen_relation
do
    echo "pair_index: ${pair_index}"
    sparq_triplet_file_path="${pair_directory_path}/sparq_pair_${pair_index}"
    echo "((i0 ${allen_relation} i1))" > "${sparq_triplet_file_path}"
    pair_index=$((pair_index + 1))
done < "${allen_relation_file_path}"
