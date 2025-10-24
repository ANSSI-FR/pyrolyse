# check for test cases with pattern wich offset is bad in reassembly and check for test cases with hole and pattern after the hole in reassembly

## zeek_signature

### TCP

```bash
${PYROLYSE_PATH}/tools/script/midpoint/check_tcp_reassembly_consistency_scenarii_latest.sh \
${PYROLYSE_PATH}/target/midpoint/zeek_signature \
${PYROLYSE_PATH}/test_data/byte_time_sequence.json
```

find . -type f -name "tcp_pe*_reassembly_consistency.json" -exec sha256sum {} | sort -k2 
