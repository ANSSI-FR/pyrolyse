
## Build csv 

### ipv4

#### use only protocol-agnostic part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_reassembly_policy_heatmap_two_cm.py" \
-t "/run/media/laubard/59562097-9d04-45d4-afb5-c793f189cdf6/pyrolyse_data/20250815_target_RAID_CAMERA_READY" \
-p "ipv4" \
-jpf "_payload.json" \
--ip-scenarii-to-use "protocol_agnostic_only" \
-op "${PYROLYSE_PATH}/ipv4_pas_stack_reassembly_policy_heatmap.pdf" \
-oc "${PYROLYSE_PATH}/ipv4_pas_stack_reassembly_policy.csv"
```


#### use only protocol-dependant part-related scenarii


#### use any scenario

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_reassembly_policy_heatmap_two_cm.py" \
-t "/run/media/laubard/59562097-9d04-45d4-afb5-c793f189cdf6/pyrolyse_data/20250815_target_RAID_CAMERA_READY" \
-p "ipv4" \
-jpf "_payload.json" \
--ip-scenarii-to-use "any" \
-op "${PYROLYSE_PATH}/ipv4_stack_reassembly_policy_heatmap.pdf" \
-oc "${PYROLYSE_PATH}/ipv4_stack_reassembly_policy.csv"
```


### ipv6

#### use only protocol-agnostic part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_reassembly_policy_heatmap_two_cm.py" \
-t "/run/media/laubard/59562097-9d04-45d4-afb5-c793f189cdf6/pyrolyse_data/20250815_target_RAID_CAMERA_READY" \
-p "ipv6" \
-jpf "_payload.json" \
--ip-scenarii-to-use "protocol_agnostic_only" \
-o "${PYROLYSE_PATH}/ipv6_pas_stack_reassembly_policy_heatmap.pdf"
```

#### use any scenario

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_reassembly_policy_heatmap_two_cm.py" \
-t "/run/media/laubard/59562097-9d04-45d4-afb5-c793f189cdf6/pyrolyse_data/20250815_target_RAID_CAMERA_READY" \
-p "ipv6" \
-jpf "_payload.json" \
--ip-scenarii-to-use "any" \
-op "${PYROLYSE_PATH}/ipv6_stack_reassembly_policy_heatmap.pdf" \
-oc "${PYROLYSE_PATH}/ipv6_stack_reassembly_policy.csv"
```

### tcp

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_reassembly_policy_heatmap_two_cm.py" \
-t "/run/media/laubard/59562097-9d04-45d4-afb5-c793f189cdf6/pyrolyse_data/20250815_target_RAID_CAMERA_READY" \
-p "tcp" \
-jpf "_payload.json" \
-op "${PYROLYSE_PATH}/tcp_stack_reassembly_policy_heatmap.pdf" \
-oc "${PYROLYSE_PATH}/tcp_stack_reassembly_policy.csv"
```