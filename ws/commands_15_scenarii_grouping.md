
## Build csv 

### ipv4

#### use only protocol-agnostic part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_csv.py" \
-t "${PYROLYSE_PATH}" \
-p "ipv4" \
-pjpf "_payload.json" \
--ip-scenarii-to-use "protocol_agnostic_only" \
-o "pas_scenario_groups.csv"
```

#### use only protocol-dependant part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_csv.py" \
-t "${PYROLYSE_PATH}" \
-p "ipv4" \
-pjpf "_payload.json" \
--ip-scenarii-to-use "protocol_dependant_only" \
-o "pds_scenario_groups.csv"
```

#### use any scenario

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_csv.py" \
-t "${PYROLYSE_PATH}" \
-p "ipv4" \
-pjpf "_payload.json" \
--ip-scenarii-to-use "any" \
-o "scenario_groups.csv"
```

#### use only scenarii with full test cases

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_csv.py" \
-t "${PYROLYSE_PATH}" \
-p "ipv4" \
-pjpf "_payload.json" \
--ip-scenarii-to-use "full_test_case_scenarii" \
-o "full_tc_scenario_groups.csv"
```

#### use only scenarii with MF unsetting sub-strategies

#### Starting 

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_csv.py" \
-t "${PYROLYSE_PATH}" \
-p "ipv4" \
-pjpf "_payload.json" \
--ip-scenarii-to-use "partial_test_case_scenarii" \
--unsetting-strat "starting" \
-o "partial_tc_starting_scenario_groups.csv"
```

#### Finishing  

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_csv.py" \
-t "${PYROLYSE_PATH}" \
-p "ipv4" \
-pjpf "_payload.json" \
--ip-scenarii-to-use "partial_test_case_scenarii" \
--unsetting-strat "finishing" \
-o "partial_tc_finishing_scenario_groups.csv"
```

### ipv6

#### use only protocol-agnostic part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_csv.py" \
-t "${PYROLYSE_PATH}" \
-p "ipv6" \
-pjpf "_payload.json" \
--ip-scenarii-to-use "protocol_agnostic_only" \
-o "pas_scenario_groups.csv"
```

#### use only protocol-dependant part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_csv.py" \
-t "${PYROLYSE_PATH}" \
-p "ipv6" \
-pjpf "_payload.json" \
--ip-scenarii-to-use "protocol_dependant_only" \
-o "pds_scenario_groups.csv"
```

#### use any scenario

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_csv.py" \
-t "${PYROLYSE_PATH}" \
-p "ipv6" \
-pjpf "_payload.json" \
--ip-scenarii-to-use "any" \
-o "scenario_groups.csv"
```

#### use only scenarii with full test cases

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_csv.py" \
-t "${PYROLYSE_PATH}" \
-p "ipv6" \
-pjpf "_payload.json" \
--ip-scenarii-to-use "full_test_case_scenarii" \
-o "full_tc_scenario_groups.csv"
```

#### use only scenarii with MF unsetting sub-strategies

#### Starting 

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_csv.py" \
-t "${PYROLYSE_PATH}" \
-p "ipv6" \
-pjpf "_payload.json" \
--ip-scenarii-to-use "partial_test_case_scenarii" \
--unsetting-strat "starting" \
-o "partial_tc_starting_scenario_groups.csv"
```

#### Finishing  

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_csv.py" \
-t "${PYROLYSE_PATH}" \
-p "ipv6" \
-pjpf "_payload.json" \
--ip-scenarii-to-use "partial_test_case_scenarii" \
--unsetting-strat "finishing" \
-o "partial_tc_finishing_scenario_groups.csv"
```

### tcp

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_csv.py" \
-t "${PYROLYSE_PATH}" \
-p "tcp" \
-pjpf "_payload.json" \
-o "scenario_groups.csv"
```

## Plot scenario groups 

### ipv4

#### use only protocol-agnostic part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_histo.py" \
-i "${PYROLYSE_PATH}/ipv4_pas_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv4_pas_scenario_groups.pdf" \
-p "ipv4"
```

#### use only protocol-dependant part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_histo.py" \
-i "${PYROLYSE_PATH}/ipv4_pds_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv4_pds_scenario_groups.pdf" \
-p "ipv4"
```

#### use any scenario

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_histo.py" \
-i "${PYROLYSE_PATH}/ipv4_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv4_scenario_groups.pdf" \
-p "ipv4"
```

### ipv6

#### use only protocol-agnostic part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_histo.py" \
-i "${PYROLYSE_PATH}/ipv6_pas_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv6_pas_scenario_groups.pdf" \
-p "ipv6"
```

#### use only protocol-dependant part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_histo.py" \
-i "${PYROLYSE_PATH}/ipv6_pds_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv6_pds_scenario_groups.pdf" \
-p "ipv6"
```

#### use any scenario

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_histo.py" \
-i "${PYROLYSE_PATH}/ipv6_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv6_scenario_groups.pdf" \
-p "ipv6"
```


### tcp

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenario_groups_histo.py" \
-i "${PYROLYSE_PATH}/tcp_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/tcp_scenario_groups.pdf" \
-p "tcp"
```

## Plot scenario heatmap

### ipv4

#### use only protocol-agnostic part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_agnostic.py" \
-i "${PYROLYSE_PATH}/ipv4_pas_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv4_pas_scenario_groups_heatmap.pdf"
```

#### use only protocol-dependant part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_dependant.py" \
-i "${PYROLYSE_PATH}/ipv4_pds_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv4_pds_scenario_groups_heatmap.pdf"
```

#### use protocol-dependant and protocol-agnostic part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_any.py" \
-i "${PYROLYSE_PATH}/ipv4_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv4_scenario_groups_heatmap.pdf"
```

#### use only scenarii with full test cases

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_any.py" \
-i "${PYROLYSE_PATH}/ipv4_full_tc_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv4_full_tc_scenario_groups_heatmap.pdf"
```

#### use only scenarii with MF unsetting sub-strategies

##### Starting

```bash
"${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_dependant_scenarii.sh" \
"${PYROLYSE_PATH}/ipv4_partial_tc_starting_scenario_groups.csv" \
"ipv4_partial_tc_starting_scenario_groups_heatmap"
```

##### Finishing

```bash
"${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_dependant_scenarii.sh" \
"${PYROLYSE_PATH}/ipv4_partial_tc_finishing_scenario_groups.csv" \
"ipv4_partial_tc_finishing_scenario_groups_heatmap"
```

### ipv6

#### use only protocol-agnostic part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_agnostic.py" \
-i "${PYROLYSE_PATH}/ipv6_pas_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv6_pas_scenario_groups_heatmap.pdf"
```

#### use only protocol-dependant part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_dependant.py" \
-i "${PYROLYSE_PATH}/ipv6_pds_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv6_pds_scenario_groups_heatmap.pdf"
```

#### use protocol-dependant and protocol-agnostic part-related scenarii

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_any.py" \
-i "${PYROLYSE_PATH}/ipv6_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv6_scenario_groups_heatmap.pdf"
```

#### use only scenarii with full test cases

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_any.py" \
-i "${PYROLYSE_PATH}/ipv6_full_tc_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv6_full_tc_scenario_groups_heatmap.pdf"
```

#### use only scenarii with MF unsetting sub-strategies

##### Starting

```bash
"${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_dependant_scenarii.sh" \
"${PYROLYSE_PATH}/ipv6_partial_tc_starting_scenario_groups.csv" \
"ipv6_partial_tc_starting_scenario_groups_heatmap"
```

##### Finishing

```bash
"${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_dependant_scenarii.sh" \
"${PYROLYSE_PATH}/ipv6_partial_tc_finishing_scenario_groups.csv" \
"ipv6_partial_tc_finishing_scenario_groups_heatmap"
```

### tcp

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_reassembly_closeness_heatmap_agnostic.py" \
-i "${PYROLYSE_PATH}/tcp_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/tcp_scenario_groups_heatmap.pdf"
```


## Plot scenario boxplot

### ipv4

#### use only scenarii with full test cases

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_boxplot.py" \
-i "${PYROLYSE_PATH}/ipv4_full_tc_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv4_full_tc_scenario_groups_boxplot.pdf" \
--ymax 23 \
--ytick-step 5
```

#### use only scenarii with MF unsetting sub-strategies

#### Starting 

```bash
"${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_boxplot_scenarii.sh" \
"${PYROLYSE_PATH}/ipv4_partial_tc_starting_scenario_groups.csv" \
"ipv4_partial_tc_starting_scenario_groups_boxplot" 
```

#### Finishing 

```bash
"${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_boxplot_scenarii.sh" \
"${PYROLYSE_PATH}/ipv4_partial_tc_finishing_scenario_groups.csv" \
"ipv4_partial_tc_finishing_scenario_groups_boxplot" 
```

### ipv6

#### use only scenarii with full test cases

```bash
python3 "${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_boxplot.py" \
-i "${PYROLYSE_PATH}/ipv6_full_tc_scenario_groups.csv" \
-o "${PYROLYSE_PATH}/ipv6_full_tc_scenario_groups_boxplot.pdf" \
--ymax 23 \
--ytick-step 5
```

#### use only scenarii with MF unsetting sub-strategies

#### Starting 

```bash
"${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_boxplot_scenarii.sh" \
"${PYROLYSE_PATH}/ipv6_partial_tc_starting_scenario_groups.csv" \
"ipv6_partial_tc_starting_scenario_groups_boxplot" 
```

#### Finishing 

```bash
"${PYROLYSE_PATH}/tools/script/policy_analysis/build_stack_scenarii_boxplot_scenarii.sh" \
"${PYROLYSE_PATH}/ipv6_partial_tc_finishing_scenario_groups.csv" \
"ipv6_partial_tc_finishing_scenario_groups_boxplot" 
```

