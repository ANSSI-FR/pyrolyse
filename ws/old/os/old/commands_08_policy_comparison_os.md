

These commands must be launched from the ./ws/ directory from the pyrolyse root.


# Compare payload across OSes

## IPv4


```bash
meld \
debian_8/ipv4_pep_payload.json \
freebsd_11/ipv4_pep_payload.json 
openbsd_6.1/ipv4_pep_payload.json \
&
```







```bash
meld \
debian_8/ipv4_pep_payload.json \
freebsd_11/ipv4_pep_payload.json \
windows_10/ipv4_pep_payload.json \
&
```






```bash
meld \
debian_8/ipv4_pep_payload.json \
freebsd_11/ipv4_pep_payload.json \
windows_10/ipv4_pep_payload.json \
&
```










# Compare policy across OSes




```bash
meld \
debian_8/ipv4_pep_policy_complicated.json \
debian_9/ipv4_pep_policy_complicated.json \
debian_10/ipv4_pep_policy_complicated.json \
&
```

=> 8 != 9 = 10


```bash
meld \
debian_9/ipv4_pep_policy_complicated.json \
debian_10/ipv4_pep_policy_complicated.json \
debian_11/ipv4_pep_policy_complicated.json \
&
```

=> 9 = 10 = 11




```bash
meld \
debian_8/ipv4_pep_policy_complicated.json \
freebsd_11/ipv4_pep_policy_complicated.json \
openbsd_6.1/ipv4_pep_policy_complicated.json \
&
```





```bash
meld \
debian_8/ipv4_pep_policy_complicated.json \
freebsd_11/ipv4_pep_policy_complicated.json \
windows_10/ipv4_pep_policy_complicated.json \
&
```





```bash
meld \
debian_8/ipv4_pep_policy_complicated.json \
lwip_2.1.2/ipv4_pep_policy_complicated.json \
uip_1.0/ipv4_pep_policy_complicated.json \
&
```







```bash
meld \
debian_8/ipv4_pep_policy_complicated.json \
debian_11/ipv4_pep_policy_complicated.json \
windows_10/ipv4_pep_policy_complicated.json \
&
```





```bash
meld \
debian_8/ipv4_pep_policy_complicated.json \
debian_11/ipv4_pep_policy_complicated.json \
freebsd_11/ipv4_pep_policy_complicated.json \
&
```





## peos


```bash
meld \
debian_8/ipv4_peos_policy_complicated.json \
lwip_2.1.2/ipv4_peos_policy_complicated.json \
uip_1.0/ipv4_peos_policy_complicated.json \
&
```






# tar complicated policies


tar -cvf ipv4_policies.tar \
**/*_policy_complicated.json



tar -cvf ipv4_policies.tar \
debian_8/ipv4_pep_policy_complicated.json \
debian_8/ipv4_peos_policy_complicated.json \
debian_8/ipv4_peoe_policy_complicated.json \
debian_8/ipv4_peose_policy_complicated.json \
debian_8/ipv4_peoes_policy_complicated.json \
debian_11/ipv4_pep_policy_complicated.json \
debian_11/ipv4_peos_policy_complicated.json \
debian_11/ipv4_peoe_policy_complicated.json \
debian_11/ipv4_peose_policy_complicated.json \
debian_11/ipv4_peoes_policy_complicated.json \
freebsd_11/ipv4_pep_policy_complicated.json \
freebsd_11/ipv4_peos_policy_complicated.json \
freebsd_11/ipv4_peoe_policy_complicated.json \
freebsd_11/ipv4_peose_policy_complicated.json \
freebsd_11/ipv4_peoes_policy_complicated.json \
openbsd_6.1/ipv4_pep_policy_complicated.json \
openbsd_6.1/ipv4_peos_policy_complicated.json \
openbsd_6.1/ipv4_peoe_policy_complicated.json \
openbsd_6.1/ipv4_peose_policy_complicated.json \
openbsd_6.1/ipv4_peoes_policy_complicated.json \
windows_10/ipv4_pep_policy_complicated.json \
windows_10/ipv4_peos_policy_complicated.json \
windows_10/ipv4_peoe_policy_complicated.json \
windows_10/ipv4_peose_policy_complicated.json \
windows_10/ipv4_peoes_policy_complicated.json \
lwip_2.1.2/ipv4_pep_policy_complicated.json \
lwip_2.1.2/ipv4_peos_policy_complicated.json \
lwip_2.1.2/ipv4_peoe_policy_complicated.json \
lwip_2.1.2/ipv4_peose_policy_complicated.json \
lwip_2.1.2/ipv4_peoes_policy_complicated.json \
uip_1.0/ipv4_pep_policy_complicated.json \
uip_1.0/ipv4_peos_policy_complicated.json \
uip_1.0/ipv4_peoe_policy_complicated.json \
uip_1.0/ipv4_peose_policy_complicated.json \
uip_1.0/ipv4_peoes_policy_complicated.json














## OS



```bash
meld \
debian_8/ipv4_pep_policy_complicated.json \
debian_11/ipv4_pep_policy_complicated.json \
freebsd_11/ipv4_pep_policy_complicated.json \
&
```




```bash
meld \
debian_8/ipv4_pep_policy_complicated.json \
freebsd_11/ipv4_pep_policy_complicated.json \
openbsd_6.1/ipv4_pep_policy_complicated.json \
&
```








