#!/bin/bash

set -eu
set -o pipefail

allen_relation_file_path=$1
allen_triplet_file_path=$2
triplet_directory_path=$3

mkdir -p "${triplet_directory_path}"

[ -e "${allen_triplet_file_path}" ] && rm "${allen_triplet_file_path}"


# https://stackoverflow.com/questions/1620946/cartesian-product-of-two-files-as-sets-of-lines-in-gnu-linux
while read -r relation_0
do
    while read -r relation_1
    do
        while read -r relation_2
        do echo "${relation_0},${relation_1},${relation_2}" >> "${allen_triplet_file_path}"
        done < "${allen_relation_file_path}"
    done < "${allen_relation_file_path}"
done < "${allen_relation_file_path}"



triplet_index=0

while read -r triplet
do
    echo "triplet_index: ${triplet_index}"
    relation_0=$(echo "${triplet}" | cut -d',' -f1)
    relation_1=$(echo "${triplet}" | cut -d',' -f2)
    relation_2=$(echo "${triplet}" | cut -d',' -f3)
    # sparq_triplet_file_path="${triplet_directory_path}/sparq_triplet_${triplet_index}"
    sparq_triplet_file_path="${triplet_directory_path}/sparq_triplet_${relation_0}_${relation_1}_${relation_2}"
    echo "((i0 ${relation_0} i1) (i0 ${relation_1} i2) (i1 ${relation_2} i2))" > "${sparq_triplet_file_path}"
    triplet_index=$((triplet_index + 1))
done < "${allen_triplet_file_path}"
