

rm -rf **/output_ipv4_*
rm -rf **/output_ipv6_*
rm -rf **/output_tcp_*
rm -rf **/suricata_tmp_*
rm -rf **/*.log



rm -rf **/*.json


## DEBUG



suricata \
-r suricata_windows/ip_fragmentation_icmp_pep_pcap/test_257.pcap \
-c conf/suricata/suricata.yaml \
-S conf/suricata/suricata.rules \
-l suricata_log_tmp





suricata \
-r suricata_windows/ip_fragmentation_icmp_pep_pcap/test_323.pcap \
-c conf/suricata/suricata.yaml \
-S conf/suricata/suricata.rules \
-l suricata_log_tmp \
--engine-analysis






### zeek - signatures


LD_LIBRARY_PATH=../ids_code/zeek_base/install/install_3.2.0/lib/ $EXPERIMENTATIONS_PATH/tcp_midpoint_analysis/implementation_fsm_testing/ids_testing/ids_code/zeek_base/install/install_3.2.0/bin/zeek \
-r suricata_windows/ip_fragmentation_icmp_pep_pcap/test_257.pcap \
-s conf/zeek/icmp_overlap_first_chunk_piece.sig



LD_LIBRARY_PATH=../ids_code/zeek_base/install/install_3.2.0/lib/ $EXPERIMENTATIONS_PATH/tcp_midpoint_analysis/implementation_fsm_testing/ids_testing/ids_code/zeek_base/install/install_3.2.0/bin/zeek \
-r suricata_windows/ip_fragmentation_icmp_pep_pcap/test_257.pcap \
-s conf/zeek/icmp_overlap_all.sig



LD_LIBRARY_PATH=../ids_code/zeek_base/install/install_3.2.0/lib/ $EXPERIMENTATIONS_PATH/tcp_midpoint_analysis/implementation_fsm_testing/ids_testing/ids_code/zeek_base/install/install_3.2.0/bin/zeek \
-r suricata_windows/ip_fragmentation_icmp_pep_pcap/test_257.pcap \
-s conf/zeek/icmp_overlap_mod.sig



in both cases (icmp_overlap.sig and icmp_overlap_mod.sig) everything chunk piece is not flagged, so I guess that fragmented payload is reassembled






### zeek - script




bro \
-r suricata_windows/ip_fragmentation_icmp_pep_pcap/test_323.pcap \
conf/zeek/http_payload.bro


bro \
-r suricata_windows/ip_fragmentation_icmp_pep_pcap/test_323.pcap \
-s conf/zeek/http_payload_from_client.sig -s conf/zeek/http_payload_from_server.sig





bro \
-r suricata_windows/ip_fragmentation_icmp_pep_pcap/test_1.pcap \
conf/zeek/http_payload.bro


bro \
-r suricata_windows/ip_fragmentation_icmp_pep_pcap/test_1.pcap \
-s conf/zeek/http_payload_from_client.sig -s conf/zeek/http_payload_from_server.sig





LD_LIBRARY_PATH=../ids_code/zeek_base/install/install_3.2.0/lib/ $EXPERIMENTATIONS_PATH/tcp_midpoint_analysis/implementation_fsm_testing/ids_testing/ids_code/zeek_base/install/install_3.2.0/bin/zeek \
-r suricata_windows/ip_fragmentation_icmp_pep_pcap/test_323.pcap \
-s conf/zeek/http_payload_from_client.sig -s conf/zeek/http_payload_from_server.sig





LD_LIBRARY_PATH=../ids_code/zeek_base/install/install_3.2.0/lib/ $EXPERIMENTATIONS_PATH/tcp_midpoint_analysis/implementation_fsm_testing/ids_testing/ids_code/zeek_base/install/install_3.2.0/bin/zeek \
-r suricata_windows/ip_fragmentation_icmp_pep_pcap/test_257.pcap \
conf/zeek/icmp.zeek









## Setup

zeek and suricata executable must be in $PATH


## IDS

### Zeek

### zeek_signatures


https://docs.zeek.org/en/master/frameworks/signatures.html

Content conditions are defined by regular expressions. We differentiate two kinds of content conditions: first, the expression may be declared with the payload statement, in which case it is matched against the raw payload of a connection (for reassembled TCP streams) or of each packet (for ICMP, UDP, and non-reassembled TCP). Second, it may be prefixed with an analyzer-specific label, in which case the expression is matched against the data as extracted by the corresponding analyzer.


need IP frag with TCP data


We use a signature to match of the first chunk piece/pattern and the full payload is present in signatures.log after.


#### IPv4

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_zeek_signatures.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/install/install_6.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
ipv4 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_zeek_signatures.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/install/install_6.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
ipv4 \
icvl8i4 \
20 \
1
```

#### IPv6

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_zeek_signatures.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/install/install_6.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
ipv6 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_zeek_signatures.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/install/install_6.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
ipv6 \
icvl8i6 \
20 \
1
```

#### TCP

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_zeek_signatures.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/install/install_6.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp" \
tcp \
icfl8b \
20 \
1
```

### zeek_script

#### IPv4 

##### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_zeek_script.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/install/install_6.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
ipv4 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_zeek_script.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/install/install_6.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
ipv4 \
icvl8i4 \
20 \
1
```


#### IPv6

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_zeek_script.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/install/install_6.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
ipv6 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_zeek_script.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/install/install_6.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
ipv6 \
icvl8i6 \
20 \
1
```

#### TCP

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_zeek_script.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/install/install_6.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/zeek_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp" \
tcp \
icfl8b \
20 \
1
```

### Suricata 


#### IPv4

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_suricata.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/install/install_7.0.7" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
ipv4 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_suricata.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/install/install_7.0.7" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
ipv4 \
icvl8i4 \
20 \
1
```

#### IPv6

##### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_suricata.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/install/install_7.0.7" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
ipv6 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_suricata.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/install/install_7.0.7" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
ipv6 \
icvl8i6 \
20 \
1
```

#### TCP

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_suricata.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/install/install_7.0.7" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/suricata_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp" \
tcp \
icfl8b \
20 \
1
```

## Snort

### IPv4

##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_snort.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.5.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
ipv4 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_snort.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.5.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv4" \
ipv4 \
icvl8i4 \
20 \
1
```

### IPv6


##### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_snort.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.5.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
ipv6 \
icfl8b \
20 \
1
```

##### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_snort.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.5.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/ipv6" \
ipv6 \
icvl8i6 \
20 \
1
```

### TCP

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/detect_scenarii_run.sh" \
"${PYROLYSE_PATH}/tools/script/midpoint/detect_copy_log_test_sequence_snort.sh" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/install/install_3.5.2.0" \
"${PYROLYSE_PATH}/target/midpoint/ids_code/snort_base/conf" \
"$reassembly_options" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}/tcp" \
tcp \
icfl8b \
20 \
1
```
