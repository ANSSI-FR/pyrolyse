#!/usr/bin/env python3

import sys
import argparse
import json
import matplotlib.pyplot as plt
import matplotlib
from math import floor
from math import ceil

# TODO delete and create scenario directories

colors = ['orange', 'green', 'purple', 'cyan', 'yellow', 'olive']
last_segment_color = 'red'
header_segment_color = 'blue'
scenarios = ["pep", "peos"]


def build_graph(tests, output_directory):
    dict_no = list(tests['hm'].keys())

    for scenario in scenarios:
        for index in dict_no:
            interval_c = tests['hm'][index]['interval_c']['hm']
            slice_data = tests['hm'][index]['chunk_c']['bm']
            print("build_graph: slice_data: ", slice_data)

            payloads = [s_info['internet_checksum_s']
                        for s_info in slice_data.values()]
            print("build_graph: payloads: ", payloads)

            plt_colors = []
            for i, s_info in enumerate(slice_data.values()):
                plt_colors.append(colors[i]) if s_info['more_chunk'] else plt_colors.append(
                    last_segment_color)

            max_offset = max(
                [offsets['end'] * 8 + 8 for offsets in interval_c.values()]) + 1
            offsets = [{'start': off['start'] * 8 + 1, 'end': off['end'] * 8 + 1}
                       for off in interval_c.values()]
            print("build_graph: offsets: ", offsets)

            xmin = [offset['start'] for offset in offsets]
            print("build_graph: xmin: ", xmin)
            xmax = [offset['end'] + 8 for offset in offsets]
            print("build_graph: xmax: ", xmax)
            y = [i for i in range(len(offsets))]
            print("build_graph: y: ", y)

            xmin_8bytes = []
            xmax_8bytes = []
            y_8bytes = []
            payloads_8bytes = [payload[i:i+8]
                               for payload in payloads for i in range(0, len(payload), 8)]
            print("build_graph: payloads_8bytes: ", payloads_8bytes)
            for i, (x_min, x_max) in enumerate(zip(xmin, xmax)):
                for n, j in enumerate(range(x_min, x_max, 8)):
                    xmin_8bytes.append(j)
                    xmax_8bytes.append(j+8)
                y_8bytes.extend([i] * (n + 1))

            print("build_graph: xmin_8bytes: ", xmin_8bytes)
            print("build_graph: xmax_8bytes: ", xmax_8bytes)
            print("build_graph: y_8bytes: ", y_8bytes)

            xmin_8_bytes_with_extra_chunk = xmin_8bytes
            xmax_8_bytes_with_extra_chunk = xmax_8bytes
            y_8_bytes_with_extra_chunk = y_8bytes
            plt_colors_8bytes = [plt_colors[y]
                                 for y in y_8_bytes_with_extra_chunk]

            if scenario == "peos":
                xmin_8_bytes_with_extra_chunk = [x + 1 for x in xmin_8bytes]
                xmax_8_bytes_with_extra_chunk = [x + 1 for x in xmax_8bytes]
                xmin_8_bytes_with_extra_chunk.append(1)
                xmax_8_bytes_with_extra_chunk.append(2)

                y_8_bytes_with_extra_chunk.append(
                    max(y_8_bytes_with_extra_chunk) + 1)
                payloads_8bytes.append("0")
                plt_colors_8bytes.append(header_segment_color)
                max_offset += 1

            print("build_graph: xmin_8_bytes_with_extra_chunk: ",
                  xmin_8_bytes_with_extra_chunk)
            print("build_graph: xmax_8_bytes_with_extra_chunk: ",
                  xmax_8_bytes_with_extra_chunk)
            print("build_graph: y_8_bytes_with_extra_chunk: ",
                  y_8_bytes_with_extra_chunk)
            print("build_graph: plt_colors_8bytes: ", plt_colors_8bytes)

            if max_offset <= 19:
                if len(offsets) <= 2:
                    fontsize = 32
                else:
                    fontsize = 30
            elif max_offset <= 25:
                if len(offsets) <= 3:
                    fontsize = 28
                else:
                    fontsize = 26
            elif max_offset <= 33:
                if len(offsets) <= 4:
                    fontsize = 24
                else:
                    fontsize = 22
            elif max_offset <= 42:
                if len(offsets) <= 5:
                    fontsize = 20
                else:
                    fontsize = 18
            elif max_offset <= 50:
                if len(offsets) <= 6:
                    fontsize = 14
                else:
                    fontsize = 10
            else:
                fontsize = 10

            font = {'size': fontsize}
            matplotlib.rc('font', **font)

            fig = plt.figure(figsize=(12, 3.8))

            hlines = plt.hlines(y_8_bytes_with_extra_chunk, xmin_8_bytes_with_extra_chunk,
                                xmax_8_bytes_with_extra_chunk, plt_colors_8bytes)

            xticks = [i for i in range(2, max_offset + 1, 8)] if scenario == 'peos' else [
                i for i in range(1, max_offset + 1, 8)]
            if scenario == 'peos':
                xticks.append(1)
            plt.xticks(xticks)

            if max(y_8_bytes_with_extra_chunk) + 1 == 2:
                plt.yticks([-1, 0, 1, 2], ('', 't$_0$', 't$_1$', ''))
            elif max(y_8_bytes_with_extra_chunk) + 1 == 3:
                plt.yticks([-1, 0, 1, 2, 3], ('', 't$_0$', 't$_1$', 't$_2$',
                                              '',))
            elif max(y_8_bytes_with_extra_chunk) + 1 == 4:
                plt.yticks([-1, 0, 1, 2, 3, 4], ('', 't$_0$', 't$_1$', 't$_2$',
                                                 't$_3$', ''))
            elif max(y_8_bytes_with_extra_chunk) + 1 == 5:
                plt.yticks([-1, 0, 1, 2, 3, 4, 5], ('', 't$_0$', 't$_1$', 't$_2$',
                                                    't$_3$', 't$_4$', ''))

            for i in range(len(xmin_8_bytes_with_extra_chunk)):
                if i == len(xmin_8_bytes_with_extra_chunk) - 1 and scenario == "peos":
                    plt.text(xmin_8_bytes_with_extra_chunk[i] + (1/len(xmin_8_bytes_with_extra_chunk)),
                             y_8_bytes_with_extra_chunk[i]+0.1,
                             payloads_8bytes[i])
                else:
                    plt.text(xmin_8_bytes_with_extra_chunk[i] + (5/len(xmin_8_bytes_with_extra_chunk)),
                             y_8_bytes_with_extra_chunk[i]+0.1,
                             payloads_8bytes[i])

            ax = plt.gca()
            ax.spines['top'].set_visible(False)
            ax.spines['right'].set_visible(False)

            plt.ylabel("time", fontsize=fontsize)
            # , loc='center', color='#314a76', fontsize=fontsize, fontfamily='DejaVu Sans')
            plt.xlabel("offset", fontsize=fontsize)
            # loc='center', color='#314a76', fontsize=fontsize, fontfamily='DejaVu Sans')

            graph_name = output_directory + "/" + scenario + \
                "/test_" + index + "_" + scenario + ".pdf"
            plt.savefig(graph_name, bbox_inches='tight')


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--byte-time-sequence", type=str, default="")
    parser.add_argument("-o", "--output-directory", type=str, default="")
    args = parser.parse_args()

    byte_time_sequence = args.byte_time_sequence
    output_directory = args.output_directory

    with open(byte_time_sequence) as f:
        data = json.load(f)

    pairs = data['byte_time_pair_sequence_c']
    triplets = data['byte_time_triplet_sequence_c']

    build_graph(pairs, output_directory)
    build_graph(triplets, output_directory)


if __name__ == "__main__":
    main(sys.argv[1:])
