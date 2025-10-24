#!/usr/bin/python
import sys
import os
import json
import argparse
import base64

def extract_data_from_alert_json(
    test_case_index:str, 
    json_log_path:str, 
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
        payload_mode
    )
    print("extract_data_from_alert_json: payload_s: ",payload_s)
    
    print("extract_data_from_alert_json: end")
    return { "is_echo_reply": payload_s != "", "number": int(payload_s != ""), "payload": payload_s }

def extract_data(
    payload_b:bytes,
    nb_starting_character_to_remove: int,
    nb_final_character_to_remove: int,
    test_case_index:str,
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

    ## we hypothesize that we can remove ending extra bytes
    if nb_final_character_to_remove == 0 or tc_theorical_ending_byte_offset + nb_final_character_to_remove > len(payload_no_starting_extra_char_b):
        payload_no_extra_char_b = payload_no_starting_extra_char_b 
    else:
        payload_no_extra_char_b = payload_no_starting_extra_char_b[:-nb_final_character_to_remove]
    print("extract_data: payload_no_extra_char_b: ",payload_no_extra_char_b)

    # from bytes to ascii
    # for now, observed duplication are '........', we keep them
    #payload_ascii_s = payload_s.decode('utf-8','replace')
    payload_ascii_s = payload_no_extra_char_b.decode('utf-8','replace')
    print("extract_data: payload_ascii_s: ",payload_ascii_s)
        
    print("extract_data: end")
    return payload_ascii_s
    
def decode_hex_str_from_rules_to_bytes(hex_str:str) -> bytes:
    return bytes.fromhex(hex_str.replace(" ", ""))

def merge_d_l(d_l):
    r = {}
    for d in d_l:
        r.update(d)
    return r

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
        nb_starting_character_to_remove: int,
        nb_final_character_to_remove: int,
        protocol:str,
        payload_mode:str,
        extraction_mode:str
):
    print('process: start')

    json_filename_l = os.listdir(log_directory)
    json_path_l = [ os.path.join(log_directory, filename) for filename in json_filename_l ]
    json_path_l.sort()

    if extraction_mode == 'alert_json':
        index_payload_d = { extract_test_case_index(log_path): extract_data_from_alert_json(extract_test_case_index(log_path),log_path,nb_starting_character_to_remove,nb_final_character_to_remove,payload_mode) for log_path in json_path_l }
    else:
        print('process: extraction mode not implemented!!!')

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
    json_path = args.json_path
    protocol = args.protocol
    nb_starting_character_to_remove = args.nb_starting_character_to_remove
    nb_final_character_to_remove = args.nb_final_character_to_remove
    payload_mode = args.payload_mode
    extraction_mode = args.extraction_mode

    print('extract_midpoint_payload_from_snort_log_directory_to_json_custom: log_directory: "%s"'%(log_directory))
    print('extract_midpoint_payload_from_snort_log_directory_to_json_custom: json_path: "%s"'%(json_path))
    print('extract_midpoint_payload_from_snort_log_directory_to_json_custom: protocol: "%s"'%(protocol))
    print('extract_midpoint_payload_from_snort_log_directory_to_json_custom: nb_starting_character_to_remove: "%s"'%(nb_starting_character_to_remove))
    print('extract_midpoint_payload_from_snort_log_directory_to_json_custom: nb_final_character_to_remove: "%s"'%(nb_final_character_to_remove))
    print('extract_midpoint_payload_from_snort_log_directory_to_json_custom: payload_mode: "%s"'%(payload_mode))
    print('extract_midpoint_payload_from_snort_log_directory_to_json_custom: extraction_mode: "%s"'%(extraction_mode))
    
    process(log_directory,
            json_path,
            nb_starting_character_to_remove,
            nb_final_character_to_remove,
            protocol,
            payload_mode,
            extraction_mode
    )

if __name__ == "__main__":
   main(sys.argv[1:])
