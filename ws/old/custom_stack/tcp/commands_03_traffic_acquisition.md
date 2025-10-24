

# Capture targeted OS's answers


These commands must be launched inside the base machine from the Vagrantfile, i.e. after doing "vagrant ssh base" in each $PYROLYSE_PATH/ws/$OS/ directory.


### uip/picotcp

### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash
"${PYROLYSE_PATH}/script/custom_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${BASE_IPV4_ADDR}" \
"${STACK_IPV4_ADDR}" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR_V4}" \
icfl8b \
1 \
1
```

### Invariant checksum for multiple reassembled payload lengths (from ipv4 based testing)

```bash
"${PYROLYSE_PATH}/script/custom_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${BASE_IPV4_ADDR}" \
"${STACK_IPV4_ADDR}" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR_V4}" \
icvl8i4 \
20 \
2
```

### Invariant checksum for multiple reassembled payload lengths (from ipv6 based testing) 

```bash
"${PYROLYSE_PATH}/script/custom_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${BASE_IPV4_ADDR}" \
"${STACK_IPV4_ADDR}" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR_V4}" \
icvl8i6 \
20 \
2
```

### lwip/seastar/smoltcp/mtcp

### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash
"${PYROLYSE_PATH}/script/custom_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${BASE_IPV4_ADDR}" \
"${APP_IPV4_ADDR}" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
icfl8b \
20 \
2
```

### Invariant checksum for multiple reassembled payload lengths (from ipv4 based testing)

```bash
"${PYROLYSE_PATH}/script/custom_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${BASE_IPV4_ADDR}" \
"${APP_IPV4_ADDR}" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
icvl8i4 \
20 \
2
```

### Invariant checksum for multiple reassembled payload lengths (from ipv6 based testing) 

```bash
"${PYROLYSE_PATH}/script/custom_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${BASE_IPV4_ADDR}" \
"${APP_IPV4_ADDR}" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
icvl8i6 \
20 \
2
```

### mirage-tcpip 9.0.0

Since mirage-tcpip does not handle well the RST terminations, we shutdown mirage-tcpip box at the end of every scenario.
The script needs to be launched with base and target boxes down. 

```bash
export PYROLYSE_PATH=/home/laubard/Projects/pyrolyse
export TARGET_NAME=mirage-tcpip_9.0.0

"${PYROLYSE_PATH}/script/custom_stack/acquire_tcp_traffic_scenarii_w_target_shutdown_miragetcpip.sh" \
"/home/vagrant/pyrolyse/target/custom_stack/mirage-tcpip_9.0.0/tcp/output" \
"/home/vagrant/pyrolyse/test_data/separated/byte_time_sequence" \
"192.168.56.200" \
"192.168.57.203" \
"00:00:00:00:02:00" \
"00:00:00:00:02:01" \
icvl8i4 \
20
```