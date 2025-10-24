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


def get_scenario_agnostic_part_from_pyrolyse_scenario_name_format_with_s(
        scenario_name: str) -> str:
    scenario_name_agnostic_part = scenario_name.split('-')[0]
    if len(scenario_name_agnostic_part) == 3:  #pep
        scenario_name_agnostic_part_modified = 'c'  # for contiguous
    elif len(scenario_name_agnostic_part) == 5:  # eg peoef
        scenario_name_agnostic_part_modified = scenario_name_agnostic_part[
            3:]  # remove peo
    elif len(scenario_name_agnostic_part) == 7:  # eg peoefsf
        scenario_name_agnostic_part_modified = scenario_name_agnostic_part[
            3:5] + ',' + scenario_name_agnostic_part[
                5:7]  # remove peo and split ef and sf
    else:
        print(
            f"Invalid scenario_name_agnostic_part len: {len(scenario_name_agnostic_part)} ({scenario_name_agnostic_part})"
        )
        sys.exit(-1)

    return r'$s^{%s}$' % scenario_name_agnostic_part_modified


def get_scenario_dependant_part_from_pyrolyse_scenario_name_format(
        scenario_name: str) -> str:
    return r"$%s$" % scenario_name.split('-')[1] if len(
        scenario_name.split('-')) > 1 else ""


def build_scenario_v_from_csv_data(csv_data_v: list) -> list:
    scenario_v = [
        scenario for hm_entry in csv_data_v
        for scenario in ast.literal_eval(hm_entry['scenario_group'])
        if "-ao" not in scenario
    ]
    #scenario_v = [ hm['scenario_group'] for hm in csv_data_v ]
    return sorted(list(set(scenario_v)))


def build_target_name_v_from_csv_data(csv_data_v: list) -> list:
    #target_name_v = [ target_name for (_,_,group_target_name_v,_) in csv_data_v for target_name in list(group_target_name_v) ]
    target_name_v = [
        target_name for hm_entry in csv_data_v
        for target_name in ast.literal_eval(hm_entry['target_name_v'])
    ]
    return sorted(list(set(target_name_v)))


def build_data_df_from_corr_matrix(corr_matrix: np.array,
                                   target_name_v: list) -> pd.DataFrame:
    return pd.DataFrame(corr_matrix,
                        columns=target_name_v,
                        index=target_name_v)


def build_custom_corr_matrix_hm(csv_data_v: list, scenario_v: list,
                                target_name_v: list) -> dict:
    print("build_custom_corr_matrix_hm: start")
    corr_matrix_v_v = []
    for scenario_1 in scenario_v:
        scenario_1_corr_matrix_v = []
        for scenario_2 in scenario_v:
            #print("build_custom_corr_matrix_hm: scenario_1,scenario_2: ",scenario_1, scenario_2)
            curr_corr_score = 0
            curr_target_name_v = []
            #for (_,group_scenario_v, group_target_name_v, _) in csv_data_v:
            for hm_entry in csv_data_v:
                group_scenario_v = ast.literal_eval(hm_entry['scenario_group'])
                group_target_name_v = ast.literal_eval(
                    hm_entry['target_name_v'])
                if scenario_1 in group_scenario_v and scenario_2 in group_scenario_v:
                    curr_target_name_v.extend(group_target_name_v)
                #print("build_custom_corr_matrix_hm: curr_target_name_v: ",curr_target_name_v)
            curr_target_name_v_no_duplicates = list(set(curr_target_name_v))
            #print("build_custom_corr_matrix_hm: curr_target_name_v_no_duplicates: ",curr_target_name_v_no_duplicates)
            curr_corr_score = len(curr_target_name_v_no_duplicates)
            #print("build_custom_corr_matrix_hm: curr_corr_score: ",curr_corr_score)
            scenario_1_corr_matrix_v.append(curr_corr_score)
            #print("build_custom_corr_matrix_hm: scenario_1_corr_matrix_v: ",scenario_1_corr_matrix_v)
        corr_matrix_v_v.append(scenario_1_corr_matrix_v)

    return np.array([[e for e in corr_matrix_v]
                     for corr_matrix_v in corr_matrix_v_v])


def process(input_csv_filepath: str, output_plot_filepath: str):
    print("process: start")

    csv_data_v = pd.read_csv(input_csv_filepath).to_dict(orient='records')
    print("process: csv_data_v: ", csv_data_v)

    scenario_v = build_scenario_v_from_csv_data(csv_data_v)
    print("process: scenario_v: ", scenario_v)
    target_name_v = build_target_name_v_from_csv_data(csv_data_v)
    print("process: target_name_v: ", target_name_v)

    corr_matrix = build_custom_corr_matrix_hm(csv_data_v, scenario_v,
                                              target_name_v)
    print("process: corr_matrix: ", corr_matrix)

    cmap = LinearSegmentedColormap.from_list('managua',
                                             ['white', 'blue', 'black'])

    cmap.set_over(
        'xkcd:purple'
    )  # see other colors https://xkcd.com/color/rgb/ or https://matplotlib.org/stable/users/explain/colors/colors.html

    fig, ax = plt.subplots()

    pa = ax.imshow(corr_matrix, cmap=cmap, vmin=0, vmax=len(target_name_v) - 1)

    divider = make_axes_locatable(ax)
    cax = divider.append_axes("right", size="5%", pad=0.1)
    cbb = plt.colorbar(pa,
                       cax=cax,
                       extend='max',
                       extendfrac=0.1,
                       ticks=ticker.IndexLocator(base=5, offset=0))

    cbb.set_label('Implementation number')

    # Show all ticks and label them with the respective list entries.
    scenario_dependant_part_v = [
        get_scenario_dependant_part_from_pyrolyse_scenario_name_format(
            scenario) for scenario in scenario_v
    ]
    ax.set_xticks(range(corr_matrix.shape[1]),
                  labels=scenario_dependant_part_v,
                  rotation='vertical',
                  rotation_mode="default")
    ax.set_yticks(range(corr_matrix.shape[0]),
                  labels=scenario_dependant_part_v)

    # only display on scenario agnostic name
    sec_y_ag = ax.secondary_yaxis(location=-0.15)
    scenario_agnostic_part_v = [
        get_scenario_agnostic_part_from_pyrolyse_scenario_name_format_with_pe(
            scenario) for scenario in scenario_v
    ]
    scenario_agnostic_part_v_counter = Counter(scenario_agnostic_part_v)
    print("process: scenario_agnostic_part_v_counter: ",
          scenario_agnostic_part_v_counter)

    scenario_agnostic_part_loc_v = []
    line_loc_v = []
    #curr_scenario_begining_loc = len(scenario_v)
    curr_scenario_begining_loc = 0
    for count in scenario_agnostic_part_v_counter.values():
        line_loc_v.append(curr_scenario_begining_loc - 0.5)
        scenario_agnostic_part_loc_v.append(curr_scenario_begining_loc +
                                            count / 2)
        curr_scenario_begining_loc += count
    print("process: scenario_agnostic_part_loc_v: ",
          scenario_agnostic_part_loc_v)

    scenario_agnostic_part_label_v = [
        get_scenario_agnostic_part_from_pyrolyse_scenario_name_format_with_s(
            agnostic_part)
        for agnostic_part in scenario_agnostic_part_v_counter.keys()
    ]
    print("process: scenario_agnostic_part_label_v: ",
          scenario_agnostic_part_label_v)

    sec_y_ag.set_yticks(scenario_agnostic_part_loc_v,
                        labels=scenario_agnostic_part_label_v,
                        ha="center",
                        va="center")
    sec_y_ag.tick_params('y', length=0)
    sec_y_ag.spines['left'].set_linewidth(0)

    sec_x_ag = ax.secondary_xaxis(location=1.15)
    sec_x_ag.set_xticks(scenario_agnostic_part_loc_v,
                        labels=scenario_agnostic_part_label_v,
                        ha="center",
                        va="center")
    sec_x_ag.tick_params('x', length=0)
    sec_x_ag.spines['top'].set_linewidth(0)

    # lines between the classes from https://stackoverflow.com/questions/19184484/how-to-add-group-labels-for-bar-charts
    sec_y_lines = ax.secondary_yaxis(location=0)
    sec_y_lines.set_yticks(line_loc_v, labels=[])
    sec_y_lines.tick_params('y', length=40, width=0.5)
    sec_y_lines.spines['left'].set_linewidth(0)

    sec_x_lines = ax.secondary_xaxis(location=1)
    sec_x_lines.set_xticks(line_loc_v, labels=[])
    sec_x_lines.tick_params('x', length=40, width=0.5)
    sec_x_lines.spines['top'].set_linewidth(0)

    # Let the horizontal axes labeling appear on top.
    ax.tick_params(top=True, bottom=False, labeltop=True, labelbottom=False)

    # Turn spines off and create white grid.
    ax.spines[:].set_visible(False)

    ax.set_xticks(np.arange(corr_matrix.shape[1] + 1) - .5, minor=True)
    ax.set_yticks(np.arange(corr_matrix.shape[0] + 1) - .5, minor=True)
    ax.grid(which="minor", color="w", linestyle='-', linewidth=1.5)
    ax.grid(which="minor", color="w")
    ax.tick_params(which="minor", bottom=False, left=False)

    plt.tight_layout()
    #plt.show()
    plt.savefig(output_plot_filepath, bbox_inches='tight')

    print("process: end")


def main(argv):
    print("build_stack_scenarii_reassembly_closeness_heatmap: start")

    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input-csv-filepath", type=str, default="")
    parser.add_argument("-o", "--output-plot-filepath", type=str, default="")
    args = parser.parse_args()

    input_csv_filepath = args.input_csv_filepath
    output_plot_filepath = args.output_plot_filepath

    print(
        f"build_stack_scenarii_reassembly_closeness_heatmap: input_csv_filepath: {input_csv_filepath}"
    )
    print(
        f"build_stack_scenarii_reassembly_closeness_heatmap: output_plot_filepath: {output_plot_filepath}"
    )

    process(input_csv_filepath, output_plot_filepath)

    print("build_stack_scenarii_reassembly_closeness_heatmap: end")


if __name__ == "__main__":
    main(sys.argv[1:])
