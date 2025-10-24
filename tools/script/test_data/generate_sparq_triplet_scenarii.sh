#!/bin/bash

set -eu
set -o pipefail

allen_relation_file_path=$1
allen_triplet_file_path=$2
triplet_directory_path=$3

mkdir -p "${triplet_directory_path}"

[ -e "${allen_triplet_file_path}" ] && rm "${allen_triplet_file_path}"


# https://stackoverflow.com/questions/1620946/cartesian-product-of-two-files-as-sets-of-lines-in-gnu-linux
while read relation_0
do
    if [[ $relation_0 == "b" || $relation_0 == "bi" ]] 
    then before_relation_number_0=1
    else before_relation_number_0=0
    fi    

    while read relation_1
    do
        if [[ $relation_1 == "b" || $relation_1 == "bi" ]] 
        then before_relation_number_1=1
        else before_relation_number_1=0
        fi    

        while read relation_2
        do 
            if [[ $relation_2 == "b" || $relation_2 == "bi" ]] 
            then before_relation_number_2=1
            else before_relation_number_2=0
            fi  

            if [[ $(($before_relation_number_0 + $before_relation_number_1 + $before_relation_number_2)) -lt 2 ]] 
            then 
                echo "${relation_0},$relation_1,$relation_2" >> "${allen_triplet_file_path}"
            fi 
        done < "${allen_relation_file_path}"
    done < "${allen_relation_file_path}"
done < "${allen_relation_file_path}"



triplet_index=0

while read triplet
do
    echo "triplet_index: ${triplet_index}"
    relation_0=$(echo $triplet | cut -d',' -f1)
    relation_1=$(echo $triplet | cut -d',' -f2)
    relation_2=$(echo $triplet | cut -d',' -f3)
    # sparq_triplet_file_path="${triplet_directory_path}/sparq_triplet_${triplet_index}"
    sparq_triplet_file_path="${triplet_directory_path}/sparq_triplet_${relation_0}_${relation_1}_${relation_2}"
    echo "((i0 ${relation_0} i1) (i0 ${relation_1} i2) (i1 ${relation_2} i2))" > "${sparq_triplet_file_path}"
    triplet_index=$((triplet_index + 1))
done < "${allen_triplet_file_path}"
