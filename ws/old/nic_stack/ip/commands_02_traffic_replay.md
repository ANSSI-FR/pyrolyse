

# Replay generated network trace and capture targeted OS's answers


These commands must be launched inside the base machine from the Vagrantfile, i.e. after doing "vagrant ssh base" in each ./ws/$OS/ directory.

TODO : fix tcpdump filter single for IPv4 and IPv6 (see OS target)


## Xilinx Onload

### Ipv4|UDP

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth_10g \
'host ${TARGET_IPV4_ADDR} and ip proto 17 and \(udp and port 7 or \(\(ip[7]\>0 or ip[6] \& 63\>0\)\)\)' \
'host ${TARGET_IPV4_ADDR} and \(ip[4:2] = $((test_index_offset+test_index)) or \(udp and port $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4" \
4 \
80 \
2
```





"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth_10g \
'host 192.168.40.212 and ip proto 17 and (udp and port 7 or ((ip[7]>0 or ip[6] & 63>0)))' \
'host 192.168.40.212 and \(ip[4:2] = $((test_index_offset+test_index)) or \(udp and port $((10000+test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/nic_stack/xilinx_onload/ipv4" \
4 \
20 \
3





"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth_10g \
'host 192.168.40.212 and ip proto 17 and (udp and port 7 or ((ip[7]>0 or ip[6] & 63>0)))' \
'host 192.168.40.212 and \(ip[4:2] = $((test_index_offset+test_index)) or \(udp and port $((10000+test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/nic_stack/xilinx_onload/ipv4" \
4 \
10 \
3


"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth_10g \
'host 192.168.40.212 and ip proto 17 and (udp and port 7 or ((ip[7]>0 or ip[6] & 63>0)))' \
'host 192.168.40.212 and \(ip[4:2] = $((test_index_offset+test_index)) or \(udp and port $((10000+test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/nic_stack/xilinx_onload/ipv4" \
4 \
5 \
3


"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth_10g \
'host 192.168.40.212 and ip proto 17 and (udp and port 7 or ((ip[7]>0 or ip[6] & 63>0)))' \
'host 192.168.40.212 and \(ip[4:2] = $((test_index_offset+test_index)) or \(udp and port $((10000+test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/nic_stack/xilinx_onload/ipv4" \
4 \
1 \
3


TODO: 10 still yields errors => reduce number of process: 5, 2 ?




### DEBUG


"${PYROLYSE_PATH}/script/network_stack/test/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.40.212 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[7]>0 or ip[6] & 63>0)))' \
'host 192.168.40.212 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/nic_stack/xilinx_onload/ipv4_icmp" \
4 \
80 \
2


"${PYROLYSE_PATH}/script/network_stack/test/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.40.212 and ip proto 17 and (udp and port 7 or ((ip[7]>0 or ip[6] & 63>0)))' \
'host 192.168.40.212 and \(ip[4:2] = $((test_index_offset+test_index)) or \(udp and port $((10000+test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/nic_stack/xilinx_onload/ipv4_udp" \
4 \
80 \
2









#### IPv6|UDP

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth_10g \
'host ${TARGET_IPV6_ADDR} and udp or \(ip6[40] != 0 and \(ip6[40] == 128 or ip6[40] == 129\) or \(ip6 proto 44\)\)' \
'host ${TARGET_IPV6_ADDR} and \(ip6[2:2] = $((test_index_offset+test_index)) or \(udp and dst port $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/nic_stack/${TARGET_NAME}/ipv6" \
6 \
80 \
2
```


"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth_10g \
'udp or (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host ${TARGET_IPV6_ADDR} and \(ip6[2:2] = $((test_index_offset+test_index)) or \(udp and dst port $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/nic_stack/xilinx_onload/ipv6" \
6 \
200 \
3

tcpdump -r toto.pcap 'udp and port dst 21267'






### Ipv4|ICMP

```bash
"${PYROLYSE_PATH}/script/os/replay_capture_test_sequence_scenarii_run.sh" \
eth_10g \
'ip proto 1 and \(icmp[icmptype] != icmp-timxceed or \(\(ip[7]\>0 or ip[6] \& 63\>0\)\)\)' \
'host ${TARGET_IPV4_ADDR} and \(\(\(icmp[icmptype] = icmp-echo or icmp[icmptype] = icmp-echoreply\) and icmp[4:2] = $((test_index_offset+test_index))\) or \(ip[20] != 0 and ip[20] != 8 and ip[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4" \
4 \
80 \
2
```



## IPv6|ICMP

```bash
"${PYROLYSE_PATH}/script/os/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and \(ip6[40] != 0 and \(ip6[40] == 128 or ip6[40] == 129\) or \(ip6 proto 44\)\)' \
'host ${TARGET_IPV6_ADDR} and \(\(ip6 proto 44 and ip6[44:4] = $((test_index_offset+test_index))\) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6" \
6 \
40 \
2
```











