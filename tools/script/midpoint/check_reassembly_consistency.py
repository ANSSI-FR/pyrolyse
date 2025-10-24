import os
from scapy.all import *
import argparse
import json

def merge_d_l(d_l):
    r = {}
    for d in d_l:
        r.update(d)
    return r

def detect_hole_quirk_in_reassembly(index_data_offset_d,offset_payload_d):
    index_quirk = { } 
    for index,data_offset in index_data_offset_d.items():
        quirk = False
        prev_offset = list(data_offset.items())[0][1]
        for data,offset in data_offset.items():
            if offset - prev_offset > 1: 
                if max(offset_payload_d[index].keys()) >= prev_offset + 1:
                    quirk = True
                    break
            prev_offset = offset
        index_quirk.update({index: { "has_hole_quirk": quirk } })

    return index_quirk

def detect_offset_quirk_in_reassembly(index_data_offset_d, index_payload_d):
    index_quirk = { } 
    for index, data_offset in index_data_offset_d.items():
        quirk = False
        for payload_offset,payload in index_payload_d[index].items():
            if data_offset.get(payload) and data_offset.get(payload) != payload_offset:
                quirk = True
                break

        index_quirk.update({index: { "has_offset_quirk": quirk } })
        
    return index_quirk

def build_index_data_offset_d(byte_time_sequence_json_path):
    print("build_index_data_offset_d: start")
    
    with open(byte_time_sequence_json_path) as f:
        json_d = json.load(f)

    pair_d = json_d["byte_time_pair_sequence_c"]["hm"]
    triplet_d = json_d["byte_time_triplet_sequence_c"]["hm"]

    index_data_offset_d_pair_tmp = { test_case_index: [ extract_chunk_piece_offset_data(chunk_data_d, test_case_index) for chunk_i, chunk_data_d in pair_test_case_d["chunk_c"]["bm"].items() ] for test_case_index, pair_test_case_d in pair_d.items() }
    
    index_data_offset_d_pair = { test_case_index: merge_d_l(d_l) for test_case_index, d_l in index_data_offset_d_pair_tmp.items() }
    
    print("build_index_data_offset_d: index_data_offset_d_pair: ",str(index_data_offset_d_pair))
    
    index_data_offset_d_triplet_tmp = { test_case_index: [ extract_chunk_piece_offset_data(chunk_data_d, test_case_index) for chunk_i, chunk_data_d in triplet_test_case_d["chunk_c"]["bm"].items() ] for test_case_index, triplet_test_case_d in triplet_d.items() }
    
    index_data_offset_d_triplet = { test_case_index: merge_d_l(d_l) for test_case_index, d_l in index_data_offset_d_triplet_tmp.items() }
    
    index_data_offset_d = merge_d_l([ index_data_offset_d_pair, index_data_offset_d_triplet ])
    
    print("build_index_data_offset_d: end")
    
    return { index: dict(sorted(data_offset.items(), key=lambda x:x[1])) for index, data_offset in index_data_offset_d.items() } 

def extract_chunk_piece_offset_data(chunk_data_d, test_case_index):
    print("extract_chunk_piece_offset_data: start")
    
    print("extract_chunk_piece_offset_data: test_case_index: ",str(test_case_index))    
    
    print("extract_chunk_piece_offset_data: chunk_data_d: ",str(chunk_data_d))
    
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

def get_index_payload(extracted_payload_file):
    with open(extracted_payload_file) as f:
        extracted_payload = json.load(f)
    res = {}
    for index, payload_and_is_echo_reply in extracted_payload['hm'].items():
        res.update({index:payload_and_is_echo_reply['payload']})
    return res


def process(extracted_payload_file, byte_time_sequence_json_path, json_output_path):
    index_data_offset_d = build_index_data_offset_d(byte_time_sequence_json_path)
    #print("process: index_data_offset_d",index_data_offset_d)

    index_payload_d = get_index_payload(extracted_payload_file)
    #print("process: index_payload_d",index_payload_d)

    offset_payload_d = { index: { int(j/8): payload[j:j+8] for j in range(0,len(payload),8) } for index, payload in index_payload_d.items() }
    print("process: offset_payload_d",offset_payload_d)
    
    index_offset_quirk = detect_offset_quirk_in_reassembly(index_data_offset_d,offset_payload_d)
    index_hole_quirk = detect_hole_quirk_in_reassembly(index_data_offset_d,offset_payload_d)

    print("\n\n")
    print("process: index_offset_quirk",index_offset_quirk)
    print("process: index_hole_quirk",index_hole_quirk)

    # merge dicts
    index_quirks = { index: {**index_offset_quirk.get(index),**index_hole_quirk.get(index)} for index in index_offset_quirk.keys() }
    print("process: index_quirks",index_quirks)

    with open(json_output_path, 'w') as fp:
        json.dump(index_quirks, fp, indent=2, sort_keys=True)


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-p", "--extracted-payload-file", type=str, default="")
    parser.add_argument("-s", "--byte-time-sequence-json-path", type=str, default="")
    parser.add_argument("-o", "--json-output-path", type=str, default="")
    args = parser.parse_args()

    extracted_payload_file = args.extracted_payload_file
    byte_time_sequence_json_path = args.byte_time_sequence_json_path
    json_output_path = args.json_output_path

    print('extract_payload_from_suricata_log_directory_to_json: extracted_payload_file: "%s"'%(extracted_payload_file))
    print('extract_payload_from_suricata_log_directory_to_json: byte_time_sequence_json_path: "%s"'%(byte_time_sequence_json_path))
    print('extract_payload_from_suricata_log_directory_to_json: json_output_path: "%s"'%(json_output_path))
    
    process(extracted_payload_file,
            byte_time_sequence_json_path,
            json_output_path
    )

if __name__ == "__main__":
   main(sys.argv[1:])