

# Capture targeted OS's answers


These commands must be launched inside the base machine from the Vagrantfile, i.e. after doing "vagrant ssh base" in each $PYROLYSE_PATH/target/os/$OS/ directory.


TODO: acquire_tcp_traffic.py uses 200MB of memory => clean this => probably need to manually create smaller json to avoid too much memory usage

TODO:
* replace all OS output directory here with (to create) environment variable from inside VM
* replace all IP here with environment variable from inside VM


## Debian 8/Jessie

```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/os/debian_8/tcp/output" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
192.168.56.10 \
192.168.56.11 \
14 \
2
```


## Debian 9/Stretch

```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/os/debian_9/tcp/output" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
192.168.56.36 \
192.168.56.37 \
20 \
2
```


## Debian 12/Bookworm - DEPRECATED


```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/os/debian_12/tcp/output" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
192.168.56.110 \
192.168.56.111 \
40 \
2
```


## Debian 12/Bookworm - Generic

```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/os/debian_12/tcp/output" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
"${BASE_IPV4_ADDR}" \
"${TARGET_IPV4_ADDR}" \ \
40 \
2
```


## Solaris 10


```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/os/solaris_10/tcp/output" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
192.168.56.46 \
192.168.56.47 \
40 \
2
```

## Solaris 11.4 


```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/os/solaris_11.4/tcp/output" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
192.168.56.42 \
192.168.56.43 \
40 \
2
```

## OpenBSD 7.2 


```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/os/openbsd_7.2/tcp/output" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
192.168.56.38 \
192.168.56.39 \
40 \
2
```

## FreeBSD 13 

```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/os/freebsd_13/tcp/output" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
192.168.56.48 \
192.168.56.49 \
40 \
2
```

## Windows 10 

```bash
"${PYROLYSE_PATH}/script/network_stack/acquire_tcp_traffic_scenarii_run.sh" \
"${PYROLYSE_PATH}/target/os/windows_10/tcp/output" \
"${PYROLYSE_PATH}/test_data_separated/byte_time_sequence" \
192.168.56.16 \
192.168.56.17 \
40 \
2
```



