#!/usr/bin/python3
import json
import os
import subprocess
import time
from scapy.all import *
from threading import Thread, Event
import argparse
from collections import OrderedDict

def get_ts_value(l):
    print('get_ts_value: start')
    print(f'get_ts_value: l: {l}')   
    ts_l = [ t for t in l if t[0] == "Timestamp"]
    assert len(ts_l) >= 1
    ts = ts_l[0]
    print(f'get_ts_value: ts: {ts}')  
    v = ts[1][0]
    print(f'get_ts_value: v: {v}')  
    print('get_ts_value: end')
    return v

class Sniffer(Thread):
    def  __init__(self, sp, dip, rstack, myack, scenario, ts_current, ts_value_to_answer, interface="eth1"):
        super().__init__()
        self.interface = interface
        self.stop_sniffer = Event()
        self.stop_sniffer_pep1 = Event()
        self.sport = sp
        self.dip = dip
        self.rstack = rstack
        #self.rstseq = rstseq
        self.index = 0
        self.scenario = scenario
        self.last_echo_reply = None
        self.myack = myack
        self.ts_current = ts_current
        self.ts_value_to_answer = ts_value_to_answer

    def join(self, timeout=1):
        super().join(timeout)

    def run(self):
        print('Sniffer: run: start')
        if self.scenario == "pep1":
            sniff(iface=self.interface, filter=f"ip and host {self.dip}", prn=self.keep_echo_reply_with_max_ack, stop_filter=lambda x: self.stop_sniffer_pep1.is_set())
            self.send_ack(self.last_echo_reply)

        # For pep1, we want to ACK all the P+A from Server after having sent the testing sequence
        sniff(iface=self.interface, filter=f"ip and host {self.dip}", prn=self.send_ack, stop_filter=lambda x: self.stop_sniffer.is_set())
        print('Sniffer: run: end')

    def keep_echo_reply_with_max_ack(self, packet):
        print('Sniffer: keep_echo_reply_with_max_ack: start')

        print('Sniffer: keep_echo_reply_with_max_ack: packet[IP].src == self.dip: ', packet[IP].src == self.dip)
        print('Sniffer: keep_echo_reply_with_max_ack: packet[TCP].dport == self.sport: ', packet[TCP].dport == self.sport)
        print('Sniffer: keep_echo_reply_with_max_ack: packet[TCP].ack: ', packet[TCP].ack)
        print('Sniffer: keep_echo_reply_with_max_ack: self.myack: ', self.myack)

        if packet[IP].src == self.dip and packet.haslayer(TCP) and packet[TCP].dport == self.sport and packet[TCP].flags == 0x18 and packet[TCP].ack >= self.myack:
            self.myack = packet[TCP].ack
            self.last_echo_reply = packet
            #self.index += 1
            print('Sniffer: keep_echo_reply_with_max_ack: myack: ', self.myack)

        print('Sniffer: keep_echo_reply_with_max_ack: end')
    
    def send_ack(self, packet):
        print('Sniffer: send_ack: start')

        if packet == None:
            return 

        ip_layer = packet.getlayer(IP)
        if b'\x00' in bytes(packet[TCP].payload) :
            b = bytes(packet[TCP].payload).replace(b'\x00', b'')
        else:
            b = packet[TCP].payload

        self.ts_value_to_answer = get_ts_value(packet["TCP"].options)

        print('Sniffer: send_ack: ip_layer.src: ', ip_layer.src)
        print('Sniffer: send_ack: self.dip: ', self.dip)
        print('Sniffer: send_ack: packet[TCP].dport: ', packet[TCP].dport)
        print('Sniffer: send_ack: self.sport: ', self.sport)
        print('Sniffer: send_ack: packet[TCP].flags == 0x18: ', packet[TCP].flags == 0x18)
        print(f"Sniffer: send_ack: self.ts_value_to_answer: {self.ts_value_to_answer}")
        
        if ip_layer.src == self.dip and packet[TCP].dport == self.sport and packet[TCP].flags == 0x18:
            print(packet[TCP].payload.load.decode("utf-8"))
            self.rstack[self.index] = packet[TCP].seq + len(b)
            #self.rstseq[self.index] = packet[TCP].ack
            ip=IP(src=ip_layer.dst, dst=ip_layer.src)
            tcp=TCP(ack=packet[TCP].seq + len(b), dport=packet[TCP].sport, sport=self.sport, flags="A",seq=packet[TCP].ack,options=[('NOP', ()),('NOP', ()),('Timestamp', (self.ts_current, self.ts_value_to_answer))])
            self.myack = packet[TCP].seq + len(b)
            self.ts_current += 1
            ackit=ip/tcp
            send(ackit)
            self.index += 1
            print('Sniffer: ack sent')


        print('Sniffer: send_ack: end')
   
 
class SingleTest(Thread):
    def  __init__(self, test_index, sp, dp, sip, dip, scenario, offset, payload, output_pcap_path, max_offset_before_any_hole):
        super().__init__()
        self.test_index = test_index
        self.sp = sp
        self.dp = dp
        self.sip = sip
        self.dip = dip
        self.scenario = scenario
        self.offset = offset
        self.payload = payload
        self.output_pcap_path = output_pcap_path
        self.ip = IP(src=self.sip, dst=self.dip)
        self.tcpseq = 0
        #self.rstseq = [0] * len(offset)
        self.max_offset_before_any_hole = max_offset_before_any_hole + 1 if scenario == 'peos' else max_offset_before_any_hole 
        self.rstack = [0] * len(offset)
        self.ts_current = 1000
        self.ts_value_to_answer = 0


    def join(self, timeout=1):
        super().join(timeout)

    def run(self):
        print('SingleTest: run: start')

        print(f"[*] Start capturing traffic for test of index {self.test_index}")
        p = subprocess.Popen(["tcpdump", "-U", "-i", "eth1", "-w", self.output_pcap_path, "-nn", f"host {self.dip} and port {self.sp}"], stdout=subprocess.PIPE)

        time.sleep(1)
        self.sendMSG()
        time.sleep(2)
        self.sendRST()
        print("Sent RST")
        time.sleep(2)
        p.terminate()

        print(f"[*] End capturing traffic for test of index {self.test_index}")        
        print('SingleTest: run: end')

    
    def sendMSG(self):
        ISN = 10
        tcp = TCP(dport=self.dp, sport=self.sp,flags="S",seq=ISN, window=64240, options=[('MSS', 1460),('SAckOK', ''),('Timestamp', (self.ts_current, self.ts_value_to_answer)),('NOP', ()),('WScale', 7)])
        # , window=65535
        # ,('Timestamp', (1098453, 0))
        syn = self.ip/tcp
        synack = sr1(syn)

        # In case scapy fail to capture server's reply, we want to reset the communication as properly as possible (i.e. sequence number of RST needs to be in window).
        #self.rstseq[0] = synack[TCP].ack 

        time.sleep(0.5)
        self.myack = synack.seq + 1
        self.tcpseq = ISN + 1
        self.ts_value_to_answer = get_ts_value(synack["TCP"].options)
        print(f"sendMSG: self.ts_value_to_answer: {self.ts_value_to_answer}")
        tcp = TCP(ack=self.myack, dport=self.dp, sport=self.sp, flags="A",seq = self.tcpseq, options=[('NOP', ()),('NOP', ()),('Timestamp', (self.ts_current, self.ts_value_to_answer))])
        # We only increase after the ACK of the handshake (and not after the initial SYN) because the Linux stack does so.
        self.ts_current += 1

        ackit = self.ip/tcp
        send(ackit)

        if self.scenario == "peos":
            self.send_peos_sequence()
        elif self.scenario == "pep1":
            self.send_pep1_sequence()
        elif self.scenario == "pep2":
            self.send_pep2_sequence()
        else: 
            print("The scenario doesn't exist")
            exit(-1)

    def send_pep2_sequence(self):
        for i in range(len(self.offset)):
            # Send payload
            tcp=TCP(ack=self.myack, dport=self.dp, sport=self.sp, flags="PA")
            tcp.seq= self.tcpseq + self.offset[i] 
            pack=self.ip/tcp/self.payload[i]

            # Sniff echo packet to send ACK
            sniffer = Sniffer(self.sp, self.dip, self.rstack, self.myack, self.scenario)
            print("[*] Start sniffing...")
            sniffer.start()

            time.sleep(1)
            #time.sleep(2)
            send(pack)
            time.sleep(1)
            print("[*] Stop sniffing")
            sniffer.stop_sniffer.set()
            sniffer.join(0)
            self.myack = sniffer.myack

    def send_pep1_sequence(self):
        # Sniff echo reply packets 
        sniffer = Sniffer(self.sp, self.dip, self.rstack, 0, self.scenario, self.ts_current, self.ts_value_to_answer)
        print("[*] Start sniffing...")
        sniffer.start()

        for i in range(len(self.offset)):
            # Send payload
            time.sleep(0.2)
            tcp=TCP(ack=self.myack, dport=self.dp, sport=self.sp, flags="PA", options=[('NOP', ()),('NOP', ()),('Timestamp', (self.ts_current, self.ts_value_to_answer))])
            self.ts_current += 1
            tcp.seq= self.tcpseq + self.offset[i] 
            pack=self.ip/tcp/self.payload[i]
            send(pack) 

        #time.sleep(0.5)
        print("[*] Stop sniffing")
        sniffer.stop_sniffer_pep1.set()
        time.sleep(6)
        sniffer.stop_sniffer.set()
        time.sleep(1)
        sniffer.join()
        self.myack = sniffer.myack

    def send_peos_sequence(self):
        for i in range(len(self.offset)):
            # Send payload
            time.sleep(0.2)
            tcp=TCP(ack=self.myack, dport=self.dp, sport=self.sp, flags="PA")
            tcp.seq= self.tcpseq + self.offset[i] + 1 
            pack=self.ip/tcp/self.payload[i]
            send(pack) 

        # Sniff echo packet after sending all segments
        sniffer = Sniffer(self.sp, self.dip, self.rstack, self.myack, self.scenario, self.ts_current, self.ts_value_to_answer)
        print("[*] Start sniffing...")
        sniffer.start()
        time.sleep(0.5)
        tcp.seq= self.tcpseq
        seg0 = "0"
        pack0=self.ip/tcp/seg0
        send(pack0)
        time.sleep(2)
        print("[*] Stop sniffing")
        sniffer.stop_sniffer.set()
        sniffer.join()
        print("[*] After SingleTest.join()")
        self.myack = sniffer.myack

    def sendRST(self):
        print("sendRST: self.tcpseq + self.max_offset_before_any_hole: ", self.tcpseq + self.max_offset_before_any_hole)
        print("sendRST: max(self.rstack): ", max(self.rstack))
        LASTRST=self.ip/TCP(sport=self.sp, dport=self.dp, flags="RA", seq=self.tcpseq + self.max_offset_before_any_hole , ack=max(self.rstack))
        send(LASTRST)

def get_offset_and_payload(test_index, byte_time_sequence_json_path):
    with open(byte_time_sequence_json_path) as f:
        data = json.load(f)

    if int(test_index) <= 12:  
        test_descr = data['byte_time_pair_sequence_c']['hm'][test_index]
    else:
        test_descr = data['byte_time_triplet_sequence_c']['hm'][test_index]

    sp = 10000 + int(test_index)

    # Get the offset
    offset = []
    for p_id, p_info in test_descr['chunk_c']['bm'].items():
        res = p_info['offset']
        offset.append(res)

    multiplied_offset = [element * 8 for element in offset]

    # Get the payload
    payload = []
    for s_id, s_info in test_descr['chunk_c']['bm'].items():
        temp = s_info['internet_checksum_s']
        payload.append(temp)

    # Keep offset/payload in line with temporary position
    temp_pos = test_descr['temporal_position_v']
    index = []
    for i_id, i_info in test_descr['chunk_c']['bm'].items():
        temp2 = i_info['index']
        index.append(temp2)

    comb = zip(index, multiplied_offset, payload)
    Z = list([x for _,x in sorted(zip(temp_pos,comb))])
    multiplied_offset = [i[1] for i in Z]
    payload = [i[2] for i in Z]

    return multiplied_offset, payload

# check if there is a hole in test case and, if so, return the maximum seq number before hole
def get_max_offset_before_any_hole(offsets, payloads):
    print("check_hole_in_test_case: offset: ", offsets)
    print("check_hole_in_test_case: payload: ", payloads)

    payload_offset_d = { payload:offsets[i] for i,payload in enumerate(payloads) }
    print("check_hole_in_test_case: payload_offset_d: ", payload_offset_d)
    sorted_payload_offset_d = dict(sorted(payload_offset_d.items(), key=lambda x:x[1]))
    print("check_hole_in_test_case: sorted_payload_offset_d: ", sorted_payload_offset_d)

    if list(sorted_payload_offset_d.values())[0] != 0:
        return 0

    max_offset_without_hole = list(sorted_payload_offset_d.values())[0] + len(list(sorted_payload_offset_d.keys())[0])
    print("check_hole_in_test_case: max_offset_without_hole: ", max_offset_without_hole)
    for payload, offset in sorted_payload_offset_d.items():
        # we got a hole
        if offset > max_offset_without_hole:
            print("check_hole_in_test_case: offset > max_offset_wihtout_hole: True")
            print("check_hole_in_test_case: max_offset_without_hole: ", max_offset_without_hole)
            return max_offset_without_hole

        # we update max_offset_wihtout_hole only if current segment finishes after
        if offset + len(payload) > max_offset_without_hole:
            print("check_hole_in_test_case: offset + payload_length > max_offset_without_hole: True")
            max_offset_without_hole = offset + len(payload) 
    
    print("check_hole_in_test_case: max_offset_without_hole: ", max_offset_without_hole)
    return max_offset_without_hole

def Process(test_index, sip, dip, dp, byte_time_sequence_json_path, output_pcap_path, scenario):
    
    offset, payload = get_offset_and_payload(test_index, byte_time_sequence_json_path)

    sp = 10000 + int(test_index)

    max_offset_before_any_hole = get_max_offset_before_any_hole(offset, payload)

    singleTest = SingleTest(test_index, sp, dp, sip, dip, scenario, offset, payload, output_pcap_path, max_offset_before_any_hole)
    singleTest.start()
    singleTest.join()

    
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-j", "--byte-time-sequence-json-path", type=str, default="")
    parser.add_argument("-o", "--output-pcap-path", type=str, default="")
    parser.add_argument("-s", "--source-ip", type=str, default="")
    parser.add_argument("-d", "--destination-ip", type=str, default="")
    parser.add_argument("-i", "--test-index", type=str, default="")
    parser.add_argument("-c", "--scenario", type=str, default="")
    args = parser.parse_args()

    byte_time_sequence_json_path = args.byte_time_sequence_json_path
    output_pcap_path = args.output_pcap_path
    sip = args.source_ip
    dip = args.destination_ip
    test_index = args.test_index
    scenario = args.scenario

    dp = 7
    
    Process(test_index, 
    sip,
    dip,
    dp,
    byte_time_sequence_json_path, 
    output_pcap_path,
    scenario)
    
if __name__ == "__main__":
    main()

