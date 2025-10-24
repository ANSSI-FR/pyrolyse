#!/usr/bin/python
import sys
import argparse
import matplotlib
import matplotlib.pyplot as plt
import matplotlib.colors as mcolors
import matplotlib as mpl
import numpy as np
import pandas as pd
import textwrap
import ast
import math

SCENARIO_DEPENDENT_SCENARII_NB_IP = 34
SCENARIO_AGNOSTIC_SCENARII_NB_IP = 8
SCENARII_NB_TCP = 11


def format_scenario_group_for_camera_ready(scenario_group: list) -> list:
    scenario_group_formatted = []

    for scenario_name in scenario_group:
        scenario_name_agnostic_part = scenario_name.split('-')[0]
        scenario_name_dependant_part = scenario_name.split('-')[1] if len(
            scenario_name.split('-')) > 1 else ""

        if len(scenario_name_agnostic_part) == 3:  # pep
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

        if scenario_name_dependant_part != "":
            scenario_name_formatted = r"$s^{%s}_{%s}$" % (
                scenario_name_agnostic_part_modified,
                scenario_name_dependant_part)
        else:
            scenario_name_formatted = r'$s^{%s}$' % scenario_name_agnostic_part_modified

        scenario_group_formatted.append(scenario_name_formatted)
    return scenario_group_formatted


def process(input_csv_filepath: str, output_plot_filepath: str, protocol: str,
            group_number: int, xmax: int):
    print("process: start")

    # retrieve data
    df = pd.read_csv(input_csv_filepath,
                     usecols=["scenario_group", "target_name_v_count"],
                     nrows=group_number)
    print("process: df: ", df)
    df = df.iloc[::-1]  # reverse order
    #df.reindex(index=df.index[::-1])

    if protocol == 'tcp':
        # remove protocol-dependant part if tcp
        y_label_v_v = [
            format_scenario_group_for_camera_ready(
                list(
                    set([
                        scenario.split('-')[0]
                        for scenario in ast.literal_eval(scenario_group)
                    ]))) for scenario_group in df['scenario_group']
        ]
        y_label_v_v = [
            y_label_v if len(y_label_v) != SCENARII_NB_TCP else ['all']
            for y_label_v in y_label_v_v
        ]

        # set bar color
        color = mcolors.TABLEAU_COLORS['tab:blue']
        y_label_v = [", ".join(y_label_v) for y_label_v in y_label_v_v]
        y_label_v = [textwrap.fill(y_label, width=47) for y_label in y_label_v]
    else:
        y_label_v_v = [
            format_scenario_group_for_camera_ready(
                ast.literal_eval(scenario_group))
            if len(ast.literal_eval(scenario_group))
            != SCENARIO_DEPENDENT_SCENARII_NB_IP
            and len(ast.literal_eval(scenario_group))
            != SCENARIO_AGNOSTIC_SCENARII_NB_IP else ['all']
            for scenario_group in df['scenario_group']
        ]
        color = mcolors.TABLEAU_COLORS[
            'tab:orange'] if protocol == "ipv4" else mcolors.TABLEAU_COLORS[
                'tab:green']
        y_label_v = [", ".join(y_label_v) for y_label_v in y_label_v_v]
        y_label_v = [textwrap.fill(y_label, width=35) for y_label in y_label_v]

    font = {'size': 18}
    matplotlib.rc('font', **font)
    fig, ax = plt.subplots()

    bars = ax.barh(y_label_v,
                   df['target_name_v_count'],
                   color=color,
                   edgecolor='black',
                   height=0.4)
    ax.set_xlim((0, xmax))

    ax.bar_label(bars, padding=3)

    # Setting boxes around y-label to differenciate scenario groups
    ytick_labels = ax.get_yticklabels()

    # Add a box around each y-axis tick label
    #for label in ytick_labels:
    #    label.set_bbox(dict(facecolor='none', edgecolor='black', boxstyle='round,pad=0.1', linestyle=(0, (5, 10)), linewidth=0.3))

    plt.savefig(output_plot_filepath, bbox_inches='tight')
    print("process: end")


def usage():
    print(
        "build_stack_scenario_groups_histo.py -t <target-directory-path> -p <pattern-json-payload-file> -o <output-json-payload-file>"
    )


def main(argv):
    print("build_stack_scenario_groups_histo: start")

    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input-csv-filepath", type=str, default="")
    parser.add_argument("-o", "--output-plot-filepath", type=str, default="")
    parser.add_argument("-p", "--protocol", type=str, default="")
    parser.add_argument("-n", "--group-number", type=int, default=10)
    parser.add_argument("-xmax", "--xmax", type=int, default=10)
    args = parser.parse_args()

    input_csv_filepath = args.input_csv_filepath
    output_plot_filepath = args.output_plot_filepath
    protocol = args.protocol
    group_number = args.group_number
    xmax = args.xmax

    print(
        f"build_stack_scenario_groups_histo: input_csv_filepath: {input_csv_filepath}"
    )
    print(
        f"build_stack_scenario_groups_histo: output_plot_filepath: {output_plot_filepath}"
    )
    print(f"build_stack_scenario_groups_histo: protocol: {protocol}")
    print(f"build_stack_scenario_groups_histo: group_number: {group_number}")
    print(f"build_stack_scenario_groups_histo: xmax: {xmax}")
    process(input_csv_filepath, output_plot_filepath, protocol, group_number,
            xmax)

    print("build_stack_scenario_groups_histo: end")


if __name__ == "__main__":
    main(sys.argv[1:])
