



# bonus

TODO: Do something cleaner for multiple mode testing (i.e. novak chunk sequence)


## extract payload


editcap vm0_icmp_0.pcap toto.pcap 4-1


tshark -r vm0_icmp_0.pcap -Y 'frame.number >= 4 and frame.number <= 4' -w toto.pcap -F pcap



## plot test case scenarii

This command needs commands_01_traffic_generation to be run before.

### IP

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/plot_ip_scenarii_from_pcap.sh" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/tc" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/plot" \
--with-payload \
5
```

### TCP

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/plot_tcp_scenarii_from_pcap.sh" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/plot" \
--with-payload \
5
```

## Test novak model

Example for Debian 8 OS (as state-of-the-work, pep scenario for IPv4 and peosf scenario for TCP) 

### Generate byte_time_sequence

```bash
"${PYROLYSE_PATH}/tools/script/test_data/byte_sequence_to_byte_time_sequence.sh" \
custom \
600 \
"${PYROLYSE_PATH}/test_data/custom/byte_sequence" \
"${PYROLYSE_PATH}/test_data/custom/vc1b_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/icvl8i4_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/icvl8i6_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence"
```
#### IPv4

##### Generate pcap

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_for_type.sh" \
"${PYROLYSE_PATH}/test_data/custom/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/custom/tc/tc_ipv4_pep-nf" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
4 \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
20000 \
icmp \
p-nf \
"icfl8b" \
"nutc" \
custom
```

##### Replay sequence

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/replay_capture_test_sequence_scenario.sh" \
eth1 \
'ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host ${BASE_IPV4_ADDR} and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
20000 \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/custom" \
"tc/tc_ipv4_pep-nf" \
2
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenario.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/custom/output" \
"pep-nf_" \
0 \
"ipv4" \
20000
```

##### Extract policy

Copy-past payload

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/extract_policy_from_novak_payload.py" \
-i  "${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/custom/output" \
-p  "pep-nf" \
-o "${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/custom/output/ipv4_pep-nf_novak.json"
```

#### IPv6

##### Generate pcap

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_for_type.sh" \
"${PYROLYSE_PATH}/test_data/custom/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/custom/tc/tc_ipv6_pep-nf" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
6 \
"${BASE_IPV6_ADDR}" \
"${TARGET_IPV6_ADDR}" \
20000 \
icmp \
p-nf \
"icfl8b" \
"nutc" \
custom
```


##### Replay sequence

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/replay_capture_test_sequence_scenario.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host ${TARGET_IPV6_ADDR} and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
20000 \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/custom" \
"tc/tc_ipv6_pep-nf" \
2
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenario.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/custom/output" \
"pep-nf_" \
0 \
"ipv6" \
20000
```

##### Extract policy

Copy-past payload

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/extract_policy_from_novak_payload.py" \
-i  "${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/custom/output" \
-p  "pep-nf" \
-o "${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/custom/output/ipv6_pep-nf_novak.json"
```


#### TCP

##### Acquire TCP traffic

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/acquire_tcp_traffic_scenario.sh" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/custom/output" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence" \
"peosf" \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
"icfl8b" \
"2"
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenario.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/custom/output" \
"peosf_" \
0 \
"tcp" \
0
```

##### Extract policy

Copy-past payload

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/extract_policy_from_novak_payload.py" \
-i  "${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/custom/output" \
-p  "peosf" \
-o "${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/custom/output/tcp_peosf_novak.json"
```


