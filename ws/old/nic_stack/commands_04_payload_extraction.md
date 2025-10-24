

# extract payload


These commands must be launched from the ./ws/ directory from the pyrolyse root.





## IPv4

### extract run payload

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4/output" \
ipv4
```

python3 /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/script/os/extract_stack_payload_from_directory_to_json.py -i /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/xilinxonload_9.0.1/ipv4/output/ipv4_pep-of_20250701_154745 -j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/xilinxonload_9.0.1/ipv4/output/ipv4_pep-of_payload_20250701_154745.json -r 0 -o 11000

### extract latest one

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4/output" \
ipv4
```

### hash display

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4/output"
```

### hash consistency

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum_inconsistent.py" \
-p "${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4/output"
```

### payload archive creation

```bash
tar czf \
${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4/ipv4_payload.tar.gz \
${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4/output/*.json
```

### pcap and payload archive creation

```bash
tar czf \
${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4/ipv4_all.tar.gz \
${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4/output/*
```


## IPv6

### extract run payload

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6/output" \
ipv6
```

### extract latest one

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6/output" \
ipv6
```

### hash display

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6/output"
```

### hash consistency

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum_inconsistent.py" \
-p "${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6/output"
```

### archive creation

```bash
tar czf \
${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6/ipv6_payload.tar.gz \
${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6/output/*.json
```


## TCP

### check consistency across run

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output" \
tcp
```

### extract latest one

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output" \
tcp
```

### hash display

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output"
```

### hash consistency

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum_inconsistent.py" \
-p "${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output"
```

### archive creation

```bash
tar czf \
${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/tcp_payload_json.tar.gz \
${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output/*.json
```

### pcap and payload archive creation

```bash
tar czf \
${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/tcp_all.tar.gz \
${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output/*
```
