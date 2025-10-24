



# bonus



## extract payload


editcap vm0_icmp_0.pcap toto.pcap 4-1


tshark -r vm0_icmp_0.pcap -Y 'frame.number >= 4 and frame.number <= 4' -w toto.pcap -F pcap



## plot test case scenarii

This command needs commands_01_traffic_generation to be run before.

### IP

```bash
"${PYROLYSE_PATH}/script/network_stack/plot_ip_scenarii_from_pcap_run.sh" \
"${PYROLYSE_PATH}/target/os/debian_12/ipv4/tc" \
"${PYROLYSE_PATH}/target/os/debian_12/ipv4/plot" \
ipv4 \
5
```

## Test novak model

Example for Debian 8 OS (as state-of-the-work, pep scenario for IPv4 and peosf scenario for TCP) 

### Generate byte_time_sequence

```bash
"${PYROLYSE_PATH}/script/test_data/byte_sequence_to_byte_time_sequence.sh" \
custom \
600 \
"${PYROLYSE_PATH}/test_data/custom/byte_sequence" \
"${PYROLYSE_PATH}/test_data/custom/simple_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence"
```
#### IPv4

##### Generate pcap

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_for_type.sh" \
"${PYROLYSE_PATH}/test_data/custom/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/debian_8/ipv4/custom/tc/tc_ipv4_penp" \
00:00:00:00:00:10 \
00:00:00:00:00:11 \
4 \
192.168.56.10 \
192.168.56.11 \
20000 \
icmp \
np \
custom
```

##### Replay sequence

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenario.sh" \
eth1 \
'host 192.168.56.11 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.11 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
20000 \
"${PYROLYSE_PATH}/target/os/debian_8/ipv4/custom" \
"tc/tc_ipv4_penp" \
1
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/script/network_stack/extract_stack_payload_scenario.sh" \
"${PYROLYSE_PATH}/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/os/debian_8/ipv4/custom/output" \
"penp_" \
0 \
"ipv4" \
20000
```

##### Extract policy

Copy-past payload

```bash
python3 "${PYROLYSE_PATH}/script/network_stack/extract_policy_from_novak_payload.py" \
-p AABBCCDDAABBDDCCAACCBBDDDDBBCCAADDCCAABBAADDBBCCAABBCCDDAABBDDCCAACCBBDDAACCDDBBAADDBBCCAADDCCBBBBAACCDDBBAADDCCBBCCAADDBBDDCCAACCAABBDDCCAADDBBCCBBAADDCCBBDDAABBDDCCAACCDDBBAACCAABBDDCCAADDBB \
-o "${PYROLYSE_PATH}/target/os/debian_8/ipv4/custom/output/ipv4_penp_novak.json"
```

#### TCP

##### Acquire TCP traffic

```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenario.sh" \
"${PYROLYSE_PATH}/target/os/debian_8/tcp/output" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence" \
"peosf" \
"192.168.56.10" \
"192.168.56.11" \
"1"
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/script/network_stack/extract_stack_payload_scenario.sh" \
"${PYROLYSE_PATH}/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/os/debian_8/tcp/output" \
"peosf_" \
0 \
"tcp" \
0
```

##### Extract policy

Copy-past payload

```bash
python3 "${PYROLYSE_PATH}/script/network_stack/extract_policy_from_novak_payload.py" \
-p AABBCCDDAABBDDCCAACCBBDDDDBBCCAADDCCAABBAADDBBCCAADDCCBBBBAACCDDBBAADDCCAACCDDBBAADDBBCCAADDCCBBBBAACCDDBBAADDCCBBCCAADDBBDDCCAACCAABBDDCCAADDBBCCBBAADDCCBBDDAACCDDAABBCCDDBBAADDAABBCCCCAADDBB \
-o "${PYROLYSE_PATH}/target/os/debian_8/tcp/output/tcp_peosf_novak.json"
```


