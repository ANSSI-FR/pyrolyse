

# Replay generated network trace and capture targeted OS's answers


These commands must be launched inside the base machine from the Vagrantfile, i.e. after doing "vagrant ssh base".

TODO : fix tcpdump filter single for IPv4 and IPv6 (see OS target)

## trash

NB: use "vagrant reload target" before each run to ensure that isolated fragments do not remain anywhere


## lwip 2.1.2


### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.57.23 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[7]>0 or ip[6] & 63>0)))' \
'host 192.168.57.23 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/lwip_2.1.2/ipv4" \
4 \
20 \
2
```

NB: we use 40 process because 80 yield some inconsistency


#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:57::23 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/lwip_2.1.2/ipv6" \
6 \
20 \
2
```


### libvirt


```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
ens6 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:57::23 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/lwip_2.1.2/ipv6" \
6 \
20 \
2
```




## uip 1.0


### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.57.27 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[7]>0 or ip[6] & 63>0)))' \
'host 192.168.57.27 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/uip_1.0/ipv4" \
4 \
1 \
2
```

NB: 20 is still inconsistent


#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:57::30 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/uip_1.0/ipv6" \
6 \
20 \
2
```



## picotcp 1.7.0

### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.57.103 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[7]>0 or ip[6] & 63>0)))' \
'host 192.168.57.103 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/picotcp_1.7.0/ipv4" \
4 \
80 \
2
```

sudo tcpreplay -i eth1 --pps=10 "${PYROLYSE_PATH}/target/custom_stack/picotcp_1.7.0/ipv4/tc/tc_ipv4_pep/test_4.pcap"


sudo tcpdump -i eth1 -c 10 -w pyrolyse/picotcp_trucated.pcap


sudo tcpdump -i tap0 -c 10 -w pyrolyse/picotcp_trucated.pcap


sudo tcpreplay -i eth1 --pps=10 "${PYROLYSE_PATH}/target/custom_stack/picotcp_1.7.0/ipv4/tc/tc_ipv4_pep/test_8.pcap"







#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:57::106 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/picotcp_1.7.0/ipv6" \
6 \
80 \
1
```


### libvirt


```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
ens6 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:57::106 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/picotcp_1.7.0/ipv6" \
6 \
80 \
2
```


## New

### uip/picotcp

#### IPv4


```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[7]>0 or ip[6] & 63>0)))' \
'host ${STACK_IPV4_ADDR} and \(\(\(icmp[icmptype] = icmp-echo or icmp[icmptype] = icmp-echoreply\) and icmp[4:2] = $((test_index_offset+test_index))\) or \(ip[20] != 0 and ip[20] != 8 and ip[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv4" \
4 \
20 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host ${STACK_IPV6_ADDR} and \(\(ip6 proto 44 and ip6[44:4] = $((test_index_offset+test_index))\) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv6" \
6 \
20 \
2
```


### lwip/seastar/smoltcp/mirage-tcpip/mtcp

#### IPv4


```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'ip proto 1 and \(icmp[icmptype] != icmp-timxceed or \(\(ip[7]>0 or ip[6] & 63>0\)\)\)' \
'host ${APP_IPV4_ADDR} and \(\(\(icmp[icmptype] = icmp-echo or icmp[icmptype] = icmp-echoreply\) and icmp[4:2] = $((test_index_offset+test_index))\) or \(ip[20] != 0 and ip[20] != 8 and ip[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv4" \
4 \
20 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host ${APP_IPV6_ADDR} and \(\(ip6 proto 44 and ip6[44:4] = $((test_index_offset+test_index))\) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv6" \
6 \
20 \
2
```

### Non default testing 

#### mtcp UDP 

##### IPv4


```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host ${APP_IPV4_ADDR} and ip proto 17 and (udp and port 7 or ((ip[7]>0 or ip[6] & 63>0)))' \
'host ${APP_IPV4_ADDR} and \(ip[4:2] = $((test_index_offset+test_index)) or \(udp and port $((10000+test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv4" \
4 \
20 \
2
```

##### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'udp or (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host ${APP_IPV6_ADDR} and \(ip6[2:2] = $((test_index_offset+test_index)) or \(udp and dst port $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/custom_stack/${TARGET_NAME}/ipv6" \
6 \
20 \
2
```