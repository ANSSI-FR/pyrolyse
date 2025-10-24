#!/usr/bin/python3
import json
import sys
import subprocess
import time
import argparse
# from scapy.all import *
from threading import Thread, Event
from scapy.all import IP, TCP, sr1, send, sniff
# from collections import OrderedDict


tcb_snd_nxt = 0

def get_tcp_payload(tcp_pdu):
    tcp_pdu_payload = tcp_pdu.payload
    if b'\x00' in bytes(tcp_pdu_payload):
        bytes_data = bytes(tcp_pdu_payload).replace(b'\x00', b'')
    else:
        bytes_data = tcp_pdu_payload
    return bytes_data


class Sniffer(Thread):
    """
    Sniff packet and sends ACKs.

    ...

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

    def __init__(
            self,
            destination_ip: str,
            source_port: int,
            # rstack: list[int],
            myack: int,
            scenario: str,
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
        self.scenario = scenario
        self.last_echo_reply = None
        self.myack = myack
        self.tcb_snd_una = myack
        self.tcb_rcv_nxt = myack

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
            sniff(
                iface=self.interface,
                filter=f"ip and host {self.destination_ip}",
                # prn=self.keep_echo_reply_with_max_ack,
                prn=self.keep_echo_reply_with_biggest_last_data,
                stop_filter=lambda x: self.stop_sniffer_pep1.is_set())
            self.send_ack(self.last_echo_reply)

        # For pep1, we want to ACK all the P+A from Server after having sent the testing sequence
        sniff(iface=self.interface,
              filter=f"ip and host {self.destination_ip}",
              prn=self.send_ack,
              stop_filter=lambda x: self.stop_sniffer.is_set())
        print("Sniffer: run: end")

    def keep_echo_reply_with_biggest_last_data(self, received_packet):
        """
        Update the last observed ACK from the destination (i.e. TCB.RCV_NXT).

        Parameters
        ----------
            received_packet : Scapy packet
                received packet
        """
        print("Sniffer: keep_echo_reply_with_biggest_last_data: start")

        # print(
        #     f"Sniffer: keep_echo_reply_with_biggest_last_data: packet['IP'].src == self.destination_ip: {received_packet['IP'].src == self.destination_ip}")
        # print(
        #     f"Sniffer: keep_echo_reply_with_biggest_last_data: packet['TCP'].dport == self.source_port: {received_packet['TCP'].dport == self.source_port}")

        if received_packet[
                'IP'].src == self.destination_ip and received_packet.haslayer(
                    'TCP') and received_packet[
                        'TCP'].dport == self.source_port and received_packet[
                            'TCP'].flags == 0x18:

            # print(f"Sniffer: keep_echo_reply_with_biggest_last_data: packet['TCP'].ack: {received_packet['TCP'].ack}")
            # print(f"Sniffer: keep_echo_reply_with_biggest_last_data: self.myack: {self.myack}")
            print(
                f"Sniffer: keep_echo_reply_with_biggest_last_data: self.tcb_rcv_nxt: {self.tcb_rcv_nxt}"
            )

            bytes_data = get_tcp_payload(received_packet['TCP'])
            data_after_tcp_payload = received_packet['TCP'].seq + len(
                bytes_data)
            print(
                f"Sniffer: keep_echo_reply_with_biggest_last_data: received_packet['TCP'].seq: {received_packet['TCP'].seq}"
            )
            print(
                f"Sniffer: keep_echo_reply_with_biggest_last_data: len(bytes_data): {len(bytes_data)}"
            )
            print(
                f"Sniffer: keep_echo_reply_with_biggest_last_data: data_after_tcp_payload: {data_after_tcp_payload}"
            )
            if self.tcb_rcv_nxt < data_after_tcp_payload:
                self.tcb_rcv_nxt = data_after_tcp_payload
                self.last_echo_reply = received_packet
                # self.index += 1
                print(
                    f"Sniffer: keep_echo_reply_with_biggest_last_data: tcb_rcv_nxt: {self.tcb_rcv_nxt}"
                )

        print("Sniffer: keep_echo_reply_with_biggest_last_data: end")

    def keep_echo_reply_with_max_ack(self, received_packet):
        """
        Update the last observed ACK from the destination (i.e. TCB.RCV_NXT).

        Parameters
        ----------
            received_packet : Scapy packet
                received packet
        """
        print("Sniffer: keep_echo_reply_with_max_ack: start")

        print(
            f"Sniffer: keep_echo_reply_with_max_ack: packet['IP'].src == self.destination_ip: {received_packet['IP'].src == self.destination_ip}"
        )
        print(
            f"Sniffer: keep_echo_reply_with_max_ack: packet['TCP'].dport == self.source_port: {received_packet['TCP'].dport == self.source_port}"
        )
        print(
            f"Sniffer: keep_echo_reply_with_max_ack: packet['TCP'].ack: {received_packet['TCP'].ack}"
        )
        print(
            f"Sniffer: keep_echo_reply_with_max_ack: self.myack: {self.myack}")

        if received_packet[
                'IP'].src == self.destination_ip and received_packet.haslayer(
                    'TCP') and received_packet[
                        'TCP'].dport == self.source_port and received_packet[
                            'TCP'].flags == 0x18 and received_packet[
                                'TCP'].ack >= self.tcb_snd_una:
            self.tcb_snd_una = received_packet['TCP'].ack
            self.last_echo_reply = received_packet
            #self.index += 1
            print(
                f"Sniffer: keep_echo_reply_with_max_ack: myack: {self.myack}")

        print("Sniffer: keep_echo_reply_with_max_ack: end")

    def send_ack(self, received_packet):
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
        print("Sniffer: send_ack: start")

        if received_packet is None:
            return

        ip_layer = received_packet.getlayer('IP')
        bytes_data = get_tcp_payload(received_packet['TCP'])
        # if b'\x00' in bytes(received_packet['TCP'].payload):
        #     bytes_data = bytes(received_packet['TCP'].payload).replace(b'\x00', b'')
        # else:
        #     bytes_data = received_packet['TCP'].payload

        print(f"Sniffer: send_ack: ip_layer.src: {ip_layer.src}")
        print(f"Sniffer: send_ack: self.destination_ip: {self.destination_ip}")
        print(
            f"Sniffer: send_ack: packet['TCP'].dport: {received_packet['TCP'].dport}"
        )
        print(f"Sniffer: send_ack: self.source_port: {self.source_port}")
        print(
            f"Sniffer: send_ack: packet['TCP'].flags == 0x18: {received_packet['TCP'].flags == 0x18}"
        )

        print(f"Sniffer: send_ack: self.index: {self.index}")
        # print(f"Sniffer: send_ack: self.rstack: {self.rstack}")
        print(f"Sniffer: send_ack: self.tcb_rcv_nxt: {self.tcb_rcv_nxt}")

        if ip_layer.src == self.destination_ip and received_packet[
                'TCP'].dport == self.source_port and received_packet[
                    'TCP'].flags == 0x18:
            print(received_packet['TCP'].payload.load.decode("utf-8"))
            # self.rstack[
            #     self.index] = received_packet['TCP'].seq + len(bytes_data)
            #self.rstseq[self.index] = packet['TCP'].ack
            ip_header = IP(src=ip_layer.dst, dst=ip_layer.src)

            # We use max() to handle packet reordering (this should never happen inside pyrolyse).
            self.tcb_rcv_nxt = max(
                self.tcb_rcv_nxt, received_packet['TCP'].seq + len(bytes_data))
            tcp_header = TCP(dport=received_packet['TCP'].sport,
                             sport=self.source_port,
                             flags="A",
                             seq=received_packet['TCP'].ack,
                             ack=self.tcb_rcv_nxt)

            ack_packet = ip_header / tcp_header
            send(ack_packet)

            self.index += 1
            print("Sniffer: ack sent")

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

    def __init__(self, test_index, source_ip, destination_ip, source_port,
                 destination_port, scenario, offset, payload, output_pcap_path,
                 max_offset_before_any_hole):
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
        self.offset = offset
        self.payload = payload
        self.output_pcap_path = output_pcap_path
        self.ip_header = IP(src=self.source_ip, dst=self.destination_ip)
        self.tcb_snd_nxt_after_handshake = 0
        self.tcb_rcv_nxt = 0
        #self.rstseq = [0] * len(offset)
        self.max_offset_before_any_hole = max_offset_before_any_hole + 1 if scenario == 'peos' else max_offset_before_any_hole
        # We use 2*len(offset) because some target send both ACK and echo back to the base.
        # self.rstack = [0] * 2 * len(offset)

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
        tcp_intial_sequence_number = 10
        tcp = TCP(dport=self.destination_port,
                  sport=self.source_port,
                  flags="S",
                  seq=tcp_intial_sequence_number)
        syn = self.ip_header / tcp
        synack = sr1(syn)

        # In case scapy fail to capture server's reply, we want to reset the communication as properly as possible (i.e. sequence number of RST needs to be in window).
        #self.rstseq[0] = synack['TCP'].ack

        # TODO: add check on SA flag

        time.sleep(0.5)
        self.myack = synack.seq + 1
        self.tcb_snd_nxt_after_handshake = tcp_intial_sequence_number + 1
        tcp_header = TCP(ack=self.myack,
                         dport=self.destination_port,
                         sport=self.source_port,
                         flags="A",
                         seq=self.tcb_snd_nxt_after_handshake)
        ack_packet = self.ip_header / tcp_header
        send(ack_packet)

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
        for i, _ in enumerate(self.offset):
            # Send payload
            tcp = TCP(ack=self.myack,
                      dport=self.destination_port,
                      sport=self.source_port,
                      flags="PA")
            tcp.seq = self.tcb_snd_nxt_after_handshake + self.offset[i]
            pack = self.ip_header / tcp / self.payload[i]

            # Sniff echo packet to send ACK
            sniffer = Sniffer(self.destination_ip, self.source_port,
                              self.myack, self.scenario)
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
        # Sniff echo reply packets
        sniffer = Sniffer(self.destination_ip, self.source_port, 0,
                          self.scenario)
        print("[*] SingleTest: send_pep1_sequence: start sniffing...")
        sniffer.start()

        for i, _ in enumerate(self.offset):
            # Send payload
            time.sleep(0.2)
            tcp = TCP(ack=self.myack,
                      dport=self.destination_port,
                      sport=self.source_port,
                      flags="PA")
            tcp.seq = self.tcb_snd_nxt_after_handshake + self.offset[i]
            pack = self.ip_header / tcp / self.payload[i]
            send(pack)

        #time.sleep(0.5)
        print("[*] SingleTest: send_pep1_sequence: stop sniffing")
        sniffer.stop_sniffer_pep1.set()
        time.sleep(6)
        sniffer.stop_sniffer.set()
        time.sleep(1)
        sniffer.join()
        self.tcb_rcv_nxt = sniffer.tcb_rcv_nxt

    def send_peos_sequence(self):
        """
        Send the test sequence with the PEOS scenario.
        """
        for i, _ in enumerate(self.offset):
            # Send payload
            time.sleep(0.2)
            tcp = TCP(ack=self.myack,
                      dport=self.destination_port,
                      sport=self.source_port,
                      flags="PA")
            tcp.seq = self.tcb_snd_nxt_after_handshake + self.offset[i] + 1
            pack = self.ip_header / tcp / self.payload[i]
            send(pack)

        # Sniff echo packet after sending all segments
        sniffer = Sniffer(self.destination_ip, self.source_port, self.myack,
                          self.scenario)
        print("[*] SingleTest: send_peos_sequence: start sniffing...")
        sniffer.start()
        time.sleep(0.5)

        # Sending last temporal chunk located at the start byte-wise
        tcp.seq = self.tcb_snd_nxt_after_handshake
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
        print(
            f"SingleTest: send_rst: self.tcb_snd_nxt_after_handshake: {self.tcb_snd_nxt_after_handshake}"
        )
        print(
            f"SingleTest: send_rst: self.max_offset_before_any_hole: {self.max_offset_before_any_hole}"
        )
        print(f"SingleTest: send_rst: self.tcb_rcv_nxt: {self.tcb_rcv_nxt}")
        # print(f"SingleTest: send_rst: self.rstack: {self.rstack}")
        # print(f"SingleTest: send_rst: max(self.rstack): {max(self.rstack)}")
        rst_packet = self.ip_header / TCP(sport=self.source_port,
                                          dport=self.destination_port,
                                          flags="RA",
                                          seq=self.tcb_snd_nxt_after_handshake
                                          + self.max_offset_before_any_hole,
                                          ack=self.tcb_rcv_nxt)
        send(rst_packet)


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
        data = json.load(file)

    if int(test_index) <= 12:
        test_descr = data['byte_time_pair_sequence_c']['hm'][test_index]
    else:
        test_descr = data['byte_time_triplet_sequence_c']['hm'][test_index]

    # sp = 10000 + int(test_index)

    # Get the offset
    # offset_l = []
    # for _, p_info in test_descr['chunk_c']['bm'].items():
    #     res = p_info['offset']
    #     offset_l.append(res)
    offset_l = [
        p_info['offset'] for i, p_info in test_descr['chunk_c']['bm'].items()
    ]

    multiplied_offset_l = [element * 8 for element in offset_l]

    # Get the payload
    # payload_l = []
    # for _, s_info in test_descr['chunk_c']['bm'].items():
    #     temp = s_info['internet_checksum_s']
    #     payload_l.append(temp)
    payload_l = [
        s_info['internet_checksum_s']
        for i, s_info in test_descr['chunk_c']['bm'].items()
    ]

    # Keep offset/payload in line with temporary position
    temp_pos = test_descr['temporal_position_v']
    # index_l = []
    # for _, i_info in test_descr['chunk_c']['bm'].items():
    #     temp2 = i_info['index']
    #     index_l.append(temp2)
    index_l = [
        i_info['index'] for i, i_info in test_descr['chunk_c']['bm'].items()
    ]

    comb = zip(index_l, multiplied_offset_l, payload_l)
    z_l = list([x for _, x in sorted(zip(temp_pos, comb))])
    multiplied_offset_l = [i[1] for i in z_l]
    payload_l = [i[2] for i in z_l]

    return multiplied_offset_l, payload_l


# check if there is a hole in test case and, if so, return the maximum seq number before hole
def get_max_offset_before_any_hole(offset_l: list[int], payload_l: list[str]):
    """
    Constructs all the necessary attributes for the person object.

    Parameters
    ----------
        offset_l : list[int]
            offet position list
        payload_l : list[str]
            payload list
    """
    print("get_max_offset_before_any_hole: offset: ", offset_l)
    print("get_max_offset_before_any_hole: payload: ", payload_l)

    payload_offset_d = {
        payload: offset_l[i]
        for i, payload in enumerate(payload_l)
    }
    print("get_max_offset_before_any_hole: payload_offset_d: ",
          payload_offset_d)
    sorted_payload_offset_d = dict(
        sorted(payload_offset_d.items(), key=lambda x: x[1]))
    print("get_max_offset_before_any_hole: sorted_payload_offset_d: ",
          sorted_payload_offset_d)

    if list(sorted_payload_offset_d.values())[0] != 0:
        return 0

    max_offset_without_hole = list(sorted_payload_offset_d.values())[0] + len(
        list(sorted_payload_offset_d.keys())[0])
    print("get_max_offset_before_any_hole: max_offset_without_hole: ",
          max_offset_without_hole)
    for payload, offset in sorted_payload_offset_d.items():
        # we got a hole
        if offset > max_offset_without_hole:
            print(
                "get_max_offset_before_any_hole: offset > max_offset_wihtout_hole: True"
            )
            print("get_max_offset_before_any_hole: max_offset_without_hole: ",
                  max_offset_without_hole)
            return max_offset_without_hole

        # we update max_offset_wihtout_hole only if current segment finishes after
        if offset + len(payload) > max_offset_without_hole:
            print(
                "get_max_offset_before_any_hole: offset + payload_length > max_offset_without_hole: True"
            )
            max_offset_without_hole = offset + len(payload)

    print("get_max_offset_before_any_hole: max_offset_without_hole: ",
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

    offset, payload = get_offset_and_payload(test_index,
                                             byte_time_sequence_json_path)

    source_port = 10000 + int(test_index)

    max_offset_before_any_hole = get_max_offset_before_any_hole(
        offset, payload)

    single_test = SingleTest(test_index, source_ip, destination_ip,
                             source_port, destination_port, scenario, offset,
                             payload, output_pcap_path,
                             max_offset_before_any_hole)
    single_test.start()
    single_test.join()


def main():
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
