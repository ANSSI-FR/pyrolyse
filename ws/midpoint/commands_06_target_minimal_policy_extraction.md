

# build policies


These commands must be launched from the ./ws/ directory from the pyrolyse root.


## IPv4

### Invariant checksum for a single reassembled payload lengths (novak-like)


```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_minimal_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}" \
"$reassembly_options" \
ipv4 \
icfl8b
``` 

### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_minimal_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}" \
"$reassembly_options" \
ipv4 \
icvl8i4
``` 

## IPv6

### Invariant checksum for a single reassembled payload lengths (novak-like)

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_minimal_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}" \
"$reassembly_options" \
ipv6 \
icfl8b
``` 

### Invariant checksum for multiple reassembled payload lengths 

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_minimal_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}" \
"$reassembly_options" \
ipv6 \
icvl8i6
``` 


## TCP

```bash
"${PYROLYSE_PATH}/tools/script/midpoint/extract_midpoint_minimal_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/midpoint/${target_reassembly_policy}" \
"$reassembly_options" \
tcp \
icvl8i4

```