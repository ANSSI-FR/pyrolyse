#!/usr/bin/env python3

import sys
import argparse
import matplotlib.pyplot as plt
import matplotlib
import pathlib
import json

start_chunk_color = 'blue'
end_chunk_color = 'red'
test_case_chunk_color = 'black'

def build_graph(
        output_file: str,
        offset_start_v: list,
        offset_end_v: list,
        chunk_time_v: list,
        chunk_color_v: list
):
    print("build_graph: start")

    plt.figure(figsize=(12, 3.8))

    fontsize = 26
    font = {'size': fontsize}
    matplotlib.rc('font', **font)

    plt.hlines(chunk_time_v, offset_start_v,
            offset_end_v, chunk_color_v, linewidth=5)

    # remove useless axis
    ax = plt.gca()
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)

    # y axis
    if max(chunk_time_v) == 0:
        plt.yticks([-1, 0, 1], ('', 't$_0$', ''))
    elif max(chunk_time_v) == 1:
        plt.yticks([-1, 0, 1, 2], ('', 't$_0$', 't$_1$', ''))
    elif max(chunk_time_v) == 2:
        plt.yticks([-1, 0, 1, 2, 3], ('', 't$_0$', 't$_1$', 't$_2$',
                                        '',))

    plt.ylabel("Time")

    # x axis
    plt.xlabel("Byte offset")
    ax.xaxis.set_major_locator(matplotlib.ticker.MaxNLocator(integer=True))
    
    # store plot
    plt.savefig(output_file, bbox_inches='tight')
    print("build_graph: end")


def process(
        input_json_file: str,
        output_directory: str
):
    print("process: start")
    print("process: input_json_file: ", input_json_file)
    with open(input_json_file) as json_file:
        json_file_content = json.load(json_file) 
    
    pathlib.Path(output_directory).mkdir(parents=True, exist_ok=True)

    for scenario_s, chunks in json_file_content.items(): 
        chunk_start_v, chunk_end_v, chunk_time_v, chunk_color_v = [], [], [], []

        for chunk in chunks.values():
            chunk_start_v.append(chunk["x_start"])
            chunk_end_v.append(chunk["x_end"])
            chunk_time_v.append(chunk["y"])
            if chunk["chunk_type"] == "test_case":
                chunk_color_v.append(test_case_chunk_color)
            elif chunk["chunk_type"] == "start":
                chunk_color_v.append(start_chunk_color)
            elif chunk["chunk_type"] == "end":
                chunk_color_v.append(end_chunk_color)

        
        output_file = f"{output_directory}/generic_{scenario_s}.pdf"
        print("process: output_file: ", output_file)
        build_graph(
            output_file,
            chunk_start_v,
            chunk_end_v,
            chunk_time_v, 
            chunk_color_v
        )

    print("process: end")

def main(argv):
    print("main: start")
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input-json-file", type=str, default="")
    parser.add_argument("-o", "--output-directory", type=str, default="")
    args = parser.parse_args()

    input_json_file = args.input_json_file
    output_directory = args.output_directory
    print("input_json_file: ", input_json_file)
    print("output_directory: ", output_directory)

    process(input_json_file,output_directory)

    print("main: end")


if __name__ == "__main__":
    main(sys.argv[1:])
