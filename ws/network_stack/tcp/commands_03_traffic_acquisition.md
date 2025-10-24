

# Capture targeted OS's answers


These commands must be launched inside the base machine from the Vagrantfile, i.e. after doing "vagrant ssh base" in each ${PYROLYSE_PATH}/target/${TARGET_FAMILY}/$OS/ directory.


TODO: acquire_tcp_traffic.py uses 200MB of memory => clean this => probably need to manually create smaller json to avoid too much memory usage

## IPv4/TCP

### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash
"${PYROLYSE_PATH}/tools/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"4" \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
icfl8b \
30 \
2
```

### Invariant checksum for multiple reassembled payload lengths (from ipv4 based testing)

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"4" \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
icvl8i4 \
20 \
2
```


### Invariant checksum for multiple reassembled payload lengths (from ipv6 based testing) 

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"4" \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
icvl8i6 \
30 \
2
```

## IPv6/TCP

### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash
"${PYROLYSE_PATH}/tools/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"6" \
"${BASE_IPV6_ADDR}" \
"${TARGET_IPV6_ADDR}" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
icfl8b \
30 \
2
```

### Invariant checksum for multiple reassembled payload lengths (from ipv4 based testing)

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"6" \
"${BASE_IPV6_ADDR}" \
"${TARGET_IPV6_ADDR}" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
icvl8i4 \
20 \
2
```


### Invariant checksum for multiple reassembled payload lengths (from ipv6 based testing) 

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"6" \
"${BASE_IPV6_ADDR}" \
"${TARGET_IPV6_ADDR}" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
icvl8i6 \
30 \
2
```