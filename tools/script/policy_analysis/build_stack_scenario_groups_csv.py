#!/usr/bin/python
import sys
import os
import argparse
import glob
import pandas as pd
import hashlib
import json

INPUT_PAYLOAD_FILE_IP_DEPENDENT_SCENARII_NB = 34
INPUT_PAYLOAD_FILE_IP_AGNOSTIC_SCENARII_NB = 8
INPUT_PAYLOAD_FILE_IP_FULL_TC_SCENARII_NB = 24
INPUT_PAYLOAD_FILE_IP_PARTIAL_TC_SCENARII_NB = 17
INPUT_PAYLOAD_FILE_NB_TCP = 18


def build_target_name_v(filename_v: list) -> list:
    target_name_v = [get_target_name(filename) for filename in filename_v]
    #print("build_target_name_v: target_name_v: ",target_name_v)
    target_name_no_duplicates_v = list(set(target_name_v))
    #print("build_target_name_v: target_name_no_duplicates_v: ",target_name_no_duplicates_v)

    return target_name_no_duplicates_v


def get_target_name(filename: str) -> str:
    return filename.split("target")[1].split("/")[2]


def get_scenario_name(filename: str) -> str:
    return os.path.basename(filename).split('_')[1]


def are_the_common_tc_of_tc_reassembly_hm_similar(
        tc_reassembly_hm_0: dict, tc_reassembly_hm_1: dict) -> bool:
    for index, reassembly_0 in tc_reassembly_hm_0.items():
        #assert index in tc_reassembly_hm_1
        if index in tc_reassembly_hm_1 and tc_reassembly_hm_1[
                index] != reassembly_0:
            #print("are_the_common_tc_of_tc_reassembly_hm_similar: tc_reassembly_hm_1[index]: ",tc_reassembly_hm_1[index])
            #print("are_the_common_tc_of_tc_reassembly_hm_similar: reassembly_0: ",reassembly_0)
            return False
    return True


def build_target_name_scenario_groups_hm(filename_v: list,
                                         target_name_v: list) -> dict:
    target_name_scenario_groups_hm = {}
    for target_name in target_name_v:
        print("build_target_name_scenario_groups_hm: target_name: ",
              target_name)
        filename_targetname_v = [
            filename for filename in filename_v
            if get_target_name(filename) == target_name
        ]
        filename_targetname_v.sort()
        scenario_tc_reassembly_hm_hm = {
            get_scenario_name(filename): json.load(open(filename, 'rb'))['hm']
            for filename in filename_targetname_v
        }
        scenario_v = list(scenario_tc_reassembly_hm_hm.keys())
        scenario_groups_t_v = []
        for index_0, (scenario_0, tc_reassembly_hm_0) in enumerate(
                scenario_tc_reassembly_hm_hm.items()):
            if next((True for scenario_group_t in scenario_groups_t_v
                     if scenario_0 in scenario_group_t), False):
                continue

            index_0_group = [scenario_0]
            #print("build_target_name_scenario_groups_hm: scenario_0: ",scenario_0)

            for index_1, _ in enumerate(scenario_v):
                if index_0 == index_1: continue
                scenario_1 = scenario_v[index_1]
                tc_reassembly_hm_1 = scenario_tc_reassembly_hm_hm[scenario_1]

                if len(tc_reassembly_hm_0) <= len(tc_reassembly_hm_1):
                    are_the_tc_similar = are_the_common_tc_of_tc_reassembly_hm_similar(
                        tc_reassembly_hm_0, tc_reassembly_hm_1)
                else:
                    are_the_tc_similar = are_the_common_tc_of_tc_reassembly_hm_similar(
                        tc_reassembly_hm_1, tc_reassembly_hm_0)

                if are_the_tc_similar:
                    index_0_group.append(scenario_1)

            scenario_groups_t_v.append(tuple(index_0_group))
        target_name_scenario_groups_hm.update(
            {target_name: scenario_groups_t_v})

    return target_name_scenario_groups_hm


def build_scenario_groups_target_name_hm_from_target_name_scenario_groups_hm(
        target_name_scenario_groups_hm: dict) -> dict:
    scenario_groups_target_name_hm = {}
    for target_name, scenario_groups in target_name_scenario_groups_hm.items():
        for scenario_group in scenario_groups:
            if scenario_group in scenario_groups_target_name_hm:
                scenario_groups_target_name_hm[scenario_group].append(
                    target_name)
            else:
                scenario_groups_target_name_hm.update(
                    {scenario_group: [target_name]})

    return scenario_groups_target_name_hm


def remove_incomplete_target_from_lists(filename_v: list, target_name_v: list,
                                        protocol: str,
                                        ip_scenarii_to_use: str) -> tuple:
    target_file_nb_hm = {
        target_name: sum(1 for filename in filename_v
                         if target_name in filename)
        for target_name in target_name_v
    }
    #print("remove_incomplete_target_from_lists: filename_v: ",filename_v)
    print("remove_incomplete_target_from_lists: target_file_nb_hm: ",
          target_file_nb_hm)
    if 'ip' in protocol:
        if ip_scenarii_to_use == "protocol_agnostic_only":
            input_payload_file_nb = INPUT_PAYLOAD_FILE_IP_AGNOSTIC_SCENARII_NB
        elif ip_scenarii_to_use == "protocol_dependant_only":
            input_payload_file_nb = INPUT_PAYLOAD_FILE_IP_DEPENDENT_SCENARII_NB
        elif ip_scenarii_to_use == "full_test_case_scenarii":
            input_payload_file_nb = INPUT_PAYLOAD_FILE_IP_FULL_TC_SCENARII_NB
        elif ip_scenarii_to_use == "partial_test_case_scenarii":
            input_payload_file_nb = INPUT_PAYLOAD_FILE_IP_PARTIAL_TC_SCENARII_NB
        else:
            input_payload_file_nb = INPUT_PAYLOAD_FILE_IP_DEPENDENT_SCENARII_NB + INPUT_PAYLOAD_FILE_IP_AGNOSTIC_SCENARII_NB
    else:
        input_payload_file_nb = INPUT_PAYLOAD_FILE_NB_TCP

    print("remove_incomplete_target_from_lists: input_payload_file_nb: ",
          input_payload_file_nb)
    targets_to_keep_v = [
        target_name for target_name in target_name_v
        if target_file_nb_hm[target_name] == input_payload_file_nb
    ]
    print("remove_incomplete_target_from_lists: targets_to_keep_v: ",
          targets_to_keep_v)
    print("remove_incomplete_target_from_lists: targets to remove: ",
          set(target_name_v) - set(targets_to_keep_v))

    new_filename_v = [
        filename for filename in filename_v
        if get_target_name(filename) in targets_to_keep_v
    ]

    return (new_filename_v, targets_to_keep_v)


def build_scenario_groups_df(filename_v: list, protocol: str,
                             ip_scenarii_to_use: str):
    print("build_scenario_groups_df: start")
    target_name_v = build_target_name_v(filename_v)

    filename_no_incomplete_v, target_name_no_incomplete_v = remove_incomplete_target_from_lists(
        filename_v, target_name_v, protocol, ip_scenarii_to_use)
    target_name_scenario_groups_hm = build_target_name_scenario_groups_hm(
        filename_no_incomplete_v, target_name_no_incomplete_v)
    #print("build_scenario_groups_df: target_name_scenario_groups_hm,",target_name_scenario_groups_hm)
    scenario_groups_target_name_hm = build_scenario_groups_target_name_hm_from_target_name_scenario_groups_hm(
        target_name_scenario_groups_hm)
    #print("build_scenario_groups_df: scenario_groups_target_name_hm,",scenario_groups_target_name_hm)

    scenario_groups_v_v = [
        list(scenario_groups_t)
        for scenario_groups_t in scenario_groups_target_name_hm.keys()
    ]
    data = {
        'scenario_group': scenario_groups_v_v,
        'target_name_v': scenario_groups_target_name_hm.values()
    }
    #df = pd.DataFrame(scenario_groups_target_name_hm.items(),columns=["scenario_group","target_name_v"])
    df = pd.DataFrame(data)

    # Count the number of elements in the 'Elements' column
    df['target_name_v_count'] = df['target_name_v'].apply(len)

    # Sort the DataFrame based on the 'Element_Count' column
    df_sorted = df.sort_values(by='target_name_v_count',
                               ascending=False,
                               ignore_index=True)

    print("build_scenario_groups_df: end")
    return df_sorted


def process(target_directory_path: str, protocol: str,
            pattern_json_payload_file: str, output_csv_file: str,
            ip_scenarii_to_use: str, unsetting_strat: str):
    print("process: start")
    filename_v = glob.glob(
        f"{target_directory_path}/**/{protocol}/**/{protocol}*{pattern_json_payload_file}",
        recursive=True)
    #print("process: filename_v: ",filename_v)

    # filter protocol-dependant ip scenarii if requested
    if 'ip' in protocol:
        if ip_scenarii_to_use == "protocol_agnostic_only":
            filename_v = [
                filename for filename in filename_v
                if len(get_scenario_name(filename).split('-')) == 1
            ]
        elif ip_scenarii_to_use == "protocol_dependant_only":
            filename_v = [
                filename for filename in filename_v
                if len(get_scenario_name(filename).split('-')) == 2
            ]
        elif ip_scenarii_to_use == "full_test_case_scenarii":
            filename_v = [
                filename for filename in filename_v
                if len(get_scenario_name(filename).split('-')) == 1 or
                (len(get_scenario_name(filename).split('-')) == 2
                 and len(get_scenario_name(filename).split('-')[1]) == 2 and
                 get_scenario_name(filename).split('-')[1] not in ['mf', 'ms'])
            ]
        elif ip_scenarii_to_use == "partial_test_case_scenarii":
            if unsetting_strat == "starting":
                filename_v = [
                    filename for filename in filename_v
                    if len(get_scenario_name(filename).split('-')) == 2
                    and 's' in get_scenario_name(filename).split('-')[1]
                ]
            elif unsetting_strat == "finishing":
                filename_v = [
                    filename for filename in filename_v
                    if len(get_scenario_name(filename).split('-')) == 2
                    and 'f' in get_scenario_name(filename).split('-')[1]
                ]
    #print("process: filename_v: ",filename_v)

    df = build_scenario_groups_df(filename_v, protocol, ip_scenarii_to_use)

    csv_output_path = f"{target_directory_path}/{protocol}_{output_csv_file}"
    print("process: csv_output_path: ", csv_output_path)
    df.to_csv(csv_output_path, mode='w')

    print("process: end")


def main(argv):
    print("build_stack_scenario_groups_csv: start")

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--target-directory-path", type=str, default="")
    parser.add_argument("-p",
                        "--protocol",
                        choices=["ipv4", "ipv6", "tcp"],
                        type=str,
                        default="")
    parser.add_argument("-pjpf",
                        "--pattern-json-payload-file",
                        type=str,
                        default="")
    parser.add_argument("-o", "--output-csv-file", type=str, default="")
    parser.add_argument("--ip-scenarii-to-use",
                        choices=[
                            "protocol_dependant_only",
                            "protocol_agnostic_only", "any",
                            "full_test_case_scenarii",
                            "partial_test_case_scenarii"
                        ],
                        required=False)
    #parser.add_argument("-", "--unsetting-strat", required='--ip-scenarii-to-use' in sys.argv, type=str,)
    parser.add_argument("-",
                        "--unsetting-strat",
                        choices=["starting", "finishing"],
                        required=False)
    args = parser.parse_args()

    target_directory_path = args.target_directory_path
    protocol = args.protocol
    pattern_json_payload_file = args.pattern_json_payload_file
    output_csv_file = args.output_csv_file
    ip_scenarii_to_use = args.ip_scenarii_to_use
    unsetting_strat = args.unsetting_strat

    print(
        f"build_stack_scenario_groups_csv: target_directory_path: {target_directory_path}"
    )
    print(f"build_stack_scenario_groups_csv: protocol: {protocol}")
    print(
        f"build_stack_scenario_groups_csv: pattern_json_payload_file: {pattern_json_payload_file}"
    )
    print(
        f"build_stack_scenario_groups_csv: output_csv_file: {output_csv_file}")
    print(
        f"build_stack_scenario_groups_csv: ip_scenarii_to_use: {ip_scenarii_to_use}"
    )
    print(
        f"build_stack_scenario_groups_csv: unsetting_strat: {unsetting_strat}")
    process(target_directory_path, protocol, pattern_json_payload_file,
            output_csv_file, ip_scenarii_to_use, unsetting_strat)

    print("build_stack_scenario_groups_csv: end")


if __name__ == "__main__":
    main(sys.argv[1:])
