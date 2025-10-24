#!/usr/bin/python
import sys
import os
import json
import argparse
import base64

def get_tc_theorical_ending_byte_offset(
    test_case_index:int,
    byte_time_sequence_hm:dict
):
    print("get_tc_theorical_ending_byte_offset: start")

    if test_case_index <= 12: 
        byte_time_sequence_d = byte_time_sequence_hm['byte_time_pair_sequence_c']['hm'].get(str(test_case_index))
    elif test_case_index >= 100 and test_case_index <= 508: 
        byte_time_sequence_d = byte_time_sequence_hm['byte_time_triplet_sequence_c']['hm'].get(str(test_case_index))
    else:
        print(f"Test case index {test_case_index} is not a pair nor a triplet")
        exit(-1)

    print("extract_data: byte_time_sequence_d: ",byte_time_sequence_d)
    if byte_time_sequence_d == None:
        print(f"Test case index {test_case_index} not present in merged byte_time_sequence json file")
        exit(-1)

    # XXX if 's' payload_mode is kept, pass payload_mode as argument and modify accordingly offset_multiplier
    offset_multiplier = 8 

    end_v = [ interval_d['end'] for interval_d in byte_time_sequence_d['interval_c']['hm'].values() ]
    print("get_tc_theorical_ending_byte_offset: end")
    return (max(end_v) + 1) * offset_multiplier


def extract_data(
        log_path:str, 
        byte_time_sequence_hm:dict, 
        nb_starting_character_to_remove: int,
        nb_final_character_to_remove: int,
):
    print("\n\nextract_data: log_path:",log_path)
    
    test_case_index = extract_test_case_index(log_path)
    print("extract_data: test_case_index: ",test_case_index)

    content = []
    with open(log_path) as f:
        content = f.readlines()

    print("extract_data: content: ",content)
    print("extract_data: len(content): ",len(content))
    
    position_to_visit_v = []
    for i,l in enumerate(content): 
        if l.startswith("snort.raw[") or l.startswith("snort.stream_tcp[") or l.startswith("snort.stream_ip["):
            position_to_visit_v.append(i + 2) 
            print("extract_data: position_to_visit: ",i + 2)

    if position_to_visit_v == []:
        return { "is_echo_reply": False, "number": 0, "payload": ""}
    
    first_payload_position = 0
    last_payload_position = 50
    payload_v = []
    for position_to_visit in position_to_visit_v:
        payload_hex_s = ""

        # retrieve data across lines
        curr_position_to_visit = position_to_visit
        while curr_position_to_visit < len(content) and not content[curr_position_to_visit].startswith("-"):
            l = content[curr_position_to_visit]
            print("extract_data: l:",l)
            payload_hex_s += l[first_payload_position:last_payload_position]
            print("extract_data: payload_hex_s:",payload_hex_s)
            curr_position_to_visit += 1

        payload_hex_no_space_hex_s = payload_hex_s.replace(" ", "")
        print("extract_data: payload_hex_no_space_hex_s:",payload_hex_no_space_hex_s)
        payload_b = bytes.fromhex(payload_hex_no_space_hex_s)
        print("extract_data: payload_b:",payload_b)
        payload_len = len(payload_b)
        print("extract_data: payload_len:",payload_len)

        if payload_len == 0: 
            continue

        if payload_len < nb_starting_character_to_remove:
            print(f"Not enough extra starting bytes to remove in payload ({payload_len} < {nb_starting_character_to_remove})")
            exit(-1)
        
        # remove extra bytes
        ## we first remove starting extra bytes
        payload_no_starting_extra_char_b = payload_b if nb_starting_character_to_remove == 0 else payload_b[nb_starting_character_to_remove:]
        print("extract_data: payload_no_starting_extra_char_b: ",payload_no_starting_extra_char_b)

        ## we then ensure that we can remove ending extra bytes
        tc_theorical_ending_byte_offset = get_tc_theorical_ending_byte_offset(test_case_index,byte_time_sequence_hm)
        print("extract_data: tc_theorical_ending_byte_offset: ",tc_theorical_ending_byte_offset)
        print("extract_data: nb_final_character_to_remove: ",nb_final_character_to_remove)
        if nb_final_character_to_remove == 0 or tc_theorical_ending_byte_offset + nb_final_character_to_remove > len(payload_no_starting_extra_char_b):
            payload_no_extra_char_b = payload_no_starting_extra_char_b 
        else:
            payload_no_extra_char_b = payload_no_starting_extra_char_b[:-nb_final_character_to_remove]
        print("extract_data: payload_no_extra_char_b: ",payload_no_extra_char_b)

        # from bytes to ascii
        payload_ascii_s = payload_no_extra_char_b.decode('utf-8','replace')
        print("extract_data: payload_ascii_s: ",payload_ascii_s)
        payload_v.append(payload_ascii_s)
    

    payload_no_duplicates_v = list(dict.fromkeys(payload_v))
    if len(payload_no_duplicates_v) == 1:
        return { "is_echo_reply": True, "number": 1, "payload": payload_no_duplicates_v[0] }
    elif len(payload_no_duplicates_v) > 1:
        print(f"extract_data: test index {test_case_index} has inconsistent reassembled payload across alerts ({payload_no_duplicates_v})")
        return { "is_echo_reply": True, "number": len(payload_no_duplicates_v), "payload": "" }

    return { "is_echo_reply": False, "number": 0, "payload": "" }

    
def extract_test_case_index(file_path: str):
    print("extract_test_case_index: start")
    print("extract_test_case_index: file_path: ",str(file_path))
    bn = os.path.basename(file_path)
    print("extract_test_case_index: bn: ",str(bn))
    bn_wo_ext = os.path.splitext(bn)[0]
    print("extract_test_case_index: bn_wo_ext: ",str(bn_wo_ext))
    index = bn_wo_ext.split("_")[-1]
    print("extract_test_case_index: index: ",str(index))
    print("extract_test_case_index: end")
    return int(index)

def process(log_directory: str,
            json_output_path: str, 
            merged_byte_time_sequence_json_path: str,
            nb_starting_character_to_remove: int,
            nb_final_character_to_remove: int
):
    filename_l = os.listdir(log_directory)
    
    path_l = [ os.path.join(log_directory, filename) for filename in filename_l ]

    with open(merged_byte_time_sequence_json_path) as json_file: 
        byte_time_sequence_hm = json.load(json_file)

    # We sort to have a deterministic processing order to ease debugging accross machines.
    path_l.sort()

    index_payload_d = { 
        extract_test_case_index(log_path): 
        extract_data(
            log_path, 
            byte_time_sequence_hm, 
            nb_starting_character_to_remove, 
            nb_final_character_to_remove
            ) for log_path in path_l }

    index_i_payload_d = { int(index):payload_d for index,payload_d in index_payload_d.items() }
    data_d = { "hm": index_i_payload_d }

    print("process: data_d: ",data_d)

    with open(json_output_path, 'w', encoding="UTF8") as fp:
        json.dump(data_d, fp, indent=2, sort_keys=True)

def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--log-directory", type=str, default="")
    parser.add_argument("-m", "--merged-byte-time-sequence-json-path", type=str, default="")
    parser.add_argument("-j", "--json-path", type=str, default="")
    parser.add_argument("-p", "--protocol", type=str, default="")
    parser.add_argument("-s", "--nb-starting-character-to-remove", type=int, default=0)
    parser.add_argument("-f", "--nb-final-character-to-remove", type=int, default=0)
    args = parser.parse_args()

    log_directory = args.log_directory
    merged_byte_time_sequence_json_path = args.merged_byte_time_sequence_json_path
    json_path = args.json_path
    protocol = args.protocol
    nb_starting_character_to_remove = args.nb_starting_character_to_remove
    nb_final_character_to_remove = args.nb_final_character_to_remove

    print('extract_payload_from_snort_log_directory_to_json: log_directory: "%s"'%(log_directory))
    print('extract_payload_from_snort_log_directory_to_json: json_path: "%s"'%(json_path))
    print('extract_payload_from_snort_log_directory_to_json: merged_byte_time_sequence_json_path: "%s"'%(merged_byte_time_sequence_json_path))
    print('extract_payload_from_snort_log_directory_to_json: protocol: "%s"'%(protocol))
    print('extract_payload_from_snort_log_directory_to_json: nb_starting_character_to_remove: "%s"'%(nb_starting_character_to_remove))
    print('extract_payload_from_snort_log_directory_to_json: nb_final_character_to_remove: "%s"'%(nb_final_character_to_remove))
    
    process(log_directory,
            json_path,
            merged_byte_time_sequence_json_path,
            nb_starting_character_to_remove,
            nb_final_character_to_remove
    )

if __name__ == "__main__":
   main(sys.argv[1:])
