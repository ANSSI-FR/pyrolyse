#!/bin/bash

set -eu
set -o pipefail

allen_relation_file_path=$1
json_pair_directory_path=$2

# allen_relation_file_path="${PYROLYSE_PATH}/test_data_shell/allen_relation"

# json_pair_directory_path="${PYROLYSE_PATH}/test_data_shell/json/json_pair"
mkdir -p "${json_pair_directory_path}"


pair_index=0

while read -r allen_relation
do
    json_pair_file_path="${json_pair_directory_path}/pair_${allen_relation}.json"

    allen_relation_mod=$(echo "${allen_relation}" | sed 's/.*/\u&/')

    # NB: we use "sed 's/.*/\u&/'" to replace the first letter of each relation by its capital.
    echo "\"${allen_relation_mod}\"" > "${json_pair_file_path}"

    pair_index=$((pair_index + 1))
done < "${allen_relation_file_path}"
