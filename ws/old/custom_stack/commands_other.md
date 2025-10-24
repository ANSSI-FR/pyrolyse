



# bonus



### extract payload


editcap vm0_icmp_0.pcap toto.pcap 4-1


tshark -r vm0_icmp_0.pcap -Y 'frame.number >= 4 and frame.number <= 4' -w toto.pcap -F pcap




