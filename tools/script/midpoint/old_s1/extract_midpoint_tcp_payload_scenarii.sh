#!/bin/bash

set -xve

PYTHON_SCRIPT_PATH=$1
OS_DIRECTORY=$2
BYTE_TIME_SEQUENCE_JSON_PATH=$3


${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenario.sh \
$PYTHON_SCRIPT_PATH \
$OS_DIRECTORY \
$BYTE_TIME_SEQUENCE_JSON_PATH \
pep1_ \
tcp 

${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenario.sh \
$PYTHON_SCRIPT_PATH \
$OS_DIRECTORY \
$BYTE_TIME_SEQUENCE_JSON_PATH \
pep2_ \
tcp 

${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenario.sh \
$PYTHON_SCRIPT_PATH \
$OS_DIRECTORY \
$BYTE_TIME_SEQUENCE_JSON_PATH \
peos_ \
tcp 

