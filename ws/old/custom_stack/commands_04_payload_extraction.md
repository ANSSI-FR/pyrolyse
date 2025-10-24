

# extract payload


These commands must be launched from the ./ws/ directory from the pyrolyse root.


## IPv4

### check consistency across run

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv4/output" \
ipv4
```

### extract latest one

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv4/output" \
ipv4
```

### display sums

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv4/output"
```

### display inconsistent scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum_inconsistent.py" \
-p "${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv4/output"
```

### build artificial payloads 

Can be run if struggle to find reassembly consistency across the runs.

```bash
"${PYROLYSE_PATH}/tools/script/custom_stack/build_artificial_reassembled_payload_scenarii.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv4/output" \
ipv4
```

## IPv6

### check consistency across run

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv6/output" \
ipv6
```

### extract latest one

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv6/output" \
ipv6
```

### display sums

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv6/output"
```

### display inconsistent scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum_inconsistent.py" \
-p "${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv6/output"
```

## TCP

### check consistency across run

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/tcp/output" \
tcp
```

### extract latest one

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/tcp/output" \
tcp
```

### display sums

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/tcp/output"
```

### display inconsistent scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum_inconsistent.py" \
-p "${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/tcp/output"
```