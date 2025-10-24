

# Generate PCAP trace to test IP fragmentation

## OLD

### lwip 2.1.2

192.168.56.20: base eth1
192.168.56.21: target eth1
192.168.57.22: lwip tap0
192.168.57.23: lwip stack/app

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/lwip_2.1.2/ipv4" \
00:00:00:00:00:20 \
00:00:00:00:00:21 \
4 \
192.168.56.20 \
192.168.57.23 \
icmp
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/lwip_2.1.2/ipv6" \
00:00:00:00:00:20 \
00:00:00:00:00:21 \
6 \
fd00:0:0:56::20 \
fd00:0:0:57::23 \
icmp
```



### uip 1.0

192.168.56.24: base eth1
192.168.56.25: target eth1
192.168.56.26: uip tap0 v4
192.168.56.27: uip app v4
fd00:0:0:56::28: target eth1 v6
fd00:0:0:56::29: uip tap0 v6
fd00:0:0:56::30: uip app v6

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/uip_1.0/ipv4" \
00:00:00:00:00:24 \
00:00:00:00:00:25 \
4 \
192.168.56.24 \
192.168.57.27 \
icmp
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/uip_1.0/ipv6" \
00:00:00:00:00:24 \
00:00:00:00:00:25 \
6 \
fd00:0:0:56::20 \
fd00:0:0:57::28 \
icmp
```



### picotcp 1.7.0

192.168.56.100: base eth1
192.168.56.101: target eth1
192.168.56.102: uip tap0 v4
192.168.56.103: uip app v4
fd00:0:0:56::104: target eth1 v6
fd00:0:0:56::105: uip tap0 v6
fd00:0:0:56::106: uip app v6

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/picotcp_1.7.0/ipv4" \
00:00:00:00:01:00 \
00:00:00:00:01:01 \
4 \
192.168.56.100 \
192.168.57.103 \
icmp
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/picotcp_1.7.0/ipv6" \
00:00:00:00:01:00 \
00:00:00:00:01:01 \
6 \
fd00:0:0:56::100 \
fd00:0:0:57::106 \
icmp
```

## NEW

### uip/picotcp

#### IPv4

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv4" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR_V4}" \
4 \
"${BASE_IPV4_ADDR}" \
"${STACK_IPV4_ADDR}" \
icmp \
"nutc" \
icfl8b
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i4_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv4" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR_V4}" \
4 \
"${BASE_IPV4_ADDR}" \
"${STACK_IPV4_ADDR}" \
icmp \
"nutc" \
icvl8i4
```

#### IPv6 

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv6" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR_V6}" \
6 \
"${BASE_IPV6_ADDR}" \
"${STACK_IPV6_ADDR}" \
icmp \
"nutc" \
icfl8b
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i6_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv6" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR_V6}" \
6 \
"${BASE_IPV6_ADDR}" \
"${STACK_IPV6_ADDR}" \
icmp \
"nutc" \
icvl8i6
```

### lwip/seastar/smoltcp/mirage-tcpip/mtcp

#### IPv4

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv4" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
4 \
"${BASE_IPV4_ADDR}" \
"${APP_IPV4_ADDR}" \
icmp \
"nutc" \
icfl8b
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i4_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv4" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
4 \
"${BASE_IPV4_ADDR}" \
"${APP_IPV4_ADDR}" \
icmp \
"nutc" \
icvl8i4
```

#### IPv6 

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv6" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
6 \
"${BASE_IPV6_ADDR}" \
"${APP_IPV6_ADDR}" \
icmp \
"nutc" \
icfl8b
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i6_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv6" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
6 \
"${BASE_IPV6_ADDR}" \
"${APP_IPV6_ADDR}" \
icmp \
"nutc" \
icvl8i6
```