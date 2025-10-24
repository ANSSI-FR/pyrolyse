#!/usr/bin/python3
import json
import sys
import subprocess
import time
import argparse
# from scapy.all import *
from threading import Thread, Event
from scapy.all import IP, TCP, sr1, send, sniff

# TODO: put full TCB as global variable

# TCB_SND_NXT stores the maximum sequence number located after sent data.
# If we force reordering, this introduces hole(s) in sent bytes.
# Even if located after a hole, a sequence number is however still valid
# because it is inside the send window.
TCB_SND_NXT = 0


# Reduce memory usage
# * https://www.pypy.org/: not working because only support python 3.9 and 3.10 (not 3.11 in latest Debian stable)
# * numba: decorator on some functions
# * cython: can be used on function but not on full script (cf https://riptutorial.com/cython), maybe ok for us if we split into two file and do the include inside only one => tested and not working


def get_tcp_payload(tcp_pdu):
    """
    Extract useful bytes from TCP payload.

    Parameters
    ----------
        tcp_pdu : ????
            TCP PDU
    """
    tcp_pdu_payload = tcp_pdu.payload
    if b'\x00' in bytes(tcp_pdu_payload):
        bytes_data = bytes(tcp_pdu_payload).replace(b'\x00', b'')
    else:
        bytes_data = tcp_pdu_payload
    return bytes_data


class Sniffer(Thread):
    """
    Sniff packet and sends ACKs.

    Attributes
    ----------
    destination_ip : str
        destination IP address
    source_port : int
        source port
    rstack : int
        sequence number of last data received

    Methods
    -------
    run():
        Thread main loop.
    """

    def __init__(self,
                 destination_ip: str,
                 source_port: int,
                 tcb_iss: int,
                 tcb_irs: int,
                 tcb_snd_una: int,
                 tcb_rcv_nxt: int,
                 scenario: str,
                 mode: str = "update",
                 interface: str = "eth1"):
        """
        Constructs all the necessary attributes for the person object.

        Parameters
        ----------
            interface : str
                interface name (e.g. eth0)
            destination_ip : str
                destination IP address
            source_port : int
                source port
            TODO: complete
        """
        super().__init__()
        self.stop_sniffer = Event()
        self.stop_sniffer_pep1 = Event()
        self.interface = interface
        self.destination_ip = destination_ip
        self.source_port = source_port
        self.index = 0
        self.last_echo_reply = None
        self.initial_time = None
        self.tcb_iss = tcb_iss
        self.tcb_irs = tcb_irs
        self.tcb_snd_una = tcb_snd_una
        self.tcb_rcv_nxt = tcb_rcv_nxt
        self.scenario = scenario
        self.mode = mode

    def join(self, timeout=1):
        """
        Join on this thread.
        """
        super().join(timeout)

    def run(self):
        """
        Wait for data and sends TCP ACKs.
        """
        print("Sniffer: run: start")
        if self.scenario == "pep1":
            sniff(iface=self.interface,
                  filter=f"ip and host {self.destination_ip}",
                  prn=self.update_last_data_and_send_ack,
                  stop_filter=lambda x: self.stop_sniffer_pep1.is_set())
        else:
            # For pep1, we want to ACK all the P+A from Server after having sent the testing
            # sequence.
            sniff(iface=self.interface,
                  filter=f"ip and host {self.destination_ip}",
                  prn=self.send_ack,
                  stop_filter=lambda x: self.stop_sniffer.is_set())
        print("Sniffer: run: end")

    def run__(self):
        """
        Wait for data and sends TCP ACKs.
        """
        print("Sniffer: run: start")
        if self.scenario == "pep1":
            print("Sniffer: run: sniff (pep1)")
            sniff(
                iface=self.interface,
                filter=f"ip and host {self.destination_ip}",
                # prn=self.keep_echo_reply_with_max_ack,
                prn=self.update_last_data,
                stop_filter=lambda x: self.stop_sniffer_pep1.is_set())
            print("Sniffer: run: send_ack (pep1)")
            self.send_ack(self.last_echo_reply)

        print("Sniffer: run: sniff")
        # For pep1, we want to ACK all the P+A from Server after having sent the testing sequence
        sniff(iface=self.interface,
              filter=f"ip and host {self.destination_ip}",
              prn=self.send_ack,
              stop_filter=lambda x: self.stop_sniffer.is_set())
        print("Sniffer: run: end")

    def update_last_data_and_send_ack(self, received_packet):
        """
        Update the last observed data and sends ACK.

        Parameters
        ----------
            received_packet : Scapy packet
                received packet
        """
        if self.mode == "update":
            self.update_last_data(received_packet)
        else:
            self.send_ack(received_packet)

    def update_last_data(self, received_packet):
        """
        Update the last observed data from the destination (i.e. TCB.RCV_NXT).

        Parameters
        ----------
            received_packet : Scapy packet
                received packet
        """
        print("Sniffer: update_last_data: start")

        # print(f"Sniffer: update_last_data: received_packet.time: {received_packet.time}")

        if self.initial_time is None:
            self.initial_time = received_packet.time
        # print(f"Sniffer: update_last_data: initial_time: {self.initial_time}")

        ip_packet = received_packet.getlayer('IP')
        tcp_pdu = received_packet['TCP']
        tcp_flags = received_packet['TCP'].flags

        bytes_data = get_tcp_payload(tcp_pdu)
        data_length = len(bytes_data)

        if ip_packet.src == self.destination_ip:
            seq_r = tcp_pdu.seq - self.tcb_irs
            ack_r = tcp_pdu.ack - self.tcb_iss
            print(f"Sniffer: update_last_data: self.tcb_iss: {self.tcb_iss}")
            print(f"Sniffer: update_last_data: tcp_pdu.ack: {tcp_pdu.ack}")
            print(
                f"Sniffer: update_last_data: {ip_packet.src} -> {ip_packet.dst} : {tcp_flags} - S={seq_r} A={ack_r} - Data: {data_length}"
            )
        else:
            seq_r = tcp_pdu.seq - self.tcb_iss
            ack_r = tcp_pdu.ack - self.tcb_irs
            print(f"Sniffer: update_last_data: self.tcb_irs: {self.tcb_irs}")
            print(f"Sniffer: update_last_data: tcp_pdu.ack: {tcp_pdu.ack}")
            print(
                f"Sniffer: update_last_data: {ip_packet.src} -> {ip_packet.dst} : {tcp_flags} - S={seq_r} A={ack_r} - Data: {data_length}"
            )

        if received_packet[
                'IP'].src == self.destination_ip and received_packet.haslayer(
                    'TCP') and received_packet[
                        'TCP'].dport == self.source_port and received_packet[
                            'TCP'].flags == 0x18:

            print(
                f"Sniffer: update_last_data: time: {received_packet.time-self.initial_time}"
            )

            print(
                f"Sniffer: update_last_data: self.tcb_rcv_nxt: {self.tcb_rcv_nxt}"
            )

            print(
                f"Sniffer: update_last_data: SEG.SEQ: {received_packet['TCP'].seq}"
            )

            bytes_data = get_tcp_payload(received_packet['TCP'])
            print(
                f"Sniffer: update_last_data: len(SEG.DATA): {len(bytes_data)}")

            seq_after_data = received_packet['TCP'].seq + len(bytes_data)
            print(
                f"Sniffer: update_last_data: seq_after_data: {seq_after_data}")

            if self.tcb_rcv_nxt < seq_after_data:
                self.tcb_rcv_nxt = seq_after_data
                self.last_echo_reply = received_packet
                # self.index += 1
                print(
                    f"Sniffer: update_last_data: tcb_rcv_nxt: {self.tcb_rcv_nxt}"
                )

        print("Sniffer: update_last_data: end")

    def keep_echo_reply_with_max_ack(self, received_packet):
        """
        Update the last observed ACK from the destination (i.e. TCB.RCV_NXT).

        Parameters
        ----------
            received_packet : Scapy packet
                received packet
        """
        print("Sniffer: keep_echo_reply_with_max_ack: start")

        if received_packet[
                'IP'].src == self.destination_ip and received_packet.haslayer(
                    'TCP') and received_packet[
                        'TCP'].dport == self.source_port and received_packet[
                            'TCP'].flags == 0x18 and received_packet[
                                'TCP'].ack >= self.tcb_snd_una:
            self.tcb_snd_una = received_packet['TCP'].ack
            self.last_echo_reply = received_packet

        print("Sniffer: keep_echo_reply_with_max_ack: end")

    def send_ack(self, received_packet):
        """
        Sends an ACK for the provided packet.

        Parameters
        ----------
            received_packet : Scapy packet
                packet to ack
        """
        global TCB_SND_NXT

        print("Sniffer: send_ack: start")

        # print(f"Sniffer: send_ack: received_packet.time: {received_packet.time}")

        if self.initial_time is None:
            self.initial_time = received_packet.time
        # print(f"Sniffer: send_ack: initial_time: {self.initial_time}")

        if received_packet is None:
            return

        ip_packet = received_packet.getlayer('IP')
        tcp_pdu = received_packet['TCP']
        bytes_data = get_tcp_payload(tcp_pdu)
        tcp_flags = received_packet['TCP'].flags
        if ip_packet.src == self.destination_ip:
            print(
                f"Sniffer: send_ack: {ip_packet.src} -> {ip_packet.dst} : {tcp_flags} - S={tcp_pdu.seq-self.tcb_irs} A={tcp_pdu.ack-self.tcb_iss} - Data: {len(bytes_data)}"
            )
        else:
            print(
                f"Sniffer: send_ack: {ip_packet.src} -> {ip_packet.dst} : {tcp_flags} - S={tcp_pdu.seq-self.tcb_iss} A={tcp_pdu.ack-self.tcb_irs} - Data: {len(bytes_data)}"
            )

        if ip_packet.src == self.destination_ip and received_packet[
                'TCP'].dport == self.source_port and "P" in received_packet[
                    'TCP'].flags and "A" in received_packet['TCP'].flags:

            print(
                f"Sniffer: send_ack: time: {received_packet.time-self.initial_time}"
            )
            # print(f"Sniffer: send_ack: self.tcb_iss: {self.tcb_iss}")
            # print(f"Sniffer: send_ack: self.tcb_irs: {self.tcb_irs}")

            print(
                f"Sniffer: send_ack: received_packet['TCP'].flags: {received_packet['TCP'].flags}"
            )
            data = received_packet['TCP'].payload.load.decode("utf-8")
            print(f"Sniffer: send_ack: data: {data}")

            ip_header = IP(src=ip_packet.dst, dst=ip_packet.src)

            print(f"Sniffer: send_ack: TCB_SND_NXT: {TCB_SND_NXT}")

            # We use max() to handle packet reordering (this should never happen inside pyrolyse).
            self.tcb_rcv_nxt = max(
                self.tcb_rcv_nxt, received_packet['TCP'].seq + len(bytes_data))
            tcp_header = TCP(dport=received_packet['TCP'].sport,
                             sport=self.source_port,
                             flags="A",
                             seq=TCB_SND_NXT,
                             ack=self.tcb_rcv_nxt)

            ack_packet = ip_header / tcp_header
            send(ack_packet)

            self.index += 1
            print("Sniffer: send_ack: ack sent")
        else:
            print("Sniffer: send_ack: packet ignored")

        print("Sniffer: send_ack: end")


class SingleTest(Thread):
    """
    Establish the TCP conenction, play the test scenario, and sends a RST.

    Attributes
    ----------
    test_index : int
        test index
    surname : str
        family name of the person
    age : int
        age of the person
    TODO: complete

    Methods
    -------
    run():
        Thread main loop.
    """

    def __init__(self, test_index: int, source_ip: str, destination_ip: str,
                 source_port: int, destination_port: int, scenario: str,
                 # offset_l: list[int], payload_l: list[str], output_pcap_path: str,
                 offset_l, payload_l, output_pcap_path: str,
                 max_offset_before_hole: int):
        """
        Constructs all the necessary attributes for the person object.

        Parameters
        ----------
            name : str
                first name of the person
            surname : str
                family name of the person
            age : int
                age of the person
        """
        super().__init__()
        self.test_index = test_index
        self.source_port = source_port
        self.destination_port = destination_port
        self.source_ip = source_ip
        self.destination_ip = destination_ip
        self.scenario = scenario
        self.offset_l = offset_l
        self.payload_l = payload_l
        self.output_pcap_path = output_pcap_path
        self.ip_header = IP(src=self.source_ip, dst=self.destination_ip)
        self.tcb_iss = 0
        self.tcb_irs = 0
        self.tcb_snd_una = 0
        self.tcb_snd_nxt_after_3whs = 0
        self.tcb_rcv_nxt = 0
        if scenario == "peos":
            value = max_offset_before_hole + 1
        else:
            value = max_offset_before_hole
        self.max_offset_before_hole = value

    def join(self, timeout=1):
        """
        Join on this thread.
        """
        super().join(timeout)

    def run(self):
        """
        Run the test and capture the traffic.
        """
        print("SingleTest: run: start")

        print(
            f"[*] SingleTest: run: start capturing traffic for test of index {self.test_index}"
        )
        created_process = subprocess.Popen([
            "tcpdump", "-U", "-i", "eth1", "-w", self.output_pcap_path, "-nn",
            f"host {self.destination_ip} and port {self.source_port}"
        ],
                                           stdout=subprocess.PIPE)

        time.sleep(1)
        self.send_message()
        time.sleep(2)
        self.send_rst()
        print("SingleTest: run: sent RST")
        time.sleep(2)
        created_process.terminate()

        print(
            f"[*] SingleTest: run: end capturing traffic for test of index {self.test_index}"
        )
        print("SingleTest: run: end")

    def send_message(self):
        """
        Established the TCP connection and call the function for the test scenario.
        """
        global TCB_SND_NXT

        # Send SYN
        self.tcb_iss = 10
        TCB_SND_NXT = self.tcb_iss
        tcp = TCP(dport=self.destination_port,
                  sport=self.source_port,
                  flags="S",
                  seq=TCB_SND_NXT)
        syn = self.ip_header / tcp
        TCB_SND_NXT += 1

        # Receive SYNACK
        synack = sr1(syn)
        # TODO: add check on SA flag
        tcp_pdu = synack['TCP']
        self.tcb_irs = int(tcp_pdu.seq)
        print(f"SingleTest: run: tcp_pdu.seq: {tcp_pdu.seq}")
        self.tcb_snd_una = synack.ack
        print(f"SingleTest: run: tcp_pdu.ack: {tcp_pdu.ack}")

        self.tcb_rcv_nxt = synack.seq + 1
        self.tcb_snd_nxt_after_3whs = self.tcb_iss + 1

        # In case scapy fail to capture server's reply, we want to reset the communication as
        # properly as possible (i.e. sequence number of RST needs to be in window).
        #self.rstseq[0] = synack['TCP'].ack

        # Send ACK
        time.sleep(0.5)
        tcp_header = TCP(sport=self.source_port,
                         dport=self.destination_port,
                         flags="A",
                         seq=TCB_SND_NXT,
                         ack=self.tcb_rcv_nxt,
                         )
        ack_packet = self.ip_header / tcp_header
        send(ack_packet)

        # Send data
        if self.scenario == "peos":
            self.send_peos_sequence()
        elif self.scenario == "pep1":
            self.send_pep1_sequence()
        elif self.scenario == "pep2":
            self.send_pep2_sequence()
        else:
            print(
                "SingleTest: run: the scenario {self.scenario} doesn't exist")
            sys.exit(-1)

    def send_pep2_sequence(self):
        """
        Send the test sequence with the PEP2 scenario.
        """
        global TCB_SND_NXT

        for i, (offset,
                payload) in enumerate(zip(self.offset_l, self.payload_l)):
            # Send payload
            print(f"[*] SingleTest: send_pep2_sequence: sending {i}")
            seg_seq = self.tcb_snd_nxt_after_3whs + offset
            tcp = TCP(dport=self.destination_port,
                      sport=self.source_port,
                      ack=self.tcb_rcv_nxt,
                      seq=seg_seq,
                      flags="PA")
            pack = self.ip_header / tcp / payload

            TCB_SND_NXT = max(TCB_SND_NXT, seg_seq + len(payload))
            print(
                f"[*] SingleTest: send_pep2_sequence: TCB_SND_NXT: {TCB_SND_NXT}"
            )

            # Sniff echo packet to send ACK
            sniffer = Sniffer(self.destination_ip, self.source_port,
                              self.tcb_iss, self.tcb_irs, self.tcb_snd_una,
                              self.tcb_rcv_nxt, self.scenario)
            print("[*] SingleTest: send_pep2_sequence: start sniffing...")
            sniffer.start()

            time.sleep(1)
            #time.sleep(2)
            send(pack)
            time.sleep(1)
            print("[*] SingleTest: send_pep2_sequence: stop sniffing")
            sniffer.stop_sniffer.set()
            sniffer.join(0)
            self.tcb_rcv_nxt = sniffer.tcb_rcv_nxt

    def send_pep1_sequence(self):
        """
        Send the test sequence with the PEP1 scenario.
        """
        global TCB_SND_NXT

        # Sniff echo reply packets
        sniffer = Sniffer(self.destination_ip, self.source_port, self.tcb_iss,
                          self.tcb_irs, self.tcb_snd_una, self.tcb_rcv_nxt,
                          self.scenario)
        print("[*] SingleTest: send_pep1_sequence: start sniffing...")
        sniffer.start()

        for i, (offset,
                payload) in enumerate(zip(self.offset_l, self.payload_l)):
            # Send payload
            print(f"[*] SingleTest: send_pep1_sequence: sending {i}")
            time.sleep(0.2)
            seg_seq = self.tcb_snd_nxt_after_3whs + offset
            tcp = TCP(dport=self.destination_port,
                      sport=self.source_port,
                      ack=self.tcb_rcv_nxt,
                      seq=seg_seq,
                      flags="PA")
            packet = self.ip_header / tcp / payload
            send(packet)

            TCB_SND_NXT = max(TCB_SND_NXT, seg_seq + len(payload))
            print(
                f"SingleTest: send_pep1_sequence: TCB_SND_NXT: {TCB_SND_NXT}")

        # PROBLEM: this half-works because:
        # 1) updating sniffer exits when stop_sniffer_pep1 event is set, this is ok
        # 2) send_ack is only performed when a packet is received
        # FIX: ACK sent in 2) should be sent right away.

        # time.sleep(0.5)
        # print("[*] SingleTest: send_pep1_sequence: stop sniffing for pep1")
        # sniffer.stop_sniffer_pep1.set()
        # time.sleep(6)
        # print("[*] SingleTest: send_pep1_sequence: stop sniffing for final ack")
        # sniffer.stop_sniffer.set()
        # time.sleep(1)

        # time.sleep(6)

        print("SingleTest: send_pep1_sequence: switch to send_ack mode")
        # If we put this after the final hack, we often miss some response.
        sniffer.mode = "send_ack"

        print("SingleTest: send_pep1_sequence: final ACK")
        ip_header = IP(src=self.source_ip, dst=self.destination_ip)
        self.tcb_rcv_nxt = sniffer.tcb_rcv_nxt
        tcp_header = TCP(dport=self.destination_port,
                         sport=self.source_port,
                         flags="A",
                         seq=TCB_SND_NXT,
                         ack=self.tcb_rcv_nxt)
        ack_packet = ip_header / tcp_header
        send(ack_packet)

        print(
            "SingleTest: send_pep1_sequence: wait a bit to ack final response")
        time.sleep(2)

        print("SingleTest: send_pep1_sequence: stop sniffer thread")
        sniffer.stop_sniffer_pep1.set()

        sniffer.join()
        self.tcb_rcv_nxt = sniffer.tcb_rcv_nxt

    def send_peos_sequence(self):
        """
        Send the test sequence with the PEOS scenario.
        """
        global TCB_SND_NXT

        for i, (offset,
                payload) in enumerate(zip(self.offset_l, self.payload_l)):
            # Send payload
            print(f"[*] SingleTest: send_peos_sequence: sending {i}")
            time.sleep(0.2)
            seg_seq = self.tcb_snd_nxt_after_3whs + offset + 1
            tcp = TCP(dport=self.destination_port,
                      sport=self.source_port,
                      seq=seg_seq,
                      ack=self.tcb_rcv_nxt,
                      flags="PA")
            pack = self.ip_header / tcp / payload
            send(pack)

            TCB_SND_NXT = max(TCB_SND_NXT, seg_seq + len(payload))

        # Sniff echo packet after sending all segments
        sniffer = Sniffer(self.destination_ip, self.source_port, self.tcb_iss,
                          self.tcb_irs, self.tcb_snd_una, self.tcb_rcv_nxt,
                          self.scenario)
        print("[*] SingleTest: send_peos_sequence: start sniffing...")
        sniffer.start()
        time.sleep(0.5)

        # Sending last temporal chunk located at the start byte-wise
        tcp.seq = self.tcb_snd_nxt_after_3whs
        seg0 = "0"
        packet_0 = self.ip_header / tcp / seg0
        send(packet_0)
        time.sleep(2)
        print("[*] SingleTest: send_peos_sequence: stop sniffing")
        sniffer.stop_sniffer.set()
        sniffer.join()
        print("[*] SingleTest: send_peos_sequence: after SingleTest.join()")
        self.tcb_rcv_nxt = sniffer.tcb_rcv_nxt

    def send_rst(self):
        """
        Sends a RST.
        """
        print("SingleTest: send_rst: start")
        print(
            f"SingleTest: send_rst: self.tcb_snd_nxt_after_3whs: {self.tcb_snd_nxt_after_3whs}"
        )
        print(
            f"SingleTest: send_rst: self.max_offset_before_hole: {self.max_offset_before_hole}"
        )
        print(f"SingleTest: send_rst: self.tcb_rcv_nxt: {self.tcb_rcv_nxt}")
        rst_packet = self.ip_header / TCP(
            sport=self.source_port,
            dport=self.destination_port,
            flags="RA",
            seq=self.tcb_snd_nxt_after_3whs + self.max_offset_before_hole,
            ack=self.tcb_rcv_nxt)
        send(rst_packet)
        print("SingleTest: send_rst: end")


def get_offset_and_payload(test_index: int, byte_time_sequence_json_path: str):
    """
    Extracts offset and payload lists.

    Parameters
    ----------
        test_index : str
            test index
        byte_time_sequence_json_path : str
            path of the JSON file with the test cases
    """
    with open(byte_time_sequence_json_path, encoding='utf8') as file:
        byte_time_sequence_d = json.load(file)

    if int(test_index) <= 12:
        test_descr_d = byte_time_sequence_d['byte_time_pair_sequence_c']['hm'][
            test_index]
    else:
        test_descr_d = byte_time_sequence_d['byte_time_triplet_sequence_c'][
            'hm'][test_index]

    del byte_time_sequence_d

    # Get the offset
    offset_l = [
        p_info['offset']
        for i, p_info in test_descr_d['chunk_c']['bm'].items()
    ]

    multiplied_offset_l = [element * 8 for element in offset_l]

    # Get the payload
    payload_l = [
        s_info['internet_checksum_s']
        for i, s_info in test_descr_d['chunk_c']['bm'].items()
    ]

    # Keep offset/payload in line with temporary position
    temp_pos = test_descr_d['temporal_position_v']
    index_l = [
        i_info['index'] for i, i_info in test_descr_d['chunk_c']['bm'].items()
    ]

    del test_descr_d

    comb = zip(index_l, multiplied_offset_l, payload_l)
    z_l = [x for _, x in sorted(zip(temp_pos, comb))]
    multiplied_offset_l = [i[1] for i in z_l]
    payload_l = [i[2] for i in z_l]

    return multiplied_offset_l, payload_l


# check if there is a hole in test case and, if so, return the maximum seq number before hole
# def get_max_offset_before_hole(offset_l: list[int], payload_l: list[str]):
def get_max_offset_before_hole(offset_l, payload_l):
    """
    Constructs all the necessary attributes for the person object.

    Parameters
    ----------
        offset_l : list[int]
            offset position list
        payload_l : list[str]
            payload list
    """
    print("get_max_offset_before_hole: offset_l: ", offset_l)
    print("get_max_offset_before_hole: payload_l: ", payload_l)

    payload_offset_d = {
        payload: offset_l[i]
        for i, payload in enumerate(payload_l)
    }
    print("get_max_offset_before_hole: payload_offset_d: ", payload_offset_d)
    sorted_payload_offset_d = dict(
        sorted(payload_offset_d.items(), key=lambda x: x[1]))
    print("get_max_offset_before_hole: sorted_payload_offset_d: ",
          sorted_payload_offset_d)

    if list(sorted_payload_offset_d.values())[0] != 0:
        return 0

    max_offset_without_hole = list(sorted_payload_offset_d.values())[0] + len(
        list(sorted_payload_offset_d.keys())[0])
    print("get_max_offset_before_hole: max_offset_without_hole: ",
          max_offset_without_hole)
    for payload, offset in sorted_payload_offset_d.items():
        # We got a hole
        if offset > max_offset_without_hole:
            print(
                "get_max_offset_before_hole: offset > max_offset_wihtout_hole: True"
            )
            print("get_max_offset_before_hole: max_offset_without_hole: ",
                  max_offset_without_hole)
            return max_offset_without_hole

        # We update max_offset_wihtout_hole only if current segment finishes after hole.
        if offset + len(payload) > max_offset_without_hole:
            print("get_max_offset_before_hole: max_offset_without_hole update")
            max_offset_without_hole = offset + len(payload)

    print("get_max_offset_before_hole: max_offset_without_hole: ",
          max_offset_without_hole)
    return max_offset_without_hole


def process(test_index: int, source_ip: str, destination_ip: str,
            destination_port: int, byte_time_sequence_json_path: str,
            output_pcap_path: str, scenario: str):
    """
    Performs the test.

    Parameters
    ----------
        test_index : int
            first name of the person
        source_ip : str
            source IP
        destination_ip : str
            destination IP
    """

    offset_l, payload_l = get_offset_and_payload(test_index,
                                                 byte_time_sequence_json_path)

    source_port = 10000 + int(test_index)

    max_offset_before_hole = get_max_offset_before_hole(offset_l, payload_l)

    single_test = SingleTest(test_index, source_ip, destination_ip,
                             source_port, destination_port, scenario, offset_l,
                             payload_l, output_pcap_path,
                             max_offset_before_hole)
    single_test.start()
    single_test.join()


def main():
    """
    Main

    Parameters
    ----------
        None
    """
    parser = argparse.ArgumentParser()
    parser.add_argument("-j",
                        "--byte-time-sequence-json-path",
                        type=str,
                        default="")
    parser.add_argument("-o", "--output-pcap-path", type=str, default="")
    parser.add_argument("-s", "--source-ip", type=str, default="")
    parser.add_argument("-d", "--destination-ip", type=str, default="")
    parser.add_argument("-i", "--test-index", type=str, default="")
    parser.add_argument("-c", "--scenario", type=str, default="")
    args = parser.parse_args()

    byte_time_sequence_json_path = args.byte_time_sequence_json_path
    output_pcap_path = args.output_pcap_path
    source_ip = args.source_ip
    destination_ip = args.destination_ip
    test_index = args.test_index
    scenario = args.scenario

    destination_port = 7

    process(test_index, source_ip, destination_ip, destination_port,
            byte_time_sequence_json_path, output_pcap_path, scenario)


if __name__ == "__main__":
    main()
