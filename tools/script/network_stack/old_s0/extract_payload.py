#!/usr/bin/python3
from PIL.Image import core as _imaging
from scapy.all import *
import sys, time, os, re
import json
import matplotlib.pyplot as plt

def diff_unique(a,b):
    #print ("a:{} = b:{}".format(a,b))
    if a < b:
        return -1
    elif a==b:
        return 0
    else:
        return 1

def compare_list(new_list, old_list):
    a = { t[0]:t[1] for t in new_list } 
    b = { t[0]:t[1] for t in old_list }
    common = list( set(a.keys())&set(b.keys()))
    return [diff_unique(a[key], b[key]) for key in common]

def mergeLoad(hexload, ssequence):
    collectPay = []
    if len(hexload) == 1:
        return ''.join(hexload)
    else:
        list_pay = []

        for j in range(len(hexload)): 
            lst = hexload[j]
            newlist = [lst[i:i + 8] for i in range(0, len(lst), 8)]
            
            sequ = [r for r in range(ssequence[j], ssequence[j] + len(newlist) * 8, 8)]
            merges = list(zip(sequ, newlist))
            print ("Merges: ", merges)
            if not collectPay:
                collectPay = merges
            else: 
                
                inc_dec_list = compare_list(merges,collectPay)
                if not inc_dec_list:
                    collectPay = [*collectPay, *merges]
                elif all(v != 0 for v in inc_dec_list) :
                    print("inc_dec_list", inc_dec_list)
                    print("Conflict in overlapping bytes")
                else:
                    print("No conflict in overlapping bytes")
                    tempy = [*collectPay, *merges]
                    tempy2 = [t for t in (set(tuple(i) for i in tempy))]
                    collectPay = sorted(tempy2, key=lambda tup: tup[0])
                    print("collectPay", collectPay)
                    continue
        
        for item in collectPay:
            list_pay.append(item[1])
        print(''.join(list_pay))
        return ''.join(list_pay)
                

def createJSON(spayload, index, path):
    #concat_payload = [''.join(i) for i in hexpayload]
    numbers = [ int(x) for x in index ]
    comb = list(zip(index, spayload))
    Z = list([x for _,x in sorted(zip(numbers,comb))])
    

    d = {}
    d['hm'] = {}
    for j in range(len(Z)):
        
        # print(temp2)
        #print(Z[j][0])
        d['hm'][str(Z[j][0])] = {}
        d['hm'][str(Z[j][0])]['is_echo_reply'] = True
        d['hm'][str(Z[j][0])]['payload'] = Z[j][1]
    
    json_name = path +"/tcp_payload_" + datetime.now().strftime("%Y-%m-%d_%H:%M") +".json"
    with open(json_name, "w") as write_file:
        json.dump(d, write_file, indent=2)

def build_graph(inx, bytepayload, seq, path):
    payload=  [x.decode('ascii') for x in bytepayload]
    multiplied_offset = seq
    print("Seq: ", multiplied_offset)
    temp_pos = list(range(len(seq)))
    print("Temp pos: ", temp_pos)
    

    # example data
    x =  multiplied_offset
    y = temp_pos
    error = [len(i) for i in payload]
    
    if len(seq) == 4:
        xerror = [(0, 0, 0, 0), error]
    elif len(seq) == 3:
        xerror = [(0, 0, 0), error]
        print(xerror)
    elif len(seq) == 2 : 
        xerror = [(0, 0), error]
        print(xerror)
    else:
        xerror = [(0,), error]
        print(xerror)

    fig = plt.figure()
    bar = plt.errorbar(x, y, xerr=xerror, fmt=',' )
    
    plt.xticks([0, 8, 16, 24, 32, 40, 48])
    plt.yticks([-1, 0, 1, 2, 3],('Start', 'First', 'Second', 'Third', 'Finish'))
    plt.xlabel("Sequence Number")
    for j in range(len(seq)):
        plt.text(x[j],y[j]+0.1, payload[j])
    graph_name = path +"echo_" + inx[0] +".png"
    print(graph_name)
    plt.savefig(graph_name)
    plt.close(fig)


def display_payload(path, filename):
    num = 0
    expay = []
    sequence = [] 
    bpay = []
    scapy_cap = rdpcap(path + "/"+ filename)
    for packet in scapy_cap:
        if TCP in packet:  
            num += 1
            SYN = 0x02
            ACK = 0x10
            if packet[TCP].flags & SYN and packet[TCP].flags & ACK :
                temp_seq  = packet[TCP].seq
                des_port = packet[TCP].dport
                continue

            # Strip of padding bytes
            
            ip_layer = packet.getlayer(IP)
            expected_payload_len = packet[IP].len - packet[IP].ihl * 4
            ip_payload_len = len(bytes(packet[TCP]))
            extra_bytes = ip_payload_len - expected_payload_len
            if extra_bytes != 0 :
                b = bytes(packet[TCP].payload)[:-(extra_bytes)]
                # print("Extra Bytes: ", extra_bytes)
                # print("Actual Payloads: ",b)
            else: 
                b = bytes(packet[TCP].payload)
                # print("Actual Payloads: ",b)

            #Extrace payload
            if len(b) != 0 and ip_layer.src == '192.168.10.11' and packet[TCP].dport == des_port:
                print("source port: ", packet[TCP].dport)
                payload = b.decode("ascii")
                #c = b.hex()
                # print("c is: ", c)
                print("No.{num} [{flg}] {src} -> {dst} \n SPORT = {SP} -> DPORT = {DP} \n SEQ = {SEQ} ACK = {ACK} \n LEN = {LEN} Payload = {PAY}\n Time = {time}\n"
                    .format(num = num, flg=packet[TCP].flags, 
                    src=ip_layer.src, 
                    dst=ip_layer.dst, 
                    SP = packet[TCP].sport,
                    DP = packet[TCP].dport,
                    SEQ = packet[TCP].seq,
                    ACK = packet[TCP].ack,
                    LEN = len(payload),
                    PAY = payload if payload else 'NULL',
                    time = packet[TCP].time))

                if not bpay and payload:
                    #expay.append(c)
                    bpay.append(b)
                    #print("packet[TCP].seq - temp_seq - 1: ", packet[TCP].seq - temp_seq - 1)
                    sequence.append(packet[TCP].seq - temp_seq - 1)
                elif b not in bpay:
                    bpay.append(b)
                    sequence.append(packet[TCP].seq - temp_seq - 1)
                else :
                    continue
        else:
            continue
    return sequence, bpay
        
    
path = os.path.abspath(os.getcwd()) + "/" + sys.argv[1]
directory = os.fsencode(path)
count = []
hexload = []
numfile = 0

for file in os.listdir(directory):
    
    filename = os.fsdecode(file)
    print(filename)
    if filename.endswith(".pcap"): 
        numfile += 1
        ssequence, byte_payload = display_payload(path,filename)
        print("Index: ", re.findall(r'\d+',filename))

        print("Payload is: ", byte_payload)
        print("Sequence: ", ssequence)
        
        #Change to ASCII payload instead of hex
        temp_payload=  [x.decode('ascii') for x in byte_payload]
        print("Temp payload: ", temp_payload)
        tempay = mergeLoad(temp_payload, ssequence)
        print("Temp pay: ", tempay)
        hexload.append(tempay.replace("0", ""))
        print("hexload is: ", hexload)
        count.append(''.join(re.findall(r'\d+',filename)))
        
        
        build_graph(re.findall(r'\d+',filename), byte_payload, ssequence, path)

        continue
    else:
        continue

#print(hexload)
createJSON(hexload, count, path)
#print(byteload)



