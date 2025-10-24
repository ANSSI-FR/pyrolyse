#!/usr/bin/python
import sys
import os
import json
import argparse
# import re
# from scapy.all import Ether,IP,ICMP,TCP,Raw,rdpcap,wrpcap,PacketList,bytes_hex
from scapy.all import IP, IPv6, ICMP, UDP, TCP, rdpcap, PacketList
# import networkx as nx

# import graph_utils as gu


def get_icmpv4_response(p_l):
    # We only keep the packet without fragmentation (MF=0 and frag_offset=0).
    p_l_nofrag = [p for p in p_l if (p[IP].flags & 1) == 0 and p[IP].frag == 0]

    # We only keep the ICMP packet.
    p_l_nofrag_icmp = [p for p in p_l_nofrag if p[IP].proto == 1]

    # We only keep the ICMP Echo Reply packet.
    p_l_nofrag_icmp_echo_reply = [
        p for p in p_l_nofrag_icmp if p[ICMP].type == 0
    ]

    return p_l_nofrag_icmp_echo_reply


def extract_data_icmpv4(index, pcap_path, nb_final_character_to_remove):
    print("\n\nextract_data_icmpv4: pcap_path :", pcap_path)
    # pkts = rdpcap('debian_jessie/output_icmp_pcap/toto.pcap')
    # pkts = rdpcap('debian_jessie/output_icmp_pcap/vm0_icmp_0.pcap')
    pkts = rdpcap(pcap_path)
    # p_l = PacketList([p for p in pkts])
    p_l = PacketList(list(pkts))

    # p_l_echo_replay = [ p[ICMP].type for p in p_l ]
    # print("p_l_echo_replay :",str(p_l_echo_replay))

    # last_packet = p_l[-1]
    # print("last_packet: ",last_packet)
    # print("last_packet[IP].proto: ",last_packet[IP].proto)

    p_l_nofrag_icmp_echo_reply = get_icmpv4_response(p_l)

    # We expect to find a single ICMP Echo Reply packet.
    if len(p_l_nofrag_icmp_echo_reply) == 1:
        icmp_echo_reply_packet = p_l_nofrag_icmp_echo_reply[0]
        ip_pdu = icmp_echo_reply_packet[IP]
        assert ip_pdu.proto == 1
        if (ip_pdu.flags & 1) == 1 or ip_pdu.frag > 0:
            print(
                "extract_data_icmpv4: last packet frag offset is greater than 0 but the MF flag is set"
            )
            is_echo_reply = False
        else:
            is_echo_reply = True
    else:
        is_echo_reply = False

    # ip_pdu = last_packet[IP]
    # assert ip_pdu.proto == 1

    # icmp_type = last_p[ICMP].type
    # # We check that we do not have fragmentation: MF is set or the fragment offset is positive.
    # if (ip_pdu.flags & 1) == 1 or ip_pdu.frag > 0:
    #     print("last packet frag offset is greater than 0 but the MF flag is set")
    #     is_echo_reply = False
    # else:
    #     icmp_pdu = last_packet[ICMP]
    #     if icmp_pdu.type != 0:
    #         is_echo_reply = False
    #     else:
    #         is_echo_reply = True

    print("extract_data_icmpv4: is_echo_reply: ", is_echo_reply)

    if not is_echo_reply:
        icmp_payload_s_wo_ending_chunk = ""
        # icmp_payload_hex_s = ""
    else:
        icmp_echo_reply_packet = p_l_nofrag_icmp_echo_reply[0]
        icmp_pdu = icmp_echo_reply_packet[ICMP]

        print("extract_data_icmpv4: icmp_pdu: ", icmp_pdu)
        print("extract_data_icmpv4: icmp_pdu.type: ", icmp_pdu.type)
        print("extract_data_icmpv4: icmp_pdu.id: ", icmp_pdu.id)

        if int(icmp_pdu.id) != int(index):
            print("extract_data_icmpv4: expected index (", index,
                  ") is not present in the packet: ", icmp_pdu.id)
            sys.exit(-2)

        # icmp_payload = icmp_pdu.payload

        ihl = ip_pdu.ihl * 4
        print("extract_data_icmpv4: ihl: ", ihl)
        total_len = ip_pdu.len
        print("extract_data_icmpv4: total_len: ", total_len)
        expected_ip_payload_len = total_len - ihl
        print("extract_data_icmpv4: expected_ip_payload_len: ",
              expected_ip_payload_len)

        ip_payload_len = len(bytes(icmp_pdu))
        extra_bytes_len = ip_payload_len - expected_ip_payload_len
        print("extract_data_icmpv4: extra_bytes_len :", extra_bytes_len)

        icmp_pdu_payload_b = bytes(icmp_pdu.payload)
        print("extract_data_icmpv4: icmp_pdu_payload_b: ", icmp_pdu_payload_b)

        if extra_bytes_len > 0:
            icmp_payload_b_wo_trailing_zeros = icmp_pdu_payload_b[:-(
                extra_bytes_len)]
        else:
            icmp_payload_b_wo_trailing_zeros = icmp_pdu_payload_b
        print("extract_data_icmpv4: icmp_payload_b_wo_trailing_zeros :",
              icmp_payload_b_wo_trailing_zeros)

        icmp_payload_s = icmp_payload_b_wo_trailing_zeros.decode('ascii')
        print("extract_data_icmpv4: icmp_payload_s :", icmp_payload_s)

        # icmp_payload_hex_b = bytes_hex(icmp_payload_b_wo_trailing_zeros)
        # print("icmp_payload_hex_b :",icmp_payload_hex_b)

        # icmp_payload_hex_s = icmp_payload_hex_b.decode('ascii')
        # print("icmp_payload_hex_s :",icmp_payload_hex_s)

        if nb_final_character_to_remove > 0:
            icmp_payload_s_wo_ending_chunk = icmp_payload_s[:-(
                nb_final_character_to_remove)]
        else:
            icmp_payload_s_wo_ending_chunk = icmp_payload_s
        print("extract_data_icmpv4: icmp_payload_s_wo_ending_chunk :",
              icmp_payload_s_wo_ending_chunk)

    return {
        "is_echo_reply": is_echo_reply,
        "payload": icmp_payload_s_wo_ending_chunk
    }


def get_icmpv6_response(p_l):
    p_l_ipv6 = [ p for p in p_l if IPv6 in p ]

    # We only keep the ICMP Echo Reply packet.
    p_l_nofrag_icmp_echo_reply = [
        p for p in p_l_ipv6 if p[IPv6].nh == 58 and p[IPv6].payload.type == 129
    ]

    return p_l_nofrag_icmp_echo_reply


def extract_data_icmpv6(index, pcap_path, nb_final_character_to_remove):
    print("\n\nextract_data_icmpv6: pcap_path :", pcap_path)
    pkts = rdpcap(pcap_path)
    p_l = PacketList(list(pkts))

    p_l_nofrag_icmp_echo_reply = get_icmpv6_response(p_l)

    # We expect to find a single ICMP Echo Reply packet.
    if len(p_l_nofrag_icmp_echo_reply) == 1:
        is_echo_reply = True
        icmp_echo_reply_packet = p_l_nofrag_icmp_echo_reply[0]
        # ip_pdu = icmp_echo_reply_packet[IPv6]
    else:
        is_echo_reply = False

    print("extract_data_icmpv6: is_echo_reply: ", is_echo_reply)

    if not is_echo_reply:
        icmp_payload_s_wo_ending_chunk = ""
    else:
        icmp_echo_reply_packet = p_l_nofrag_icmp_echo_reply[0]
        icmp_pdu = icmp_echo_reply_packet[IPv6].payload

        icmp_payload_s_wo_ending_chunk = bytes(icmp_pdu)[8:]
        icmp_payload_s_wo_ending_chunk = icmp_payload_s_wo_ending_chunk.decode(
            'ascii')

        if nb_final_character_to_remove > 0:
            icmp_payload_s_wo_ending_chunk = icmp_payload_s_wo_ending_chunk[:-(
                nb_final_character_to_remove)]
        # else:
        #    icmp_payload_s_wo_ending_chunk = icmp_payload_s_wo_ending_chunk

        if int(icmp_pdu.id) != int(index):
            print("extract_data_icmpv6: expected index (", index,
                  ") is not present in the packet: ", icmp_pdu.id)
            sys.exit(-2)
        print("extract_data_icmpv6: icmp_payload_s_wo_ending_chunk :",
              icmp_payload_s_wo_ending_chunk)

    return {
        "is_echo_reply": is_echo_reply,
        "payload": icmp_payload_s_wo_ending_chunk
    }


def get_udp_response(p_l):
    # We only keep the packet without fragmentation (MF=0 and frag_offset=0).
    p_l_nofrag = [p for p in p_l if (p[IP].flags & 1) == 0 and p[IP].frag == 0]

    # We only keep the UDP packet.
    p_l_nofrag_udp = [p for p in p_l_nofrag if p[IP].proto == 17]

    # We only keep the UDP Echo Reply packet.
    p_l_nofrag_udp_response_packet = [
        p for p in p_l_nofrag_udp if p[UDP].sport == 7
    ]

    return p_l_nofrag_udp_response_packet


def extract_data_udp(index, pcap_path, nb_final_character_to_remove):
    print("\n\nextract_data_udp: start")
    print("extract_data_udp: pcap_path :", pcap_path)
    pkts = rdpcap(pcap_path)
    # p_l = PacketList([p for p in pkts])
    p_l = PacketList(list(pkts))

    # p_l_echo_replay = [ p[ICMP].type for p in p_l ]
    # print("p_l_echo_replay :",str(p_l_echo_replay))

    # last_packet = p_l[-1]
    # print("last_packet: ",last_packet)
    # print("last_packet[IP].proto: ",last_packet[IP].proto)

    p_l_nofrag_udp_response_packet = get_udp_response(p_l)

    # We expect to find a single UDP response packet.
    if len(p_l_nofrag_udp_response_packet) == 1:
        udp_response_packet = p_l_nofrag_udp_response_packet[0]
        ip_pdu = udp_response_packet[IP]
        assert ip_pdu.proto == 17
        if (ip_pdu.flags & 1) == 1 or ip_pdu.frag > 0:
            print(
                "extract_data_udp: last packet frag offset is greater than 0 but the MF flag is set"
            )
            is_echo_reply = False
        else:
            is_echo_reply = True
    else:
        is_echo_reply = False

    # ip_pdu = last_packet[IP]
    # assert ip_pdu.proto == 1

    print("extract_data_udp: is_echo_reply: ", is_echo_reply)

    if not is_echo_reply:
        udp_payload_s_wo_ending_chunk = ""
        # udp_payload_hex_s = ""
    else:
        udp_response_packet = p_l_nofrag_udp_response_packet[0]
        udp_pdu = udp_response_packet[UDP]

        print("extract_data_udp: udp_pdu: ", udp_pdu)

        ihl = ip_pdu.ihl * 4
        print("extract_data_udp: ihl: ", ihl)
        total_len = ip_pdu.len
        print("extract_data_udp: total_len: ", total_len)
        expected_ip_payload_len = total_len - ihl
        print("extract_data_udp: expected_ip_payload_len: ",
              expected_ip_payload_len)

        ip_payload_len = len(bytes(udp_pdu))
        extra_bytes_len = ip_payload_len - expected_ip_payload_len
        print("extract_data_udp: extra_bytes_len :", extra_bytes_len)

        udp_pdu_payload_b = bytes(udp_pdu.payload)
        print("extract_data_udp: udp_pdu_payload_b: ", udp_pdu_payload_b)

        if extra_bytes_len > 0:
            udp_payload_b_wo_trailing_zeros = udp_pdu_payload_b[:-(
                extra_bytes_len)]
        else:
            udp_payload_b_wo_trailing_zeros = udp_pdu_payload_b
        print("extract_data_udp: udp_payload_b_wo_trailing_zeros :",
              udp_payload_b_wo_trailing_zeros)

        udp_payload_s = udp_payload_b_wo_trailing_zeros.decode('ascii')
        print("extract_data_udp: udp_payload_s :", udp_payload_s)

        # udp_payload_hex_b = bytes_hex(udp_payload_b_wo_trailing_zeros)
        # print("udp_payload_hex_b :",udp_payload_hex_b)

        # udp_payload_hex_s = udp_payload_hex_b.decode('ascii')
        # print("udp_payload_hex_s :",udp_payload_hex_s)

        if nb_final_character_to_remove > 0:
            udp_payload_s_wo_ending_chunk = udp_payload_s[:-(
                nb_final_character_to_remove)]
        else:
            udp_payload_s_wo_ending_chunk = udp_payload_s
        print("extract_data_udp: udp_payload_s_wo_ending_chunk :",
              udp_payload_s_wo_ending_chunk)

    print("\n\nextract_data_udp: end")

    return {
        "is_echo_reply": is_echo_reply,
        "payload": udp_payload_s_wo_ending_chunk
    }


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

def extract_data_tcp(path, filename):
    """Extract echoed TCP payload from pcap."""
    print("extract_data_tcp: start")
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
                print(f"extract_data_tcp: target_tcp_port: {target_tcp_port}")
                continue

            if packet[TCP].flags == "SA":
                target_tcb_iss = packet[TCP].seq
                print(f"extract_data_tcp: target_tcb_iss: {target_tcb_iss}")
                continue

            # Strip of padding bytes

            expected_payload_len = packet[IP].len - packet[IP].ihl * 4
            ip_payload_len = len(bytes(packet[TCP]))
            extra_bytes = ip_payload_len - expected_payload_len
            if extra_bytes != 0:
                payload_bytes = bytes(packet[TCP].payload)[:-(extra_bytes)]
                print(f"extract_data_tcp: extra_bytes: {extra_bytes}")
                print(f"extract_data_tcp: actual Payloads: {payload_bytes}")
            else:
                payload_bytes = bytes(packet[TCP].payload)
                print(f"extract_data_tcp: actual payload: {payload_bytes}")

            if len(payload_bytes) != 0 and target_tcp_port == -1:
                print(
                    "extract_data_tcp: observed echoed payload before potential SYNACK => failure"
                )
                sys.exit(-1)

            # Extract payload
            if len(payload_bytes
                   ) != 0 and packet[TCP].sport == target_tcp_port:
                print(
                    f"extract_data_tcp: destination port: {packet[TCP].dport}")
                payload_str = payload_bytes.decode("ascii")
                print(
                    f"extract_data_tcp: type(payload_str): {type(payload_str)}"
                )

                display_packet_data(num, packet, payload_str)

                tcp_seq_rel = packet[TCP].seq - target_tcb_iss - 1
                print(f"extract_data_tcp: tcp_seq_rel: {tcp_seq_rel}")
                if not payload_l and payload_str:
                    payload_l.append(payload_bytes)
                    tcb_seq_l.append(tcp_seq_rel)
                elif payload_bytes not in payload_l:
                    payload_l.append(payload_bytes)
                    tcb_seq_l.append(tcp_seq_rel)

    # Change to ASCII payload instead of hex
    temp_payload = [x.decode('ascii') for x in payload_l]
    tempay = merge_load(temp_payload, tcb_seq_l)
    # test_index = ''.join(re.findall(r'\d+', filename))

    print("extract_data_tcp: end")

    return {"is_echo_reply": tempay != "", "payload": tempay}


# TODO: add TCP extraction


def extract_index(pcap_path):
    basename = os.path.basename(pcap_path)
    basename_wo_ext = os.path.splitext(basename)[0]
    index = basename_wo_ext.split("_")[-1]
    return index


def get_response_packet(pcap_path):
    # print("get_response_packet: start")

    # print(f"get_response_packet: pcap_path: {pcap_path}")

    pkts = rdpcap(pcap_path)
    p_l = PacketList(list(pkts))

    p_l_nofrag_icmpv4_echo_reply = get_icmpv4_response(p_l)
    p_l_nofrag_icmpv6_echo_reply = get_icmpv6_response(p_l)
    p_l_nofrag_udp_response = get_udp_response(p_l)
    p_l_nofrag_udp_response_packet = get_udp_response(p_l)

    # print("get_response_packet: end")

    return p_l_nofrag_icmpv4_echo_reply + p_l_nofrag_icmpv6_echo_reply + p_l_nofrag_udp_response + p_l_nofrag_udp_response_packet

    
def process(pcap_directory, ip_version, json_output_path,
            nb_final_character_to_remove):
    print("process: start")
    filename_l = os.listdir(pcap_directory)

    path_l = [
        os.path.join(pcap_directory, filename) for filename in filename_l
    ]

    p_l_l_response = [ get_response_packet(path) for path in path_l]
    p_l_response = [item for sublist in p_l_l_response for item in sublist]
    is_ipv4_icmp = any(IP in p and ICMP in p for p in p_l_response)
    is_ipv6_icmp = any(IPv6 in p and p[IPv6].nh == 58 and p[IPv6].payload.type == 129 for p in p_l_response)
    is_ipv4_udp = any(IP in p and UDP in p for p in p_l_response)
    is_ipv6_udp = any(IPv6 in p and UDP in p for p in p_l_response)
    is_tcp = any(TCP in p for p in p_l_response)

    print(f"process: is_ipv4_icmp: {is_ipv4_icmp} ; is_ipv6_icmp: {is_ipv6_icmp}")
    print(f"process: is_ipv4_udp: {is_ipv4_udp} ; is_ipv6_udp: {is_ipv6_udp}")
    print(f"process: is_tcp: {is_ipv4_icmp}")

    if is_ipv4_icmp:
        index_payload_d = {
            extract_index(pcap_path):
            extract_data_icmpv4(extract_index(pcap_path), pcap_path,
                                nb_final_character_to_remove)
            for pcap_path in path_l
        }
    elif is_ipv6_icmp:
        index_payload_d = {
            extract_index(pcap_path):
            extract_data_icmpv6(extract_index(pcap_path), pcap_path,
                                nb_final_character_to_remove)
            for pcap_path in path_l
        }
    elif is_ipv4_udp:
        index_payload_d = {
            extract_index(pcap_path):
            extract_data_udp(extract_index(pcap_path), pcap_path,
                                nb_final_character_to_remove)
            for pcap_path in path_l
        }
    elif is_tcp:
        index_payload_d = {
            extract_index(pcap_path):
            extract_data_tcp(extract_index(pcap_path), pcap_path,
                             nb_final_character_to_remove)
            for pcap_path in path_l
        }
    else:
        print("Could not find any payload")
        sys.exit(-2)

    data_d = {"hm": index_payload_d}

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
    parser.add_argument("-i", "--input-pcap-directory", type=str, default="")
    parser.add_argument("-p", "--protocol", type=str, default="4")
    parser.add_argument("-j", "--json-path", type=str, default="")
    parser.add_argument("-r", "--nb-character-to-remove", type=int, default=0)
    args = parser.parse_args()

    input_pcap_directory = args.input_pcap_directory
    protocol = args.protocol
    json_path = args.json_path
    nb_final_character_to_remove = args.nb_character_to_remove

    print(
        f"extract_os_icmp_payload_from_directory_to_json: input_pcap_directory: {input_pcap_directory}"
    )
    print(
        f"extract_os_icmp_payload_from_directory_to_json: protocol: {protocol}"
    )
    print(
        f"extract_os_icmp_payload_from_directory_to_json: json_path: {json_path}"
    )

    process(input_pcap_directory, protocol, json_path,
            nb_final_character_to_remove)

    print("extract_os_icmp_payload_from_directory_to_json: end")


if __name__ == "__main__":
    main(sys.argv[1:])
