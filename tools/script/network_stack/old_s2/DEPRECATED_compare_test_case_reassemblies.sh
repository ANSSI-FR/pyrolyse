#!/bin/bash

set -eu
set -o pipefail

echo "compare_test_case_reassemblies: start"

json_path_1=$1
json_path_2=$2

export json_path_1
export json_path_2

function compare_reassemblies() {
  target_directory_path=$1
  protocol=$2
  scenario=$3
    

}

export -f compare_reassemblies

json_content_1=$(jq ".hm" "${json_path_1}")
json_content_2=$(jq ".hm" "${json_path_2}")

tc_1_l=( $(echo ${json_content_1} | jq 'keys[]') ) 
tc_2_l=( $(echo ${json_content_2} | jq 'keys[]') ) 

# Compute test case scenarii intersection (from https://stackoverflow.com/questions/7870230/array-intersection-in-bash)
l2=" ${tc_2_l[*]} "                    # add framing blanks
for item in ${tc_1_l[@]}; do
  if [[ $l2 =~ " $item " ]] ; then    # use $item as regexp
    tc_intersection_l+=(${item})
  fi
done
echo "++ Test case intersection:  ${tc_intersection_l[*]}" 

# 
for test_index in "${tc_intersection_l[@]}"
do
  #echo "$test_index"
  payload_1=$(echo "${json_content_1}" | jq ".${test_index}.payload")
  payload_2=$(echo "${json_content_2}" | jq ".${test_index}.payload")

  echo "payload_1: ${payload_1}"
  echo "payload_2: ${payload_2}"
  
  # or do whatever with individual element of the array
done


#find "${byte_time_sequence_path}" -name "*.json" | parallel --no-notice --bar --halt now,fail=1 -j "${nb_processes}" "acquire_single_test_trace {};"

echo "compare_test_case_reassemblies: end"
