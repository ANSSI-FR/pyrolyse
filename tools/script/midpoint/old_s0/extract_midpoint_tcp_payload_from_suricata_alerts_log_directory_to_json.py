#!/usr/bin/python
import sys
import os
import json
import argparse

# import graph_utils as gu

def extract_data(index_data_offset_d, test_case_index, log_path, nb_final_character_to_remove):
    print("\n\nextract_data: log_path:",log_path)
    
    print("extract_data: test_case_index: ",str(test_case_index))
    
    print("extract_data: index_data_offset_d[",str(test_case_index),"]: ",str(index_data_offset_d[test_case_index]))
    
    content = [ ]

    with open(log_path) as f:
        for jsonObj in f:
            o = json.loads(jsonObj)
            content.append(o)

    # We keep only the matched keyword.
    data_l = [log["alert"]["signature"].split(" ")[2] for log in content if "payload_printable" in log.keys()]

    print("extract_data: data_l: ",str(data_l))

    # We keep data actually present inside test cases.
    # Some matched keywords are present in reassembled data, but overlap chunk piece boundaries.
    data_l_in_test_case = [ data for data in data_l if data in index_data_offset_d[test_case_index].keys() ]   
    print("extract_data: data_l_in_test_case: ",str(data_l_in_test_case))
    
    position_data_l = [ (index_data_offset_d[test_case_index][data], data) for data in data_l_in_test_case ]
    position_data_l_sorted = sorted(position_data_l, key=lambda tup: tup[0])
    data_l_sorted = [ data for (position, data) in position_data_l_sorted ]

    print("extract_data: data_l_sorted: ",str(data_l_sorted))

    position_set = set([ position for (position, data) in position_data_l_sorted ])
    print("extract_data: position_set: ",str(position_set))
    if len(position_set) != len(data_l_in_test_case):
        print("\n\n\n\nextract_data: WARNING: we extracted more pieces than expected position index")
        print("extract_data: position unique values (",str(position_set),") are not consistent with the number of pieces (",str(len(data_l_in_test_case)),")")
        print("extract_data: position_data_l_sorted: ",str(position_data_l_sorted))

        # in this case, we need an extra information from payload_printable to know whether the signature(s) matched starts at multiple of 8 characters
        print("extract_data: let's try to get extra information from payload_printable field")
        payload_printable = ""
        for log in content:
            if "payload_printable" in log.keys():
                payload_printable = log["payload_printable"]
                break 
        
        if payload_printable == "":
            print("extract_data: payload_printable field is empty")
            sys.exit(-1)

        print("extract_data: payload_printable: ",str(payload_printable))

        # depending on the scenario, we remove extra characters
        if nb_final_character_to_remove != 0:
            payload_printable = payload_printable[:-nb_final_character_to_remove]
        payload_printable = payload_printable.replace('0','')
        
        payload_printable_sliced_by_byte_a = [ payload_printable[j:j+8] for j in range(0,len(payload_printable),8) ]
        # remove duplicates
        payload_printable_sliced_by_byte_a = list(dict.fromkeys(payload_printable_sliced_by_byte_a))
        print("extract_data: payload_printable_sliced_by_byte_a: ",str(payload_printable_sliced_by_byte_a))
        
        # remove chunks from data_l_sorted that are not present in the payload_printable sliced by byte  
        data_l_sorted = [ data for data in data_l_sorted if data in payload_printable_sliced_by_byte_a ] 
        print("extract_data: updated data_l_sorted: ",str(data_l_sorted))

        # if still, we got a missmatch between size of lists, we got a problem
        if len(position_set) != len(data_l_sorted):
            print("\n\n\n\nextract_data: ERROR: we extracted more pieces than expected position index and we can't fix the problem with payload_printable extra information. ")
            print("extract_data: exiting")
            sys.exit(-1)
        
    
    payload_s = "".join(data_l_sorted)
            
    return { "is_icmp_echo_reply": True, "payload": payload_s }

# def get_offset(index_data_offset_d, index, data):
   
def merge_d_l(d_l):
    r = {}
    for d in d_l:
        r.update(d)
    return r
    
def extract_chunk_piece_offset_data(use_internet_checksum_payload, chunk_data_d, test_case_index):
    print("extract_chunk_piece_offset_data: start")
    
    print("extract_chunk_piece_offset_data: test_case_index: ",str(test_case_index))    
    
    print("extract_chunk_piece_offset_data: chunk_data_d: ",str(chunk_data_d))
    
    if use_internet_checksum_payload:
        data_s = chunk_data_d["internet_checksum_s"]
    else:
        data_s = chunk_data_d["simple_s"]
        
    if use_internet_checksum_payload:
        piece_size = 8
    else:
        piece_size = 1
        
    # We split the chunk string into several piece for each pattern.
    piece_l = [ data_s[i:i+piece_size] for i in range(0, len(data_s), piece_size) ]
    
    print("extract_chunk_piece_offset_data: piece_l: ",str(piece_l))
    
    # We add the chunk offset to the position.
    piece_position_l = [ position + chunk_data_d["offset"] for position in list(range(0, len(piece_l))) ]
    
    print("extract_chunk_piece_offset_data: end")
    
    return { piece: position for piece, position in zip(piece_l, piece_position_l) }

def build_index_data_offset_d(use_internet_checksum_payload, byte_time_sequence_json_path):
    print("build_index_data_offset_d: start")
    
    with open(byte_time_sequence_json_path) as f:
        json_d = json.load(f)
    # print(d)
    pair_d = json_d["byte_time_pair_sequence_c"]["hm"]
    triplet_d = json_d["byte_time_triplet_sequence_c"]["hm"]
    
    index_data_offset_d_pair_tmp = { test_case_index: [ extract_chunk_piece_offset_data(use_internet_checksum_payload, chunk_data_d, test_case_index) for chunk_i, chunk_data_d in pair_test_case_d["chunk_c"]["bm"].items() ] for test_case_index, pair_test_case_d in pair_d.items() }
    
    print("build_index_data_offset_d: index_data_offset_d_pair_tmp: ",str(index_data_offset_d_pair_tmp))


    index_data_offset_d_pair = { test_case_index: merge_d_l(d_l) for test_case_index, d_l in index_data_offset_d_pair_tmp.items() }
    
    print("build_index_data_offset_d: index_data_offset_d_pair: ",str(index_data_offset_d_pair))
    
    index_data_offset_d_triplet_tmp = { test_case_index: [ extract_chunk_piece_offset_data(use_internet_checksum_payload, chunk_data_d, test_case_index) for chunk_i, chunk_data_d in triplet_test_case_d["chunk_c"]["bm"].items() ] for test_case_index, triplet_test_case_d in triplet_d.items() }
    
    index_data_offset_d_triplet = { test_case_index: merge_d_l(d_l) for test_case_index, d_l in index_data_offset_d_triplet_tmp.items() }
    
    index_data_offset_d = merge_d_l([ index_data_offset_d_pair, index_data_offset_d_triplet ])
    
    print("build_index_data_offset_d: end")
    
    return index_data_offset_d     

def extract_test_case_index(file_path):
    print("extract_test_case_index: start")
    print("extract_test_case_index: file_path: ",str(file_path))
    bn = os.path.basename(file_path)
    print("extract_test_case_index: bn: ",str(bn))
    bn_wo_ext = os.path.splitext(bn)[0]
    print("extract_test_case_index: bn_wo_ext: ",str(bn_wo_ext))
    index = bn_wo_ext.split("_")[-1]
    print("extract_test_case_index: index: ",str(index))
    print("extract_test_case_index: start")
    return index

def usage():
    print('train_evaluate_closed_world.py -m <config_file> -d -o <output_directory> -s <output_file_suffix> -t')

def process(use_internet_checksum_payload, log_directory, byte_time_sequence_json_path, json_output_path, nb_final_character_to_remove):
    index_data_offset_d = build_index_data_offset_d(use_internet_checksum_payload, byte_time_sequence_json_path)

    filename_l = os.listdir(log_directory)
    
    path_l = [ os.path.join(log_directory, filename) for filename in filename_l ]

    # We sort to have a deterministic processing order to ease debugging accross machines.
    path_l.sort()

    index_payload_d = { extract_test_case_index(log_path): extract_data(index_data_offset_d, extract_test_case_index(log_path),log_path,nb_final_character_to_remove) for log_path in path_l }

    data_d = { "hm": index_payload_d }

    with open(json_output_path, 'w') as fp:
        json.dump(data_d, fp, indent=2, sort_keys=True)

def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--internet-checksum-payload-mode", default=True)
    # TODO: change to -l
    parser.add_argument("-p", "--log-directory", type=str, default="")
    parser.add_argument("-s", "--byte-time-sequence-json-path", type=str, default="")
    parser.add_argument("-j", "--json-path", type=str, default="")
    # TODO: remove when ensured that there is no problem
    parser.add_argument("-r", "--nb-character-to-remove", type=int, default=0)
    args = parser.parse_args()

    use_internet_checksum_payload = args.internet_checksum_payload_mode
    log_directory = args.log_directory
    byte_time_sequence_json_path = args.byte_time_sequence_json_path
    json_path = args.json_path
    nb_final_character_to_remove = args.nb_character_to_remove

    print('extract_payload_from_suricata_log_directory_to_json: log_directory: "%s"'%(log_directory))
    print('extract_payload_from_suricata_log_directory_to_json: byte_time_sequence_json_path: "%s"'%(byte_time_sequence_json_path))
    print('extract_payload_from_suricata_log_directory_to_json: json_path: "%s"'%(json_path))
    
    process(use_internet_checksum_payload,
            log_directory,
            byte_time_sequence_json_path,
            json_path,
            nb_final_character_to_remove
    )

if __name__ == "__main__":
   main(sys.argv[1:])
