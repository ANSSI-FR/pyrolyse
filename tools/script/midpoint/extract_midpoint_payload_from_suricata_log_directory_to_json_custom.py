#!/usr/bin/python
import sys
import os
import json
import argparse
import base64
from distutils.util import strtobool


def extract_data_from_payload_field(
    test_case_index:str, 
    log_path:str, 
    nb_starting_character_to_remove: int,
    nb_final_character_to_remove: int,
    payload_mode: str,
    protocol:str
):
    print("\n\nextract_data_from_payload_field: start")
    print("extract_data_from_payload_field: log_path:",log_path)
    print("extract_data_from_payload_field: test_case_index: ",str(test_case_index))
    
    with open(log_path) as f:
        content = [ json.loads(jsonObj) for jsonObj in f ]

    payload_v = [ extract_data(log["payload"],nb_starting_character_to_remove,nb_final_character_to_remove,test_case_index,payload_mode) for log in content if "payload" in log.keys() ]
    if protocol == 'tcp':
        payload_v_w_stream = [ extract_data(log["payload"],nb_starting_character_to_remove,nb_final_character_to_remove,test_case_index,payload_mode) for log in content if "payload" in log.keys() and "stream" in log.keys() and log["stream"] == 1 ]
        assert payload_v == payload_v_w_stream
    payload_v_no_duplicates = list(set(payload_v))

    if len(payload_v_no_duplicates) == 0:
        payload_str = ""
    elif len(payload_v_no_duplicates) == 1:
        payload_str = payload_v_no_duplicates[0]
    else:
        # TODO for IP with the multiple MF unsetting strategies, we can have several inconsistent payload fields. What should we do ? 
        print("extract_data_from_payload_field: ERROR inconsistent payload fields: ",payload_v_no_duplicates)
        sys.exit(-1)
    print("extract_data_from_payload_field: payload_str: ",payload_str)

    
    return { "is_echo_reply": payload_str != "", "number": int(payload_str != ""), "payload": payload_str }


def extract_data(
    payload_base_64:str,
    nb_starting_character_to_remove: int,
    nb_final_character_to_remove: int,
    test_case_index:str,
    payload_mode: str
) -> str:
    
    payload_b = base64.b64decode(payload_base_64)
    print("extract_data: payload_b: ",payload_b)
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

    ## we hypothesize we can remove ending extra bytes
    if nb_final_character_to_remove == 0 or tc_theorical_ending_byte_offset + nb_final_character_to_remove > len(payload_no_starting_extra_char_b):
        payload_no_extra_char_b = payload_no_starting_extra_char_b 
    else:
        payload_no_extra_char_b = payload_no_starting_extra_char_b[:-nb_final_character_to_remove]
    print("extract_data: payload_no_extra_char_b: ",payload_no_extra_char_b)

    # There are duplicated patterns in field payload (see report suricata-tcp-bug) - we thus remove them
    # We do not remove duplicated patterns for custom
    #offset_multiplier = get_offset_multiplier(payload_mode)
    #payload_sliced_by_byte_a = [ payload_no_extra_char_b[j:j+offset_multiplier] for j in range(0,len(payload_no_extra_char_b),offset_multiplier) ]
    #print("extract_data: payload_sliced_by_byte_a: ",payload_sliced_by_byte_a)
    #payload_sliced_by_byte_a_no_duplicates = list(dict.fromkeys(payload_sliced_by_byte_a))
    #print("extract_data: payload_sliced_by_byte_a_no_duplicates: ",payload_sliced_by_byte_a_no_duplicates)
    #payload_s = b''.join(payload_sliced_by_byte_a_no_duplicates)


    # from bytes to ascii
    #payload_ascii_s = payload_s.decode('utf-8','replace')
    payload_ascii_s = payload_no_extra_char_b.decode('utf-8','replace')
    print("extract_data: payload_ascii_s: ",payload_ascii_s)
    
    return payload_ascii_s

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

def decode_hex_str_from_rules_to_bytes(hex_str:str) -> bytes:
    return bytes.fromhex(hex_str.replace(" ", ""))

def process(log_directory: str,
            json_output_path: str, 
            nb_starting_character_to_remove: int,
            nb_final_character_to_remove: int,
            protocol:str,
            payload_mode:str,
            extraction_mode:bool
):
    filename_l = os.listdir(log_directory)
    path_l = [ os.path.join(log_directory, filename) for filename in filename_l ]
    ## We sort to have a deterministic processing order to ease debugging accross machines.
    path_l.sort()
    
    if extraction_mode == 'payload':
        index_payload_d = { extract_test_case_index(log_path): extract_data_from_payload_field(extract_test_case_index(log_path),log_path,nb_starting_character_to_remove,nb_final_character_to_remove,payload_mode,protocol) for log_path in path_l }
    else:
        print('process: extraction mode not implemented!!!')

    index_i_payload_d = { int(index):payload_d for index,payload_d in index_payload_d.items() }
    data_d = { "hm": index_i_payload_d }

    with open(json_output_path, 'w', encoding="UTF8") as fp:
        json.dump(data_d, fp, indent=2, sort_keys=True)

def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--log-directory", type=str)
    parser.add_argument("-m", "--merged-byte-time-sequence-json-path", type=str)
    parser.add_argument("-j", "--json-path", type=str)
    parser.add_argument("-p", "--protocol", type=str, choices=['ipv4','ipv6','tcp'])
    parser.add_argument("-s", "--nb-starting-character-to-remove", type=int, default=0)
    parser.add_argument("-f", "--nb-final-character-to-remove", type=int, default=0)
    parser.add_argument("-pm", "--payload-mode", type=str, choices=['vc1b','icfl8b','icvl8i4','icvl8i6'])
    parser.add_argument("-em", "--extraction-mode", choices=['signatures','payload'],required=True)
    args = parser.parse_args()

    log_directory = args.log_directory
    json_path = args.json_path
    nb_starting_character_to_remove = args.nb_starting_character_to_remove
    nb_final_character_to_remove = args.nb_final_character_to_remove
    protocol = args.protocol
    payload_mode = args.payload_mode
    extraction_mode = args.extraction_mode

    print('extract_payload_from_suricata_log_directory_to_json: log_directory: "%s"'%(log_directory))
    print('extract_payload_from_suricata_log_directory_to_json: json_path: "%s"'%(json_path))
    print('extract_payload_from_suricata_log_directory_to_json: nb_starting_character_to_remove: "%s"'%(nb_starting_character_to_remove))
    print('extract_payload_from_suricata_log_directory_to_json: nb_final_character_to_remove: "%s"'%(nb_final_character_to_remove))
    print('extract_payload_from_suricata_log_directory_to_json: protocol: "%s"'%(protocol))
    print('extract_payload_from_suricata_log_directory_to_json: payload_mode: "%s"'%(payload_mode))
    print('extract_payload_from_suricata_log_directory_to_json: extraction_mode: "%s"'%(extraction_mode))
    
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
