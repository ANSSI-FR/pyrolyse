import os 
import sys 
import argparse
import json

def get_line(chunk, offset, sid):
    msg = f"Bad keyword {chunk} detected for chunk starting at offset {offset}!!!"

    return  f'alert ip [192.168.20.0/24] any -> any any (msg:"{msg}"; content:"{chunk}"; offset: {offset}; classtype:bad-unknown; sid: {sid}; rev: 7; metadata:created_at 2010_09_23, updated_at 2010_09_23;)\n'

def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-b", "--byte-time-sequence-path",type=str)
    parser.add_argument("-s", "--suricata-rules-path",type=str)
    args = parser.parse_args()

    byte_time_sequence_path = args.byte_time_sequence_path
    suricata_rules_path = args.suricata_rules_path

    with open(byte_time_sequence_path) as f:
        data = json.load(f)

    internet_checksum_chunk_pattern_v = data['internet_checksum_chunk_pattern_v']

    simple_payload_byte_length = [data['byte_time_triplet_sequence_c']['hm'][i]['simple_payload_byte_length'] for i in data['byte_time_triplet_sequence_c']['hm'].keys()]

    offsets = [8 * i for i in range(max(simple_payload_byte_length) + 2)]

    with open(suricata_rules_path,'w') as s_file:    
        for i,chunk in enumerate(internet_checksum_chunk_pattern_v): 
            for j,offset in enumerate(offsets):
                s_file.write(get_line(chunk, offset, int(str(i+1)+str(j+1))))

if __name__ == "__main__":
   main(sys.argv[1:])