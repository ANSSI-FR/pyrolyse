#!/usr/bin/python
import sys
import os
# import re
# from scapy.all import Ether,IP,ICMP,TCP,Raw,rdpcap,wrpcap,PacketList,bytes_hex
# from scapy.all import IP,ICMP,rdpcap,PacketList,bytes_hex
# import networkx as nx
import json
import argparse

# import graph_utils as gu

global MAX_PAYLOAD_SIZE
MAX_PAYLOAD_SIZE = 5 * 8 

def get_pattern_seq(test_case_index, pattern, byte_time_sequence_json_path):
    print("get_pattern_seq: pattern: ",pattern)

    with open(byte_time_sequence_json_path) as f:
        json_d = json.load(f)

    sequence = json_d["byte_time_pair_sequence_c"] if int(test_case_index) <= 12 else json_d["byte_time_triplet_sequence_c"]

    chunks_info = sequence["hm"][str(test_case_index)]["chunk_c"]["bm"]

    for chunk in chunks_info:
        chunk_s = chunks_info[chunk]["internet_checksum_s"]
        seq = chunks_info[chunk]["offset"] * 8 + 1

        print("get_pattern_seq: chunk_s: ",chunk_s)
        print("get_pattern_seq: seq: ",seq)


        # split big chunk in 8-bytes chunks
        byte_chunk = [chunk_s[j:j+8] for j in range(0,len(chunk_s),8)] 
        
        print("get_pattern_seq: byte_chunk: ",byte_chunk)


        if pattern in byte_chunk:
            print("get_pattern_seq: seq: ",seq)
            return seq + byte_chunk.index(pattern) * 8

    return -1


def extract_data(index_data_offset_d, test_case_index, log_path, byte_time_sequence_json_path):
    print("\n\nextract_data: log_path:",log_path)
    
    print("extract_data: test_case_index: ",str(test_case_index))
    
    with open(log_path) as f:
        content = f.readlines()

    if content == []:
        return { "is_echo_reply": False, "payload": "" }

    # you may also want to remove whitespace characters like `\n` at the end of each line
    line_l = [ x.strip() for x in content ]
    
    line_l_w_payload = [ line for line in line_l if "tcp::Tcp_content_From_Client" in line or "Signatures::Sensitive_Signature" in line ]

    tcp_payload_s = ""
    tcp_payload_seq_d= {}
    for line in line_l_w_payload: 
        print("extract_data: line: ",str(line))

        # For script
        if "tcp::Tcp_content_From_Client" in line:
            current_tcp_payload_s = line.split("\t")[11].split(",")[0].split(":")[1]
            current_tcp_seq_s = line.split("\t")[11].split(",")[1].split(":")[1]

        # For signatures
        else:
            assert "Signatures::Sensitive_Signature" in line  
            current_tcp_payload_s = line.split("\t")[9]

            current_tcp_seq_s = get_pattern_seq(test_case_index, current_tcp_payload_s[:8], byte_time_sequence_json_path)

        tcp_payload_seq_d.update( { int(current_tcp_seq_s): current_tcp_payload_s} )
    
    print("extract_data: tcp_payload_seq_d: ", tcp_payload_seq_d)

    # 1. Verify that there is no hole in reassembly
    #hole = False
    #next_expected_seq = -1
    #for i, (seq, payload) in enumerate(tcp_payload_seq_d.items()):
#
    #    if i == 0:
    #        next_expected_seq = seq + len(payload)
    #        continue
    #    if next_expected_seq != seq:
    #        hole = True
    #        print("extract_data: hole: ", hole)
    #        break
    #    next_expected_seq += len(payload)

    # 2. Concatenate "sub-payloads" and adding "." substring if there is a hole
    tmp_tcp_payload_s = "." * MAX_PAYLOAD_SIZE
    for i, (seq, payload) in enumerate(tcp_payload_seq_d.items()):
        tmp_tcp_payload_s = tmp_tcp_payload_s[:seq - 1] + payload + tmp_tcp_payload_s[seq + len(payload) - 1:]
        print("extract_data: tmp_tcp_payload_s", tmp_tcp_payload_s)
    tcp_payload_s = tmp_tcp_payload_s[:seq + len(payload) - 1]

    return { "is_echo_reply": True, "payload": tcp_payload_s.replace('0','') }

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
    # index_data_offset_d = build_index_data_offset_d(use_internet_checksum_payload, byte_time_sequence_json_path)

    filename_l = os.listdir(log_directory)
    
    path_l = [ os.path.join(log_directory, filename) for filename in filename_l ]

    # We sort to have a deterministic processing order to ease debugging accross machines.
    path_l.sort()

    # index_payload_d = { extract_test_case_index(log_path): extract_data(index_data_offset_d, extract_test_case_index(log_path),log_path) for log_path in path_l }
    index_payload_d = { extract_test_case_index(log_path): extract_data({}, extract_test_case_index(log_path),log_path, byte_time_sequence_json_path) for log_path in path_l }

    data_d = { "hm": index_payload_d }

    with open(json_output_path, 'w') as fp:
        json.dump(data_d, fp, indent=2, sort_keys=True)

def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--internet-checksum-payload-mode", action='store_true')
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

    print('log_directory: "%s"'%(log_directory))
    print('byte_time_sequence_json_path: "%s"'%(byte_time_sequence_json_path))
    print('json_path: "%s"'%(json_path))
    
    process(use_internet_checksum_payload,
            log_directory,
            byte_time_sequence_json_path,
            json_path
    )

if __name__ == "__main__":
   main(sys.argv[1:])
