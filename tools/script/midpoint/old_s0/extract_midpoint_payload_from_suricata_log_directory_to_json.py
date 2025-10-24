#!/usr/bin/python
import sys
import os
import json
import argparse
import base64

# TODO build payloads from alerts 
# TODO build payloads from payload field but removing pattern repeatition ?

# TODO check if we want to keep this function
def merge_d_l(d_l):
    r = {}
    for d in d_l:
        r.update(d)
    return r

# TODO check if we want to keep this function
def extract_chunk_piece_offset_data(payload_mode, chunk_data_d, test_case_index):
    print("extract_chunk_piece_offset_data: start")
    print("extract_chunk_piece_offset_data: test_case_index: ",str(test_case_index))    
    print("extract_chunk_piece_offset_data: chunk_data_d: ",str(chunk_data_d))
    
    if payload_mode == 'vc1b':
        data_s = chunk_data_d["simple_s"]
        piece_size = 1
    elif payload_mode == 'icvl8i4':
        data_s = chunk_data_d["ipv4_invariant_checksum_s"]
        piece_size = 8
    elif payload_mode == 'icvl8i6':
        data_s = chunk_data_d["ipv6_invariant_checksum_s"]
        piece_size = 8
    else:
        data_s = chunk_data_d["internet_checksum_s"]
        piece_size = 8

    # We split the chunk string into several piece for each pattern.
    piece_l = [ data_s[i:i+piece_size] for i in range(0, len(data_s), piece_size) ]
    
    print("extract_chunk_piece_offset_data: piece_l: ",str(piece_l))
    
    # We add the chunk offset to the position.
    piece_position_l = [ position + chunk_data_d["offset"] for position in list(range(0, len(piece_l))) ]
    print("extract_chunk_piece_offset_data: piece_position_l: ",piece_position_l)
    
    print("extract_chunk_piece_offset_data: end")
    
    return { piece: position for piece, position in zip(piece_l, piece_position_l) }

# TODO check if we want to keep this function
def build_index_data_offset_d(payload_mode, byte_time_sequence_json_path):
    print("build_index_data_offset_d: start")
    
    with open(byte_time_sequence_json_path) as f:
        json_d = json.load(f)
    # print(d)
    pair_d = json_d["byte_time_pair_sequence_c"]["hm"]
    triplet_d = json_d["byte_time_triplet_sequence_c"]["hm"]
    
    index_data_offset_d_pair_tmp = { test_case_index: [ extract_chunk_piece_offset_data(payload_mode, chunk_data_d, test_case_index) for chunk_i, chunk_data_d in pair_test_case_d["chunk_c"]["bm"].items() ] for test_case_index, pair_test_case_d in pair_d.items() }
    
    index_data_offset_d_pair = { test_case_index: merge_d_l(d_l) for test_case_index, d_l in index_data_offset_d_pair_tmp.items() }
    
    print("build_index_data_offset_d: index_data_offset_d_pair: ",str(index_data_offset_d_pair))
    
    index_data_offset_d_triplet_tmp = { test_case_index: [ extract_chunk_piece_offset_data(payload_mode, chunk_data_d, test_case_index) for chunk_i, chunk_data_d in triplet_test_case_d["chunk_c"]["bm"].items() ] for test_case_index, triplet_test_case_d in triplet_d.items() }
    
    index_data_offset_d_triplet = { test_case_index: merge_d_l(d_l) for test_case_index, d_l in index_data_offset_d_triplet_tmp.items() }
    
    index_data_offset_d = merge_d_l([ index_data_offset_d_pair, index_data_offset_d_triplet ])
    
    print("build_index_data_offset_d: end")
    
    return index_data_offset_d     

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

#def extract_data_ip(
#        log_path:str, 
#        byte_time_sequence_hm:dict, 
#        nb_starting_character_to_remove: int,
#        nb_final_character_to_remove: int,
#):
#    print("\n\nextract_data: log_path:",log_path)
#    test_case_index = extract_test_case_index(log_path)
#    print("extract_data: test_case_index: ",test_case_index)   
#    
#    with open(log_path) as f:
#        log_objects = [ json.loads(jsonObj) for jsonObj in f if "payload" in json.loads(jsonObj).keys() ]
#
#    payload_v = [ ]
#    for log_object in log_objects:
#        payload_b = base64.b64decode(log_object["payload"])
#        print("extract_data: payload_b: ",payload_b)
#        payload_len = len(payload_b)
#        print("extract_data: payload_len:",payload_len)
#
#        if payload_len == 0: 
#            continue
#        if payload_len < nb_starting_character_to_remove:
#            print(f"Not enough extra starting bytes to remove in payload ({payload_len} < {nb_starting_character_to_remove})")
#            exit(-1)

def extract_data(
        log_path:str, 
        byte_time_sequence_hm:dict, 
        nb_starting_character_to_remove: int,
        nb_final_character_to_remove: int,
):
    print("\n\nextract_data: log_path:",log_path)
    test_case_index = extract_test_case_index(log_path)
    print("extract_data: test_case_index: ",test_case_index)

    #protocol_s = os.path.dirname(log_path).split('/')[-1].split('_')[0]
    #print("extract_data: protocol_s:",protocol_s)
    #if protocol_s not in ["ipv4","ipv6","tcp"]:
    #    print('Error in log file path')
    #    exit(-1)
    
    log_objects = [ ]
    with open(log_path) as f:
        for jsonObj in f:
            o = json.loads(jsonObj)
            log_objects.append(o)

    payload_v = [ ]
    for log_object in log_objects:
        if "payload" in log_object.keys():
            payload_b = base64.b64decode(log_object["payload"])
            print("extract_data: payload_b: ",payload_b)
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
        payload_s = payload_v[0]
        print("extract_data: payload_s: ",payload_s)
        byte_payload = [payload_s[j:j+8] for j in range(0,len(payload_s),8)]
        print("extract_data: byte_payload: ",byte_payload)
        payload_s = ''.join(list(dict.fromkeys(byte_payload)))
        print("extract_data: payload_s: ",payload_s)
        return { "is_echo_reply": True, "number": 1, "payload": payload_s }
    else:
        print("extract_data: payload_no_duplicates_v: ",payload_no_duplicates_v)

    #payload_no_duplicates_v = list(dict.fromkeys(payload_v))
    #if len(payload_no_duplicates_v) == 1:
    #    return { "is_echo_reply": True, "number": 1, "payload": payload_no_duplicates_v[0] }
    #elif len(payload_no_duplicates_v) > 1:
    #    return { "is_echo_reply": True, "number": len(payload_no_duplicates_v), "payload": "" }

    return { "is_echo_reply": False, "number": 0, "payload": "" }

    
def extract_test_case_index(file_path: str) -> int:
    print("extract_test_case_index: start")
    print("extract_test_case_index: file_path: ",str(file_path))
    bn = os.path.basename(file_path)
    print("extract_test_case_index: bn: ",str(bn))
    bn_wo_ext = os.path.splitext(bn)[0]
    print("extract_test_case_index: bn_wo_ext: ",str(bn_wo_ext))
    index = bn_wo_ext.split("_")[-1]
    print("extract_test_case_index: index: ",str(index))
    print("extract_test_case_index: start")
    return int(index)


def process(log_directory: str,
            json_output_path: str, 
            merged_byte_time_sequence_json_path: str,
            nb_starting_character_to_remove: int,
            nb_final_character_to_remove: int
):
    filename_l = os.listdir(log_directory)
    
    path_l = [ os.path.join(log_directory, filename) for filename in filename_l ]

    # We sort to have a deterministic processing order to ease debugging accross machines.
    path_l.sort()

    with open(merged_byte_time_sequence_json_path) as json_file: 
        byte_time_sequence_hm = json.load(json_file)

    index_payload_d = {
        extract_test_case_index(log_path): 
        extract_data(
            log_path,
            byte_time_sequence_hm,
            nb_starting_character_to_remove, 
            nb_final_character_to_remove
        ) for log_path in path_l
    }

    data_d = { "hm": index_payload_d }

    print("process: data_d: ",data_d)

    with open(json_output_path, 'w') as fp:
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
