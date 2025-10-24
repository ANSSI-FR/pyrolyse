#!/bin/bash

set -xve

os_directory=$1
mac_src=$2
mac_dst=$3
ip_src=$4
ip_dst=$5

os_directory_path=${PYROLYSE_PATH}/target/midpoint/$os_directory

# pep

mkdir -p $os_directory_path/ipv4_fragmentation_icmp_pep_pcap

nb_pcap=$(ls -A $os_directory_path/ipv4_fragmentation_icmp_pep_pcap | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ipv4_fragmentation_icmp_pep_pcap"
    rm $os_directory_path/ipv4_fragmentation_icmp_pep_pcap/*
fi

RUST_LOG=debug \
${PYROLYSE_PATH}/tools/pyrolyse-rs/target/debug/generate_trace_ip_fragmentation_icmp \
-i ${PYROLYSE_PATH}/test_data/byte_time_sequence.json \
-o $os_directory_path/ipv4_fragmentation_icmp_pep_pcap \
-p p \
--ip-version 4 \
--mac-src $mac_src \
--mac-dst $mac_dst \
--ipv4-src $ip_src \
--ipv4-dst $ip_dst



# peos

mkdir -p $os_directory_path/ipv4_fragmentation_icmp_peos_pcap

nb_pcap=$(ls -A $os_directory_path/ipv4_fragmentation_icmp_peos_pcap | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ipv4_fragmentation_icmp_peos_pcap"
    rm $os_directory_path/ipv4_fragmentation_icmp_peos_pcap/*
fi

RUST_LOG=debug \
${PYROLYSE_PATH}/tools/pyrolyse-rs/target/debug/generate_trace_ip_fragmentation_icmp \
-i ${PYROLYSE_PATH}/test_data/byte_time_sequence.json \
-o $os_directory_path/ipv4_fragmentation_icmp_peos_pcap \
-p os \
--ip-version 4 \
--mac-src $mac_src \
--mac-dst $mac_dst \
--ipv4-src $ip_src \
--ipv4-dst $ip_dst



# peoe

mkdir -p $os_directory_path/ipv4_fragmentation_icmp_peoe_pcap

nb_pcap=$(ls -A $os_directory_path/ipv4_fragmentation_icmp_peoe_pcap | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ipv4_fragmentation_icmp_peoe_pcap"
    rm $os_directory_path/ipv4_fragmentation_icmp_peoe_pcap/*
fi

RUST_LOG=debug \
${PYROLYSE_PATH}/tools/pyrolyse-rs/target/debug/generate_trace_ip_fragmentation_icmp \
-i ${PYROLYSE_PATH}/test_data/byte_time_sequence.json \
-o $os_directory_path/ipv4_fragmentation_icmp_peoe_pcap \
-p oe \
--ip-version 4 \
--mac-src $mac_src \
--mac-dst $mac_dst \
--ipv4-src $ip_src \
--ipv4-dst $ip_dst



# peose

mkdir -p $os_directory_path/ipv4_fragmentation_icmp_peose_pcap

nb_pcap=$(ls -A $os_directory_path/ipv4_fragmentation_icmp_peose_pcap | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ipv4_fragmentation_icmp_peose_pcap"
    rm $os_directory_path/ipv4_fragmentation_icmp_peose_pcap/*
fi

RUST_LOG=debug \
${PYROLYSE_PATH}/tools/pyrolyse-rs/target/debug/generate_trace_ip_fragmentation_icmp \
-i ${PYROLYSE_PATH}/test_data/byte_time_sequence.json \
-o $os_directory_path/ipv4_fragmentation_icmp_peose_pcap \
-p ose \
--ip-version 4 \
--mac-src $mac_src \
--mac-dst $mac_dst \
--ipv4-src $ip_src \
--ipv4-dst $ip_dst



# peoes

mkdir -p $os_directory_path/ipv4_fragmentation_icmp_peoes_pcap

nb_pcap=$(ls -A $os_directory_path/ipv4_fragmentation_icmp_peoes_pcap | wc -l)
if [ $nb_pcap -gt 0 ]; then
    echo "Files present in ipv4_fragmentation_icmp_peoes_pcap"
    rm $os_directory_path/ipv4_fragmentation_icmp_peoes_pcap/*
fi

RUST_LOG=debug \
${PYROLYSE_PATH}/tools/pyrolyse-rs/target/debug/generate_trace_ip_fragmentation_icmp \
-i ${PYROLYSE_PATH}/test_data/byte_time_sequence.json \
-o $os_directory_path/ipv4_fragmentation_icmp_peoes_pcap \
-p oes \
--ip-version 4 \
--mac-src $mac_src \
--mac-dst $mac_dst \
--ipv4-src $ip_src \
--ipv4-dst $ip_dst










