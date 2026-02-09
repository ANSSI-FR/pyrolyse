#!/usr/bin/python
import sys
import os
import argparse
import glob
import pandas as pd
import json
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.colors as mcolors
import matplotlib as mpl
import pandas as pd

from matplotlib.colors import LinearSegmentedColormap
from mpl_toolkits.axes_grid1 import make_axes_locatable
from numpy.ma import masked_array
import matplotlib.ticker as ticker

import ast
from collections import Counter

from distutils.util import strtobool


def get_scenario_agnostic_part_from_pyrolyse_scenario_name_format_with_pe(scenario_name:str) -> str:
    return scenario_name.split('-')[0]

def get_scenario_from_pyrolyse_scenario_name_format_with_s(scenario_name:str) -> str:
    scenario_name_agnostic_part = scenario_name.split('-')[0]
    if len(scenario_name_agnostic_part) == 3: #pep
            scenario_name_agnostic_part_modified = 'c' # for contiguous
    elif len(scenario_name_agnostic_part) == 5: # eg peoef
        scenario_name_agnostic_part_modified = scenario_name_agnostic_part[3:] # remove peo 
    elif len(scenario_name_agnostic_part) == 7: # eg peoefsf
        scenario_name_agnostic_part_modified = scenario_name_agnostic_part[3:5] + ',' + scenario_name_agnostic_part[5:7] # remove peo and split ef and sf 
    else:
        print(f"Invalid scenario_name_agnostic_part len: {len(scenario_name_agnostic_part)} ({scenario_name_agnostic_part})")
        sys.exit(-1)
    
    if len(scenario_name.split('-')) > 1 and scenario_name.split('-')[1] != "ap":
        scenario_name_dependent_part = scenario_name.split('-')[1]
        return r'$s^{%s}_{%s}$' %(scenario_name_agnostic_part_modified,scenario_name_dependent_part)
    else:
        return r'$s^{%s}$' %scenario_name_agnostic_part_modified
    

def get_scenario_dependant_part_from_pyrolyse_scenario_name_format(scenario_name:str) -> str:
    return r"$%s$"%scenario_name.split('-')[1] if len(scenario_name.split('-')) > 1 else ""


def compute_hm_corr_score(hm_1:dict,hm_2:dict,scenario_1:str,scenario_2:str, target:str) -> tuple:
    """ Compute the correlation score between two targets as the number of test cases with the same reassembly"""
    corr_score = 0
    tc_nb_not_present = 0
    if len(hm_1) != len(hm_2):
        print(f"compute_hm_corr_score: the number of test cases in the payload files of {scenario_1} and {scenario_2} for target {target} mismatch")
        #sys.exit(-1)
        tc_nb_not_present = abs(len(hm_1) - len(hm_2))
        for tc_index_1,tc_payload_1 in hm_1.items():
            # tc that are not present in both dict count for 0 in the correlation score
            corr_score = corr_score + 1 if tc_index_1 in hm_2 and hm_2[tc_index_1] == tc_payload_1 else corr_score
        #remaining_tc_index_s = set(hm_2.keys()) - set(hm_1.keys())
    else:    
        for (tc_index_1,tc_payload_1), (tc_index_2,tc_payload_2) in zip(hm_1.items(), hm_2.items()):
            assert tc_index_1 == tc_index_2
            #if tc_index_1 != tc_index_2:
            #    print(f"compute_hm_corr_score: tc_index_1 {tc_index_1} != tc_index_2 {tc_index_2} for {target_1} and {target_2} target with scenario {scenario}")
            if tc_payload_1 == tc_payload_2:
                corr_score += 1
        if len(hm_1) < 422:
            corr_score += 422 - len(hm_1)
            
    return (corr_score,tc_nb_not_present)

def build_index_payload_hm(filepath:str) -> dict:
    with open(filepath,'r') as f:
        json_hm = json.load(f)

    if json_hm == { }:
        print(f"build_index_payload_hm: filepath {filepath} is empty")
        sys.exit(-1)
    
    hm = { tc_index:tc_reassembly["payload"] for tc_index, tc_reassembly in json_hm['hm'].items() }
    hm_ordered = dict(sorted(hm.items()))
    
    return hm_ordered

def get_target_name_from_filepath(filepath: str) -> str:
    return filepath.split("/")[-4]

def get_scenario_name_from_filepath(filepath: str) -> str:
    return os.path.basename(filepath).split('_')[1]

def build_custom_corr_matrix_hm(filepath_v:list, target_name_v:list, scenario_v: list, total_tc_nb: int) -> dict:
    print("build_custom_corr_matrix_hm: start")
    # matrix is scenario_v * target_v
    corr_matrix_v_v = [ ]
    total_tc_nb_not_present_v_v = [ ]
    for scenario_1 in scenario_v:
        scenario_1_corr_matrix_v = [ ]
        filepath_scenario_1_v = [ filepath for filepath in filepath_v if scenario_1 == get_scenario_name_from_filepath(filepath) ]
        scenario_1_tc_nb_not_present_v = [ ]
        for scenario_2 in scenario_v:
            if scenario_1 == scenario_2:
                continue

            filepath_scenario_2_v = [ filepath for filepath in filepath_v if scenario_2 == get_scenario_name_from_filepath(filepath) ]
            curr_corr_score = 0
            scenario_1_2_tc_nb_not_present = 0

            for target in target_name_v: 
                filepath_scenario_1 = next(filepath_scenario for filepath_scenario in filepath_scenario_1_v if get_target_name_from_filepath(filepath_scenario) == target)
                filepath_scenario_2 = next(filepath_scenario for filepath_scenario in filepath_scenario_2_v if get_target_name_from_filepath(filepath_scenario) == target)

                hm_1 = build_index_payload_hm(filepath_scenario_1)
                hm_2 = build_index_payload_hm(filepath_scenario_2)
                hm_corr_score,tc_nb_not_present = compute_hm_corr_score(hm_1,hm_2,scenario_1,scenario_2,target)
                curr_corr_score += hm_corr_score
                scenario_1_2_tc_nb_not_present += tc_nb_not_present

            scenario_1_corr_matrix_v.append(curr_corr_score)
            scenario_1_tc_nb_not_present_v.append(scenario_1_2_tc_nb_not_present)

            if len(scenario_1.split('-')) == 2 and scenario_1.split('-')[0] == scenario_2.split('-')[0] and scenario_1.split('-')[1][0] == scenario_2.split('-')[1][0] and scenario_1.split('-')[1][1] != scenario_2.split('-')[1][1]:
                print(f"{scenario_1} vs {scenario_2}: {curr_corr_score} similarly reassembled tc across implem -> {curr_corr_score/ ( total_tc_nb * (len(target_name_v)) - scenario_1_2_tc_nb_not_present ) } similarity between the two scenarii")
        corr_matrix_v_v.append(scenario_1_corr_matrix_v)
        total_tc_nb_not_present_v_v.append(scenario_1_tc_nb_not_present_v)

    return [ [ e_cm / ( total_tc_nb * (len(target_name_v)) - tc_nb )  for e_cm,tc_nb in zip(cm_v,tc_nb_v) ] for cm_v,tc_nb_v in zip(corr_matrix_v_v,total_tc_nb_not_present_v_v) ]


def build_scenario_v_from_filepath_v(filepath_v: list, only_full_tc_w_protocol_dependent_scenarii:bool) -> list:
    scenario_v = list(set([ get_scenario_name_from_filepath(filepath) for filepath in filepath_v ]))
    scenario_agnostic_v = [ scenario for scenario in scenario_v if len(scenario.split('-')) == 1 and only_full_tc_w_protocol_dependent_scenarii == False ]
    scenario_dependent_v = [ scenario for scenario in scenario_v if len(scenario.split('-')) == 2 and (len(scenario.split('-')[1]) == 2  and scenario.split('-')[1] not in ['mf','ms','ao']) ]

    return scenario_agnostic_v + scenario_dependent_v

def build_target_v_from_filepath_v(filepath_v:list) -> list:
    return sorted(list(set([ get_target_name_from_filepath(filepath) for filepath in filepath_v ])))

def process(target_directory_path:str, protocol:str,
            json_payload_file:str, output_plot_filepath: str,
            only_full_tc_w_protocol_dependent_scenarii:str):
    print("process: start")

    filepath_v = glob.glob(f"{target_directory_path}/**/{protocol}/**/{protocol}*{json_payload_file}",recursive=True)
    #print("process: filepath_v: ",filepath_v)
    scenario_v = build_scenario_v_from_filepath_v(filepath_v,only_full_tc_w_protocol_dependent_scenarii)
    print("process: scenario_v: ",scenario_v)
    target_name_v = build_target_v_from_filepath_v(filepath_v)
    print("process: target_name_v: ",target_name_v)

    corr_matrix = build_custom_corr_matrix_hm(filepath_v,target_name_v,scenario_v,422)
    #print("process: corr_matrix: ",corr_matrix)
    print("process: len(corr_matrix): ",len(corr_matrix))
    df = pd.DataFrame(corr_matrix,index=scenario_v)
    print("process: len(df): ",len(df.values))
    print("process: df: ",df)

    # from https://stackoverflow.com/questions/21912634/how-can-i-sort-a-boxplot-in-pandas-by-the-median-values
    meds = df.median(1)
    print("process: len(meds): ",len(meds))
    print("process: meds: ",meds)
    meds.sort_values(ascending=False, inplace=True)
    print("process: meds: ",meds)
    print("process: meds.index: ",meds.index)
    df_sorted = df.loc[meds.index]
    print("process: df_sorted: ",df_sorted)
    print("process: df_sorted.index: ",df_sorted.index)

    fig,ax = plt.subplots(figsize=(6, 3))

    ax.boxplot(df_sorted.T)
    ax.set_xticklabels(df_sorted.index)
    scenario_formatted_name_v = [ get_scenario_from_pyrolyse_scenario_name_format_with_s(scenario) for scenario in df_sorted.index ] 
    ax.set_xticklabels(scenario_formatted_name_v)
    # For the minor ticks, use no labels; default NullFormatter.
    ax.yaxis.set_minor_locator(mpl.ticker.MultipleLocator(1))
    ax.set_ylabel("reassembly similarity \n with other scenarii")
    ax.set_ylim(0.6,1)
    
    plt.tight_layout()
    #plt.show()
    plt.savefig(output_plot_filepath, bbox_inches='tight')

    print("process: end")


def main(argv):
    print("build_stack_scenarii_reassembly_boxplot: start")

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--target-directory-path", type=str, default="")
    parser.add_argument("-p", "--protocol", choices=["ipv4", "ipv6", "tcp"], type=str, default="")
    parser.add_argument("-jpf", "--json-payload-file", type=str, default="")
    parser.add_argument("-o", "--output-plot-filepath", type=str, default="")
    parser.add_argument("-s","--only-full-tc-w-protocol-dependent-scenarii", dest='subset_scenarii', type=lambda x: bool(strtobool(x)), default=False)
    args = parser.parse_args()

    target_directory_path = args.target_directory_path
    protocol = args.protocol
    json_payload_file = args.json_payload_file
    output_plot_filepath = args.output_plot_filepath
    only_full_tc_w_protocol_dependent_scenarii = args.subset_scenarii

    print(
        f"build_stack_scenarii_reassembly_boxplot: target_directory_path: {target_directory_path}"
    )
    print(
        f"build_stack_scenarii_reassembly_boxplot: protocol: {protocol}"
    )
    print(
        f"build_stack_scenarii_reassembly_boxplot: json_payload_file: {json_payload_file}"
    )
    print(
        f"build_stack_scenarii_reassembly_boxplot: output_plot_filepath: {output_plot_filepath}"
    )
    print(
        f"build_stack_scenarii_reassembly_boxplot: only_full_tc_w_protocol_dependent_scenarii: {only_full_tc_w_protocol_dependent_scenarii}"
    )

    process(target_directory_path, protocol, json_payload_file, output_plot_filepath, only_full_tc_w_protocol_dependent_scenarii)

    print("build_stack_scenarii_reassembly_boxplot: end")


if __name__ == "__main__":
    main(sys.argv[1:])
