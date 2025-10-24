

# IDS

## Zeek

### zeek_signatures


https://docs.zeek.org/en/master/frameworks/signatures.html

Content conditions are defined by regular expressions. We differentiate two kinds of content conditions: first, the expression may be declared with the payload statement, in which case it is matched against the raw payload of a connection (for reassembled TCP streams) or of each packet (for ICMP, UDP, and non-reassembled TCP). Second, it may be prefixed with an analyzer-specific label, in which case the expression is matched against the data as extracted by the corresponding analyzer.

We use a signature to match of the first chunk piece/pattern and the full payload is present in signatures.log after.


#### IPv4

##### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:7.1.1' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" -s "/ids_conf/zeek_${protocol_s}_${payload_mode}_${reassembly_options}.sig"' \
signatures.log \
"$reassembly_options" \
ipv4 \
icfl8b \
20 \
1
```


##### Invariant checksum for multiple reassembled payload lengths 

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:7.1.1' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" -s "/ids_conf/zeek_${protocol_s}_${payload_mode}_${reassembly_options}.sig"' \
signatures.log \
"$reassembly_options" \
ipv4 \
icvl8i4 \
20 \
1
```

#### IPv6

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:7.1.1' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" -s "/ids_conf/zeek_${protocol_s}_${payload_mode}_${reassembly_options}.sig"' \
signatures.log \
"$reassembly_options" \
ipv6 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:7.1.1' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" -s "/ids_conf/zeek_${protocol_s}_${payload_mode}_${reassembly_options}.sig"' \
signatures.log \
"$reassembly_options" \
ipv6 \
icvl8i6 \
20 \
1
```


#### TCP

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:7.1.1' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" -s "/ids_conf/zeek_${protocol_s}_${payload_mode}_${reassembly_options}.sig"' \
signatures.log \
"$reassembly_options" \
tcp \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:7.1.1' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" -s "/ids_conf/zeek_${protocol_s}_${payload_mode}_${reassembly_options}.sig"' \
signatures.log \
"$reassembly_options" \
tcp \
icvl8i4 \
20 \
1
```

### zeek_script

#### IPv4 

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:7.1.1' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" "/ids_conf/zeek_ip.zeek"' \
notice.log \
"$reassembly_options" \
ipv4 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:7.1.1' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" "/ids_conf/zeek_ip.zeek"' \
notice.log \
"$reassembly_options" \
ipv4 \
icvl8i4 \
20 \
1
```

#### IPv6

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:7.1.1' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" "/ids_conf/zeek_ip.zeek"' \
notice.log \
"$reassembly_options" \
ipv6 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:7.1.1' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" "/ids_conf/zeek_ip.zeek"' \
notice.log \
"$reassembly_options" \
ipv6 \
icvl8i6 \
20 \
1
```

#### TCP

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:7.1.1' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" "/ids_conf/zeek_tcp.zeek"' \
notice.log \
"$reassembly_options" \
tcp \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
zeek/zeek:7.1.1' \
'zeek -r "/target_protocol_dir/${pcap_relative_path}" "/ids_conf/zeek_tcp.zeek"' \
notice.log \
"$reassembly_options" \
tcp \
icvl8i4 \
20 \
1
```

### Suricata 


#### IPv4

##### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
jasonish/suricata:7.0.9' \
'-S "/ids_conf/suricata_${protocol_s}_${payload_mode}_${reassembly_options}.rules" -c "/ids_conf/suricata.yaml" -r "/target_protocol_dir/${pcap_relative_path}"' \
eve.json \
"$reassembly_options" \
ipv4 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
'sudo docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
jasonish/suricata:7.0.9' \
'-S "/ids_conf/suricata_${protocol_s}_${payload_mode}_${reassembly_options}.rules" -c "/ids_conf/suricata.yaml" -r "/target_protocol_dir/${pcap_relative_path}"' \
eve.json \
"$reassembly_options" \
ipv4 \
icvl8i4 \
20 \
1
```

#### IPv6

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
jasonish/suricata:7.0.9' \
'-S "/ids_conf/suricata_${protocol_s}_${payload_mode}_${reassembly_options}.rules" -c "/ids_conf/suricata.yaml" -r "/target_protocol_dir/${pcap_relative_path}"' \
eve.json \
"$reassembly_options" \
ipv6 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
jasonish/suricata:7.0.9' \
'-S "/ids_conf/suricata_${protocol_s}_${payload_mode}_${reassembly_options}.rules" -c "/ids_conf/suricata.yaml" -r "/target_protocol_dir/${pcap_relative_path}"' \
eve.json \
"$reassembly_options" \
ipv6 \
icvl8i6 \
20 \
1
```

#### TCP

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
jasonish/suricata:7.0.9' \
'-S "/ids_conf/suricata_${protocol_s}_${payload_mode}_${reassembly_options}.rules" -c "/ids_conf/suricata.yaml" -r "/target_protocol_dir/${pcap_relative_path}"' \
eve.json \
"$reassembly_options" \
tcp \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp" \
'docker run --rm \
-v ${target_directory_path}/:/target_protocol_dir \
-v ${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf/:/ids_conf \
-v ${ids_log_tmp_directory_path_current}/:/work \
-w /work \
jasonish/suricata:7.0.9' \
'-S "/ids_conf/suricata_${protocol_s}_${payload_mode}_${reassembly_options}.rules" -c "/ids_conf/suricata.yaml" -r "/target_protocol_dir/${pcap_relative_path}"' \
eve.json \
"$reassembly_options" \
tcp \
icvl8i4 \
20 \
1
```

## Snort

### IPv4

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
'LD_LIBRARY_PATH="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.7.1.0/bin/snort"' \
'--daq-dir "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/daq/" \
-r "${pcap_path}" \
-c "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort.lua" \
-R "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort_${protocol_s}_${payload_mode}_${reassembly_options}.rules" \
-l "${ids_log_tmp_directory_path_current}" \
-d' \
alert_json.txt \
"$reassembly_options" \
ipv4 \
icfl8b \
20 \
1
```


##### Invariant checksum for multiple reassembled payload lengths 

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
'LD_LIBRARY_PATH="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.7.1.0/bin/snort"' \
'--daq-dir "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/daq/" \
-r "${pcap_path}" \
-c "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort.lua" \
-R "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort_${protocol_s}_${payload_mode}_${reassembly_options}.rules" \
-l "${ids_log_tmp_directory_path_current}" \
-d' \
alert_json.txt \
"$reassembly_options" \
ipv4 \
icvl8i4 \
20 \
1
```

### IPv6


##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
'LD_LIBRARY_PATH="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.7.1.0/bin/snort"' \
'--daq-dir "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/daq/" \
-r "${pcap_path}" \
-c "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort.lua" \
-R "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort_${protocol_s}_${payload_mode}_${reassembly_options}.rules" \
-l "${ids_log_tmp_directory_path_current}" \
-d' \
alert_json.txt \
"$reassembly_options" \
ipv6 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
'LD_LIBRARY_PATH="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.7.1.0/bin/snort"' \
'--daq-dir "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/daq/" \
-r "${pcap_path}" \
-c "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort.lua" \
-R "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort_${protocol_s}_${payload_mode}_${reassembly_options}.rules" \
-l "${ids_log_tmp_directory_path_current}" \
-d' \
alert_json.txt \
"$reassembly_options" \
ipv6 \
icvl8i6 \
20 \
1
```

### TCP

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp" \
'LD_LIBRARY_PATH="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.7.1.0/bin/snort"' \
'--daq-dir "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/daq/" \
-r "${pcap_path}" \
-c "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort.lua" \
-R "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort_${protocol_s}_${payload_mode}_${reassembly_options}.rules" \
-l "${ids_log_tmp_directory_path_current}" \
-d' \
alert_json.txt \
"$reassembly_options" \
tcp \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash 
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run_generic.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp" \
'LD_LIBRARY_PATH="${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.7.1.0/bin/snort"' \
'--daq-dir "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_libdaq/lib/daq/" \
-r "${pcap_path}" \
-c "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort.lua" \
-R "${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf/snort_${protocol_s}_${payload_mode}_${reassembly_options}.rules" \
-l "${ids_log_tmp_directory_path_current}" \
-d' \
alert_json.txt \
"$reassembly_options" \
tcp \
icvl8i4 \
20 \
1
```