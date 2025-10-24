

# Compare policy across scenarii


These commands must be launched from the ./ws/ directory from the pyrolyse root.

### Example : Debian 8 for some scenarios 

```bash
export TARGET_NAME=${PYROLYSE_PATH}/target/os/debian_8"
```


#### IPv4

```bash
meld \
"${TARGET_NAME}/ipv4_pep_policy_complicated.json" \
"${TARGET_NAME}/ipv4_peosp_policy_complicated.json" \
"${TARGET_NAME}/ipv4_peosf_policy_complicated.json" \
&
```

#### IPv6

```bash
meld \
"${TARGET_NAME}/ipv6_pep_policy_complicated.json" \
"${TARGET_NAME}/ipv6_peosf_policy_complicated.json" \
"${TARGET_NAME}/ipv6_peosp_policy_complicated.json" \
&
```

#### TCP

```bash
meld \
"${TARGET_NAME}/tcp_peosf_olicy_complicated.json" \
"${TARGET_NAME}/tcp_peosfef_policy_complicated.json" \
"${TARGET_NAME}/tcp_peoefsf_policy_complicated.json" \
&
```