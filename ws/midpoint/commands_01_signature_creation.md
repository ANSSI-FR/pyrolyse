# Create signature files to test IPv4, IPv6 and TCP midpoint reassemblies

```bash
"${PYROLYSE_PATH}/tools/pyrolyse-rs/target/debug/generate_ids_signatures" \
--vc1b "${PYROLYSE_PATH}/test_data/separated/vc1b_pattern.json" \
--icfl8b "${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
--icvl8i4 "${PYROLYSE_PATH}/test_data/separated/icvl8i4_pattern.json" \
--icvl8i6 "${PYROLYSE_PATH}/test_data/separated/icvl8i6_pattern.json" \
--ids-code-path "${PYROLYSE_PATH}/target/midpoint/ids_code"
```