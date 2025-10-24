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


def get_scenario_agnostic_part_from_pyrolyse_scenario_name_format_with_pe(
        scenario_name: str) -> str:
    return scenario_name.split('-')[0]


def get_scenario_from_pyrolyse_scenario_name_format_with_s(
        scenario_name: str) -> str:
    scenario_name_agnostic_part = scenario_name.split('-')[0]
    if len(scenario_name_agnostic_part) == 3:  # pep (pyrolyse-like name)
        scenario_name_agnostic_part_modified = 'c'  # for contiguous (name of pep in publications)
    elif len(scenario_name_agnostic_part) == 5:  # eg peoef
        scenario_name_agnostic_part_modified = scenario_name_agnostic_part[
            3:]  # remove "peo" part from scenario name
    elif len(scenario_name_agnostic_part) == 7:  # eg peoefsf
        scenario_name_agnostic_part_modified = scenario_name_agnostic_part[
            3:5] + ',' + scenario_name_agnostic_part[
                5:7]  # remove peo and split ef and sf
    else:
        print(
            f"Invalid scenario_name_agnostic_part len: {len(scenario_name_agnostic_part)} ({scenario_name_agnostic_part})"
        )
        sys.exit(-1)

    if len(scenario_name.split('-')) > 1:
        scenario_name_dependent_part = scenario_name.split('-')[1]
        return r'$s^{%s}_{%s}$' % (scenario_name_agnostic_part_modified,
                                   scenario_name_dependent_part)
    else:
        return r'$s^{%s}$' % scenario_name_agnostic_part_modified


def get_scenario_dependant_part_from_pyrolyse_scenario_name_format(
        scenario_name: str) -> str:
    return r"$%s$" % scenario_name.split('-')[1] if len(
        scenario_name.split('-')) > 1 else ""


def build_scenario_v_from_csv_data(csv_data_v: list,
                                   agnostic_scenario_part: str) -> list:
    scenario_agnostic_v = sorted(
        list(
            set([
                scenario for hm_entry in csv_data_v
                for scenario in ast.literal_eval(hm_entry['scenario_group']) if
                len(scenario.split('-')) == 1 and agnostic_scenario_part == ""
            ])))
    scenario_dependent_v = sorted(
        list(
            set([
                scenario for hm_entry in csv_data_v
                for scenario in ast.literal_eval(hm_entry['scenario_group'])
                if len(scenario.split('-')) == 2 and (
                    agnostic_scenario_part == ""
                    or scenario.split('-')[0] == agnostic_scenario_part)
            ])))
    #scenario_v = [ scenario for hm_entry in csv_data_v for scenario in ast.literal_eval(hm_entry['scenario_group']) ]
    #return sorted(list(set(scenario_v)))
    return scenario_agnostic_v + scenario_dependent_v


def build_target_name_v_from_csv_data(csv_data_v: list) -> list:
    #target_name_v = [ target_name for (_,_,group_target_name_v,_) in csv_data_v for target_name in list(group_target_name_v) ]
    target_name_v = [
        target_name for hm_entry in csv_data_v
        for target_name in ast.literal_eval(hm_entry['target_name_v'])
    ]
    return sorted(list(set(target_name_v)))


def build_custom_corr_matrix_hm(csv_data_v: list, scenario_v: list,
                                target_name_v: list,
                                agnostic_scenario_part: str) -> dict:
    print("build_custom_corr_matrix_hm: start")
    # matrix is scenario_v * target_v
    corr_matrix_v_v = []

    for scenario_1 in scenario_v:
        scenario_1_corr_matrix_v = []

        for target in target_name_v:
            curr_corr_score = 0
            curr_target_scenario_v = []

            for hm_entry in csv_data_v:
                group_scenario_v = ast.literal_eval(hm_entry['scenario_group'])
                group_target_name_v = ast.literal_eval(
                    hm_entry['target_name_v'])
                if scenario_1 in group_scenario_v and target in group_target_name_v:
                    group_scenario_to_add = group_scenario_v if agnostic_scenario_part == "" else [
                        scenario for scenario in group_scenario_v
                        if agnostic_scenario_part == scenario.split('-')[0]
                    ]
                    curr_target_scenario_v.extend(group_scenario_to_add)

            curr_target_scenario_v_no_duplicates = list(
                set(curr_target_scenario_v))
            curr_target_scenario_v_no_duplicates_no_scenario_1 = [
                scenario for scenario in curr_target_scenario_v_no_duplicates
                if scenario != scenario_1
            ]
            curr_corr_score = len(
                curr_target_scenario_v_no_duplicates_no_scenario_1)
            scenario_1_corr_matrix_v.append(curr_corr_score)

        corr_matrix_v_v.append(scenario_1_corr_matrix_v)
    print("build_custom_corr_matrix_hm: len(corr_matrix_v_v): ",
          len(corr_matrix_v_v))

    #return np.array([ [ (e / len(target_name_v)) * 100  for e in corr_matrix_v  ] for corr_matrix_v in corr_matrix_v_v ])
    #return np.array([ [ e for e in corr_matrix_v  ] for corr_matrix_v in corr_matrix_v_v ])
    return corr_matrix_v_v


def process(input_csv_filepath: str, output_plot_filepath: str, ymax: int,
            ytick_step: int, agnostic_scenario_part: str):
    print("process: start")

    csv_data_v = pd.read_csv(input_csv_filepath).to_dict(orient='records')
    #print("process: csv_data_v: ",csv_data_v)

    scenario_v = build_scenario_v_from_csv_data(csv_data_v,
                                                agnostic_scenario_part)
    print("process: scenario_v: ", scenario_v)
    target_name_v = build_target_name_v_from_csv_data(csv_data_v)
    print("process: target_name_v: ", target_name_v)

    corr_matrix = build_custom_corr_matrix_hm(csv_data_v, scenario_v,
                                              target_name_v,
                                              agnostic_scenario_part)
    print("process: corr_matrix: ", corr_matrix)
    print("process: len(corr_matrix): ", len(corr_matrix))

    fig, ax = plt.subplots()

    ax.boxplot(corr_matrix, whis=0)
    scenario_formatted_name_v = [
        get_scenario_from_pyrolyse_scenario_name_format_with_s(scenario)
        for scenario in scenario_v
    ]
    ax.set_xticklabels(scenario_formatted_name_v)
    scenario_agnostic_len = 8 if agnostic_scenario_part == "" else 0
    for i in range(scenario_agnostic_len):
        ax.get_xticklabels()[i].set_rotation(45)
    ytick_v = [i for i in range(0, ymax, ytick_step)]
    ytick_label_v = [i for i in range(0, ymax, ytick_step)]
    ytick_v.append(ymax)
    ytick_label_v.append(f"{ymax}\n(max)")
    plt.yticks(ytick_v,
               ytick_label_v)  # ax.set_yticklabels() does not do what i want
    # For the minor ticks, use no labels; default NullFormatter.
    ax.yaxis.set_minor_locator(mpl.ticker.MultipleLocator(1))
    ax.set_ylabel(
        "number of scenarii with same reassembly as the \n considered one accross implementations",
        fontsize="10")
    ax.set_ylim(top=ymax + 1)

    plt.tight_layout()
    #plt.show()
    plt.savefig(output_plot_filepath, bbox_inches='tight')

    print("process: end")


def main(argv):
    print("build_stack_scenarii_reassembly_boxplot: start")

    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input-csv-filepath", type=str, default="")
    parser.add_argument("-o", "--output-plot-filepath", type=str, default="")
    parser.add_argument("--ymax", type=int, default=23)
    parser.add_argument("--ytick-step", type=int, default=5)
    parser.add_argument("-s", "--agnostic-scenario-part", type=str, default="")
    args = parser.parse_args()

    input_csv_filepath = args.input_csv_filepath
    output_plot_filepath = args.output_plot_filepath
    ymax = args.ymax
    ytick_step = args.ytick_step
    agnostic_scenario_part = args.agnostic_scenario_part

    print(
        f"build_stack_scenarii_reassembly_boxplot: input_csv_filepath: {input_csv_filepath}"
    )
    print(
        f"build_stack_scenarii_reassembly_boxplot: output_plot_filepath: {output_plot_filepath}"
    )
    print(f"build_stack_scenarii_reassembly_boxplot: ymax: {ymax}")
    print(f"build_stack_scenarii_reassembly_boxplot: ytick_step: {ytick_step}")
    print(
        f"build_stack_scenarii_reassembly_boxplot: agnostic_scenario_part: {agnostic_scenario_part}"
    )

    process(input_csv_filepath, output_plot_filepath, ymax, ytick_step,
            agnostic_scenario_part)

    print("build_stack_scenarii_reassembly_boxplot: end")


if __name__ == "__main__":
    main(sys.argv[1:])
