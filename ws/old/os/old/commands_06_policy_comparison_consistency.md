

## consistency


These commands are used to check consistency during developmeent of the Rust pipeline.
These commands must be launched from the `$PYROLYSE_PATH/ws/` directory from the pyrolyse root.


### Example : Debian 8 for some scenarios

`${d1}` is the date of the first results obtained. 
`${d2}` is the date of the second results obtained. 

#### IPv4

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv4_pep_payload_${d1}.json \
$PYROLYSE_PATH/target/os/debian_8/$PYROLYSE_PATH/target/os/debian_8/ipv4_pep_payload_${d2}.json \
&
```

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv4_peoe_payload_${d1}.json \
$PYROLYSE_PATH/target/os/debian_8/$PYROLYSE_PATH/target/os/debian_8/ipv4_peoe_payload_${d2}.json \
&
```

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv4_peos_payload_${d1}.json \
$PYROLYSE_PATH/target/os/debian_8/$PYROLYSE_PATH/target/os/debian_8/ipv4_peos_payload_${d2}.json \
&
```

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv4_peose_payload_${d1}.json \
$PYROLYSE_PATH/target/os/debian_8/$PYROLYSE_PATH/target/os/debian_8/ipv4_peose_payload_${d2}.json \
&
```

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv4_peoes_payload_${d1}.json \
$PYROLYSE_PATH/target/os/debian_8/$PYROLYSE_PATH/target/os/debian_8/ipv4_peoes_payload_${d2}.json \
&
```

#### IPv6

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv6_pep_payload_${d1}.json \
$PYROLYSE_PATH/target/os/debian_8/$PYROLYSE_PATH/target/os/debian_8/ipv6_pep_payload_${d2}.json \
&
```

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv6_peoe_payload_${d1}.json \
$PYROLYSE_PATH/target/os/debian_8/$PYROLYSE_PATH/target/os/debian_8/ipv6_peoe_payload_${d2}.json \
&
```

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv6_peos_payload_${d1}.json \
$PYROLYSE_PATH/target/os/debian_8/$PYROLYSE_PATH/target/os/debian_8/ipv6_peos_payload_${d2}.json \
&
```

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv6_peose_payload_${d1}.json \
$PYROLYSE_PATH/target/os/debian_8/$PYROLYSE_PATH/target/os/debian_8/ipv6_peose_payload_${d2}.json \
&
```

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/ipv6_peoes_payload_${d1}.json \
$PYROLYSE_PATH/target/os/debian_8/$PYROLYSE_PATH/target/os/debian_8/ipv6_peoes_payload_${d2}.json \
&
```

#### TCP

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/tcp_peos_payload_${d1}.json \
$PYROLYSE_PATH/target/os/debian_8/$PYROLYSE_PATH/target/os/debian_8/tcp_peos_payload_${d2}.json \
&
```

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/tcp_pep1_payload_${d1}.json \
$PYROLYSE_PATH/target/os/debian_8/$PYROLYSE_PATH/target/os/debian_8/tcp_pep1_payload_${d2}.json \
&
```

```bash
meld \
$PYROLYSE_PATH/target/os/debian_8/tcp_pep2_payload_${d1}.json \
$PYROLYSE_PATH/target/os/debian_8/$PYROLYSE_PATH/target/os/debian_8/tcp_pep2_payload_${d2}.json \
&
```