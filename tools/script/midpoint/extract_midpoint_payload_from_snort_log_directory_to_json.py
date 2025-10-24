#!/usr/bin/python
import sys
import os
import json
import argparse
import base64

def extract_data_from_alert_fast(
    index_data_offset_d:dict,
    test_case_index:str, 
    fast_log_path:str, 
    byte_time_sequence_hm:dict, 
    nb_starting_character_to_remove: int,
    nb_final_character_to_remove: int,
    payload_mode: str,
    protocol:str
):
    print("extract_data_from_alert_fast: start")
    content_v = []
    with open(fast_log_path) as f:
        content_v = f.readlines()
    print("extract_data_from_alert_fast: content_v: ",content_v)
    print("extract_data_from_alert_fast: len(content_v): ",len(content_v))
    
    position_to_visit_v = []
    for i,l in enumerate(content_v): 
        if (protocol in ['ipv4','ipv6'] and (l.startswith("snort.raw[") or l.startswith("snort.stream_ip["))) \
         or (protocol == 'tcp' and l.startswith("snort.stream_tcp[")) :
            position_to_visit_v.append(i + 2) 
            print("extract_data_from_alert_fast: position_to_visit: ",i + 2)

    if position_to_visit_v == []:
        return { "is_echo_reply": False, "number": 0, "payload": "" }

    first_payload_position = 0
    last_payload_position = 50
    payload_b_v = []
    for position_to_visit in position_to_visit_v:
        payload_hex_s = ""

        # retrieve data across lines
        curr_position_to_visit = position_to_visit
        while curr_position_to_visit < len(content_v) and not content_v[curr_position_to_visit].startswith("-"):
            l = content_v[curr_position_to_visit]
            print("extract_data_from_alert_fast: l:",l)
            payload_hex_s += l[first_payload_position:last_payload_position]
            print("extract_data_from_alert_fast: payload_hex_s:",payload_hex_s)
            curr_position_to_visit += 1

        payload_b = decode_hex_str_from_rules_to_bytes(payload_hex_s)
        print("extract_data_from_alert_fast: payload_b:",payload_b)

        payload_b_v.append(payload_b)

   
    payload_b_v_no_duplicates_v = list(dict.fromkeys(payload_b_v))
    if len(payload_b_v_no_duplicates_v) > 1:
        print("extract_data_from_payload_field: ERROR inconsistent payload fields: ",payload_b_v)
        sys.exit(-1)
    
    payload_b = payload_b_v[0]
    print("extract_data_from_alert_fast: payload_b:",payload_b)
    payload_s = extract_data(
            payload_b,
            nb_starting_character_to_remove,
            nb_final_character_to_remove,
            test_case_index,
            byte_time_sequence_hm,
            payload_mode
        )
    print("extract_data_from_alert_fast: payload_s:",payload_s)

    if not are_individual_pattern_positions_correct(index_data_offset_d,test_case_index,payload_mode,payload_s):
        print("extract_data_from_alert_fast: WARNING: pattern with incorrect position")

    print("extract_data_from_alert_fast: end")
    return { "is_echo_reply": payload_s != "", "number": int(payload_s != ""), "payload": payload_s }
    
def extract_data_from_alert_json(
    index_data_offset_d:dict,
    test_case_index:str, 
    json_log_path:str, 
    byte_time_sequence_hm:dict, 
    nb_starting_character_to_remove: int,
    nb_final_character_to_remove: int,
    payload_mode: str,
):
    print("extract_data_from_alert_json: start")
    print("extract_data_from_alert_json: json_log_path: ",json_log_path)
    with open(json_log_path) as f:
        json_content_v = [ json.loads(jsonObj) for jsonObj in f ]
    print("extract_data_from_alert_json: len(json_content): ",len(json_content_v))

    if json_content_v == []:
        return { "is_echo_reply": False, "number": 0, "payload": "" }
    payload_base64_v = [ json_content['b64_data'] for json_content in json_content_v]
    print("extract_data_from_alert_json: payload_base64_v: ",payload_base64_v)
    payload_base64_no_duplicates_v = list(dict.fromkeys(payload_base64_v))
    if len(payload_base64_no_duplicates_v) > 1:
        print("extract_data_from_alert_json: ERROR inconsistent payload fields: ",payload_base64_no_duplicates_v)
        sys.exit(-1)

    payload_base64 = payload_base64_v[0]
    payload_b = base64.b64decode(payload_base64)
    print("extract_data_from_alert_json: payload_b: ",payload_b)

    payload_s = extract_data(
        payload_b,
        nb_starting_character_to_remove,
        nb_final_character_to_remove,
        test_case_index,
        byte_time_sequence_hm,
        payload_mode
    )
    print("extract_data_from_alert_json: payload_s: ",payload_s)
    
    if not are_individual_pattern_positions_correct(index_data_offset_d,test_case_index,payload_mode,payload_s):
        print("extract_data_from_alert_json: WARNING: pattern with incorrect position")

    print("extract_data_from_alert_json: end")
    return { "is_echo_reply": payload_s != "", "number": int(payload_s != ""), "payload": payload_s }

def extract_data(
    payload_b:bytes,
    nb_starting_character_to_remove: int,
    nb_final_character_to_remove: int,
    test_case_index:str,
    byte_time_sequence_hm: dict,
    payload_mode: str
) -> str:
    payload_len = len(payload_b)
    print("extract_data: payload_len:",payload_len)

    if payload_len == 0: 
        return ""
    
    if payload_len < nb_starting_character_to_remove:
        print(f"Not enough extra starting bytes to remove in payload ({payload_len} < {nb_starting_character_to_remove})")
        sys.exit(-1)

    # remove extra bytes
    ## we first remove starting extra bytes
    payload_no_starting_extra_char_b = payload_b if nb_starting_character_to_remove == 0 else payload_b[nb_starting_character_to_remove:]
    print("extract_data: payload_no_starting_extra_char_b: ",payload_no_starting_extra_char_b)

    ## we then ensure that we can remove ending extra bytes
    tc_theorical_ending_byte_offset = get_tc_theorical_ending_byte_offset(test_case_index,byte_time_sequence_hm,payload_mode)
    print("extract_data: tc_theorical_ending_byte_offset: ",tc_theorical_ending_byte_offset)
    print("extract_data: nb_final_character_to_remove: ",nb_final_character_to_remove)
    if nb_final_character_to_remove == 0 or tc_theorical_ending_byte_offset + nb_final_character_to_remove > len(payload_no_starting_extra_char_b):
        payload_no_extra_char_b = payload_no_starting_extra_char_b 
    else:
        payload_no_extra_char_b = payload_no_starting_extra_char_b[:-nb_final_character_to_remove]
    print("extract_data: payload_no_extra_char_b: ",payload_no_extra_char_b)

    # Check that there is no duplicated patterns in reconstructed payload 
    offset_multiplier = get_offset_multiplier(payload_mode)
    payload_sliced_by_byte_a = [ payload_no_extra_char_b[j:j+offset_multiplier] for j in range(0,len(payload_no_extra_char_b),offset_multiplier) ]
    print("extract_data: payload_sliced_by_byte_a: ",payload_sliced_by_byte_a)
    payload_sliced_by_byte_a_no_duplicates = list(dict.fromkeys(payload_sliced_by_byte_a))
    print("extract_data: payload_sliced_by_byte_a_no_duplicates: ",payload_sliced_by_byte_a_no_duplicates)
    payload_s = b''.join(payload_sliced_by_byte_a_no_duplicates)

    if payload_sliced_by_byte_a != payload_sliced_by_byte_a_no_duplicates:
        print("\nextract_data: WARNING: duplicates in initial payload")

    # from bytes to ascii
    # for now, observed duplication are '........', we keep them
    #payload_ascii_s = payload_s.decode('utf-8','replace')
    payload_ascii_s = payload_no_extra_char_b.decode('utf-8','replace')
    print("extract_data: payload_ascii_s: ",payload_ascii_s)
        
    print("extract_data: end")
    return payload_ascii_s
    
def decode_hex_str_from_rules_to_bytes(hex_str:str) -> bytes:
    return bytes.fromhex(hex_str.replace(" ", ""))

def are_individual_pattern_positions_correct(
    index_data_offset_d:dict,
    test_case_index:str,
    payload_mode:str,
    payload_s:str
) -> bool:
    print("are_individual_pattern_positions_correct: start")

    offset_multiplier = get_offset_multiplier(payload_mode)
    index_data_offset = index_data_offset_d[test_case_index]
    print("are_individual_pattern_positions_correct: index_data_offset: ",index_data_offset)

    payload_sliced_by_byte_a = [ payload_s[j:j+offset_multiplier] for j in range(0,len(payload_s),offset_multiplier) ]
    print("are_individual_pattern_positions_correct: payload_sliced_by_byte_a: ",payload_sliced_by_byte_a)
    for (i,payload_sliced_by_byte) in enumerate(payload_sliced_by_byte_a):
        if payload_sliced_by_byte in index_data_offset and index_data_offset[payload_sliced_by_byte] != i:
            return False
    print("are_individual_pattern_positions_correct: end")
    return True

def get_tc_theorical_ending_byte_offset(
    test_case_index:str,
    byte_time_sequence_hm:dict,
    payload_mode: str
):
    print("get_tc_theorical_ending_byte_offset: start")

    if int(test_case_index) <= 12: 
        byte_time_sequence_d = byte_time_sequence_hm['byte_time_pair_sequence_c']['hm'].get(test_case_index)
    elif int(test_case_index) >= 100 and int(test_case_index) <= 508: 
        byte_time_sequence_d = byte_time_sequence_hm['byte_time_triplet_sequence_c']['hm'].get(test_case_index)
    else:
        print(f"Test case index {test_case_index} is not a pair nor a triplet")
        exit(-1)

    print("get_tc_theorical_ending_byte_offset: byte_time_sequence_d: ",byte_time_sequence_d)
    if byte_time_sequence_d == None:
        print(f"Test case index {test_case_index} not present in merged byte_time_sequence json file")
        exit(-1)

    offset_multiplier = get_offset_multiplier(payload_mode)

    end_v = [ interval_d['end'] for interval_d in byte_time_sequence_d['interval_c']['hm'].values() ]
    print("get_tc_theorical_ending_byte_offset: end")
    return (max(end_v) + 1) * offset_multiplier


def merge_d_l(d_l):
    r = {}
    for d in d_l:
        r.update(d)
    return r

def extract_chunk_piece_offset_data(
        payload_mode:str, 
        chunk_data_d:dict, 
        test_case_index:int
    ):
    print("extract_chunk_piece_offset_data: start")
    print("extract_chunk_piece_offset_data: test_case_index: ",str(test_case_index))    
    print("extract_chunk_piece_offset_data: chunk_data_d: ",chunk_data_d)
    
    if payload_mode == "vc1b":
        data_s = chunk_data_d["simple_s"]
    elif payload_mode == "icfl8b":
        data_s = chunk_data_d["internet_checksum_s"]
    elif payload_mode == "icvl8i4":
        data_s = chunk_data_d["ipv4_invariant_checksum_s"]
    elif payload_mode == "icvl8i6":
        data_s = chunk_data_d["ipv6_invariant_checksum_s"]
    offset_multiplier = get_offset_multiplier(payload_mode)
        
    # We split the chunk string into several piece for each pattern.
    piece_l = [ data_s[i:i+offset_multiplier] for i in range(0, len(data_s), offset_multiplier) ]
    
    print("extract_chunk_piece_offset_data: piece_l: ",str(piece_l))
    
    # We add the chunk offset to the position.
    piece_position_l = [ position + chunk_data_d["offset"] for position in list(range(0, len(piece_l))) ]
    
    print("extract_chunk_piece_offset_data: end")
    
    return { piece: position for piece, position in zip(piece_l, piece_position_l) }

def build_index_data_offset_d(payload_mode, byte_time_sequence_json_path):
    print("build_index_data_offset_d: start")
    
    with open(byte_time_sequence_json_path) as f:
        json_d = json.load(f)
    # print(d)
    pair_d = json_d["byte_time_pair_sequence_c"]["hm"]
    triplet_d = json_d["byte_time_triplet_sequence_c"]["hm"]
    
    index_data_offset_d_pair_tmp = { test_case_index: [ extract_chunk_piece_offset_data(payload_mode, chunk_data_d, test_case_index) for chunk_i, chunk_data_d in pair_test_case_d["chunk_c"]["bm"].items() ] for test_case_index, pair_test_case_d in pair_d.items() }
    
    print("build_index_data_offset_d: index_data_offset_d_pair_tmp: ",str(index_data_offset_d_pair_tmp))

    index_data_offset_d_pair = { test_case_index: merge_d_l(d_l) for test_case_index, d_l in index_data_offset_d_pair_tmp.items() }
    
    print("build_index_data_offset_d: index_data_offset_d_pair: ",str(index_data_offset_d_pair))
    
    index_data_offset_d_triplet_tmp = { test_case_index: [ extract_chunk_piece_offset_data(payload_mode, chunk_data_d, test_case_index) for chunk_i, chunk_data_d in triplet_test_case_d["chunk_c"]["bm"].items() ] for test_case_index, triplet_test_case_d in triplet_d.items() }
    
    index_data_offset_d_triplet = { test_case_index: merge_d_l(d_l) for test_case_index, d_l in index_data_offset_d_triplet_tmp.items() }
    
    index_data_offset_d = merge_d_l([ index_data_offset_d_pair, index_data_offset_d_triplet ])
    
    print("build_index_data_offset_d: end")
    
    return index_data_offset_d     
    
def extract_test_case_index(file_path:str) -> str:
    print("extract_test_case_index: start")
    print("extract_test_case_index: file_path: ",str(file_path))
    bn = os.path.basename(file_path)
    print("extract_test_case_index: bn: ",str(bn))
    bn_wo_ext = os.path.splitext(bn)[0]
    print("extract_test_case_index: bn_wo_ext: ",str(bn_wo_ext))
    index = bn_wo_ext.split("_")[-1]
    print("extract_test_case_index: index: ",index)
    print("extract_test_case_index: start")
    return index

def get_offset_multiplier(payload_mode:str) -> int:
    if payload_mode == "vc1b":
        return 1
    elif payload_mode == "icfl8b":
        return 8
    elif payload_mode == "icvl8i4":
        return 8
    elif payload_mode == "icvl8i6":
        return 8
    else:
        print('Invalid payload_mode: ',payload_mode)
        sys.exit(-1)

def process(
        log_directory: str,
        json_output_path: str, 
        merged_byte_time_sequence_json_path: str,
        nb_starting_character_to_remove: int,
        nb_final_character_to_remove: int,
        protocol:str,
        payload_mode:str,
        extraction_mode:str
):
    print('process: start')
    index_data_offset_d = build_index_data_offset_d(payload_mode, merged_byte_time_sequence_json_path)

    #fast_log_directory = f"{log_directory}/alert_fast"
    #print('process: fast_log_directory: ',fast_log_directory)
    #json_log_directory = f"{log_directory}/alert_json"
    #print('process: json_log_directory: ',json_log_directory)

    #fast_filename_l = os.listdir(fast_log_directory)
    #json_filename_l = os.listdir(json_log_directory)
    #fast_path_l = [ os.path.join(fast_log_directory, filename) for filename in fast_filename_l ]
    #json_path_l = [ os.path.join(json_log_directory, filename) for filename in json_filename_l ]
    ## We sort to have a deterministic processing order to ease debugging accross machines.
    #fast_path_l.sort()
    #json_path_l.sort()
    json_filename_l = os.listdir(log_directory)
    json_path_l = [ os.path.join(log_directory, filename) for filename in json_filename_l ]
    json_path_l.sort()

    with open(merged_byte_time_sequence_json_path) as json_file: 
        byte_time_sequence_hm = json.load(json_file)
    
    if extraction_mode == 'alert_json':
        index_payload_d = { extract_test_case_index(log_path): extract_data_from_alert_json(index_data_offset_d,extract_test_case_index(log_path),log_path,byte_time_sequence_hm,nb_starting_character_to_remove,nb_final_character_to_remove,payload_mode) for log_path in json_path_l }

        #other_index_payload_d = { extract_test_case_index(log_path): extract_data_from_alert_fast(index_data_offset_d,extract_test_case_index(log_path),log_path,byte_time_sequence_hm,nb_starting_character_to_remove,nb_final_character_to_remove,payload_mode,protocol) for log_path in fast_path_l }
    else:
        index_payload_d = { extract_test_case_index(log_path): extract_data_from_alert_fast(index_data_offset_d,extract_test_case_index(log_path),log_path,byte_time_sequence_hm,nb_starting_character_to_remove,nb_final_character_to_remove,payload_mode,protocol) for log_path in fast_path_l }

        other_index_payload_d = { extract_test_case_index(log_path): extract_data_from_alert_json(index_data_offset_d,extract_test_case_index(log_path),log_path,byte_time_sequence_hm,nb_starting_character_to_remove,nb_final_character_to_remove,payload_mode) for log_path in json_path_l }

    # Consistency check across method extraction
    #if index_payload_d != other_index_payload_d:
    #    print(f"\n\nprocess: WARNING: payload inconsistencies across extraction modes")
    #    inconsistent_tc_d = [ index_sig for (index_sig,payload_d),(_,other_payload_d) in zip(index_payload_d.items(),other_index_payload_d.items()) if payload_d != other_payload_d ]
    #    for tc_index in inconsistent_tc_d:
    #        print(f"process: tc_index ({tc_index}): payload_d ({index_payload_d[tc_index]}) != other_payload_d ({other_index_payload_d[tc_index]})")

    index_i_payload_d = { int(index):payload_d for index,payload_d in index_payload_d.items() }
    data_d = { "hm": index_i_payload_d }

    with open(json_output_path, 'w', encoding="UTF8") as fp:
        json.dump(data_d, fp, indent=2, sort_keys=True)
    print('process: end')

def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--log-directory", type=str, default="")
    parser.add_argument("-m", "--merged-byte-time-sequence-json-path", type=str, default="")
    parser.add_argument("-j", "--json-path", type=str, default="")
    parser.add_argument("-p", "--protocol", type=str, default="")
    parser.add_argument("-s", "--nb-starting-character-to-remove", type=int, default=0)
    parser.add_argument("-f", "--nb-final-character-to-remove", type=int, default=0)
    parser.add_argument("-pm", "--payload-mode", type=str, choices=['vc1b','icfl8b','icvl8i4','icvl8i6'])
    parser.add_argument("-em", "--extraction-mode", choices=['alert_json','alert_fast'],required=True)
    args = parser.parse_args()

    log_directory = args.log_directory
    merged_byte_time_sequence_json_path = args.merged_byte_time_sequence_json_path
    json_path = args.json_path
    protocol = args.protocol
    nb_starting_character_to_remove = args.nb_starting_character_to_remove
    nb_final_character_to_remove = args.nb_final_character_to_remove
    payload_mode = args.payload_mode
    extraction_mode = args.extraction_mode

    print('extract_payload_from_snort_log_directory_to_json: log_directory: "%s"'%(log_directory))
    print('extract_payload_from_snort_log_directory_to_json: json_path: "%s"'%(json_path))
    print('extract_payload_from_snort_log_directory_to_json: merged_byte_time_sequence_json_path: "%s"'%(merged_byte_time_sequence_json_path))
    print('extract_payload_from_snort_log_directory_to_json: protocol: "%s"'%(protocol))
    print('extract_payload_from_snort_log_directory_to_json: nb_starting_character_to_remove: "%s"'%(nb_starting_character_to_remove))
    print('extract_payload_from_snort_log_directory_to_json: nb_final_character_to_remove: "%s"'%(nb_final_character_to_remove))
    print('extract_payload_from_snort_log_directory_to_json: payload_mode: "%s"'%(payload_mode))
    print('extract_payload_from_snort_log_directory_to_json: extraction_mode: "%s"'%(extraction_mode))
    
    process(log_directory,
            json_path,
            merged_byte_time_sequence_json_path,
            nb_starting_character_to_remove,
            nb_final_character_to_remove,
            protocol,
            payload_mode,
            extraction_mode
    )

if __name__ == "__main__":
   main(sys.argv[1:])
