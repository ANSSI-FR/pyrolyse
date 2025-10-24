

# Compare policy across scenarii


These commands must be launched from the ./ws/ directory from the pyrolyse root.

## IPv4

```bash
meld \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4_pep_policy_complicated.json" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4_peosp_policy_complicated.json" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv4_peosf_policy_complicated.json" \
&
```

## IPv6

```bash
meld \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6_pep_policy_complicated.json" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6_peosf_policy_complicated.json" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/ipv6_peosp_policy_complicated.json" \
&
```

## TCP

```bash
meld \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp_peosf_olicy_complicated.json" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp_peosfef_policy_complicated.json" \
"${PYROLYSE_PATH}/target/${TARGET_FAMILY}/${TARGET_NAME}/tcp_peoefsf_policy_complicated.json" \
&
```