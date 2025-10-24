

# Capture targeted OS's answers


These commands must be launched inside the base machine from the Vagrantfile, i.e. after doing "vagrant ssh base" in each $PYROLYSE_PATH/target/os/$OS/ directory.


TODO: acquire_tcp_traffic.py uses 200MB of memory => clean this => probably need to manually create smaller json to avoid too much memory usage

### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
icfl8b \
30 \
2
```

### Invariant checksum for multiple reassembled payload lengths (from ipv4 based testing)

```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
icvl8i4 \
20 \
2
```


sudo RUST_LOG=DEBUG \
/home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk \
-j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/triplet_b_mi_bi.json \
-o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/chelsiotoe_3.19.0.3/tcp/output/tcp_pep-ao_20250523_132916/test_167.pcap \
-s 192.168.40.200 \
-d 192.168.40.214 \
-c pep-ao \
--tio 11000 \
--payload-mode icvl8i4 \
-i 167 \
--input-mode sbts \
--connection-end-mode mrst





sudo RUST_LOG=DEBUG \
/home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk \
-j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/triplet_b_mi_bi.json \
-o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/chelsiotoe_3.19.0.3/tcp/output/test_167.pcap \
-s 192.168.40.200 \
-d 192.168.40.214 \
-c pep-ao \
--tio 11000 \
--payload-mode icvl8i4 \
-i 167 \
--input-mode sbts \
--connection-end-mode mrst





sudo RUST_LOG=DEBUG \
/home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk \
-j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/triplet_b_mi_bi.json \
-o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/chelsiotoe_3.19.0.3/tcp/output/test_167.pcap \
--mac-src 40:a6:b7:b4:3a:d0 \
--mac-dst 00:0f:53:79:19:b0 \
-s 192.168.40.200 \
-d 192.168.40.214 \
-c pep-ao \
--tio 11000 \
--payload-mode icvl8i4 \
-i 167 \
--input-mode sbts \
--connection-end-mode mrst






### Invariant checksum for multiple reassembled payload lengths (from ipv6 based testing) 

```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
icvl8i6 \
30 \
2
```
