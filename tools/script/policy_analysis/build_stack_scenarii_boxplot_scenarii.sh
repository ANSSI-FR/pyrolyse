#!/bin/bash

set -eu
set -o pipefail

echo "build_stack_scenarii_boxplot_scenarii: start"

infilepath=$1
outfile_pattern=$2

echo "build_stack_scenarii_boxplot_scenarii: infilepath: ${infilepath}"
echo "build_stack_scenarii_boxplot_scenarii: outfile_pattern: ${outfile_pattern}"

infile_dirpath=$(dirname "$infilepath")
echo "build_stack_scenarii_boxplot_scenarii: infile_dirpath: ${infile_dirpath}"

ystep=1

export infilepath
export outfile_pattern
export ystep

function build_heatmap_scenario() {
    set -e

    echo ""
    echo ""
    echo ""
    echo "build_heatmap_scenario: start"
    scenario_name=$1
    echo "build_heatmap_scenario: scenario_name: ${scenario_name}"
    ymax=$2
    echo "build_heatmap_scenario: ymax: ${ymax}"

    outfilename="${outfile_pattern}_${scenario_name}.pdf"
    echo "build_heatmap_scenario: outfilename: ${outfilename}"
    outfilepath="${infile_dirpath}/${outfile_pattern}_${scenario_name}.pdf"
    echo "build_heatmap_scenario: outfilepath: ${outfilepath}"

    python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_boxplot.py" \
    -i "$infilepath" \
    -o "$outfilepath" \
    --ymax "$ymax" \
    --ytick-step "$ystep" \
    -s "$scenario_name"

    echo "build_heatmap_scenario: end"
}
export -f build_heatmap_scenario

build_heatmap_scenario pep 2
build_heatmap_scenario peosp 6
build_heatmap_scenario peosf 6

echo "build_stack_scenarii_boxplot_scenarii: end"


