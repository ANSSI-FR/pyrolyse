#!/bin/bash

set -x
set -v
set -e

OS_DIRECTORY_PATH=$1


RUST_LOG=debug $PYROLYSE_PATH/rust_code/reassembly_test_pipeline/target/debug/extract_complicated_policies \
--all-chunk-json-path $PYROLYSE_PATH/test_data/byte_time_sequence.json \
--payload-json-path $OS_DIRECTORY_PATH/tcp_pep1_payload.json \
--policy-json-path $OS_DIRECTORY_PATH/tcp_pep1_policy_complicated.json \
--payload-mode i \
-s

RUST_LOG=debug $PYROLYSE_PATH/rust_code/reassembly_test_pipeline/target/debug/extract_complicated_policies \
--all-chunk-json-path $PYROLYSE_PATH/test_data/byte_time_sequence.json \
--payload-json-path $OS_DIRECTORY_PATH/tcp_pep2_payload.json \
--policy-json-path $OS_DIRECTORY_PATH/tcp_pep2_policy_complicated.json \
--payload-mode i \
-s


RUST_LOG=debug $PYROLYSE_PATH/rust_code/reassembly_test_pipeline/target/debug/extract_complicated_policies \
--all-chunk-json-path $PYROLYSE_PATH/test_data/byte_time_sequence.json \
--payload-json-path $OS_DIRECTORY_PATH/tcp_peos_payload.json \
--policy-json-path $OS_DIRECTORY_PATH/tcp_peos_policy_complicated.json \
--payload-mode i \
-s

