#!/usr/bin/python

import sys
import os
import argparse
import glob
import hashlib

def extract_scenario_from_json_basename(json: str) -> str:
    # TODO scenario location inside json filename is inconsistent across target: make it conisstent and change this function 
    split_v = json.split('_')
    for split in split_v:
        if 'pe' in split:
            return split
    print(f"Could not extract scenario from json_basename {json}")
    sys.exit(-1)

def process(path: str):
    print(f"process: start")

    # find files
    json_payload_filepath_v = glob.glob(f"{path}/**/*payload_20*.json",recursive=True)
    # print(f"process: json_payload_filepath_v: {json_payload_filepath_v}")
    print(f"process: json_payload_filepath_v len: {len(json_payload_filepath_v)}")

    # dict file:shasum
    json_shasum_hm = { os.path.basename(json_path):hashlib.sha256(open(json_path,'rb').read()).hexdigest() for json_path in json_payload_filepath_v }
    # print(f"process: json_shasum_hm: {json_shasum_hm}")

    # re-arrange dict file:shasum into dict scenario:[shasum, .., shasum]
    scenario_shasum_no_duplicates_v_hm = { }
    scenario_shasum_v_hm = { }
    for json,shasum in json_shasum_hm.items():
        scenario = extract_scenario_from_json_basename(json)
        if scenario in scenario_shasum_no_duplicates_v_hm:
            shasum_no_duplicates_v = scenario_shasum_no_duplicates_v_hm[scenario]
            shasum_no_duplicates_v.append(shasum)
            shasum_no_duplicates_v = list(set(shasum_no_duplicates_v))

            shasum_v = scenario_shasum_v_hm[scenario]
            shasum_v.append(shasum)
        else:
            shasum_no_duplicates_v,shasum_v = [shasum], [shasum]
        scenario_shasum_no_duplicates_v_hm.update({scenario:shasum_no_duplicates_v})
        scenario_shasum_v_hm.update({scenario:shasum_v})
    scenario_shasum_no_duplicates_v_hm = dict(sorted(scenario_shasum_no_duplicates_v_hm.items()))


    # compare shasum across scenarii 
    for scenario, shasum_v in scenario_shasum_no_duplicates_v_hm.items():
        if len(shasum_v) != 1:
            print(f"process: {scenario} - {len(shasum_v)}/{len(scenario_shasum_v_hm[scenario])} different run outputs")

    print(f"process: end")


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-p", "--path", type=str, default="")
    args = parser.parse_args()

    path = args.path
    print(f"display_sha256sum_inconsistent: path: {path}")
        
    process(path)

if __name__ == "__main__":
    main(sys.argv[1:])

