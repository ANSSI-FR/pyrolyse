#!/usr/bin/python
import sys
import os
import json
import argparse
import base64


def extract_data(index_data_offset_d, test_case_index, log_path):
    print("\n\nextract_data: log_path:",log_path)
    print("extract_data: test_case_index: ",str(test_case_index))
    print("extract_data: index_data_offset_d[",str(test_case_index),"]: ",str(index_data_offset_d[test_case_index]))
    
    content = [ ]

    with open(log_path) as f:
        for jsonObj in f:
            o = json.loads(jsonObj)
            content.append(o)

    for log in content:
        if "payload_printable" in log.keys() and "stream" in log.keys() and log["stream"] == 1:
            # for peos scenario, remove "0" extra character
            payload_s =  log["payload_printable"].replace('0','')

            # remove duplicate pattern
            byte_payload = [payload_s[j:j+8] for j in range(0,len(payload_s),8)]
            payload_s = ''.join(list(dict.fromkeys(byte_payload)))



            print("extract_data: is_echo_reply: True")
            print("extract_data: payload: ",payload_s)

            return  { "is_echo_reply": True, "payload": payload_s}

    return { "is_echo_reply": False, "payload": "" }

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
    print("extract_chunk_piece_offset_data: piece_position_l: ",piece_position_l)
    
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

def process(use_internet_checksum_payload, log_directory, byte_time_sequence_json_path, json_output_path):
    index_data_offset_d = build_index_data_offset_d(use_internet_checksum_payload, byte_time_sequence_json_path)

    filename_l = os.listdir(log_directory)
    
    path_l = [ os.path.join(log_directory, filename) for filename in filename_l ]

    # We sort to have a deterministic processing order to ease debugging accross machines.
    path_l.sort()

    index_payload_d = { extract_test_case_index(log_path): extract_data(index_data_offset_d, extract_test_case_index(log_path),log_path) for log_path in path_l }

    data_d = { "hm": index_payload_d }

    print("process: data_d: ",data_d)

    with open(json_output_path, 'w') as fp:
        json.dump(data_d, fp, indent=2, sort_keys=True)

def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--internet-checksum-payload-mode", action='store_false')
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

    print('extract_payload_from_suricata_log_directory_to_json: log_directory: "%s"'%(log_directory))
    print('extract_payload_from_suricata_log_directory_to_json: byte_time_sequence_json_path: "%s"'%(byte_time_sequence_json_path))
    print('extract_payload_from_suricata_log_directory_to_json: json_path: "%s"'%(json_path))
    
    process(use_internet_checksum_payload,
            log_directory,
            byte_time_sequence_json_path,
            json_path    
            )

if __name__ == "__main__":
   main(sys.argv[1:])
