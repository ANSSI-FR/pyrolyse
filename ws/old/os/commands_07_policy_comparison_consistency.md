

## consistency


These commands are used to check consistency during developmeent of the Rust pipeline.
These commands must be launched from the `$PYROLYSE_PATH/ws/` directory from the pyrolyse root.


### Example : Debian 8 for some scenarios

`${TARGET_NAME}` is the target directory name 
`${d1}` is the date of the first results obtained. 
`${d2}` is the date of the second results obtained. 


```bash
export TARGET_NAME=${PYROLYSE_PATH}/target/os/debian_8"
```

#### IPv4

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peoef_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peoef_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peoep_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peoep_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peosfef_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peosfef_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peoefsf_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peoefsf_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peospef_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peospef_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peoepsf_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peoepsf_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peospep_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peospep_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peoepsp_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv4/output/ipv4_peoepsp_payload_${d2}.json \
&
```

#### IPv6

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peoef_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peoef_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peoep_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peoep_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peosfef_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peosfef_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peoefsf_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peoefsf_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peospef_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peospef_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peoepsf_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peoepsf_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peospep_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peospep_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peoepsp_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/ipv6/output/ipv6_peoepsp_payload_${d2}.json \
&
```

#### TCP

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/output/tcp_peosf_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/output/tcp_peosf_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/output/tcp_peosfef_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/output/tcp_peosfef_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/output/tcp_peoefsf_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/output/tcp_peoefsf_payload_${d2}.json \
&
```

```bash
meld \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/output/tcp_peoepsf_payload_${d1}.json" \
"${PYROLYSE_PATH}/target/os/${TARGET_NAME}/tcp/output/tcp_peoepsf_payload_${d2}.json \
&
```