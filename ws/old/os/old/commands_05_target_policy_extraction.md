

# build policies


These commands must be launched from the ./ws/ directory from the pyrolyse root.


## Debian 8/Jessie

### IPv4

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/debian_8 \
ipv4
``` 

```bash
RUST_LOG=debug $PYTROLYSE_PATH/rust_code/reassembly_test_pipeline/target/debug/extract_complicated_policies \
--all-chunk-json-path $PYROLYSE_PATH/test_data/byte_time_sequence.json \
--payload-json-path $PYROLYSE_PATH/target/os/debian_8/ipv4_fragmentation_icmp_peo_payload.json \
--policy-json-path $PYROLYSE_PATH/target/os/debian_8/ipv4_fragmentation_peo_policy_complicated.json \
--payload-mode i \
-s
```

### IPv6

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/debian_8 \
ipv6
``` 


### TCP

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/debian_8 \
tcp
```


## Debian 9/Stretch

### IPv4


```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
debian_9 \
ipv4
```

### IPv6

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/debian_9 \
ipv6
``` 


### TCP

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/debian_9 \
tcp
```



## Debian 10/Stretch

### IPv4


```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/debian_10 \
ipv4 
```

### IPv6

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/debian_10 \
ipv6
``` 


### TCP

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/debian_10 \
tcp
```


## Debian 11/Bullseye

### IPv4

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/debian_11 \
ipv4 
```

### IPv6

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/debian_11 \
ipv6
``` 


### TCP

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/debian_11 \
tcp
```

## FreeBSD 11.4

### IPv4

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/freebsd_11 \
ipv4
```

### IPv6

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/freebsd_11.4 \
ipv6
``` 


### TCP

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/freebsd_11.4 \
tcp
```



## OpenBSD 6.1


### IPv4

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/openbsd_6.1 \
ipv4
```

### IPv6

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/openbsd_6 \
ipv6
``` 


### TCP

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/openbsd_6 \
tcp
```



## Windows 10


### IPv4

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/windows_10 \
ipv4
```

### IPv6

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/windows_10 \
ipv6
``` 


### TCP

```bash
$PYROLYSE_PATH/script/network_stack/extract_stack_complicated_policies_scenarii.sh \
$PYROLYSE_PATH/target/os/windows_10 \
tcp
```










