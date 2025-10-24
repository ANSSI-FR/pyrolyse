#!/usr/bin/python3
from scapy.all import *
import sys
import time
import os, re

def read_packet(number, filename):
    num = 0
    sequence = []
    timestamp = []
    scapy_cap = rdpcap(filename)
    print(scapy_cap)
    mark_name = "tcp_test_case_" + str(number) + ".md"
    f = open(mark_name, "w")
    begin_text = "```sequence (theme=\"simple\") \nTitle:Test Case " + number
    handshake = "\nNote over Client, Server: TCP handshake"
    body = ""
    for packet in scapy_cap:
        if TCP in packet:
            timestamp.append(packet.time)
            num += 1
            ip_layer = packet.getlayer(IP)

            # Strip of padding bytes
            print("Raw echo bytes: ", bytes(packet[TCP].payload))
            expected_payload_len = packet[IP].len - packet[IP].ihl * 4
            ip_payload_len = len(bytes(packet[TCP]))
            extra_bytes = ip_payload_len - expected_payload_len
            if extra_bytes != 0 :
                b = bytes(packet[TCP].payload)[:-(extra_bytes)]
            else: 
                b = bytes(packet[TCP].payload)
            
            
            
            if ip_layer.src == '192.168.10.11':
                sequence.append(packet[TCP].seq)
                c2s = "\nServer -> Client:["
            else:
                c2s = "\nClient -> Server:["

            
            SEQ = packet[TCP].seq if packet[TCP].seq < 80 else packet[TCP].seq - min(sequence)
            ACK = packet[TCP].ack if packet[TCP].ack < 80 else packet[TCP].ack - min(sequence)
            if packet[TCP].flags == 0x18 or packet[TCP].flags == 0x04 or packet[TCP].flags == 0x10:
                payload_str = ""
                # Extrace payload
                if len(b) != 0:
                    payload = b.decode("ascii")
                    #print(payload)
                    if ip_layer.src == '192.168.10.11':
                        payload_str = "\nNote over Server: echo:\\n"+payload
                    else:
                        payload_str = "\nNote over Client: send segment:\\n"+payload
                temp_pac = payload_str + c2s +  str(packet[TCP].flags) +  "] SEQ=" + str(SEQ) + " ACK=" + str(ACK) + " time=" +str(packet.time - min(timestamp)) 
                body += temp_pac
            
            


    

    # for packet in scapy_cap:
    #     if TCP in packet:  
    #         num += 1 
    #         ip_layer = packet.getlayer(IP)
    #         timestamp.append(packet.time)
    #         if ip_layer.src == '192.168.10.11':
    #             sequence.append(packet[TCP].seq)
    #             # print("Sequence: ", sequence)
    #         if len(packet[TCP].payload) == 0:
    #             payload = ''
    #         elif b'\x00' in bytes(packet[TCP].payload) :
    #             b = bytes(packet[TCP].payload).replace(b'\x00', b'')
    #             payload = b.decode("utf-8")
    #         else:
    #             payload = packet[TCP].payload.load.decode("utf-8")

            
    #         print("No.{num} [{flg}] {src} -> {dst} \n SPORT = {SP} -> DPORT = {DP} \n SEQ = {SEQ} ACK = {ACK} \n LEN = {LEN} Payload = {PAY}\n Time = {time}\n"
    #                 .format(num = num, flg=packet[TCP].flags, 
    #                 src=ip_layer.src, 
    #                 dst=ip_layer.dst, 
    #                 SP = packet[TCP].sport,
    #                 DP = packet[TCP].dport,
    #                 SEQ = packet[TCP].seq if packet[TCP].seq < 80 else packet[TCP].seq - min(sequence) ,
    #                 ACK = packet[TCP].ack if packet[TCP].ack < 80 else packet[TCP].ack - min(sequence) ,
    #                 LEN = len(payload) if payload else 0,
    #                 PAY = payload if payload else 'N/A',
    #                 time = packet.time - min(timestamp)))
    #         # print("IHL field: ",str(packet)[:(packet[IP].ihl * 4)])
    #         # print("Total IP layer Length: ", len(packet[IP]))
    #         # print("Total TCP layer Length: ", len(packet[TCP]))
    #         # print("Raw Payload: ", packet[TCP].payload)
    #         # print("Payload Type: ", type(packet[TCP].payload))
    #         print("\n\n")
    #     else:
    #         continue
    print(body)
    end_text = "\n```"
    f.write(begin_text+handshake+body + end_text)
    f.close()
    
path = os.path.abspath(os.getcwd()) + "/" + sys.argv[1]

split_name = re.split('_',re.split('/',path)[-1])[-1]
number = split_name.split(".")[0]
#print(number)
filename = os.fsdecode(path)

if filename.endswith(".pcap"): 
    read_packet(number,filename)
else:
    print("Please specify the pcap pair or triplet path")




# for file in os.listdir(directory):
#     filename = os.fsdecode(file)
#     #print(filename)
#     if filename.endswith(".pcap"): 
#         display_payload(path,filename)
#         continue
#     else:
#         continue



