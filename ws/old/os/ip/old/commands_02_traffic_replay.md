

# Replay generated network trace and capture targeted OS's answers


These commands must be launched inside the base machine from the Vagrantfile, i.e. after doing "vagrant ssh base" in each ./ws/$OS/ directory.


## Debian 8/Jessie

### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.11 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.11 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/debian_8/ipv4" \
4 \
14 \
2
```

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.11 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.11 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/debian_8/ipv4" \
4 \
80 \
2
```

##### UDP

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.11 and ip proto 17 and (udp and port 7 or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.11 and \(ip[4:2] = $((test_index_offset+test_index))\)' \
"${PYROLYSE_PATH}/target/os/debian_8/ipv4" \
4 \
80 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:56::11 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/debian_8/ipv6" \
6 \
80 \
2
```

## Debian 9/Stretch


cd ~/workspace


### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.37 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.37 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/debian_9/ipv4" \
4 \
80 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:56::37 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/debian_9/ipv6" \
6 \
80 \
2
```


## Debian 10/Stretch


cd ~/workspace


### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.35 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.35 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/debian_8" \
4 \
14 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:56::35 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/debian_8/ipv6" \
6 \
14 \
2
```


## Debian 11/Bullseye


cd ~/workspace


### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.33 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.33 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/debian_11/ipv4" \
4 \
80 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:56::33 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/debian_11/ipv6" \
6 \
14 \
2
```

### libvirt

```bash
./script/network_stack/replay_capture_test_sequence_scenarii_run.sh \
ens6 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fe80::200:ff:fe01:33 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = ${test_index}\)\)' \
"${PYROLYSE_PATH}/target/os/debian_11/ipv4" \
6 \
14 \
2
```











## Debian 12/Bookworm


cd ~/workspace


### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.111 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.111 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/debian_12/ipv4" \
4 \
80 \
2
```

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host ${TARGET_IPV4_ADDR} and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host ${TARGET_IPV4_ADDR} and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4" \
4 \
40 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:56::111 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/debian_12/ipv6" \
6 \
80 \
2
```




## FreeBSD 11.4


cd ~/workspace


### Virtualbox


#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.13 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.13 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/freebsd_11/ipv4" \
4 \
14 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:56::13 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/freebsd_11/ipv6" \
6 \
14 \
2
```

### libvirt

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
ens6 \
'host 192.168.56.13 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.13 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/freebsd_11/ipv4" \
4 \
14 \
2
```


## FreeBSD 13


cd ~/workspace


### Virtualbox


#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.49 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.49 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/freebsd_13/ipv4" \
4 \
80 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:56::49 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/freebsd_13/ipv6" \
6 \
80 \
2
```

### libvirt

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
ens6 \
'host 192.168.56.49 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.49 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/freebsd_13/ipv4" \
4 \
80 \
2
```


## OpenBSD 6.1


cd ~/workspace


### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.15 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.15 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/openbsd_6.1/ipv4" \
4 \
14 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:56::15 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/openbsd_6.1/ipv6" \
6 \
14 \
2
```

### libvirt

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
ens6 \
'host 192.168.56.15 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.15 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/openbsd_6.1/ipv4" \
4 \
14 \
2
```

## OpenBSD 7.2


### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.39 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.39 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/openbsd_7.2/ipv4" \
4 \
80 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:56::39 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/openbsd_7.2/ipv6" \
6 \
80 \
2
```

### libvirt

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
ens6 \
'host 192.168.56.39 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.39 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/openbsd_7.2/ipv4" \
4 \
80 \
2
```

## OpenBSD 7.4


### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.115 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.115 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/openbsd_7.4/ipv4" \
4 \
40 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:56::115 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/openbsd_7.4/ipv6" \
6 \
40 \
2
```

### libvirt

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
ens6 \
'host 192.168.56.115 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.115 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/openbsd_7.4/ipv4" \
4 \
40 \
2
```

## Solaris 11.2


cd ~/workspace


### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.117 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.117 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/solaris_11.2/ipv4" \
4 \
80 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fe80::200:ff:fe00:117 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/solaris_11.2/ipv6" \
6 \
80 \
2
```


### libvirt

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
ens6 \
'host 192.168.56.117 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.117 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/solaris_11.2/ipv4" \
4 \
80 \
2
```

## Solaris 11.4


cd ~/workspace


### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.43 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.43 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/solaris_11.4/ipv4" \
4 \
80 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fe80::200:ff:fe00:43 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/solaris_11.4/ipv6" \
6 \
80 \
2
```


### libvirt

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
ens6 \
'host 192.168.56.43 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.43 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/solaris_11.4/ipv4" \
4 \
80 \
2
```

## Windows 10


cd ~/workspace


### Virtualbox

#### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'host 192.168.56.17 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.17 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/windows_10/ipv4" \
4 \
80 \
2
```

#### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
eth1 \
'icmp6 and (ip6[40] != 0 and (ip6[40] == 128 or ip6[40] == 129) or (ip6 proto 44))' \
'host fd00:0:0:56::17 and \(ip6[2:2] = $((test_index_offset+test_index)) or \(icmp6[icmp6type] = icmp6-echoreply and icmp6[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/windows_10/ipv6" \
6 \
80 \
2
```

### libvirt

```bash
"${PYROLYSE_PATH}/script/network_stack/replay_capture_test_sequence_scenarii_run.sh" \
ens6 \
'host 192.168.56.17 and ip proto 1 and (icmp[icmptype] != icmp-timxceed or ((ip[6:2] > 0) and (not ip[6] = 64)))' \
'host 192.168.56.17 and \(ip[4:2] = $((test_index_offset+test_index)) or \(icmp[icmptype] = icmp-echoreply and icmp[4:2] = $((test_index_offset+test_index))\)\)' \
"${PYROLYSE_PATH}/target/os/windows_10/ipv4" \
4 \
80 \
2
```









