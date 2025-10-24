



# bonus



## extract payload


editcap vm0_icmp_0.pcap toto.pcap 4-1


tshark -r vm0_icmp_0.pcap -Y 'frame.number >= 4 and frame.number <= 4' -w toto.pcap -F pcap



## plot test case scenarii

This command needs commands_01_traffic_generation to be run before.

### IP

#### from ic8s patterns

##### with payload

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/plot_ip_scenarii_from_pcap.sh" \
"${PYROLYSE_PATH}/target/os/solaris_11.2/ipv4/tc" \
"${PYROLYSE_PATH}/target/os/solaris_11.2/ipv4/plot/ic8s" \
"--with_payload"
5
```

##### without payload

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/plot_ip_scenarii_from_pcap.sh" \
"${PYROLYSE_PATH}/target/os/solaris_11.2/ipv4/tc" \
"${PYROLYSE_PATH}/target/os/solaris_11.2/ipv4/plot/ic8s" \
""
5
```

#### from ic8a patterns

##### with payload

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/plot_ip_scenarii_from_pcap.sh" \
"${PYROLYSE_PATH}/target/os/solaris_11.2/ipv4/tc" \
"${PYROLYSE_PATH}/target/os/solaris_11.2/ipv4/plot/ic8a" \
"--with-payload" \
80
```

##### without payload

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/plot_ip_scenarii_from_pcap.sh" \
"${PYROLYSE_PATH}/target/os/solaris_11.2/ipv4/tc" \
"${PYROLYSE_PATH}/target/os/solaris_11.2/ipv4/plot/ic8a" \
"--no-with-payload" \
80
```

## Test novak model

Example for Snort (as state-of-the-work, pep scenario for IPv4 and peosf scenario for TCP) 

### Generate byte_time_sequence

```bash
"${PYROLYSE_PATH}/tools/script/test_data/byte_sequence_to_byte_time_sequence.sh" \
custom \
600 \
"${PYROLYSE_PATH}/test_data/custom/byte_sequence" \
"${PYROLYSE_PATH}/test_data/custom/vc1b_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence"
```

### Snort

#### IPv4


##### Generate pcap

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_for_type.sh" \
"${PYROLYSE_PATH}/test_data/custom/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom/tc/tc_ipv4_pep-nf" \
00:00:00:00:00:00 \
00:00:00:00:00:01 \
4 \
192.168.0.1 \
"${target_ipv4_addr}" \
20000 \
icmp \
p-nf \
icfl8b \
"nutc" \
custom
```

##### Detection

###### with generic scripts

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenario_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom" \
"tc/tc_ipv4_pep-nf" \
'LD_LIBRARY_PATH="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.6.3.0/bin/snort"' \
'--daq-dir "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/daq/" \
-r "${pcap_path}" \
-c "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort.lua" \
-R "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort_${protocol_s}_${payload_mode}_${reassembly_options}.rules" \
-l "${ids_log_tmp_directory_path_current}" \
-d' \
alert_json.txt \
"$reassembly_options" \
icfl8b \
ipv4 \
1
```

###### with non generic scripts

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_snort.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom" \
"tc/tc_ipv4_pep-nf" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.6.3.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf" \
"${reassembly_options}" \
1
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenario.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_snort_log_directory_to_json_custom.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom/output/${reassembly_options}" \
"pep-nf" \
"ipv4" \
0 \
0 \
icfl8b \
"alert_json" \
1
```

##### Extract policy

Copy-past payload

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/extract_policy_from_novak_payload.py" \
-i  "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom/output/${reassembly_options}" \
-p  "pep-nf" \
-o "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom/output/${reassembly_options}/ipv4_pep-nf_novak.json"
```

#### IPv6

##### Generate pcap

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_for_type.sh" \
"${PYROLYSE_PATH}/test_data/custom/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom/tc/tc_ipv6_pep-nf" \
00:00:00:00:00:00 \
00:00:00:00:00:01 \
6 \
fe80::200:ff:fe01:1 \
"${target_ipv6_addr}" \
20000 \
icmp \
p-nf \
icfl8b \
"nutc" \
custom
```

##### Detection

###### with generic scripts

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenario_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom" \
"tc/tc_ipv6_pep-nf" \
'LD_LIBRARY_PATH="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.6.3.0/bin/snort"' \
'--daq-dir "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/daq/" \
-r "${pcap_path}" \
-c "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort.lua" \
-R "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort_${protocol_s}_${payload_mode}_${reassembly_options}.rules" \
-l "${ids_log_tmp_directory_path_current}" \
-d' \
alert_json.txt \
"$reassembly_options" \
icfl8b \
ipv6 \
1
```


###### with non generic scripts

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_snort.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom" \
"tc/tc_ipv6_pep-nf" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.6.3.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf" \
"${reassembly_options}" \
1
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenario.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_snort_log_directory_to_json_custom.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom/output/${reassembly_options}" \
"pep-nf" \
"ipv6" \
0 \
0 \
icfl8b \
"alert_json" \
1
```

##### Extract policy

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/extract_policy_from_novak_payload.py" \
-i  "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom/output/${reassembly_options}/" \
-p  "pep-nf" \
-o "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom/output/${reassembly_options}/ipv6_pep-nf_novak.json"
```

#### TCP

##### Traffic adaptation


```bash
mkdir -p "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/tc/tc_tcp_peosf"

cp -r "${PYROLYSE_PATH}/target/os/solaris_11.2/tcp/custom/output/tcp_peosf_latest/"* "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/tc/tc_tcp_peosf"

python3 "${PYROLYSE_PATH}/tools/script/midpoint/adapt_ip_address_from_os_to_midpoint_tcp_tests.py" \
-i "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/tc/tc_tcp_peosf" \
-s  "192.168.0.1" \
-d "${target_ipv4_addr}"
```

##### Detection

###### with generic scripts

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenario_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom" \
"tc/tc_tcp_peosf" \
'LD_LIBRARY_PATH="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.6.3.0/bin/snort"' \
'--daq-dir "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/daq/" \
-r "${pcap_path}" \
-c "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort.lua" \
-R "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort_${protocol_s}_${payload_mode}_${reassembly_options}.rules" \
-l "${ids_log_tmp_directory_path_current}" \
-d' \
alert_json.txt \
"$reassembly_options" \
icfl8b \
tcp \
1
```

###### with non generic scripts

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_snort.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom" \
"tc/tc_tcp_peosf" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.6.3.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf" \
"${reassembly_options}" \
1
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenario.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_snort_log_directory_to_json_custom.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/output/${reassembly_options}" \
"peosf" \
"tcp" \
1 \
0 \
icfl8b \
"alert_json" \
1
```

##### Extract policy

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/extract_policy_from_novak_payload.py" \
-i  "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/output/${reassembly_options}" \
-p  "peosf" \
-o "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/output/${reassembly_options}/tcp_peosf_novak.json"
```

### Suricata

#### IPv4


##### Generate pcap

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_for_type.sh" \
"${PYROLYSE_PATH}/test_data/custom/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom/tc/tc_ipv4_pep-nf" \
00:00:00:00:00:00 \
00:00:00:00:00:01 \
4 \
192.168.0.1 \
"${target_ipv4_addr}" \
20000 \
icmp \
p-nf \
icfl8b \
"nutc" \
custom
```


##### Detection

###### with generic scripts

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenario_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom" \
"tc/tc_ipv4_pep-nf" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
jasonish/suricata:7.0.4' \
'-S "/ids_conf/suricata_${protocol_s}_${payload_mode}_${reassembly_options}.rules" -c "/ids_conf/suricata.yaml" -r "/target_protocol_dir/${pcap_relative_path}"' \
eve.json \
"$reassembly_options" \
icfl8b \
ipv4 \
1
```

###### with non generic scripts


```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_suricata.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom" \
"tc/tc_ipv4_pep-nf" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/install/install_7.0.4" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf" \
"${reassembly_options}" \
1
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenario.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_suricata_log_directory_to_json_custom.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom/output/${reassembly_options}" \
"pep-nf" \
"ipv4" \
0 \
0 \
icfl8b \
payload \
1
```

##### Extract policy

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/extract_policy_from_novak_payload.py" \
-i  "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom/output/${reassembly_options}" \
-p  "pep-nf" \
-o "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom/output/${reassembly_options}/ipv4_pep-nf_novak.json"
```

#### IPv6

##### Generate pcap

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_for_type.sh" \
"${PYROLYSE_PATH}/test_data/custom/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom/tc/tc_ipv6_pep-nf" \
00:00:00:00:00:00 \
00:00:00:00:00:01 \
6 \
fe80::200:ff:fe01:1 \
"${target_ipv6_addr}" \
20000 \
icmp \
p-nf \
icfl8b \
"nutc" \
custom
```

##### Detection

###### with generic scripts

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenario_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom" \
"tc/tc_ipv6_pep-nf" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
jasonish/suricata:7.0.4' \
'-S "/ids_conf/suricata_${protocol_s}_${payload_mode}_${reassembly_options}.rules" -c "/ids_conf/suricata.yaml" -r "/target_protocol_dir/${pcap_relative_path}"' \
eve.json \
"$reassembly_options" \
icfl8b \
ipv6 \
1
```

###### with non generic scripts


```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_suricata.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom" \
"tc/tc_ipv6_pep-nf" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/install/install_7.0.4" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf" \
"${reassembly_options}" \
1
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenario.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_suricata_log_directory_to_json_custom.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom/output/${reassembly_options}" \
"pep-nf" \
"ipv6" \
0 \
0 \
icfl8b \
payload \
1
```

##### Extract policy

Copy-past payload

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/extract_policy_from_novak_payload.py" \
-i  "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom/output/${reassembly_options}" \
-p  "pep-nf" \
-o "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom/output/${reassembly_options}/ipv6_pep-nf_novak.json"
```

#### TCP

##### Traffic adaptation


```bash
mkdir -p "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/tc/tc_tcp_peosf"

cp -r "${PYROLYSE_PATH}/target/os/solaris_11.2/tcp/custom/output/tcp_peosf_latest/"* "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/tc/tc_tcp_peosf"

python3 "${PYROLYSE_PATH}/tools/script/midpoint/adapt_ip_address_from_os_to_midpoint_tcp_tests.py" \
-i "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/tc/tc_tcp_peosf" \
-s  "192.168.0.1" \
-d "${target_ipv4_addr}"
```

##### Detection

###### with generic scripts

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenario_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom" \
"tc/tc_tcp_peosf" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
jasonish/suricata:7.0.7' \
'-S "/ids_conf/suricata_${protocol_s}_${payload_mode}_${reassembly_options}.rules" -c "/ids_conf/suricata.yaml" -r "/target_protocol_dir/${pcap_relative_path}"' \
eve.json \
"$reassembly_options" \
icfl8b \
tcp \
1
```

###### with non generic scripts

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_suricata.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom" \
"tc/tc_tcp_peosf" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/install/install_7.0.7" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf" \
"${reassembly_options}" \
1
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenario.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_suricata_log_directory_to_json_custom.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/output/${reassembly_options}" \
"peosf" \
"tcp" \
1 \
0 \
icfl8b \
payload \
1
```

##### Extract policy

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/extract_policy_from_novak_payload.py" \
-i  "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/output/${reassembly_options}" \
-p  "peosf" \
-o "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/output/${reassembly_options}/tcp_peosf_novak.json"
```


### Zeek

##### Generate pcap

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_for_type.sh" \
"${PYROLYSE_PATH}/test_data/custom/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom/tc/tc_ipv4_pep-nf" \
00:00:00:00:00:00 \
00:00:00:00:00:01 \
4 \
192.168.0.1 \
"${target_ipv4_addr}" \
20000 \
icmp \
p-nf \
icfl8b \
"nutc" \
custom
```

##### Detection


###### with generic scripts
 
Script

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenario_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom" \
"tc/tc_ipv4_pep-nf" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:6.2.0' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" "/ids_conf/zeek_ip.zeek"' \
notice.log \
"$reassembly_options" \
icfl8b \
ipv4 \
1
```

Signature

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenario_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom" \
"tc/tc_ipv4_pep-nf" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:6.2.0' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" -s "/ids_conf/zeek_${protocol_s}_${payload_mode}_${reassembly_options}.sig"' \
signatures.log \
"$reassembly_options" \
icfl8b \
ipv4 \
1
```

###### with non generic scripts

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_zeek_script.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom" \
"tc/tc_ipv4_pep-nf" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/install/install_6.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf" \
"${reassembly_options}" \
1
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenario.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_zeek_log_directory_to_json_custom.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom/output/${reassembly_options}" \
"pep-nf" \
"ipv4" \
0 \
0 \
icfl8b \
"" \
1
```

##### Extract policy

Copy-past payload

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/extract_policy_from_novak_payload.py" \
-i  "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom/output/${reassembly_options}" \
-p  "pep-nf" \
-o "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/custom/output/ipv4_pep-nf_novak.json"
```

#### IPv6

##### Generate pcap

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/byte_time_sequence_to_pcap_for_type.sh" \
"${PYROLYSE_PATH}/test_data/custom/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/custom/byte_time_sequence" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom/tc/tc_ipv6_pep-nf" \
00:00:00:00:00:00 \
00:00:00:00:00:01 \
6 \
fe80::200:ff:fe01:1 \
"${target_ipv6_addr}" \
20000 \
icmp \
p-nf \
icfl8b \
"nutc" \
custom
```

##### Detection


###### with generic scripts
 
Script

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenario_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom" \
"tc/tc_ipv6_pep-nf" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:6.2.0' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" "/ids_conf/zeek_ip.zeek"' \
notice.log \
"$reassembly_options" \
icfl8b \
ipv6 \
1
```

Signature

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenario_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom" \
"tc/tc_ipv6_pep-nf" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:6.2.0' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" -s "/ids_conf/zeek_${protocol_s}_${payload_mode}_${reassembly_options}.sig"' \
signatures.log \
"$reassembly_options" \
icfl8b \
ipv6 \
1
```

###### with non generic scripts

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_zeek_script.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom" \
"tc/tc_ipv6_pep-nf" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/install/install_6.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf" \
"${reassembly_options}" \
1
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenario.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_zeek_log_directory_to_json_custom.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom/output/${reassembly_options}" \
"pep-nf" \
"ipv6" \
0 \
0 \
icfl8b \
"" \
1
```

##### Extract policy

Copy-past payload

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/extract_policy_from_novak_payload.py" \
-i  "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom/output/${reassembly_options}" \
-p  "pep-nf" \
-o "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/custom/output/ipv6_pep-nf_novak.json"
```

#### TCP

##### Traffic adaptation


```bash
mkdir -p "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/tc/tc_tcp_peosf"

cp -r "${PYROLYSE_PATH}/target/os/solaris_11.2/tcp/custom/output/tcp_peosf_latest/"* "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/tc/tc_tcp_peosf"

python3 "${PYROLYSE_PATH}/tools/script/midpoint/adapt_ip_address_from_os_to_midpoint_tcp_tests.py" \
-i "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/tc/tc_tcp_peosf" \
-s  "192.168.0.1" \
-d "${target_ipv4_addr}"
```

##### Detection

###### with generic scripts
 
Script

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenario_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom" \
"tc/tc_tcp_peosf" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:6.2.0' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" "/ids_conf/zeek_tcp.zeek"' \
notice.log \
"$reassembly_options" \
icfl8b \
tcp \
1
```

Signature

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenario_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom" \
"tc/tc_tcp_peosf" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:6.2.0' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" -s "/ids_conf/zeek_${protocol_s}_${payload_mode}_${reassembly_options}.sig"' \
signatures.log \
"$reassembly_options" \
icfl8b \
tcp \
1
```

###### with non generic scripts

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_zeek_script.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom" \
"tc/tc_tcp_peosf" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/install/install_6.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf" \
"${reassembly_options}" \
1
```

##### Extract payload

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenario.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_zeek_log_directory_to_json_custom.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/output/${reassembly_options}" \
"peosf" \
"tcp" \
1 \
0 \
icfl8b \
"" \
1
```



##### Extract policy

Copy-past payload

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/extract_policy_from_novak_payload.py" \
-i  "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/output/${reassembly_options}" \
-p  "peosf" \
-o "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/custom/output/tcp_peosf_novak.json"
```