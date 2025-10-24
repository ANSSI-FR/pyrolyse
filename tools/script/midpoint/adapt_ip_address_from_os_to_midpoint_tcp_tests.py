import os
import sys
from scapy.all import *
import argparse

def change_ip(path_f:str,new_ip_dst:str,new_ip_src:str):
    packets = rdpcap(path_f)

    if not packets:
        sys.exit(-1)
    
    SYN = 0x02
    # first packet should be the SYN otherwise trace is corrupted
    if TCP in packets[0] and packets[0][TCP].flags & SYN:
        ip_dst = packets[0][IP].dst 
    else:
        sys.exit(-1)
    
    for packet in packets:
        if IP in packet and packet[IP].src == ip_dst:
            packet[IP].src = new_ip_dst
            packet[IP].dst = new_ip_src
        if IP in packet and packet[IP].dst == ip_dst:
            packet[IP].dst = new_ip_dst
            packet[IP].src = new_ip_src

        # force recomputation of checksum
        del packet[IP].chksum
        del packet[TCP].chksum

    return packets


def process(pcap_directory:str,new_ip_dst:str,new_ip_src:str):
    print("process: start")
    filename_l = os.listdir(pcap_directory)
    print("process: filename_l: ", filename_l)

    path_l = [ os.path.join(pcap_directory, filename) for filename in filename_l ]

    # We sort to have a deterministic processing order to ease debugging accross machines.
    path_l.sort()

    for path_f in path_l:
        packets_new_src_ip = change_ip(path_f,new_ip_dst,new_ip_src)
        wrpcap(path_f,packets_new_src_ip)
    print("process: end")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--pcap-directory", type=str, default="")
    parser.add_argument("-d", "--new-ip-dst", type=str, default="")
    parser.add_argument("-s", "--new-ip-src", type=str, default="")
    args = parser.parse_args()

    pcap_directory = args.pcap_directory
    new_ip_dst = args.new_ip_dst
    new_ip_src = args.new_ip_src

    print('pcap_directory: "%s"'%(pcap_directory))
    
    process(pcap_directory,new_ip_dst,new_ip_src)

if __name__ == "__main__":
    main()