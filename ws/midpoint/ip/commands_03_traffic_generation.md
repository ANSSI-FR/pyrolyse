# Generate PCAP trace to test IP fragmentation

## IPv4

### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
00:00:00:00:00:00 \
00:00:00:00:00:01 \
4 \
"192.168.56.1" \
"$target_ipv4_addr" \
icmp \
"nutc" \
icfl8b
```

### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i4_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
00:00:00:00:00:00 \
00:00:00:00:00:01 \
4 \
"192.168.56.1" \
"$target_ipv4_addr" \
icmp \
"nutc" \
icvl8i4
```


## IPv6

### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
00:00:00:00:00:00 \
00:00:00:00:00:01 \
6 \
fe80::200:ff:fe02:10 \
"$target_ipv6_addr" \
icmp \
"nutc" \
icfl8b
```

### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i6_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
00:00:00:00:00:00 \
00:00:00:00:00:01 \
6 \
fe80::200:ff:fe02:10 \
"$target_ipv6_addr" \
icmp \
"nutc" \
icvl8i6
```