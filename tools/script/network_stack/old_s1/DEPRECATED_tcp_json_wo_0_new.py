#!/usr/bin/python3
import json, os, subprocess
import random, time, sys
from scapy.all import *
from datetime import datetime
from threading import Thread, Event
import glob

class Sniffer(Thread):
    def  __init__(self, sp, rstack, rstseq, index, myack, interface="eth1"):
        super().__init__()
        self.interface = interface
        self.stop_sniffer = Event()
        self.sport = sp
        self.rstack = rstack
        self.rstseq = rstseq
        self.index = index
        self.myack = myack

    def join(self, timeout=1):
        #self.stop_sniffer.set()
        super().join(timeout)

    def run(self):
        sniff(iface=self.interface, filter=f"ip and host {dip}", prn=self.send_ack, stop_filter=lambda x: self.stop_sniffer.is_set())
        #sniff(filter="port 7 and host 192.168.10.11", prn=self.print_packet)

    def send_ack(self, packet):
        ip_layer = packet.getlayer(IP)
        if b'\x00' in bytes(packet[TCP].payload) :
            b = bytes(packet[TCP].payload).replace(b'\x00', b'')
        else:
            b = packet[TCP].payload
 
        print("[!] New Packet: {src} -> {dst}".format(src=ip_layer.src, dst=ip_layer.dst))
        #print(len(packet),len(packet[TCP].payload), self.sport, packet[TCP].dport)
        
        if ip_layer.src == dip and packet[TCP].dport == self.sport and packet[TCP].flags == 0x18:
          print(packet[TCP].payload.load.decode("utf-8"))
          self.rstack[self.index] = packet[TCP].seq + len(b)
          self.rstseq[self.index] = packet[TCP].ack
          ip=IP(src=ip_layer.dst, dst=ip_layer.src)
          tcp=TCP(ack=packet[TCP].seq + len(b), dport=packet[TCP].sport, sport=self.sport, flags="A",seq=packet[TCP].ack) #window=5480
          self.myack = packet[TCP].seq + len(b)
          ackit=ip/tcp
          send(ackit)

# callback function - called for every packet
def traffic_monitor_callbak(pkt):
    if p in pkt:
        print (len(p),len(p[TCP].payload))

def sendMSG(offset, payload):
    global sp
    sp = random.randint(1,65535)
    ISN =10
    ip=IP(src=sip, dst=dip)
    tcp=TCP(dport=dp, sport=sp,flags="S",seq=ISN) #window=5480,options=[('MSS',1460),('SACK_PERM', 1)]
    syn=ip/tcp
    synack=sr1(syn)
    
    time.sleep(0.5)
    myack=synack.seq + 1
    tcpseq=ISN+1
    tcp=TCP(ack=myack, dport=dp, sport=sp, flags="A",seq=tcpseq)
    ackit=ip/tcp
    send(ackit)

    #global sendack
    global rstseq
    rstseq = [0] * len(offset)
    global rstack
    rstack = [0] * len(offset)
    for i in range(len(offset)):
        
        # Send payload
        tcp=TCP(ack=myack, dport=dp, sport=sp, flags="PA")
        tcp.seq= tcpseq + offset[i] 
        pack=ip/tcp/payload[i]
        
        # Sniff echo packet to send ACK
        sniffer = Sniffer(sp, rstack, rstseq, i, myack)
        print("[*] Start sniffing...")
        sniffer.start()

        time.sleep(0.5)
        send(pack)
        time.sleep(0.5)
        print("[*] Stop sniffing")
        sniffer.stop_sniffer.set()
        sniffer.join()
        myack = sniffer.myack
        

def sendRST(rsack, rsseq):
    ip=IP(src=sip, dst=dip)
    LASTRST=ip/TCP(sport=sp, dport=dp, flags="R", seq=rsseq , ack=rsack)
    send(LASTRST)

def ExecuteData(dicts, curr_time):
    dict_no = list(dicts['hm'].keys())
    for i in dict_no:
        slice_data = dicts['hm'][i]['chunk_c']['bm']

        #   Get the offset
        offset = []
        for p_id, p_info in slice_data.items():
            res = p_info['offset']
            offset.append(res)

        multiplied_offset = [element * 8 for element in offset]

        # Get the payload
        payload = []
        for s_id, s_info in slice_data.items():
            temp = s_info['internet_checksum_s']
            payload.append(temp)

        # Keep offset/payload in line with temporary position
        temp_pos = dicts['hm'][i]['temporal_position_v']
        index = []
        for i_id, i_info in slice_data.items():
            temp2 = i_info['index']
            index.append(temp2)
        

        comb = zip(index, multiplied_offset, payload)
        Z = list([x for _,x in sorted(zip(temp_pos,comb))])
        multiplied_offset = [i[1] for i in Z]
        payload = [i[2] for i in Z]

        capture_file_name = curr_time + "/tcp_test_"  + i +  ".pcap"
        p = subprocess.Popen(["tcpdump", "-U", "-i", "any", "-w", capture_file_name, "-nn", f"host {dip}"], stdout=subprocess.PIPE)
        time.sleep(0.5)
        sendMSG(multiplied_offset, payload)
        time.sleep(2)
        print("RST ACK: ", max(rstack))
        print("RST SEQ: ", max(rstseq))
        sendRST(max(rstack), max(rstseq))
        print("Sent RST")
        time.sleep(1)
        p.terminate()
        print("terminated")
    
def main():
    global dip
    global sip
    global dp
    dip = ""
    sip = ""
    dp = 7

    os_directory = input("Enter os directory: ")
    print(os_directory)

    # Look for destination and source ip
    vagrant_file = open(glob.glob(f"{os_directory}/Vagrantfile")[0],"r")

    for line in vagrant_file:
        if 'base.vm.network "private_network"' in line:
            sip = str(line.split('ip: ')[1].split('"')[1])
        elif 'target.vm.network "private_network"' in line:
            dip = str(line.split('ip: ')[1].split('"')[1])

    with open('test_data/byte_time_sequence.json') as f:
      data = json.load(f)

    #Drop RST for TCP server
    os.system(f'iptables -I OUTPUT 1 -d {dip} -p tcp --tcp-flags RST RST -j DROP')
    time.sleep(0.5)
    #os.system('route del -net 192.168.10.0 netmask 255.255.255.0 gw 0.0.0.0')

    pairs = data['byte_time_pair_sequence_c']
    triplets = data['byte_time_triplet_sequence_c']

    #Create path to store the pcap
    curr_time = os_directory + "/TCP_segmentation_scenario_1_" + datetime.now().strftime("%Y-%m-%d_%H:%M")
    directory = os.path.abspath(os.getcwd()) 
    path = os.path.join(directory, curr_time)
    print(path)
    try:
        #original_umask = os.umask(0)
        #os.makedirs(path, mode=0o777, exist_ok = True)
        os.system("sudo mkdir '%s'" % curr_time)
        print("Directory '%s' created successfully" % curr_time)
    except OSError as error:
        print("Directory '%s' can not be created" % curr_time)
        print(error)

    print("Sending pairs")
    ExecuteData(pairs, curr_time)
    time.sleep(2)
    print("Sending triplets")
    ExecuteData(triplets, curr_time)

if __name__ == "__main__":
    main()

