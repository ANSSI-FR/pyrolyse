#!/usr/bin/python
import sys
import os
import json
import argparse
import base64
import glob

from distutils.util import strtobool


def get_before_relation_nb(rc:dict) -> int:
    return int(rc["relation_01"] == "B" or rc["relation_01"] == "Bi") \
          + int(rc["relation_02"] == "B" or rc["relation_02"] == "Bi") \
          + int(rc["relation_12"] == "B" or rc["relation_12"] == "Bi")

def filter_tc_without_hole(
    target_hm:dict,
    byte_time_sequence_hm:dict
) -> dict:
    pair_index_with_hole_v = [ index for index,data in byte_time_sequence_hm['byte_time_pair_sequence_c']['hm'].items() if data['rc'] == 'B' or data['rc'] == 'Bi' ]
    print("filter_tc_without_hole: pair_index_with_hole_v: ",pair_index_with_hole_v)
    triplet_index_with_hole_v = [ index for index,data in byte_time_sequence_hm['byte_time_triplet_sequence_c']['hm'].items() if get_before_relation_nb(data['rc']) >= 2 ]
    print("filter_tc_without_hole: triplet_index_with_hole_v: ",triplet_index_with_hole_v)
    return { index:data for index,data in target_hm.items() if index not in pair_index_with_hole_v and index not in triplet_index_with_hole_v } 

def process(target_midpoint_directory_path:str,
            target_os_directory_path:str,
            ending_file_regex:str,
            scenario:str,
            output_filepath: str,
            merged_byte_time_sequence_json_path: str,
            without_test_case_with_hole:bool
):
    print("process: start")
    with open(merged_byte_time_sequence_json_path) as json_file: 
        byte_time_sequence_hm = json.load(json_file)
    
    target_midpoint_filepath_v = glob.glob(f"{target_midpoint_directory_path}/*_{scenario}_{ending_file_regex}")
    print("process: target_midpoint_filepath: ",target_midpoint_filepath_v)
    assert len(target_midpoint_filepath_v) == 1
    target_midpoint_filepath = target_midpoint_filepath_v[0]
    with open(target_midpoint_filepath) as json_file: 
        midpoint_json = json.load(json_file)
    midpoint_hm = filter_tc_without_hole(midpoint_json['hm'],byte_time_sequence_hm) if without_test_case_with_hole else midpoint_json['hm']


    target_os_filepath_v = glob.glob(f"{target_os_directory_path}/*_{scenario}_{ending_file_regex}")
    print("process: target_os_filepath_v: ",target_os_filepath_v)
    assert len(target_os_filepath_v) == 1
    target_os_filepath = target_os_filepath_v[0]
    with open(target_os_filepath) as json_file: 
        os_json = json.load(json_file)
    os_hm = filter_tc_without_hole(os_json['hm'],byte_time_sequence_hm) if without_test_case_with_hole else os_json['hm']

    if len(midpoint_hm) != len(os_hm):
        print("process: midpoint_hm and os_hm lengths mismatch")
        print("process: inconsistent files")
        sys.exit(-1) 

    #tc_inconcistency_v = [ midpoint_k for (midpoint_k,midpoint_v),(_,os_v) in zip(midpoint_hm.items(),os_hm.items()) if midpoint_v != os_v ]
    #if tc_inconcistency_v != []:
    #    print(f"process: tc_inconcistency_v: ",tc_inconcistency_v)
#
    #data = f"{scenario}: {len(tc_inconcistency_v)} ({tc_inconcistency_v})\n"
    #with open(output_filepath, 'a') as opened_file:
    #    opened_file.write(data)

    values_hm = { int(midpoint_k):midpoint_v == os_v for (midpoint_k,midpoint_v),(_,os_v) in zip(midpoint_hm.items(),os_hm.items()) } 
    hm = { 'hm':values_hm }

    with open(output_filepath, 'w', encoding="UTF8") as fp:
        json.dump(hm, fp, indent=2, sort_keys=True)

    print("process: end")


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-tm", "--target-midpoint-directory-path", type=str,required=True)
    parser.add_argument("-to", "--target-os-directory-path", type=str,required=True)
    parser.add_argument("-r", "--ending-file-regex", type=str,required=True)
    parser.add_argument("-s", "--scenario", type=str, required=True)
    parser.add_argument("-o", "--output-filepath", type=str, required=True)
    parser.add_argument("-m", "--merged-byte-time-sequence-json-path", type=str)
    # the following command is just for Suricata, that has a bug for TCP test case with hole
    parser.add_argument("-wh", "--without-test-case-with-hole", type=lambda x: bool(strtobool(x)),default=True)
    args = parser.parse_args()


    target_midpoint_directory_path = args.target_midpoint_directory_path
    target_os_directory_path = args.target_os_directory_path
    ending_file_regex = args.ending_file_regex
    scenario = args.scenario
    output_filepath = args.output_filepath
    merged_byte_time_sequence_json_path = args.merged_byte_time_sequence_json_path
    without_test_case_with_hole = args.without_test_case_with_hole

    print('extract_reassembly_consistency_scenario: target_midpoint_directory_path: "%s"'%(target_midpoint_directory_path))
    print('extract_reassembly_consistency_scenario: target_os_directory_path: "%s"'%(target_os_directory_path))
    print('extract_reassembly_consistency_scenario: ending_file_regex: "%s"'%(ending_file_regex))
    print('extract_reassembly_consistency_scenario: scenario: "%s"'%(scenario))
    print('extract_reassembly_consistency_scenario: output_filepath: "%s"'%(output_filepath))
    print('extract_reassembly_consistency_scenario: merged_byte_time_sequence_json_path: "%s"'%(merged_byte_time_sequence_json_path))
    print('extract_reassembly_consistency_scenario: without_test_case_with_hole: "%s"'%(without_test_case_with_hole))
    
    process(target_midpoint_directory_path,
            target_os_directory_path,
            ending_file_regex,
            scenario,
            output_filepath,
            merged_byte_time_sequence_json_path,
            without_test_case_with_hole
    )

if __name__ == "__main__":
   main(sys.argv[1:])
