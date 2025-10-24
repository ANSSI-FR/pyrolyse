#!/bin/bash

set -eu
set -o pipefail

path=$1

function display() {
    # echo "display: start"
    path=$1
    
    sha_path_s=$(sha256sum "${path}")
    # echo "display: sha_path_s: ${sha_path_s}"

    a=( "${sha_path_s}" )
    sha_s=${a[0]}
    # echo "display: sha_s: ${sha_s}"
    path_s=${a[1]}
    # echo "display: path_s: ${path_s}"

    # sha_s=$(echo "${sha_path_s}" | cut -d' ' -f1)
    # echo "display: sha_s: ${sha_s}"
    # path_s=$(echo "${sha_path_s}" | cut -d' ' -f2)
    # echo "display: path_s: ${path_s}"

    basename_s=$(basename "${path_s}")
    # echo "display: basename_s: ${basename_s}"

    echo "${sha_s} ${basename_s}"
    # echo "display: end"
}
export -f display


find "${path}" -name "*payload*.json" -type f \
| sort -k2 \
| parallel --no-notice --halt now,fail=1 -j 1 "display {};" 
