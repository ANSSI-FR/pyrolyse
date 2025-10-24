#!/usr/bin/python3
"""Extract TCP payload for all PCAP in directory."""

import sys
import os
import re
import argparse

import json
import matplotlib.pyplot as plt
from scapy.all import IP, TCP, rdpcap


def diff_unique(v_0, v_1):
    """Diff unique (complete)."""
    # print(f"v_0: {v_0} = v_1:{v_1}")
    if v_0 < v_1:
        diff_value = -1
    elif v_0 == v_1:
        diff_value = 0
    else:
        diff_value = 1
    return diff_value


def compare_list(l_0: list, l_1: list):
    """Compare two list."""
    new_dict = {t[0]: t[1] for t in l_0}
    old_dict = {t[0]: t[1] for t in l_1}
    common = list(set(new_dict.keys()) & set(old_dict.keys()))
    return [diff_unique(new_dict[key], old_dict[key]) for key in common]


def merge_load(hexload_s_l, test_index_l):
    """Merge payload."""
    print("merge_load: start")
    print(f"merge_load: type(hexload_s_l): {type(hexload_s_l)}")
    print(f"merge_load: type(test_index_l): {type(test_index_l)}")
    collected_payload = []
    if len(hexload_s_l) == 1:
        merged_string = ''.join(hexload_s_l)
    else:
        list_pay = []

        for j, hexload_s in enumerate(hexload_s_l):
            # hexload_s = hexload_s_l[j]
            print(f"merge_load: type(hexload_s): {type(hexload_s)}")

            newlist = [hexload_s[i:i + 8] for i in range(0, len(hexload_s), 8)]

            sequ = list(
                range(test_index_l[j], test_index_l[j] + len(newlist) * 8, 8))
            merges = list(zip(sequ, newlist))
            print("merge_load: merges: ", merges)
            if not collected_payload:
                collected_payload = merges
            else:

                inc_dec_list = compare_list(merges, collected_payload)
                if not inc_dec_list:
                    collected_payload = [*collected_payload, *merges]
                elif all(v != 0 for v in inc_dec_list):
                    print(f"merge_load: inc_dec_list: {inc_dec_list}")
                    print("merge_load: conflict in overlapping bytes")
                else:
                    print("merge_load: no conflict in overlapping bytes")
                    tempy = [*collected_payload, *merges]
                    tempy2 = list(t for t in (set(tuple(i) for i in tempy)))
                    collected_payload = sorted(tempy2, key=lambda tup: tup[0])
                    print("merge_load: collected_payload", collected_payload)
                    continue

        for item in collected_payload:
            list_pay.append(item[1])

        print("merge_load: end")
        merged_string = ''.join(list_pay)

    return merged_string


def export_to_json(payload_s_l, index_l, path: str):
    """Export to a JSON file."""
    # concat_payload = [''.join(i) for i in hexpayload]
    number_l = [int(x) for x in index_l]
    comb_l = list(zip(index_l, payload_s_l))
    comb_l_sorted = list(x for _, x in sorted(zip(number_l, comb_l)))

    data_d = {}
    data_d['hm'] = {}
    for _, comb in enumerate(comb_l_sorted):
        payload_key = str(comb[0])
        payload_data_d = {}
        payload_data_d['is_echo_reply'] = comb[1] != ""
        payload_data_d['payload'] = comb[1]
        data_d['hm'][payload_key] = payload_data_d

    with open(path, "w", encoding="UTF8") as write_file:
        json.dump(data_d, write_file, indent=2)


def build_graph(inx, bytepayload, seq, path):
    """Create a graph for packet sequence."""
    print("build_graph: start")

    payload = [x.decode('ascii') for x in bytepayload]
    multiplied_offset = seq
    print(f"build_graph: multiplied_offset: {multiplied_offset}")
    temp_pos = list(range(len(seq)))
    print(f"build_graph: temp pos: {temp_pos}")

    # example data
    x_data = multiplied_offset
    y_data = temp_pos
    error = [len(i) for i in payload]

    if len(seq) == 4:
        xerror = [(0, 0, 0, 0), error]
    elif len(seq) == 3:
        xerror = [(0, 0, 0), error]
        print(xerror)
    elif len(seq) == 2:
        xerror = [(0, 0), error]
        print(xerror)
    else:
        xerror = [(0, ), error]
        print(xerror)

    fig = plt.figure()
    plt.errorbar(x_data, y_data, xerr=xerror, fmt=',')

    plt.xticks([0, 8, 16, 24, 32, 40, 48])
    plt.yticks([-1, 0, 1, 2, 3],
               ('Start', 'First', 'Second', 'Third', 'Finish'))
    plt.xlabel("Sequence Number")
    for j in range(len(seq)):
        plt.text(x_data[j], y_data[j] + 0.1, payload[j])
    graph_name = path + "echo_" + inx[0] + ".png"
    print(graph_name)
    plt.savefig(graph_name)
    plt.close(fig)

    print("build_graph: end")


def display_packet_data(num, packet, payload_str):
    """Display packet data."""
    ip_layer = packet.getlayer(IP)
    time = packet[TCP].time
    src = ip_layer.src
    dst = ip_layer.dst
    src_port = packet[TCP].sport
    dst_port = packet[TCP].dport
    flag = packet[TCP].flags
    tcp_seq = packet[TCP].seq
    tcp_ack = packet[TCP].ack
    length = len(payload_str)
    payload = payload_str if payload_str else 'NULL'
    print(
        f"{time} {num}, {src}:{src_port} -> {dst}:{dst_port}, {flag} S:{tcp_seq} A:{tcp_ack}, {payload} ({length})"
    )


def extract_payload(path, filename):
    """Extract echoed TCP payload from pcap."""
    print("extract_payload: start")
    num = 0
    tcb_seq_l = []
    payload_l = []
    target_tcb_iss = -1
    target_tcp_port = -1
    scapy_cap = rdpcap(path + "/" + filename)
    for packet in scapy_cap:
        if TCP in packet:
            num += 1
            if packet[TCP].flags == "S":
                target_tcp_port = packet[TCP].dport
                print(f"extract_payload: target_tcp_port: {target_tcp_port}")
                continue

            if packet[TCP].flags == "SA":
                target_tcb_iss = packet[TCP].seq
                print(f"extract_payload: target_tcb_iss: {target_tcb_iss}")
                continue

            # Strip of padding bytes

            expected_payload_len = packet[IP].len - packet[IP].ihl * 4
            ip_payload_len = len(bytes(packet[TCP]))
            extra_bytes = ip_payload_len - expected_payload_len
            if extra_bytes != 0:
                payload_bytes = bytes(packet[TCP].payload)[:-(extra_bytes)]
                print(f"extract_payload: extra_bytes: {extra_bytes}")
                print(f"extract_payload: actual Payloads: {payload_bytes}")
            else:
                payload_bytes = bytes(packet[TCP].payload)
                print(f"extract_payload: actual payload: {payload_bytes}")

            if len(payload_bytes) != 0 and target_tcp_port == -1:
                print(
                    "extract_payload: observed echoed payload before potential SYNACK => failure"
                )
                sys.exit(-1)

            # Extract payload
            if len(payload_bytes
                   ) != 0 and packet[TCP].sport == target_tcp_port:
                print(
                    f"extract_payload: destination port: {packet[TCP].dport}")
                payload_str = payload_bytes.decode("ascii")
                print(
                    f"extract_payload: type(payload_str): {type(payload_str)}")

                display_packet_data(num, packet, payload_str)

                tcp_seq_rel = packet[TCP].seq - target_tcb_iss - 1
                print(f"extract_payload: tcp_seq_rel: {tcp_seq_rel}")
                if not payload_l and payload_str:
                    payload_l.append(payload_bytes)
                    tcb_seq_l.append(tcp_seq_rel)
                elif payload_bytes not in payload_l:
                    payload_l.append(payload_bytes)
                    tcb_seq_l.append(tcp_seq_rel)
    print("extract_payload: end")
    return tcb_seq_l, payload_l


def main(argv):
    """main"""
    # pylint: disable=unused-argument
    parser = argparse.ArgumentParser()
    parser.add_argument("-p", "--pcap-directory", type=str, default="")
    parser.add_argument("-j", "--json-path", type=str, default="")
    args = parser.parse_args()

    pcap_directory = args.pcap_directory
    json_path = args.json_path

    print(f"extract_os_tcp_payload: pcap_directory: {pcap_directory}")
    print(f"extract_os_tcp_payload: json_path: {json_path}")

    directory = os.fsencode(pcap_directory)
    test_index_l = []
    hexload_s_l = []
    numfile = 0

    for file in os.listdir(directory):

        filename = os.fsdecode(file)
        print(f"extract_os_tcp_payload: filename: {filename}")
        if filename.endswith(".pcap"):
            numfile += 1
            tcb_seq_l, byte_payload = extract_payload(pcap_directory, filename)
            print("index: ", re.findall(r'\d+', filename))

            # Change to ASCII payload instead of hex
            temp_payload = [x.decode('ascii') for x in byte_payload]
            tempay = merge_load(temp_payload, tcb_seq_l)
            hexload_s_l.append(tempay.replace("0", ""))
            test_index_l.append(''.join(re.findall(r'\d+', filename)))

            # build_graph(re.findall(r'\d+',filename), byte_payload, tcb_seq_l, path)

    print(f"extract_os_tcp_payload: test_index_l: {test_index_l}")
    export_to_json(hexload_s_l, test_index_l, json_path)


if __name__ == "__main__":
    main(sys.argv[1:])
