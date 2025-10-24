# Adapt PCAP trace obtained with the OSes' segmentation tests to test midpoint TCP segmentation reassembly


```bash
"${PYROLYSE_PATH}/tools/script/midpoint/adapt_ip_address_tcp_tests_scenarii.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp/tc" \
"${PYROLYSE_PATH}/target/os/solaris_11.2/tcp/output" \
"192.168.56.1" \
"$target_ipv4_addr"
```
