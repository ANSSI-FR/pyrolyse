#!/usr/bin/python
import sys
import os
import json
import argparse


def process(input_directory_path: str, json_output_path: str):
    print("process: start")
    filename_l = os.listdir(input_directory_path)

    path_l = [
        os.path.join(input_directory_path, filename) for filename in filename_l
    ]

    path_l_pair = [path for path in path_l if "pair" in path]
    path_l_triplet = [path for path in path_l if "triplet" in path]

    d_l_pair = [json.load(open(path)) for path in path_l_pair]
    d_pair = {d["byte_sequence_index"]: d for d in d_l_pair}

    d_l_triplet = [json.load(open(path)) for path in path_l_triplet]
    d_triplet = {d["byte_sequence_index"]: d for d in d_l_triplet}

    data_d = {
        "byte_time_pair_sequence_c": {
            "hm": d_pair
        },
        "byte_time_triplet_sequence_c": {
            "hm": d_triplet
        }
    }

    with open(json_output_path, 'w', encoding="UTF8") as opened_file:
        json.dump(data_d, opened_file, indent=2, sort_keys=True)

    print("process: end")


def usage():
    print(
        "extract_os_icmp_payload_from_directory_to_json.py -p <pcap_directory> -v <ip_version> -o <json_path> -r <nb_final_character_to_remove>"
    )


def main(argv):
    print("extract_os_icmp_payload_from_directory_to_json: start")

    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input-directory-path", type=str, default="")
    parser.add_argument("-o", "--json-output-path", type=str, default="")
    args = parser.parse_args()

    input_directory_path = args.input_directory_path
    json_output_path = args.json_output_path

    print(
        f"extract_os_icmp_payload_from_directory_to_json: input_directory_path: {input_directory_path}"
    )
    print(
        f"extract_os_icmp_payload_from_directory_to_json: json_output_path: {json_output_path}"
    )

    process(input_directory_path, json_output_path)

    print("extract_os_icmp_payload_from_directory_to_json: end")


if __name__ == "__main__":
    main(sys.argv[1:])
