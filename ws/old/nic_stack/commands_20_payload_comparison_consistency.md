

## consistency


These commands are used to check consistency during developmeent of the Rust pipeline.
These commands must be launched from the ./ws/ directory from the pyrolyse root.


### Debian 8

#### IPv4


```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv4_fragmentation_pep_policy_complicated.json \
$PYROLYSE_PATH/target/os/debian_8/ipv4_fragmentation_pep_policy_complicated_s0.json \
&
```


```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv4_fragmentation_peo_policy_complicated.json \
$PYROLYSE_PATH/target/os/debian_8/ipv4_fragmentation_peo_policy_complicated_s0.json \
&
```

#### IPv6

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv6_fragmentation_pep_policy_complicated.json \
$PYROLYSE_PATH/target/os/debian_8/ipv6_fragmentation_pep_policy_complicated_s0.json \
&
```


```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv6_fragmentation_peo_policy_complicated.json \
$PYROLYSE_PATH/target/os/debian_8/ipv6_fragmentation_peo_policy_complicated_s0.json \
&
```



## TCP


### Xilinx Onload


```bash
meld \
$PYROLYSE_PATH/target/os/debian_11/tcp_peos_payload.json \
$PYROLYSE_PATH/target/user_stack/xilinx_onload/tcp_peos_payload.json \
&
```

```bash
meld \
$PYROLYSE_PATH/target/os/debian_11/tcp_pep1_payload.json \
$PYROLYSE_PATH/target/user_stack/xilinx_onload/tcp_pep1_payload.json \
&
```

```bash
meld \
$PYROLYSE_PATH/target/os/debian_11/tcp_pep2_payload.json \
$PYROLYSE_PATH/target/user_stack/xilinx_onload/tcp_pep2_payload.json \
&
```









