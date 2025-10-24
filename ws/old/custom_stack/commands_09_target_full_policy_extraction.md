

# build full policies (i.e. prefered overlapping chunk data + extra capabilities)


These commands must be launched inside the base machine from the Vagrantfile, i.e. after doing "vagrant ssh base" in each ./ws/$CUSTOM_STACK/ directory.


## IPv4

### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_ip_full_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}" \
icfl8b \
4
```

### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_ip_full_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}" \
icvl8i4 \
4
```

## IPv6

### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_ip_full_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}" \
icfl8b \
6
```

### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_ip_full_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}" \
icvl8i6 \
6
```

## TCP

### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_tcp_full_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}" \
icfl8b
```

### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_tcp_full_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}" \
icvl8i4
```