#!/bin/bash

set -eu
set -o pipefail

echo "build_stack_scenarii_reassembly_closeness_heatmap_dependant_scenarii: start"

infilepath=$1
outfile_pattern=$2

echo "build_stack_scenarii_reassembly_closeness_heatmap_dependant_scenarii: infilepath: ${infilepath}"
echo "build_stack_scenarii_reassembly_closeness_heatmap_dependant_scenarii: outfile_pattern: ${outfile_pattern}"

infile_dirpath=$(dirname "$infilepath")
echo "build_heatmap_scenario: infile_dirpath: ${infile_dirpath}"

export infilepath
export outfile_pattern
export infile_dirpath

function build_heatmap_scenario() {
    set -e

    echo ""
    echo ""
    echo ""
    echo "build_heatmap_scenario: start"
    scenario_name=$1
    echo "build_heatmap_scenario: scenario_name: ${scenario_name}"

    outfilename="${outfile_pattern}_${scenario_name}.pdf"
    echo "build_heatmap_scenario: outfilename: ${outfilename}"
    outfilepath="${infile_dirpath}/${outfile_pattern}_${scenario_name}.pdf"
    echo "build_heatmap_scenario: outfilepath: ${outfilepath}"

    python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_dependant_scenario.py" \
    -i "$infilepath" \
    -o "$outfilepath" \
    -s "$scenario_name"

    echo "build_heatmap_scenario: end"
}
export -f build_heatmap_scenario

build_heatmap_scenario pep
build_heatmap_scenario peosp
build_heatmap_scenario peosf

echo "build_stack_scenarii_reassembly_closeness_heatmap_dependant_scenarii: end"


