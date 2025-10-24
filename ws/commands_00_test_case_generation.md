

These commands must be launched from the ./ws/ directory from the pyrolyse root.


# init SparQ


This is doing some LISP stuff. :)

```bash
"${PYROLYSE_PATH}/sparq_dir/SparQ_$HOSTNAME/sparq" quantify allen "((i12 b i14) (i12 o i13) (i13 fi i14))"
```



# Generate relation pair and triplet using SparQ

```bash
"${PYROLYSE_PATH}/tools/script/test_data/generate_sparq_pair.sh" \
"${PYROLYSE_PATH}/test_data/separated/allen_relation" \
"${PYROLYSE_PATH}/test_data/separated/sparq/sparq_pair"

"${PYROLYSE_PATH}/tools/script/test_data/generate_sparq_triplet.sh" \
"${PYROLYSE_PATH}/test_data/separated/allen_relation" \
"${PYROLYSE_PATH}/test_data/separated/allen_relation_triplet" \
"${PYROLYSE_PATH}/test_data/separated/sparq/sparq_triplet_all"
```

# Generate consistent relation pair and triplet using SparQ

```bash
"${PYROLYSE_PATH}/tools/script/test_data/check_triplet_consistency.sh" \
"${PYROLYSE_PATH}/test_data/separated/sparq/sparq_triplet_all" \
"${PYROLYSE_PATH}/test_data/shell/sparq_consistency_output" \
"${PYROLYSE_PATH}/test_data/separated/sparq/sparq_triplet_consistent"
```

# Convert pair and triplet relations to JSON

```bash
"${PYROLYSE_PATH}/tools/script/test_data/allen_relation_to_json.sh" \
"${PYROLYSE_PATH}/test_data/separated/allen_relation" \
"${PYROLYSE_PATH}/test_data/separated/json/json_pair"
```

```bash
"${PYROLYSE_PATH}/tools/script/test_data/sparq_triplet_to_json.sh" \
"${PYROLYSE_PATH}/test_data/separated/sparq/sparq_triplet_consistent" \
"${PYROLYSE_PATH}/test_data/separated/json/json_triplet_consistent"
```

# Generate byte sequence using SparQ

```bash
"${PYROLYSE_PATH}/tools/script/test_data/json_to_byte_sequence.sh" \
pair \
"${PYROLYSE_PATH}/test_data/separated/json/json_pair" \
"${PYROLYSE_PATH}/test_data/separated/byte_sequence"
```

```bash
"${PYROLYSE_PATH}/tools/script/test_data/json_to_byte_sequence.sh" \
triplet \
"${PYROLYSE_PATH}/test_data/separated/json/json_triplet_consistent" \
"${PYROLYSE_PATH}/test_data/separated/byte_sequence"
```

# Generate byte/time sequences (also named chunk sequences)


```bash
"${PYROLYSE_PATH}/tools/pyrolyse-rs/target/debug/export_chunk_pattern" \
--vc1b "${PYROLYSE_PATH}/test_data/separated/vc1b_pattern.json" \
--icfl8b "${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
--icvl8i4 "${PYROLYSE_PATH}/test_data/separated/icvl8i4_pattern.json" \
--icvl8i6 "${PYROLYSE_PATH}/test_data/separated/icvl8i6_pattern.json"
```

```bash
"${PYROLYSE_PATH}/tools/script/test_data/byte_sequence_to_byte_time_sequence.sh" \
pair \
0 \
"${PYROLYSE_PATH}/test_data/separated/byte_sequence" \
"${PYROLYSE_PATH}/test_data/separated/vc1b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i4_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i6_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence"
```

```bash
"${PYROLYSE_PATH}/tools/script/test_data/byte_sequence_to_byte_time_sequence.sh" \
triplet \
100 \
"${PYROLYSE_PATH}/test_data/separated/byte_sequence" \
"${PYROLYSE_PATH}/test_data/separated/vc1b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/icfl8b_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i4_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/icvl8i6_pattern.json" \
"${PYROLYSE_PATH}/test_data/separated/byte_time_sequence"
```

```bash
mkdir -p "${PYROLYSE_PATH}/test_data/merged"

python3 "${PYROLYSE_PATH}/tools/script/test_data/build_byte_time_sequence_json.py" \
-i "${PYROLYSE_PATH}/test_data/separated/byte_time_sequence" \
-o "${PYROLYSE_PATH}/test_data/merged/byte_time_sequence.json"
```