

# Capture targeted OS's answers


These commands must be launched inside a machine that can reach the NIC.


```
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
icvl8i4 \
40 \
2
```


##### Process number

* Xilinx Onload 9.0.1 (SFN8522-PLUS)
  - 40 processes : ok

* Chelsio TOE 3.19.0.3
  - 40 processes => not working


NB: we use 10 processes because bigger number yield timeout after SYN.



timeout -s SIGTERM 15



BUG
* peospef-ap ok si parallel --delay 0.5 (pas ok : 0.2, ok : 0.3)





## DEBUG


### Chelsio



#### pair_mi


sudo RUST_LOG=DEBUG /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk \
-j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/pair_mi.json \
-o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/chelsiotoe_3.19.0.3/tcp/output/test_7_10000_0.pcap \
-s 192.168.40.200 \
-d 192.168.40.214 \
-c pep-ap \
--tio 10000 \
--payload-mode icvl8i4 \
-i 7 \
--input-mode sbts \
--connection-end-mode fhs



sudo RUST_LOG=DEBUG /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk \
-j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/pair_mi.json \
-o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/chelsiotoe_3.19.0.3/tcp/output/test_7_20000_0.pcap \
-s 192.168.40.200 \
-d 192.168.40.214 \
-c pep-ap \
--tio 20000 \
--payload-mode icvl8i4 \
-i 7 \
--input-mode sbts \
--connection-end-mode fhs











#### pair_m


sudo RUST_LOG=DEBUG /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk \
-j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/pair_m.json \
-o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/chelsiotoe_3.19.0.3/tcp/output/test_8_10000_0.pcap \
-s 192.168.40.200 \
-d 192.168.40.214 \
-c pep-ap \
--tio 10000 \
--payload-mode icvl8i4 \
-i 8 \
--input-mode sbts




sudo RUST_LOG=DEBUG /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk \
-j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/pair_m.json \
-o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/chelsiotoe_3.19.0.3/tcp/output/test_8_20007_0.pcap \
-s 192.168.40.200 \
-d 192.168.40.214 \
-c pep-ap \
--tio 20008 \
--payload-mode icvl8i4 \
-i 8 \
--input-mode sbts


sudo RUST_LOG=DEBUG /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk \
-j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/pair_m.json \
-o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/chelsiotoe_3.19.0.3/tcp/output/test_7_20008_1.pcap \
-s 192.168.40.200 \
-d 192.168.40.214 \
-c pep-ap \
--tio 20008 \
--payload-mode icvl8i4 \
-i 8 \
--input-mode sbts


sudo RUST_LOG=DEBUG /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk \
-j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/pair_m.json \
-o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/chelsiotoe_3.19.0.3/tcp/output/test_7_20008_2.pcap \
-s 192.168.40.200 \
-d 192.168.40.214 \
-c pep-ap \
--tio 20008 \
--payload-mode icvl8i4 \
-i 8 \
--input-mode sbts










### Xilinx


sudo RUST_LOG=DEBUG /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk \
-j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/pair_m.json \
-o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/xilinx_onload/tcp/output/test_8.pcap \
-s 192.168.40.200 \
-d 192.168.40.212 \
-c pep-ap \
--tio 10008 \
--payload-mode icvl8i4 \
-i 8 \
--input-mode sbts




sudo RUST_LOG=DEBUG /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk -j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/triplet_d_d_s.json -o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/xilinx_onload/tcp/output/tcp_pep-ap_20250317_110717/test_201.pcap -s 192.168.40.200 -d 192.168.40.212 -c pep-ap --tio 10000 --payload-mode icvl8i4 -i 201 --input-mode sbts


sudo RUST_LOG=DEBUG /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk -j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/triplet_m_di_bi.json -o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/xilinx_onload/tcp/output/tcp_peosp-ap_20250317_161600/test_335.pcap -s 192.168.40.200 -d 192.168.40.212 -c peosp-ap --tio 10000 --payload-mode icvl8i4 -i 335 --input-mode sbts


sudo RUST_LOG=DEBUG /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk -j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/triplet_o_di_si.json -o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/xilinx_onload/tcp/output/tcp_peosp-ap_20250317_131905/test_389.pcap -s 192.168.40.200 -d 192.168.40.212 -c peosp-ap --tio 10000 --payload-mode icvl8i4 -i 389 --input-mode sbts


sudo RUST_LOG=DEBUG /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/rust_code/reassembly_test_pipeline/target/debug/send_tcp_chunk -j /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/test_data/separated/byte_time_sequence/triplet_oi_si_o.json -o /home/jmazel/ssd0/research/experimentations/protocol_reassembly/policy/pyrolyse/target/nic_stack/xilinx_onload/tcp/output/tcp_pep-ao_20250317_172442/test_435.pcap -s 192.168.40.200 -d 192.168.40.212 -c pep-ao --tio 10000 --payload-mode icvl8i4 -i 435 --input-mode sbts






ls -lht **/*.pcap | grep -a test_145 | grep -v latest



ls -lht **/*.pcap 


cat scenario_peospef-ap.log | cut -d$'\t' -f1


cat scenario_peospef-ap.log | cut -d$'\t' -f1 | sort -n | tail -n +2 | head -n 420





## Chelsio ChelsioUwire 3.18.0.0 (520-CR)

NB: we use 10 processes because bigger number yield timeout after SYN.

```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/nic_stack/chelsio_toe/tcp" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
192.168.40.200 \
192.168.40.214 \
10 \
2
```


```
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp/output" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
icvl8i4 \
40 \
2
```


```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/nic_stack/xilinx_onload/tcp" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
192.168.40.200 \
192.168.40.212 \
100 \
3
```



### DEBUG


sudo tcpdump -i eth1 "${PYROLYSE_PATH}/target/nic_stack/chelsio_toe/toto.pcap"



tcp.port == 10100 || udp.port == 10100










