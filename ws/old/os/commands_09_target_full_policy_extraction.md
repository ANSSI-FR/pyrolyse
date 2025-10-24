

# build full policies (i.e. prefered overlapping chunk data + extra capabilities)


These commands must be launched inside the base machine from the Vagrantfile, i.e. after doing "vagrant ssh base" in each ./ws/$OS/ directory.


## IPv4

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_ip_full_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}" \
icvl8i4 \
4
```

## IPv6

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_ip_full_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}" \
icvl8i6 \
6
```


## TCP

```bash
"${PYROLYSE_PATH}/tools/script/network_stack/extract_stack_tcp_full_policies_scenarii.sh" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}" \
icfl8b
```