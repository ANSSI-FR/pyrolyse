#!/usr/bin/python
import sys
import os
import json
import argparse
import glob
from collections import Counter
import pandas as pd


def build_test_case_reassembly_consensus_across_runs(json_payload_v:list):
    """Build test case reassembly consensus across runs."""
    print("build_test_case_reassembly_consensus_across_runs: start")
    json_tc_nb_v = [ len(json_payload) for json_payload in json_payload_v ] 
    json_tc_nb_no_duplicates_v = list(set(json_tc_nb_v))
    if len(json_tc_nb_no_duplicates_v) > 1:
        print("build_test_case_reassembly_consensus_across_runs: inconsistent test case number across runs. Remove uncomplete runs before re-running the script.")
        sys.exit(-1)

    json_tc_index_v = [ int(tc_index) for tc_index in json_payload_v[0]['hm'].keys() ]
    print("build_test_case_reassembly_consensus_across_runs: len(json_tc_index_v): ", len(json_tc_index_v))
    json_tc_index_v.sort()

    data = { } 
    for json_tc_index_i in json_tc_index_v:
        json_tc_index_str = str(json_tc_index_i)
        tc_reassembly_v = [ json_payload['hm'][json_tc_index_str] for json_payload in json_payload_v ] 

        # Remove duplicates
        b = list({json.dumps(d, sort_keys=True) for d in tc_reassembly_v})
        tc_reassembly_no_duplicates_v = [json.loads(d) for d in b]

        if len(tc_reassembly_no_duplicates_v) == 1: 
            consensus = tc_reassembly_no_duplicates_v[0]
        elif len(tc_reassembly_no_duplicates_v) == 2 and tc_reassembly_no_duplicates_v[0]['is_echo_reply'] != tc_reassembly_no_duplicates_v[1]['is_echo_reply']:
            # most observed inconsistency
            consensus = tc_reassembly_no_duplicates_v[0] if tc_reassembly_no_duplicates_v[0]['is_echo_reply'] == True else tc_reassembly_no_duplicates_v[1]
        else:
            #tc_reassembly_count_hm = Counter(tc_reassembly_v) 
            tc_reassembly_str_count_hm = Counter([ tc_reassembly['payload'] for tc_reassembly in tc_reassembly_v ])    

            tc_reassembly_str_consensus = list(tc_reassembly_str_count_hm.keys())[0] 
            tc_reassembly_consensus = next(tc_reassembly for tc_reassembly in tc_reassembly_v if tc_reassembly['payload'] == tc_reassembly_str_consensus)

            if tc_reassembly_consensus['is_echo_reply'] == True: 
                if list(tc_reassembly_str_count_hm.values())[0] == list(tc_reassembly_str_count_hm.values())[1] and list(tc_reassembly_str_count_hm.keys())[1] != '':
                    print(f"build_test_case_reassembly_consensus_across_runs: the test case {json_tc_index_str} can't reach consensus since we count (at least) the same times two different reassemblies.")
                    sys.exit(-1)

                consensus = tc_reassembly_consensus
            else: 
                #print(f"build_test_case_reassembly_consensus_across_runs: the test case {json_tc_index_str} consensus results in prefering the 'ignore' reassembly, while some reassembled payloads werre observed but not in majority. What to do ? Modify script accordingly")
                #sys.exit(-1)
                print(f"build_test_case_reassembly_consensus_across_runs: WARNING the test case {json_tc_index_str} consensus results in prefering the 'ignore' reassembly, while some reassembled payloads werre observed but not in majority.")
                consensus = tc_reassembly_consensus

        data.update({json_tc_index_str:consensus})

    
    print("build_test_case_reassembly_consensus_across_runs: end")
    #return dict(sorted(data.items()))
    return data


def process(target_directory_path:str, pattern_json_payload_file:str, 
            output_json_payload_file: str):
    print("process: start")
    filename_v = glob.glob(f"{target_directory_path}/{pattern_json_payload_file}*")
    #print("process: filename_v: ",filename_v)

    json_payload_v = [ json.load(open(filename,'r')) for filename in filename_v ]
    #print("process: json_payload_v: ",json_payload_v)
    index_payload_d = build_test_case_reassembly_consensus_across_runs(json_payload_v)

    data_d = {"hm": index_payload_d}

    json_output_path = f"{target_directory_path}/{output_json_payload_file}"
    print("process: json_output_path: ",json_output_path)

    with open(json_output_path, 'w', encoding="UTF8") as opened_file:
        json.dump(data_d, opened_file, indent=2)

    print("process: end")


def usage():
    print(
        "build_artificial_latest_payload.py -t <target-directory-path> -p <pattern-json-payload-file> -o <output-json-payload-file>"  
    )


def main(argv):
    print("build_artificial_latest_payload: start")

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--target-directory-path", type=str, default="")
    parser.add_argument("-p", "--pattern-json-payload-file", type=str, default="")
    parser.add_argument("-o", "--output-json-payload-file", type=str, default="")
    args = parser.parse_args()

    target_directory_path = args.target_directory_path
    pattern_json_payload_file = args.pattern_json_payload_file
    output_json_payload_file = args.output_json_payload_file

    print(
        f"build_artificial_latest_payload: target_directory_path: {target_directory_path}"
    )
    print(
        f"build_artificial_latest_payload: pattern_json_payload_file: {pattern_json_payload_file}"
    )
    print(
        f"build_artificial_latest_payload: output_json_payload_file: {output_json_payload_file}"
    )

    process(target_directory_path, pattern_json_payload_file, output_json_payload_file)

    print("build_artificial_latest_payload: end")


if __name__ == "__main__":
    main(sys.argv[1:])
