

# Generate PCAP trace to test IP fragmentation


## Debian 8/Jessie

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/debian_8/tc" \
00:00:00:00:00:10 \
00:00:00:00:00:11 \
192.168.56.10 \
192.168.56.11 \
4
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/debian_8/ipv4" \
00:00:00:00:00:10 \
00:00:00:00:00:11 \
4 \
192.168.56.10 \
192.168.56.11 \
icmp
```


### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/debian_8/tc" \
00:00:00:00:00:10 \
00:00:00:00:00:11 \
fd00:0:0:56::10 \
fd00:0:0:56::11 \
6
```


```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/debian_8/ipv6" \
00:00:00:00:00:10 \
00:00:00:00:00:11 \
6 \
fd00:0:0:56::10 \
fd00:0:0:56::11 \
icmp
```


## Debian 9/Stretch

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/debian_9/ipv4" \
00:00:00:00:00:36 \
00:00:00:00:00:37 \
4 \
192.168.56.36 \
192.168.56.37 \
icmp
```


### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/debian_9/ipv6" \
00:00:00:00:00:36 \
00:00:00:00:00:37 \
6 \
fd00:0:0:56::36 \
fd00:0:0:56::37 \
icmp
```


## Debian 10/Buster

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/debian_10/ipv4" \
00:00:00:00:00:34 \
00:00:00:00:00:35 \
4 \
192.168.56.34 \
192.168.56.35 \
icmp
```


### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/debian_10/ipv6" \
00:00:00:00:00:34 \
00:00:00:00:00:35 \
6 \
fd00:0:0:56::34 \
fd00:0:0:56::35 \
icmp
```


## Debian 11/Bullseye

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/debian_11/tc" \
00:00:00:00:00:32 \
00:00:00:00:00:33 \
192.168.56.32 \
192.168.56.33 \
4
```

RUST_LOG=debug \
"${PYROLYSE_PATH}/rust_code/reassembly_test_pipeline/target/debug/generate_trace_ip_fragmentation_icmp_single" \
--cp "${PYROLYSE_PATH}/test_data_single/chunk_pattern.json" \
-s "${PYROLYSE_PATH}/test_data_single/byte_time_sequence_o.json" \
-o "${PYROLYSE_PATH}/test_data_single" \
-p p \
--mac-src 00:00:00:00:00:32 \
--mac-dst 00:00:00:00:00:33 \
--ip-version 4 \
--ipv4-src 192.168.56.32 \
--ipv4-dst 192.168.56.33






### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/debian_11/tc" \
00:00:00:00:00:32 \
00:00:00:00:00:33 \
fe80::200:ff:fe01:32 \
fe80::200:ff:fe01:33 \
6
```


```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/debian_11/ipv6" \
00:00:00:00:00:32 \
00:00:00:00:00:33 \
6 \
fd00:0:0:56::32 \
fd00:0:0:56::33 \
icmp
```

## Debian 12/Bookworm -- DEPRECATED

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/debian_12/ipv4" \
00:00:00:00:01:10 \
00:00:00:00:01:11 \
4 \
192.168.56.110 \
192.168.56.111 \
icmp
```


### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/debian_12/ipv6" \
00:00:00:00:01:10 \
00:00:00:00:01:11 \
6 \
fd00:0:0:56::110 \
fd00:0:0:56::111 \
icmp
```


## Debian 12/Bookworm - Generic

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/debian_12/ipv4" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
4 \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
icmp
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
4 \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \
icmp
```


### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/debian_12/ipv6" \
"${BASE_MAC_ADDR}" \
"${TARGET_MAC_ADDR}" \
6 \
"${BASE_IPV6_ADDR}" \
"${TARGET_IPV6_ADDR}" \
icmp
```





## FreeBSD 11.4

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/freebsd_11.4/tc" \
00:00:00:00:00:12 \
00:00:00:00:00:13 \
192.168.56.12 \
192.168.56.13 \
4
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/freebsd_11.4/ipv4" \
00:00:00:00:00:12 \
00:00:00:00:00:13 \
4 \
192.168.56.12 \
192.168.56.13 \
icmp
```

### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/freebsd_11.4/tc" \
00:00:00:00:00:12 \
00:00:00:00:00:13 \
fe80::200:ff:fe01:12 \
fe80::200:ff:fe01:13 \
6
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/freebsd_11.4/ipv6" \
00:00:00:00:00:12 \
00:00:00:00:00:13 \
6 \
fd00:0:0:56::12 \
fd00:0:0:56::13 \
icmp
```

## OpenBSD 6.1

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/openbsd_6.1/tc" \
00:00:00:00:00:14 \
00:00:00:00:00:15 \
192.168.56.14 \
192.168.56.15 \
4
```

### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/openbsd_6.1/tc" \
00:00:00:00:00:14 \
00:00:00:00:00:15 \
fe80::200:ff:fe01:14 \
fe80::200:ff:fe01:15 \
6
```

## OpenBSD 7.2

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/openbsd_7.2/tc" \
00:00:00:00:00:38 \
00:00:00:00:00:39 \
192.168.56.38 \
192.168.56.39 \
4
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/openbsd_7.2/ipv4" \
00:00:00:00:00:38 \
00:00:00:00:00:39 \
4 \
192.168.56.38 \
192.168.56.39 \
icmp
```

### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/openbsd_7.2/tc" \
00:00:00:00:00:38 \
00:00:00:00:00:39 \
fd00:0:0:56::38 \
fd00:0:0:56::39 \
6
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/openbsd_7.2/ipv6" \
00:00:00:00:00:38 \
00:00:00:00:00:39 \
6 \
fd00:0:0:56::38 \
fd00:0:0:56::39 \
icmp
```

## OpenBSD 7.4

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/openbsd_7.4/tc" \
00:00:00:00:01:14 \
00:00:00:00:01:15 \
192.168.56.114 \
192.168.56.115 \
4
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/openbsd_7.4/ipv4" \
00:00:00:00:01:14 \
00:00:00:00:01:15 \
4 \
192.168.56.114 \
192.168.56.115 \
icmp
```

### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/openbsd_7.4/tc" \
00:00:00:00:01:14 \
00:00:00:00:01:15 \
fd00:0:0:56::114 \
fd00:0:0:56::115 \
6
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/openbsd_7.4/ipv6" \
00:00:00:00:01:14 \
00:00:00:00:01:15 \
6 \
fd00:0:0:56::114 \
fd00:0:0:56::115 \
icmp
```

## FreeBSD 13

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/freebsd_13/tc" \
00:00:00:00:00:48 \
00:00:00:00:00:49 \
192.168.56.48 \
192.168.56.49 \
4
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/freebsd_13/ipv4" \
00:00:00:00:00:48 \
00:00:00:00:00:49 \
4 \
192.168.56.48 \
192.168.56.49 \
icmp
```

### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/freebsd_13/tc" \
00:00:00:00:00:48 \
00:00:00:00:00:49 \
fd00:0:0:56::48 \
fd00:0:0:56::49 \
6
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/freebsd_13/ipv6" \
00:00:00:00:00:48 \
00:00:00:00:00:49 \
6 \
fd00:0:0:56::48 \
fd00:0:0:56::49 \
icmp
```

## Windows 10

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/windows_10/tc" \
00:00:00:00:00:16 \
00:00:00:00:00:17 \
192.168.56.16 \
192.168.56.17
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/windows_10/ipv4" \
00:00:00:00:00:16 \
00:00:00:00:00:17 \
4 \
192.168.56.16 \
192.168.56.17 \
icmp
```

### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/windows_10/tc" \
00:00:00:00:00:16 \
00:00:00:00:00:17 \
fe80::200:ff:fe01:16 \
fe80::200:ff:fe01:17 \
6
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/windows_10/ipv6" \
00:00:00:00:00:16 \
00:00:00:00:00:17 \
6 \
fd00:0:0:56::16 \
fd00:0:0:56::17 \
icmp
```

## Solaris 10

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/solaris_10/tc" \
00:00:00:00:00:46 \
00:00:00:00:00:47 \
192.168.56.46 \
192.168.56.47
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/solaris_10/ipv4" \
00:00:00:00:00:46 \
00:00:00:00:00:47 \
4 \
192.168.56.46 \
192.168.56.47 \
icmp
```

### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/solaris_10/tc" \
00:00:00:00:00:46 \
00:00:00:00:00:47 \
fe80::200:ff:fe01:46 \
fe80::200:ff:fe01:47 \
6
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/solaris_10/ipv6" \
00:00:00:00:00:46 \
00:00:00:00:00:47 \
6 \
fd00:0:0:56::46 \
fd00:0:0:56::47 \
icmp
```

## Solaris 11.2

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/solaris_11.2/tc" \
00:00:00:00:01:16 \
00:00:00:00:01:17 \
192.168.56.116 \
192.168.56.117
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/solaris_11.2/ipv4" \
00:00:00:00:01:16 \
00:00:00:00:01:17 \
4 \
192.168.56.116 \
192.168.56.117 \
icmp
```

### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/solaris_11.2/tc" \
00:00:00:00:01:16 \
00:00:00:00:01:17 \
fe80::200:ff:fe00:116 \
fe80::200:ff:fe00:117 \
6
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/solaris_11.2/ipv6" \
00:00:00:00:01:16 \
00:00:00:00:01:17 \
6 \
fe80::200:ff:fe00:116 \
fe80::200:ff:fe00:117 \
icmp
```

## Solaris 11.4

### IPv4

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/solaris_11.4/tc" \
00:00:00:00:00:42 \
00:00:00:00:00:43 \
192.168.56.42 \
192.168.56.43
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/solaris_11.4/ipv4" \
00:00:00:00:00:42 \
00:00:00:00:00:43 \
4 \
192.168.56.42 \
192.168.56.43 \
icmp
```

### IPv6

```bash
"${PYROLYSE_PATH}/script/network_stack/generate_trace_fragmentation_icmp_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/solaris_11.4/tc" \
00:00:00:00:00:42 \
00:00:00:00:00:43 \
fe80::200:ff:fe00:42 \
fe80::200:ff:fe00:43 \
6
```

```bash
"${PYROLYSE_PATH}/script/network_stack/byte_time_sequence_to_pcap_scenarii.sh" \
"${PYROLYSE_PATH}/test_data_separated/internet_checksum_chunk_pattern.json" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${PYROLYSE_PATH}/target/os/solaris_11.4/ipv6" \
00:00:00:00:00:42 \
00:00:00:00:00:43 \
6 \
fe80::200:ff:fe00:42 \
fe80::200:ff:fe00:43 \
icmp
```