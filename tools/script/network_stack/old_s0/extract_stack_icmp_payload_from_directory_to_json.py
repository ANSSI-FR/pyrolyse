#!/usr/bin/python
import sys
import os
import json
import argparse
# import re
# from scapy.all import Ether,IP,ICMP,TCP,Raw,rdpcap,wrpcap,PacketList,bytes_hex
from scapy.all import IP, IPv6, ICMP, rdpcap, PacketList
# import networkx as nx

# import graph_utils as gu


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

    # We only keep the packet without fragmentation (MF=0 and frag_offset=0).
    p_l_nofrag = [p for p in p_l if (p[IP].flags & 1) == 0 and p[IP].frag == 0]

    # We only keep the ICMP packet.
    p_l_nofrag_icmp = [p for p in p_l_nofrag if p[IP].proto == 1]

    # We only keep the ICMP Echo Reply packet.
    p_l_nofrag_icmp_echo_reply = [
        p for p in p_l_nofrag_icmp if p[ICMP].type == 0
    ]
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


def extract_data_icmpv6(index, pcap_path, nb_final_character_to_remove):
    print("\n\nextract_data_icmpv6: pcap_path :", pcap_path)
    pkts = rdpcap(pcap_path)
    p_l = PacketList(list(pkts))

    # We only keep the ICMP Echo Reply packet.
    p_l_nofrag_icmp_echo_reply = [
        p for p in p_l if p[IPv6].nh == 58 and p[IPv6].payload.type == 129
    ]

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


def extract_index(pcap_path):
    basename = os.path.basename(pcap_path)
    basename_wo_ext = os.path.splitext(basename)[0]
    index = basename_wo_ext.split("_")[-1]
    return index


def process(pcap_directory, protocol, json_output_path,
            nb_final_character_to_remove):
    filename_l = os.listdir(pcap_directory)

    path_l = [
        os.path.join(pcap_directory, filename) for filename in filename_l
    ]

    if protocol == "ipv4":
        index_payload_d = {
            extract_index(pcap_path):
            extract_data_icmpv4(extract_index(pcap_path), pcap_path,
                                nb_final_character_to_remove)
            for pcap_path in path_l
        }
    elif protocol == "ipv6":
        index_payload_d = {
            extract_index(pcap_path):
            extract_data_icmpv6(extract_index(pcap_path), pcap_path,
                                nb_final_character_to_remove)
            for pcap_path in path_l
        }
    else:
        print("Unexpected protocol provided")
        sys.exit(-2)

    data_d = {"hm": index_payload_d}

    with open(json_output_path, 'w', encoding="UTF8") as f:
        json.dump(data_d, f, indent=2, sort_keys=True)


def usage():
    print(
        "extract_os_icmp_payload_from_directory_to_json.py -i <input_pcap_directory> -p <protocol> -o <json_path> -r <nb_final_character_to_remove>"
    )


def main(argv):
    print("extract_os_icmp_payload_from_directory_to_json: start")

    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input-pcap-directory", type=str, default="")
    parser.add_argument("-p", "--protocol", type=str, default="4")
    parser.add_argument("-j", "--json-path", type=str, default="")
    parser.add_argument("-r", "--nb-character-to-remove", type=int, default=0)
    args = parser.parse_args()

    pcap_directory = args.pcap_directory
    protocol = args.protocol
    json_path = args.json_path
    nb_final_character_to_remove = args.nb_character_to_remove

    print(
        f"extract_os_icmp_payload_from_directory_to_json: pcap_directory: {pcap_directory}"
    )
    print(
        f"extract_os_icmp_payload_from_directory_to_json: protocol: {protocol}"
    )
    print(
        f"extract_os_icmp_payload_from_directory_to_json: json_path: {json_path}"
    )

    process(pcap_directory, protocol, json_path,
            nb_final_character_to_remove)

    print("extract_os_icmp_payload_from_directory_to_json: end")


if __name__ == "__main__":
    main(sys.argv[1:])
