

# Generate PCAP trace to test IP fragmentation

These commands must be launched inside the base machine from the Vagrantfile, i.e. after doing "vagrant ssh base" in each ./ws/$OS/ directory.

## One target box for IPv4 and IPv6 testing

### IPv4

#### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
4 \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
icmp \
"nutc" \
icfl8b
```

#### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i4_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
4 \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
icmp \
"nutc" \
icvl8i4
```

### IPv6

#### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
6 \
"${BASE_IPV6_ADDR}" \
"${TARGET_IPV6_ADDR}" \
icmp \
"nutc" \
icfl8b
```

#### Invariant checksum for multiple reassembled payload lengths 


```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i6_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
6 \
"${BASE_IPV6_ADDR}" \
"${TARGET_IPV6_ADDR}" \
icmp \
"nutc" \
icvl8i6
```

## Two target boxes for IPv4 and IPv6 testing

### IPv4

#### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR_V4}" \
4 \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
icmp \
"nutc" \
icfl8b
```

#### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i4_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR_V4}" \
4 \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
icmp \
"nutc" \
icvl8i4
```

### IPv6

#### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR_V6}" \
6 \
"${BASE_IPV6_ADDR}" \
"${TARGET_IPV6_ADDR}" \
icmp \
"nutc" \
icfl8b
```

#### Invariant checksum for multiple reassembled payload lengths 


```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i6_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR_V6}" \
6 \
"${BASE_IPV6_ADDR}" \
"${TARGET_IPV6_ADDR}" \
icmp \
"nutc" \
icvl8i6
```