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

# Global variables

INPUT_PAYLOAD_FILE_IP_DEPENDENT_SCENARII_NB = 34
INPUT_PAYLOAD_FILE_IP_AGNOSTIC_SCENARII_NB = 8
INPUT_PAYLOAD_FILE_NB_TCP = 18

TOTAL_TC_NUMBER_TCP = 7596
TOTAL_TC_NUMBER_TCP_LWIP = 7596 - 9 * 18
TOTAL_TC_NUMBER_PROTOCOL_AGNOSTIC_IP = 3376
TOTAL_TC_NUMBER_PROTOCOL_DEPENDENT_IP = 6986

TARGET_LIST_TCP = [
    "debian_8", "debian_12", "freebsd_10.2", "freebsd_12.1", "freebsd_14.1",
    "openbsd_6.0", "openbsd_7.6", "netbsd_7.0", "netbsd_9.3", "solaris_11.4",
    "windows_2016", "windows_11", "lwip_2.2.1", "uip_1.0", "picotcp_1.7.0",
    "smoltcp_0.12.0", "seastar_22.11.0", "mirage_9.0.0", "chelsiotoe_3.19.0.3",
    "xilinxonload_9.0.1", "snort_default", "suricata_default", "zeek_default"
]
TARGET_LIST_IPV4 = [
    "debian_8", "debian_12", "freebsd_10.2", "freebsd_12.1", "freebsd_14.1",
    "openbsd_6.0", "openbsd_7.6", "solaris_11.4", "windows_2016", "windows_11",
    "lwip_2.2.1", "picotcp_1.7.0", "smoltcp_0.12.0", "seastar_22.11.0",
    "mirage_9.0.0", "snort_default", "suricata_default", "zeek_default"
]
TARGET_LIST_IPV6 = [
    "debian_8", "debian_12", "freebsd_10.2", "freebsd_12.1", "freebsd_14.1",
    "openbsd_6.0", "openbsd_7.6", "netbsd_7.0", "netbsd_9.3", "solaris_11.4",
    "windows_2016", "windows_11", "lwip_2.2.1", "uip_1.0", "picotcp_1.7.0",
    "smoltcp_0.12.0", "mirage_9.0.0", "snort_default", "suricata_default",
    "zeek_default"
]

TARGET_DISPLAY_NAME_IP = {
    "debian_8": r'Debian 8',
    "debian_12": r'Debian 9$\rightarrow$12',
    "freebsd_10.2": r'FreeBSD 10.2$\rightarrow$11.2',
    "freebsd_12.1": r'FreeBSD 11.3$\rightarrow$12.1',
    "freebsd_14.1": r'FreeBSD 12.2$\rightarrow$14.1',
    "openbsd_6.0": r'OpenBSD 6.0',
    "openbsd_7.6": r'OpenBSD 6.1$\rightarrow$7.6',
    "netbsd_7.0": r'NetBSD 7.0',
    "netbsd_9.3": r'NetBSD 8.0$\rightarrow$9.3',
    "solaris_11.4": r'Solaris 11.2$\rightarrow$11.4',
    "windows_2016": r'Windows$<$x build',
    "windows_11": r'Windows$\geq$x build',
    "lwip_2.2.1": r'lwIP 2.2.1',
    "uip_1.0": r'uIP 1.0',
    "picotcp_1.7.0": r'picoTCP 1.7.0',
    "smoltcp_0.12.0": r'smoltcp 0.12.0',
    "mirage_9.0.0": r'mirage-tcpip 9.0.0',
    "seastar_22.11.0": r'Seastar 22.11.0',
    "snort_default": r'Snort-${linux}$ 3.7.1.0',
    "suricata_default": r'Suricata-${bsd}$ 7.0.9',
    "zeek_default": r'Zeek 7.1.1',
}

TARGET_DISPLAY_NAME_TCP = {
    "debian_8": r'Debian 8',
    "debian_12": r'Debian 9$\rightarrow$12',
    "freebsd_10.2": r'FreeBSD 10.2$\rightarrow$11.2',
    "freebsd_12.1": r'FreeBSD 11.3$\rightarrow$12.1',
    "freebsd_14.1": r'FreeBSD 12.2$\rightarrow$14.1',
    "openbsd_6.0": r'OpenBSD 6.0',
    "openbsd_7.6": r'OpenBSD 6.1$\rightarrow$7.6',
    "netbsd_7.0": r'NetBSD 7.0',
    "netbsd_9.3": r'NetBSD 8.0$\rightarrow$9.3',
    "solaris_11.4": r'Solaris 11.2$\rightarrow$11.4',
    "windows_2016": r'Windows$<$x build',
    "windows_11": r'Windows$\geq$x build',
    "lwip_2.2.1": r'lwIP 2.2.1',
    "uip_1.0": r'uIP 1.0',
    "picotcp_1.7.0": r'picoTCP 1.7.0',
    "smoltcp_0.12.0": r'smoltcp 0.12.0',
    "mirage_9.0.0": r'mirage-tcpip 9.0.0',
    "seastar_22.11.0": r'Seastar 22.11.0',
    "chelsiotoe_3.19.0.3": r'Chelsio T520-CR',
    "xilinxonload_9.0.1": r'Xilinx Onload 9.0.1',
    "snort_default": r'Snort-${bsd}$ 3.7.1.0',
    "suricata_default": r'Suricata-${bsd}$ 7.0.9',
    "zeek_default": r'Zeek 7.1.1',
}


def get_scenarii_related_const_values(protocol: str,
                                      ip_scenarii_to_use: str) -> tuple:
    """ Provide protocol-related information (number of scenarii, number of test cases, target names as in pyroyse, target names as in paper)"""
    if 'ip' in protocol:
        if ip_scenarii_to_use == "protocol_agnostic_only":
            input_payload_file_nb = INPUT_PAYLOAD_FILE_IP_AGNOSTIC_SCENARII_NB
            total_tc_nb = TOTAL_TC_NUMBER_PROTOCOL_AGNOSTIC_IP
        elif ip_scenarii_to_use == "protocol_dependant_only":
            input_payload_file_nb = INPUT_PAYLOAD_FILE_IP_DEPENDENT_SCENARII_NB
            total_tc_nb = TOTAL_TC_NUMBER_PROTOCOL_DEPENDENT_IP
        else:
            input_payload_file_nb = INPUT_PAYLOAD_FILE_IP_DEPENDENT_SCENARII_NB + INPUT_PAYLOAD_FILE_IP_AGNOSTIC_SCENARII_NB
            total_tc_nb = TOTAL_TC_NUMBER_PROTOCOL_AGNOSTIC_IP + TOTAL_TC_NUMBER_PROTOCOL_DEPENDENT_IP

        target_list = TARGET_LIST_IPV4 if protocol == "ipv4" else TARGET_LIST_IPV6
        target_display_name = TARGET_DISPLAY_NAME_IP
    else:
        input_payload_file_nb = INPUT_PAYLOAD_FILE_NB_TCP
        total_tc_nb = TOTAL_TC_NUMBER_TCP
        target_list = TARGET_LIST_TCP
        target_display_name = TARGET_DISPLAY_NAME_TCP

    return input_payload_file_nb, total_tc_nb, target_list, target_display_name


def build_target_name_v(filename_v: list) -> list:
    """ Build a list of target names without duplicates from a list of payload json filenames"""
    target_name_v = [get_target_name(filename) for filename in filename_v]
    #print("build_target_name_v: target_name_v: ",target_name_v)
    target_name_no_duplicates_v = list(set(target_name_v))
    #print("build_target_name_v: target_name_no_duplicates_v: ",target_name_no_duplicates_v)
    target_name_no_duplicates_sorted_v = sorted(target_name_no_duplicates_v)

    return target_name_no_duplicates_sorted_v


def get_target_name(filename: str) -> str:
    return filename.split("target")[1].split("/")[2]


def get_scenario_name(filename: str) -> str:
    return os.path.basename(filename).split('_')[1]


def remove_incomplete_target_from_lists(filename_v: list, target_name_v: list,
                                        input_payload_file_nb: int) -> tuple:
    target_file_nb_hm = {
        target_name: sum(1 for filename in filename_v
                         if target_name in filename)
        for target_name in target_name_v
    }
    print("remove_incomplete_target_from_lists: target_file_nb_hm: ",
          target_file_nb_hm)
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


def build_index_payload_hm(filename: str) -> dict:
    with open(filename, 'r') as f:
        json_hm = json.load(f)

    if json_hm == {}:
        print(f"build_index_payload_hm: filename {filename} is empty")
        sys.exit(-1)

    hm = {
        tc_index: tc_reassembly["payload"]
        for tc_index, tc_reassembly in json_hm['hm'].items()
    }
    hm_ordered = dict(sorted(hm.items()))

    return hm_ordered


def compute_hm_corr_score(hm_1: dict, hm_2: dict, target_1: str, target_2: str,
                          scenario: str) -> float:
    """ Compute the correlation score between two targets as the number of test cases with the same reassembly"""
    corr_score = 0
    if len(hm_1) != len(hm_2):
        print(
            f"compute_hm_corr_score: the number of test cases in the payload files of {target_1} and {target_2} target with scenario {scenario} mismatch"
        )
        for tc_index_1, tc_payload_1 in hm_1.items():
            # tc that are not present in both dict count for 0 in the correlation score
            corr_score = corr_score + 1 if tc_index_1 in hm_2 and hm_2[
                tc_index_1] == tc_payload_1 else corr_score
        #remaining_tc_index_s = set(hm_2.keys()) - set(hm_1.keys())
    else:
        for (tc_index_1,
             tc_payload_1), (tc_index_2,
                             tc_payload_2) in zip(hm_1.items(), hm_2.items()):
            assert tc_index_1 == tc_index_2
            #if tc_index_1 != tc_index_2:
            #    print(f"compute_hm_corr_score: tc_index_1 {tc_index_1} != tc_index_2 {tc_index_2} for {target_1} and {target_2} target with scenario {scenario}")
            if tc_payload_1 == tc_payload_2:
                corr_score += 1
    return corr_score


def build_custom_corr_matrix_hm(filename_v: list, target_name_v: list,
                                total_tc_nb: int, protocol: str) -> dict:
    print("build_custom_corr_matrix_hm: start")
    corr_matrix_v_v = []
    for target_name_1 in target_name_v:
        filename_target_1_v = [
            filename for filename in filename_v if target_name_1 in filename
        ]
        target_1_corr_matrix_v = []
        for target_name_2 in target_name_v:
            filename_target_2_v = [
                filename for filename in filename_v
                if target_name_2 in filename
            ]

            curr_corr_score = 0
            for filename_target_1 in filename_target_1_v:
                scenario = get_scenario_name(filename_target_1)
                filename_target_2 = next(
                    filename_target for filename_target in filename_target_2_v
                    if get_scenario_name(filename_target) == scenario)

                hm_1 = build_index_payload_hm(filename_target_1)
                hm_2 = build_index_payload_hm(filename_target_2)

                curr_corr_score += compute_hm_corr_score(
                    hm_1, hm_2, target_name_1, target_name_2, scenario)
            target_1_corr_matrix_v.append(curr_corr_score)
        corr_matrix_v_v.append(target_1_corr_matrix_v)

    if protocol == 'tcp':
        # we need to do the following because of the lwip reassembly error
        lwip_index = next((i for i, target_name in enumerate(target_name_v)
                           if target_name == "lwip_2.2.1"), None)
        print("build_custom_corr_matrix_hm: lwip_index: ", lwip_index)
        return np.array([[(e / TOTAL_TC_NUMBER_TCP_LWIP) *
                          100 if i == lwip_index or j == lwip_index else
                          (e / total_tc_nb) * 100
                          for j, e in enumerate(corr_matrix_v)]
                         for i, corr_matrix_v in enumerate(corr_matrix_v_v)])

    return np.array([[(e / total_tc_nb) * 100 for e in corr_matrix_v]
                     for corr_matrix_v in corr_matrix_v_v])


def build_data_df_from_corr_matrix(corr_matrix: np.array,
                                   target_name_v: list) -> pd.DataFrame:
    return pd.DataFrame(corr_matrix,
                        columns=target_name_v,
                        index=target_name_v)


def process(target_directory_path: str, protocol: str, json_payload_file: str,
            output_plot_file: str, output_csv_file: str,
            ip_scenarii_to_use: str):
    print("process: start")
    filename_v = glob.glob(
        f"{target_directory_path}/**/{protocol}/**/{protocol}*{json_payload_file}",
        recursive=True)
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
    #print("process: filename_v: ",filename_v)

    input_payload_file_nb, total_tc_nb, target_name_v, target_display_name_hm = get_scenarii_related_const_values(
        protocol, ip_scenarii_to_use)
    print("process: input_payload_file_nb: ", input_payload_file_nb)
    print("process: total_tc_nb: ", total_tc_nb)
    print("process: target_name_v: ", target_name_v)

    filename_to_keep_v, target_name_to_keep_v = remove_incomplete_target_from_lists(
        filename_v, target_name_v, input_payload_file_nb)
    target_display_name_to_keep_v = [
        target_display_name_hm[target] for target in target_name_to_keep_v
    ]

    corr_matrix = build_custom_corr_matrix_hm(filename_to_keep_v,
                                              target_name_to_keep_v,
                                              total_tc_nb, protocol)
    print("process: corr_matrix: ", corr_matrix)

    mask_os_only = [[
        False if j < len(target_name_to_keep_v) - 3
        and i < len(target_name_to_keep_v) - 3 else True
        for j in range(len(corr_v))
    ] for i, corr_v in enumerate(corr_matrix)]
    mask_os_nids = [[not mask for mask in mask_v] for mask_v in mask_os_only]

    corr_matrix_os_only = masked_array(corr_matrix, mask_os_only)
    corr_matrix_os_nids = masked_array(corr_matrix, mask_os_nids)

    cmap_os_only = LinearSegmentedColormap.from_list(
        'managua', ['white', 'blue', 'black'])
    cmap_os_nids = LinearSegmentedColormap.from_list(
        'managua', ['red', 'orange', 'xkcd:grass green'])

    cmap_os_nids.set_over(
        'xkcd:purple'
    )  # see other colors https://xkcd.com/color/rgb/ or https://matplotlib.org/stable/users/explain/colors/colors.html
    cmap_os_only.set_over('xkcd:purple')

    fig, ax = plt.subplots()

    pa = ax.imshow(corr_matrix_os_only,
                   cmap=cmap_os_only,
                   vmin=0,
                   vmax=99.99999999)
    pb = ax.imshow(corr_matrix_os_nids,
                   cmap=cmap_os_nids,
                   vmin=0,
                   vmax=99.99999999)

    divider = make_axes_locatable(ax)
    cax = divider.append_axes("right", size="5%", pad=0.3)
    cba = plt.colorbar(pa, cax=cax, ticks=None, extend='max', extendfrac=0.1)
    cba.set_ticks(ticks=[], labels=[])

    cax = divider.append_axes("right", size="5%", pad=0.1)
    cbb = plt.colorbar(pb, cax=cax, extend='max', extendfrac=0.1)

    cba.set_label('% of test cases similarly reassembled', labelpad=-25)

    # Show all ticks and label them with the respective list entries.
    ax.set_xticks(range(corr_matrix.shape[1]),
                  labels=target_display_name_to_keep_v,
                  rotation=-45,
                  ha="right",
                  rotation_mode="anchor")
    ax.set_yticks(range(corr_matrix.shape[0]),
                  labels=target_display_name_to_keep_v)

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
    plt.savefig(output_plot_file, bbox_inches='tight')

    data_df = build_data_df_from_corr_matrix(corr_matrix, target_name_v)
    print("process: data_df: ", data_df)
    data_df.to_csv(output_csv_file, mode='w')
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
    parser.add_argument("-jpf", "--json-payload-file", type=str, default="")
    parser.add_argument("-op", "--output-plot-file", type=str, default="")
    parser.add_argument("-oc", "--output-csv-file", type=str, default="")
    parser.add_argument(
        "--ip-scenarii-to-use",
        choices=["protocol_dependant_only", "protocol_agnostic_only", "any"],
        default="any",
        required=False)
    args = parser.parse_args()

    target_directory_path = args.target_directory_path
    protocol = args.protocol
    json_payload_file = args.json_payload_file
    output_plot_file = args.output_plot_file
    output_csv_file = args.output_csv_file
    ip_scenarii_to_use = args.ip_scenarii_to_use

    print(
        f"build_stack_scenario_groups_csv: target_directory_path: {target_directory_path}"
    )
    print(f"build_stack_scenario_groups_csv: protocol: {protocol}")
    print(
        f"build_stack_scenario_groups_csv: json_payload_file: {json_payload_file}"
    )
    print(
        f"build_stack_scenario_groups_csv: output_plot_file: {output_plot_file}"
    )
    print(
        f"build_stack_scenario_groups_csv: output_csv_file: {output_csv_file}")
    print(
        f"build_stack_scenario_groups_csv: ip_scenarii_to_use: {ip_scenarii_to_use}"
    )

    process(target_directory_path, protocol, json_payload_file,
            output_plot_file, output_csv_file, ip_scenarii_to_use)

    print("build_stack_scenario_groups_csv: end")


if __name__ == "__main__":
    main(sys.argv[1:])
