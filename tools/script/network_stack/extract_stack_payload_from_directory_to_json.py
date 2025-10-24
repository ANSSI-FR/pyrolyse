#!/usr/bin/python
import sys
import os
import json
import argparse
from scapy.all import IP, IPv6, ICMP, UDP, TCP, rdpcap, PacketList

# TODO adapt TCP data extraction to new patterns
# TODO check if received enough data for IP (e.g., pb with OpenBSD 7.4 peosfef, test 361) ??

def get_icmpv4_response(p_l: list):
    """
    Extract ICMP echo request response from packet list.
    :param p_l: Packet list
    :return: Packet list
    """
    p_l_ipv4 = [p for p in p_l if IP in p]

    # We only keep the packet without fragmentation (MF=0 and frag_offset=0).
    p_l_nofrag = [
        p for p in p_l_ipv4 if (p[IP].flags & 1) == 0 and p[IP].frag == 0
    ]

    # We only keep the ICMP packet.
    p_l_nofrag_icmp = [p for p in p_l_nofrag if p[IP].proto == 1]

    # We only keep the ICMP Echo Reply packet.
    p_l_nofrag_icmp_echo_reply = [
        p for p in p_l_nofrag_icmp if p[ICMP].type == 0
    ]

    return p_l_nofrag_icmp_echo_reply


def extract_data_icmpv4(index: int, pcap_path: str,
                        nb_finishing_character_to_remove: int):
    """

    :param index: Test index.
    :param pcap_path: Path of the PCAP file.
    :param nb_finishing_character_to_remove: Number of character to remove at the end of the payload (peoe, peoes, peose scenarii).
    :return: String.
    """
    print("\n\nextract_data_icmpv4: pcap_path :", pcap_path)
    pkts = rdpcap(pcap_path)
    p_l = list(PacketList(list(pkts)))

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
    elif len(p_l_nofrag_icmp_echo_reply) > 1:
        is_echo_reply = True
    else:
        is_echo_reply = False

    number = len(p_l_nofrag_icmp_echo_reply)

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

    # TODO: improve logic to avoid definition of ip_pdu in previous if

    if not is_echo_reply or number > 1:
        icmp_payload_s_wo_ending_chunk = ""
        # icmp_payload_hex_s = ""
    else:
        icmp_echo_reply_packet = p_l_nofrag_icmp_echo_reply[0]
        icmp_pdu = icmp_echo_reply_packet[ICMP]

        print("extract_data_icmpv4: icmp_pdu: ", icmp_pdu)
        print("extract_data_icmpv4: icmp_pdu.type: ", icmp_pdu.type)
        print("extract_data_icmpv4: icmp_pdu.id: ", icmp_pdu.id)

        # TODO: change check against modulo of icmp_pdu.id
        # TODO: check why we need so much modulo
        if int(icmp_pdu.id % 1000) != int(index % 1000):
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

        #icmp_payload_s = icmp_payload_b_wo_trailing_zeros.decode('ascii')
        icmp_payload_s = icmp_payload_b_wo_trailing_zeros.decode('UTF8','replace')
        print("extract_data_icmpv4: icmp_payload_s :", icmp_payload_s)

        # icmp_payload_hex_b = bytes_hex(icmp_payload_b_wo_trailing_zeros)
        # print("icmp_payload_hex_b :",icmp_payload_hex_b)

        # icmp_payload_hex_s = icmp_payload_hex_b.decode('ascii')
        # print("icmp_payload_hex_s :",icmp_payload_hex_s)

        if nb_finishing_character_to_remove > 0:
            icmp_payload_s_wo_ending_chunk = icmp_payload_s[:-(
                nb_finishing_character_to_remove)]
        else:
            icmp_payload_s_wo_ending_chunk = icmp_payload_s
        print("extract_data_icmpv4: icmp_payload_s_wo_ending_chunk :",
              icmp_payload_s_wo_ending_chunk)
    
    #icmp_payload_s_wo_starting_strategy_bonus = icmp_payload_s_wo_ending_chunk.replace(0 as bytes, "")
    #print(
    #    f"extract_data_tcp: icmp_payload_s_wo_starting_strategy_bonus: {icmp_payload_s_wo_starting_strategy_bonus}"
    #)

    return {
        "is_echo_reply": is_echo_reply,
        "number": number,
        "payload": icmp_payload_s_wo_ending_chunk
    }


def get_icmpv6_response(p_l):
    p_l_ipv6 = [p for p in p_l if IPv6 in p]

    # We only keep the ICMP Echo Reply packet.
    p_l_nofrag_icmp_echo_reply = [
        p for p in p_l_ipv6 if p[IPv6].nh == 58 and p[IPv6].payload.type == 129
    ]

    return p_l_nofrag_icmp_echo_reply


def extract_data_icmpv6(index, pcap_path, nb_finishing_character_to_remove):
    print("\n\nextract_data_icmpv6: pcap_path :", pcap_path)
    pkts = rdpcap(pcap_path)
    p_l = PacketList(list(pkts))

    p_l_nofrag_icmp_echo_reply = get_icmpv6_response(p_l)

    # We expect to find a single ICMP Echo Reply packet.
    if len(p_l_nofrag_icmp_echo_reply) == 1:
        is_echo_reply = True
        icmp_echo_reply_packet = p_l_nofrag_icmp_echo_reply[0]
        # ip_pdu = icmp_echo_reply_packet[IPv6]
    elif len(p_l_nofrag_icmp_echo_reply) > 1:
        is_echo_reply = True
    else:
        is_echo_reply = False

    print("extract_data_icmpv6: is_echo_reply: ", is_echo_reply)
    number = len(p_l_nofrag_icmp_echo_reply)

    if not is_echo_reply or number > 1:
        icmp_payload_s_wo_ending_chunk = ""
    else:
        icmp_echo_reply_packet = p_l_nofrag_icmp_echo_reply[0]
        icmp_pdu = icmp_echo_reply_packet[IPv6].payload

        icmp_payload_s_wo_ending_chunk = bytes(icmp_pdu)[8:]
        icmp_payload_s_wo_ending_chunk = icmp_payload_s_wo_ending_chunk.decode('UTF8','replace')

        if nb_finishing_character_to_remove > 0:
            icmp_payload_s_wo_ending_chunk = icmp_payload_s_wo_ending_chunk[:-(
                nb_finishing_character_to_remove)]
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
        "number": number,
        "payload": icmp_payload_s_wo_ending_chunk
    }


def get_ipv4_udp_response(p_l):
    p_l_ipv4 = [p for p in p_l if IP in p]

    # We only keep the packet without fragmentation (MF=0 and frag_offset=0).
    p_l_ipv4_nofrag = [
        p for p in p_l_ipv4 if (p[IP].flags & 1) == 0 and p[IP].frag == 0
    ]

    # We only keep the UDP packet.
    p_l_ipv4_nofrag_udp = [p for p in p_l_ipv4_nofrag if p[IP].proto == 17]

    # We only keep the UDP Echo Reply packet.
    p_l_ipv4_nofrag_udp_response_packet = [
        p for p in p_l_ipv4_nofrag_udp if p[UDP].sport == 7
    ]

    return p_l_ipv4_nofrag_udp_response_packet


def extract_data_ipv4_udp(pcap_path, nb_finishing_character_to_remove):
    print("\n\nextract_data_ipv4_udp: start")
    print("extract_data_ipv4_udp: pcap_path :", pcap_path)
    pkts = rdpcap(pcap_path)
    # p_l = PacketList([p for p in pkts])
    p_l = PacketList(list(pkts))

    # p_l_echo_replay = [ p[ICMP].type for p in p_l ]
    # print("p_l_echo_replay :",str(p_l_echo_replay))

    # last_packet = p_l[-1]
    # print("last_packet: ",last_packet)
    # print("last_packet[IP].proto: ",last_packet[IP].proto)

    p_l_ipv4_nofrag_udp_response_packet = get_ipv4_udp_response(p_l)

    # We expect to find a single UDP response packet.
    if len(p_l_ipv4_nofrag_udp_response_packet) == 1:
        udp_response_packet = p_l_ipv4_nofrag_udp_response_packet[0]
        ip_pdu = udp_response_packet[IP]
        assert ip_pdu.proto == 17
        if (ip_pdu.flags & 1) == 1 or ip_pdu.frag > 0:
            print(
                "extract_data_ipv4_udp: last packet frag offset is greater than 0 but the MF flag is set"
            )
            is_echo_reply = False
        else:
            is_echo_reply = True
    elif len(p_l_ipv4_nofrag_udp_response_packet) > 1:
        is_echo_reply = True
    else:
        is_echo_reply = False

    # ip_pdu = last_packet[IP]
    # assert ip_pdu.proto == 1

    print("extract_data_ipv4_udp: is_echo_reply: ", is_echo_reply)
    number = len(p_l_ipv4_nofrag_udp_response_packet)

    if not is_echo_reply or number > 1:
        udp_payload_s_wo_ending_chunk = ""
        # udp_payload_hex_s = ""
    else:
        udp_response_packet = p_l_ipv4_nofrag_udp_response_packet[0]
        udp_pdu = udp_response_packet[UDP]

        print("extract_data_ipv4_udp: udp_pdu: ", udp_pdu)

        ihl = ip_pdu.ihl * 4
        print("extract_data_ipv4_udp: ihl: ", ihl)
        total_len = ip_pdu.len
        print("extract_data_ipv4_udp: total_len: ", total_len)
        expected_ip_payload_len = total_len - ihl
        print("extract_data_ipv4_udp: expected_ip_payload_len: ",
              expected_ip_payload_len)

        ip_payload_len = len(bytes(udp_pdu))
        extra_bytes_len = ip_payload_len - expected_ip_payload_len
        print("extract_data_ipv4_udp: extra_bytes_len :", extra_bytes_len)

        udp_pdu_payload_b = bytes(udp_pdu.payload)
        print("extract_data_ipv4_udp: udp_pdu_payload_b: ", udp_pdu_payload_b)

        if extra_bytes_len > 0:
            udp_payload_b_wo_trailing_zeros = udp_pdu_payload_b[:-(
                extra_bytes_len)]
        else:
            udp_payload_b_wo_trailing_zeros = udp_pdu_payload_b
        print("extract_data_ipv4_udp: udp_payload_b_wo_trailing_zeros :",
              udp_payload_b_wo_trailing_zeros)

        #udp_payload_s = udp_payload_b_wo_trailing_zeros.decode('ascii')
        udp_payload_s = udp_payload_b_wo_trailing_zeros.decode('UTF8','replace')
        print("extract_data_ipv4_udp: udp_payload_s :", udp_payload_s)

        # udp_payload_hex_b = bytes_hex(udp_payload_b_wo_trailing_zeros)
        # print("udp_payload_hex_b :",udp_payload_hex_b)

        # udp_payload_hex_s = udp_payload_hex_b.decode('ascii')
        # print("udp_payload_hex_s :",udp_payload_hex_s)

        if nb_finishing_character_to_remove > 0:
            udp_payload_s_wo_ending_chunk = udp_payload_s[:-(
                nb_finishing_character_to_remove)]
        else:
            udp_payload_s_wo_ending_chunk = udp_payload_s
        print("extract_data_ipv4_udp: udp_payload_s_wo_ending_chunk :",
              udp_payload_s_wo_ending_chunk)

    print("extract_data_ipv4_udp: end")

    return {
        "is_echo_reply": is_echo_reply,
        "number": number,
        "payload": udp_payload_s_wo_ending_chunk
    }


def get_ipv4_tcp_response(p_l):
    print("get_ipv4_tcp_response: start")

    p_l_tcp = [p for p in p_l if TCP in p]

    print(f"get_ipv4_tcp_response: nb TCP: {len(p_l_tcp)}")

    p_l_tcp_response_packet = [
        p for p in p_l_tcp
        if p[TCP].sport == 7 and len(bytes(p[TCP].payload)) > 0
    ]
    print(
        f"get_ipv4_tcp_response: nb TCP with response: {len(p_l_tcp_response_packet)}"
    )

    print("get_ipv4_tcp_response: end")

    return p_l_tcp_response_packet


def get_ipv6_udp_response(p_l):
    p_l_ipv6 = [p for p in p_l if IPv6 in p]

    # We only keep the packet without fragmentation (proto != 44).
    p_l_ipv6_nofrag = [p for p in p_l_ipv6 if p[IPv6].nh != 44]

    # We only keep the UDP packet.
    p_l_ipv6_nofrag_udp = [p for p in p_l_ipv6_nofrag if p[IPv6].nh == 17]

    # We only keep the UDP Echo Reply packet.
    p_l_ipv6_nofrag_udp_response_packet = [
        p for p in p_l_ipv6_nofrag_udp if p[UDP].sport == 7
    ]

    return p_l_ipv6_nofrag_udp_response_packet


def extract_data_ipv6_udp(pcap_path, nb_finishing_character_to_remove):
    print("\n\nextract_data_ipv6_udp: start")
    print("extract_data_ipv6_udp: pcap_path :", pcap_path)
    pkts = rdpcap(pcap_path)
    # p_l = PacketList([p for p in pkts])
    p_l = PacketList(list(pkts))

    # p_l_echo_replay = [ p[ICMP].type for p in p_l ]
    # print("p_l_echo_replay :",str(p_l_echo_replay))

    # last_packet = p_l[-1]
    # print("last_packet: ",last_packet)
    # print("last_packet[IP].proto: ",last_packet[IP].proto)

    p_l_ipv6_nofrag_udp_response_packet = get_ipv6_udp_response(p_l)

    # We expect to find a single UDP response packet.
    if len(p_l_ipv6_nofrag_udp_response_packet) == 1:
        udp_response_packet = p_l_ipv6_nofrag_udp_response_packet[0]
        ip_pdu = udp_response_packet[IPv6]
        if ip_pdu.nh == 44:
            print("extract_data_ipv6_udp: next header is Fragment")
            is_echo_reply = False
        else:
            assert ip_pdu.nh == 17
            is_echo_reply = True
    elif len(p_l_ipv6_nofrag_udp_response_packet) > 1:
        is_echo_reply = True
    else:
        is_echo_reply = False

    # ip_pdu = last_packet[IP]
    # assert ip_pdu.proto == 1

    print("extract_data_ipv6_udp: is_echo_reply: ", is_echo_reply)
    number = len(p_l_ipv6_nofrag_udp_response_packet)

    if not is_echo_reply or number > 1:
        udp_payload_s_wo_ending_chunk = ""
        # udp_payload_hex_s = ""
    else:
        udp_response_packet = p_l_ipv6_nofrag_udp_response_packet[0]
        udp_pdu = udp_response_packet[UDP]

        print("extract_data_ipv6_udp: udp_pdu: ", udp_pdu)

        # ihl = ip_pdu.ihl * 4
        # print("extract_data_ipv6_udp: ihl: ", ihl)
        # total_len = ip_pdu.plen
        # print("extract_data_ipv6_udp: total_len: ", total_len)
        expected_ip_payload_len = ip_pdu.plen
        print("extract_data_ipv6_udp: expected_ip_payload_len: ",
              expected_ip_payload_len)

        ip_payload_len = len(bytes(udp_pdu))
        extra_bytes_len = ip_payload_len - expected_ip_payload_len
        print("extract_data_ipv6_udp: extra_bytes_len :", extra_bytes_len)

        udp_pdu_payload_b = bytes(udp_pdu.payload)
        print("extract_data_ipv6_udp: udp_pdu_payload_b: ", udp_pdu_payload_b)

        if extra_bytes_len > 0:
            udp_payload_b_wo_trailing_zeros = udp_pdu_payload_b[:-(
                extra_bytes_len)]
        else:
            udp_payload_b_wo_trailing_zeros = udp_pdu_payload_b
        print("extract_data_ipv6_udp: udp_payload_b_wo_trailing_zeros :",
              udp_payload_b_wo_trailing_zeros)

        #udp_payload_s = udp_payload_b_wo_trailing_zeros.decode('ascii')
        udp_payload_s = udp_payload_b_wo_trailing_zeros.decode('UTF8','replace')
        print("extract_data_ipv6_udp: udp_payload_s :", udp_payload_s)

        # udp_payload_hex_b = bytes_hex(udp_payload_b_wo_trailing_zeros)
        # print("udp_payload_hex_b :",udp_payload_hex_b)

        # udp_payload_hex_s = udp_payload_hex_b.decode('ascii')
        # print("udp_payload_hex_s :",udp_payload_hex_s)

        if nb_finishing_character_to_remove > 0:
            udp_payload_s_wo_ending_chunk = udp_payload_s[:-(
                nb_finishing_character_to_remove)]
        else:
            udp_payload_s_wo_ending_chunk = udp_payload_s
        print("extract_data_ipv6_udp: udp_payload_s_wo_ending_chunk :",
              udp_payload_s_wo_ending_chunk)

    print("extract_data_ipv6_udp: end")

    return {
        "is_echo_reply": is_echo_reply,
        "number": number,
        "payload": udp_payload_s_wo_ending_chunk
    }


def display_packet_data(num, packet, payload_str):
    """Display packet data."""
    ip_layer = packet.getlayer(IP)
    #ip_layer = packet.getlayer(IPv6)
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
                    exit(-1)
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


## def extract_data_tcp(test_index: int, path: str):
#def extract_data_tcp(path: str):
#    """Extract echoed TCP payload from pcap."""
#    print("\n\nextract_data_tcp: start")
#
#    # print(f"extract_data_tcp: test_index: {test_index}")
#    num = 0
#    tcb_seq_l = []
#    payload_l = []
#    target_tcb_iss = -1
#    target_tcp_port = -1
#    scapy_cap = rdpcap(path)
#    for packet in scapy_cap:
#        if TCP in packet:
#            num += 1
#            if packet[TCP].flags == "S":
#                target_tcp_port = packet[TCP].dport
#                print(f"extract_data_tcp: target_tcp_port: {target_tcp_port}")
#                continue
#
#            if packet[TCP].flags == "SA":
#                target_tcb_iss = packet[TCP].seq
#                print(f"extract_data_tcp: target_tcb_iss: {target_tcb_iss}")
#                continue
#
#            # Strip of padding bytes
#
#            expected_payload_len = packet[IP].len - packet[IP].ihl * 4
#            ip_payload_len = len(bytes(packet[TCP]))
#            extra_bytes = ip_payload_len - expected_payload_len
#            if extra_bytes != 0:
#                payload_bytes = bytes(packet[TCP].payload)[:-(extra_bytes)]
#                print(f"extract_data_tcp: extra_bytes: {extra_bytes}")
#                print(f"extract_data_tcp: payload_bytes: {payload_bytes}")
#            else:
#                payload_bytes = bytes(packet[TCP].payload)
#                print(f"extract_data_tcp: payload_bytes: {payload_bytes}")
#
#            if len(payload_bytes) != 0 and target_tcp_port == -1:
#                print(
#                    "extract_data_tcp: observed echoed payload before potential SYNACK => failure"
#                )
#                sys.exit(-1)
#
#            # Extract payload
#            if len(payload_bytes
#                   ) != 0 and packet[TCP].sport == target_tcp_port:
#                print(
#                    f"extract_data_tcp: destination port: {packet[TCP].dport}")
#                payload_str = payload_bytes.decode('UTF8','replace')
#                print(
#                    f"extract_data_tcp: type(payload_str): {type(payload_str)}"
#                )
#
#                display_packet_data(num, packet, payload_str)
#
#                tcp_seq_rel = packet[TCP].seq - target_tcb_iss - 1
#                print(f"extract_data_tcp: tcp_seq_rel: {tcp_seq_rel}")
#                if not payload_l and payload_str:
#                    payload_l.append(payload_bytes)
#                    tcb_seq_l.append(tcp_seq_rel)
#                elif payload_bytes not in payload_l:
#                    payload_l.append(payload_bytes)
#                    tcb_seq_l.append(tcp_seq_rel)
#    
#    # TODO check if bug wuth implementation sending partial retransmissions (ie )
#
#    # Change to ASCII payload instead of hex
#    #temp_payload = [x.decode('ascii') for x in payload_l]
#    temp_payload = [x.decode('UTF8','replace') for x in payload_l]
#    merged_payload = merge_load(temp_payload, tcb_seq_l)
#    print(f"extract_data_tcp: merged_payload: {merged_payload}")
#
#    payload_without_peos_bonus = merged_payload.replace("0", "")
#    print(
#        f"extract_data_tcp: payload_without_peos_bonus: {payload_without_peos_bonus}"
#    )
#    # test_index = ''.join(re.findall(r'\d+', filename))
#
#    print("extract_data_tcp: end")
#
#    return {
#        "is_echo_reply": payload_without_peos_bonus != "",
#        "number": int(payload_without_peos_bonus != ""),
#        "payload": payload_without_peos_bonus
#    }

# def extract_data_tcp(test_index: int, path: str):
def extract_data_tcp(path: str, nb_starting_character_to_remove: int, nb_finishing_character_to_remove: int):
    """Extract echoed TCP payload from pcap."""
    print("\n\nextract_data_tcp: start")

    # print(f"extract_data_tcp: test_index: {test_index}")
    num = 0
    tcb_seq_l = []
    payload_l = []
    target_tcb_iss = -1
    target_tcp_port = -1
    scapy_cap = rdpcap(path)
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
            #expected_payload_len = packet[IPv6].plen
            ip_payload_len = len(bytes(packet[TCP]))
            extra_bytes = ip_payload_len - expected_payload_len
            if extra_bytes != 0:
                payload_bytes = bytes(packet[TCP].payload)[:-(extra_bytes)]
                print(f"extract_data_tcp: extra_bytes: {extra_bytes}")
                print(f"extract_data_tcp: payload_bytes: {payload_bytes}")
            else:
                payload_bytes = bytes(packet[TCP].payload)
                print(f"extract_data_tcp: payload_bytes: {payload_bytes}")

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
                payload_str = payload_bytes.decode('UTF8','replace')
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
    
    # TODO check if bug wuth implementation sending partial retransmissions (ie )

    # Change to ASCII payload instead of hex
    #temp_payload = [x.decode('ascii') for x in payload_l]
    temp_payload = [x.decode('UTF8','replace') for x in payload_l]
    merged_payload = merge_load(temp_payload, tcb_seq_l)
    print(f"extract_data_tcp: merged_payload: {merged_payload}")

    #payload_without_peos_bonus = merged_payload.replace("0", "")
    #print(
    #    f"extract_data_tcp: payload_without_peos_bonus: {payload_without_peos_bonus}"
    #)
    if len(merged_payload) % 8 == 0:
        if nb_starting_character_to_remove != 0 and len(merged_payload) > 0:
            print(f"extract_data_tcp: can't truncate {nb_starting_character_to_remove} bytes on received payload {merged_payload}")
            sys.exit(-1)
        payload_without_bonus = merged_payload
    elif len(merged_payload) % 8 == 1:
        #assert (nb_starting_character_to_remove == 0 and nb_finishing_character_to_remove != 0) or (nb_starting_character_to_remove != 0 and nb_finishing_character_to_remove == 0)
        if len(merged_payload) == 1:
            print(f"extract_data_tcp: WARNING: only received extra starting chunk charactere !")
        payload_without_bonus = merged_payload[nb_starting_character_to_remove:len(merged_payload)] if nb_starting_character_to_remove != 0 else merged_payload[0:len(merged_payload) - nb_finishing_character_to_remove]
    else:
        assert len(merged_payload) > 2
        payload_without_bonus = merged_payload[nb_starting_character_to_remove:len(merged_payload) - nb_finishing_character_to_remove]
    # test_index = ''.join(re.findall(r'\d+', filename))

    print("extract_data_tcp: payload_without_bonus: ",payload_without_bonus)
    print("extract_data_tcp: end")

    return {
        "is_echo_reply": payload_without_bonus != "",
        "number": int(payload_without_bonus != ""),
        "payload": payload_without_bonus
    }



def extract_index(pcap_path: str):
    basename = os.path.basename(pcap_path)
    basename_wo_ext = os.path.splitext(basename)[0]
    index = int(basename_wo_ext.split("_")[-1])
    return index


def get_response_packet(pcap_path):
    print("get_response_packet: start")

    print(f"get_response_packet: pcap_path: {pcap_path}")

    pkts = rdpcap(pcap_path)
    p_l = PacketList(list(pkts))

    p_l_nofrag_icmpv4_echo_reply = get_icmpv4_response(p_l)
    p_l_nofrag_icmpv6_echo_reply = get_icmpv6_response(p_l)
    p_l_ipv4_nofrag_udp_response = get_ipv4_udp_response(p_l)
    p_l_ipv6_nofrag_udp_response = get_ipv6_udp_response(p_l)
    p_l_tcp_response = get_ipv4_tcp_response(p_l)

    print("get_response_packet: end")

    return p_l_nofrag_icmpv4_echo_reply + p_l_nofrag_icmpv6_echo_reply + p_l_ipv4_nofrag_udp_response + p_l_ipv6_nofrag_udp_response + p_l_tcp_response

def get_empty_json_entry() -> dict:
    return {
        "is_echo_reply": False,
        "number": 0,
        "payload": ""
    }

def process(pcap_directory: str, json_output_path: str, nb_starting_character_to_remove: int,
            nb_finishing_character_to_remove: int, test_index_offset: int):
    print("process: start")
    filename_l = os.listdir(pcap_directory)

    path_l = [
        os.path.join(pcap_directory, filename) for filename in filename_l
    ]

    p_l_l_response = [get_response_packet(path) for path in path_l]
    p_l_response = [item for sublist in p_l_l_response for item in sublist]
    print(f"process: response number: {len(p_l_response)}")

    is_ipv4_icmp = any(IP in p and ICMP in p for p in p_l_response)
    is_ipv6_icmp = any(
        IPv6 in p and p[IPv6].nh == 58 and p[IPv6].payload.type == 129
        for p in p_l_response)
    is_ipv4_udp = any(IP in p and UDP in p for p in p_l_response)
    is_ipv6_udp = any(IPv6 in p and UDP in p for p in p_l_response)
    is_tcp = any(TCP in p for p in p_l_response)

    print(
        f"process: is_ipv4_icmp: {is_ipv4_icmp} ; is_ipv6_icmp: {is_ipv6_icmp}"
    )
    print(f"process: is_ipv4_udp: {is_ipv4_udp} ; is_ipv6_udp: {is_ipv6_udp}")
    print(f"process: is_tcp: {is_tcp}")

    if is_ipv4_icmp:
        index_payload_d = {
            extract_index(pcap_path):
            extract_data_icmpv4(test_index_offset + extract_index(pcap_path),
                                pcap_path, nb_finishing_character_to_remove)
            for pcap_path in path_l
        }
    elif is_ipv4_udp:
        index_payload_d = {
            extract_index(pcap_path):
            extract_data_ipv4_udp(pcap_path, nb_finishing_character_to_remove)
            for pcap_path in path_l
        }
    elif is_ipv6_icmp:
        index_payload_d = {
            extract_index(pcap_path):
            extract_data_icmpv6(test_index_offset + extract_index(pcap_path),
                                pcap_path, nb_finishing_character_to_remove)
            for pcap_path in path_l
        }
    elif is_ipv6_udp:
        index_payload_d = {
            extract_index(pcap_path):
            extract_data_ipv6_udp(pcap_path, nb_finishing_character_to_remove)
            for pcap_path in path_l
        }
    elif is_tcp:
        index_payload_d = {
            extract_index(pcap_path):
            extract_data_tcp(pcap_path, nb_starting_character_to_remove, nb_finishing_character_to_remove)
            for pcap_path in path_l
        }
    else:
        #index_payload_d = { }
        index_payload_d = { 
            extract_index(pcap_path):
            get_empty_json_entry()
            for pcap_path in path_l
        }
    #else:
    #    print("process: could not find any payload")
    #    sys.exit(2)

    data_d = {"hm": index_payload_d}

    with open(json_output_path, 'w', encoding="UTF8") as opened_file:
        json.dump(data_d, opened_file, indent=2, sort_keys=True)

    print("process: end")


def usage():
    print(
        "extract_os_icmp_payload_from_directory_to_json.py -p <pcap_directory> -o <json_path> -r <nb_finishing_character_to_remove>"
    )


def main(argv):
    print("extract_os_icmp_payload_from_directory_to_json: start")

    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input-pcap-directory", type=str, default="")
    # parser.add_argument("-p", "--protocol", type=str, default="4")
    parser.add_argument("-j", "--json-path", type=str, default="")
    parser.add_argument("-sr", "--nb-starting-character-to-remove", type=int, default=0)
    parser.add_argument("-fr", "--nb-finishing-character-to-remove", type=int, default=0)
    parser.add_argument("-o", "--test-index-offset", type=int, default=0)
    args = parser.parse_args()

    input_pcap_directory = args.input_pcap_directory
    # protocol = args.protocol
    json_path = args.json_path
    nb_starting_character_to_remove = args.nb_starting_character_to_remove
    nb_finishing_character_to_remove = args.nb_finishing_character_to_remove
    test_index_offset = args.test_index_offset

    print(
        f"extract_os_icmp_payload_from_directory_to_json: input_pcap_directory: {input_pcap_directory}"
    )
    print(
        f"extract_os_icmp_payload_from_directory_to_json: json_path: {json_path}"
    )
    print(
        f"extract_os_icmp_payload_from_directory_to_json: nb_starting_character_to_remove: {nb_starting_character_to_remove}"
    )
    print(
        f"extract_os_icmp_payload_from_directory_to_json: nb_finishing_character_to_remove: {nb_finishing_character_to_remove}"
    )
    print(
        f"extract_os_icmp_payload_from_directory_to_json: test_index_offset: {test_index_offset}"
    )

    process(input_pcap_directory, json_path, nb_starting_character_to_remove,
            nb_finishing_character_to_remove, test_index_offset)

    print("extract_os_icmp_payload_from_directory_to_json: end")


if __name__ == "__main__":
    main(sys.argv[1:])
