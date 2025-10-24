

# build policies


These commands must be launched from the ./ws/ directory from the pyrolyse root.


## IPv4

### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_minimal_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}" \
ipv4 \
icfl8b
```

### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_minimal_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}" \
ipv4 \
icvl8i4
```

## IPv6

### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_minimal_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}" \
ipv6 \
icfl8b
```

### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_minimal_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}" \
ipv6 \
icvl8i6
```


## TCP

### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_minimal_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}" \
tcp \
icfl8b
```

### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_minimal_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}" \
tcp \
icvl8i4
```