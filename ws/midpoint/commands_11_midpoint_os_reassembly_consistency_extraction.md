# IPv4

## from payload

### Suricata

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_os_reassembly_consistency_scenarii.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/output/${reassembly_options}" \
"${PYROLYSE_PATH}/target/os/${target_os}/ipv4/output/" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${target_os}" \
"payload.json" \
"ipv4" \
"true"
```

### Snort/Zeek

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_os_reassembly_consistency_scenarii.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/output/${reassembly_options}" \
"${PYROLYSE_PATH}/target/os/${target_os}/ipv4/output/" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${target_os}" \
"payload.json" \
"ipv4" \
"false"
```

## from simple_policy

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_os_reassembly_consistency_scenarii.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4/output/${reassembly_options}" \
"${PYROLYSE_PATH}/target/os/${target_os}/ipv4/output/" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${target_os}" \
"policy_simple.json" \
"ipv4" \
"true"
```

# TCP

## from payload

### Suricata

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_os_reassembly_consistency_scenarii.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/output/${reassembly_options}" \
"${PYROLYSE_PATH}/target/os/${target_os}/tcp/output/" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${target_os}" \
"payload.json" \
"tcp" \
"false"
```

### Snort/Zeek

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_os_reassembly_consistency_scenarii.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/output/${reassembly_options}" \
"${PYROLYSE_PATH}/target/os/${target_os}/tcp/output/" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${target_os}" \
"payload.json" \
"tcp" \
"false"
```

## from simple_policy

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_os_reassembly_consistency_scenarii.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/output/${reassembly_options}" \
"${PYROLYSE_PATH}/target/os/${target_os}/tcp/output/" \
"${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json" \
"${target_os}" \
"policy_simple.json" \
"tcp" \
"true"
```