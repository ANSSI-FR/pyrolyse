

# extract payload

These commands must be launched inside the base machine from the Vagrantfile, i.e. after doing "vagrant ssh base" in each ./ws/$OS/ directory.

## IPv4

### check consistency across run

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output" \
ipv4
```

### extract latest one

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output" \
ipv4
```

### hash display

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum.sh" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output"
```

### hash consistency

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum_inconsistent.py" \
-p "${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4/output"
```


## IPv6

### check consistency across run

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output" \
ipv6
```

### extract latest one

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output" \
ipv6
```

### display

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum.sh" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output"
```

### display inconsistent scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/network_stack/display_sha256sum_inconsistent.py" \
-p "${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output"
```


## TCP

### check consistency across run

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/output" \
tcp
```

### extract latest one

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_scenarii_latest.sh" \
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_payload_from_directory_to_json.py" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/output" \
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



