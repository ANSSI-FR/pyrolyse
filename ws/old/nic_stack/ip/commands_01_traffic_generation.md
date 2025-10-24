

# Generate PCAP trace to test IP fragmentation


These commands must be launched from the `$PYROLYSE_PATH/ws/` directory from the pyrolyse root.


## Xilinx Onload

### IPv4|UDP

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i4_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
4 \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
udp \
"nutc" \
icvl8i4
```










${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/nic_stack/xilinx_onload/ipv4" \
40:a6:b7:b4:3a:d0 \
00:0f:53:79:19:b0 \
4 \
192.168.40.200 \
192.168.40.212 \
udp






${PYROLYSE_PATH}/script/network_stack/test/byte_time_sequence_to_pcap_scenarii.sh \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/nic_stack/xilinx_onload/ipv4_icmp" \
40:a6:b7:b4:3a:d0 \
00:0f:53:79:19:b0 \
4 \
192.168.40.200 \
192.168.40.212 \
icmp


${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/nic_stack/xilinx_onload/ipv4_udp" \
40:a6:b7:b4:3a:d0 \
00:0f:53:79:19:b0 \
4 \
192.168.40.200 \
192.168.40.212 \
udp




### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i4_pattern.json" \
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

```bash
$PYROLYSE_PATH/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh \
$PYROLYSE_PATH/target/os/debian_8 \
40:a6:b7:b4:3a:d0 \
00:0f:53:79:19:b0 \
fe80::200:ff:fe01:10 \
fe80::200:ff:fe01:11 \
6
```


${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/nic_stack/xilinx_onload/ipv6" \
40:a6:b7:b4:3a:d0 \
00:0f:53:79:19:b0 \
6 \
fd00:0:0:40::200 \
fd00:0:0:40::212 \
udp


### IPv6|ICMP

```bash
"${PYROLYSE_PATH}/script/os/byte_time_sequence_to_pcap_scenarii.sh" \
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

### IPv6|ICMP

```bash
"${PYROLYSE_PATH}/script/os/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i4_pattern.json" \
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

### IPv6|UDP

```bash
"${PYROLYSE_PATH}/script/os/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i4_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
6 \
"${BASE_IPV6_ADDR}" \
"${TARGET_IPV6_ADDR}" \
udp \
"nutc" \
icvl8i6
```
