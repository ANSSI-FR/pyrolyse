

# Replay generated network trace and capture targeted OS's answers


These commands must be launched inside the base machine from the Vagrantfile, i.e. after doing "vagrant ssh base" in each ./ws/$OS/ directory.


## IPv4


```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'ip proto 1 and \(icmp[icmptype] != icmp-timxceed or \(\(ip[7]\>0 or ip[6] \& 63\>0\)\)\)' \
'host ${TARGET_IPV4_ADDR} and \(\(\(icmp[icmptype] = icmp-echo or icmp[icmptype] = icmp-echoreply\) and icmp[4:2] = $((test_index_offset+test_index))\) or \(ip[20] != 0 and ip[20] != 8 and ip[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4" \
4 \
40 \
2
```

## IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and \(ip6[40] != 0 and \(ip6[40] == 128 or ip6[40] == 129\) or \(ip6 proto 44\)\)' \
'host ${TARGET_IPV6_ADDR} and \(\(ip6 proto 44 and ip6[44:4] = $((test_index_offset+test_index))\) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6" \
6 \
40 \
2
```

