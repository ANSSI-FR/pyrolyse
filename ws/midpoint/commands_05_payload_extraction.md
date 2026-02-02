



# extract payload


## Zeek


### check consistency across run

#### IPv4


```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_zeek_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/output/${reassembly_options}" \
ipv4 \
"icvl8i4" \
"" \
20
```


#### IPv6

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_zeek_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/output/${reassembly_options}" \
ipv6 \
"icvl8i6" \
"" \
20
```

sha256sum ${target_reassembly_policy}/*payload*.json | sort -k2

#### TCP

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_zeek_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/output/${reassembly_options}" \
tcp \
"icvl8i4" \
"" \
20
```


### extract latest one


#### IPv4

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_zeek_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/output/${reassembly_options}" \
ipv4 \
"icvl8i4" \
""
```

#### IPv6

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_zeek_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/output/${reassembly_options}" \
ipv6 \
"icvl8i4" \
""
```


sha256sum ${target_reassembly_policy}/*payload*.json | sort -k2


#### TCP

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_zeek_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/output/${reassembly_options}" \
tcp \
"icvl8i4" \
""
```



## Suricata

### check consistency across run

#### IPv4


```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_suricata_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/output/${reassembly_options}" \
ipv4 \
20
```

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_suricata_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/output/${reassembly_options}" \
ipv4 \
"icvl8i4" \
"payload" \
20
```

#### IPv6

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_suricata_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/output/${reassembly_options}" \
ipv6 \
"icvl8i6" \
"payload" \
20
```

sha256sum "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/*payload*.json" | sort -k2

#### TCP

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_suricata_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/output/${reassembly_options}" \
tcp \
"icvl8i4" \
"payload" \
20
```

### extract latest one


#### IPv4

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_suricata_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/output/${reassembly_options}" \
ipv4 \
"icvl8i4" \
"payload"
```

#### IPv6

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_suricata_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/output/${reassembly_options}" \
ipv6 \
"icvl8i6" \
"payload"
```


sha256sum "${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/*payload*.json" | sort -k2

#### TCP

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_suricata_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/output/${reassembly_options}" \
tcp \
"icvl8i4" \
"payload" \
20
```

## Snort

### check consistency across run

#### IPv4

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_snort_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/output/${reassembly_options}" \
ipv4 \
"icvl8i4" \
"alert_fast" \
20
```

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_snort_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/output/${reassembly_options}" \
ipv4 \
"icvl8i4" \
"alert_json" \
20
```

#### IPv6


```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_snort_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/output/${reassembly_options}" \
ipv6 \
"icvl8i6" \
"alert_fast" \
20
```

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_snort_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/output/${reassembly_options}" \
ipv6 \
"icvl8i6" \
"alert_json" \
20
```

#### TCP


```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_snort_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/output/${reassembly_options}" \
tcp \
"icvl8i4" \
"alert_fast" \
20
```

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_snort_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/output/${reassembly_options}" \
tcp \
"icvl8i4" \
"alert_json" \
20
```


### extract latest one


#### IPv4

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_snort_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/output/${reassembly_options}" \
ipv4 \
"icvl8i4" \
"alert_json"
```

#### IPv6


```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_snort_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6/output/${reassembly_options}" \
ipv6 \
"icvl8i6" \
"alert_json"
```

#### TCP


```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_payload_from_snort_log_directory_to_json.py" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/output/${reassembly_options}" \
tcp \
"icfiicvl8i4cvl8i48b" \
"alert_json"
```
