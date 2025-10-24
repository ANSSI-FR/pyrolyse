

# Compare policy across scenarii


These commands must be launched from the ./ws/ directory from the pyrolyse root.


## pep/peo/peos/peoe/peose/peoes

### IPv4

```bash
meld \
debian_8/ipv4_pep_policy_complicated.json \
debian_8/ipv4_peos_policy_complicated.json \
debian_8/ipv4_peoe_policy_complicated.json \
&
```

=> pep != peos = peoe


```bash
meld \
debian_8/ipv4_peos_policy_complicated.json \
debian_8/ipv4_peose_policy_complicated.json \
debian_8/ipv4_peoes_policy_complicated.json \
&
```

=> same






```bash
meld \
debian_11/ipv4_pep_policy_complicated.json \
debian_11/ipv4_peos_policy_complicated.json \
debian_11/ipv4_peoe_policy_complicated.json \
&
```

=> pep != peos != peoe


```bash
meld \
debian_11/ipv4_peos_policy_complicated.json \
debian_11/ipv4_peose_policy_complicated.json \
debian_11/ipv4_peoes_policy_complicated.json \
&
```

=> same






```bash
meld \
freebsd_11/ipv4_pep_policy_complicated.json \
freebsd_11/ipv4_peos_policy_complicated.json \
freebsd_11/ipv4_peoe_policy_complicated.json \
&
```

=> pep != peos != peoe


```bash
meld \
freebsd_11/ipv4_peos_policy_complicated.json \
freebsd_11/ipv4_peose_policy_complicated.json \
freebsd_11/ipv4_peoes_policy_complicated.json \
&
```

=> peos != peose = peoes


```bash
meld \
freebsd_11/ipv4_peoe_policy_complicated.json \
freebsd_11/ipv4_peose_policy_complicated.json \
freebsd_11/ipv4_peoes_policy_complicated.json \
&
```

=> peoe = peose = peoes






```bash
meld \
openbsd_6.1/ipv4_pep_policy_complicated.json \
openbsd_6.1/ipv4_peos_policy_complicated.json \
openbsd_6.1/ipv4_peoe_policy_complicated.json \
&
```

=> pep != peos != peoe


```bash
meld \
openbsd_6.1/ipv4_peos_policy_complicated.json \
openbsd_6.1/ipv4_peose_policy_complicated.json \
openbsd_6.1/ipv4_peoes_policy_complicated.json \
&
```

=> data missing






```bash
meld \
windows_10/ipv4_pep_policy_complicated.json \
windows_10/ipv4_peos_policy_complicated.json \
windows_10/ipv4_peoe_policy_complicated.json \
&
```

=> pep != peos != peoe


```bash
meld \
windows_10/ipv4_peos_policy_complicated.json \
windows_10/ipv4_peose_policy_complicated.json \
windows_10/ipv4_peoes_policy_complicated.json \
&
```

=> same






```bash
meld \
lwip_2.1.2/ipv4_pep_policy_complicated.json \
lwip_2.1.2/ipv4_peos_policy_complicated.json \
lwip_2.1.2/ipv4_peoe_policy_complicated.json \
&
```


```bash
meld \
lwip_2.1.2/ipv4_peos_policy_complicated.json \
lwip_2.1.2/ipv4_peose_policy_complicated.json \
lwip_2.1.2/ipv4_peoes_policy_complicated.json \
&
```






```bash
meld \
uip_1.0/ipv4_pep_policy_complicated.json \
uip_1.0/ipv4_peos_policy_complicated.json \
uip_1.0/ipv4_peoe_policy_complicated.json \
&
```


```bash
meld \
uip_1.0/ipv4_peos_policy_complicated.json \
uip_1.0/ipv4_peose_policy_complicated.json \
uip_1.0/ipv4_peoes_policy_complicated.json \
&
```









