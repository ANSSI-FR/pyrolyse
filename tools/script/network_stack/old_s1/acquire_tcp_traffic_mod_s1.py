#!/usr/bin/python3
import json
import sys
import subprocess
import time
import argparse
# from scapy.all import *
from threading import Thread, Event
from scapy.all import IP,TCP,sr1, send, sniff
# from collections import OrderedDict


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

    def __init__(self,
                 destination_ip: str,
                 source_port: int,
                 rstack: int,
                 myack: int,
                 scenario: str,
                 interface: str="eth1"):
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
        self.rstack = rstack
        #self.rstseq = rstseq
        self.index = 0
        self.scenario = scenario
        self.last_echo_reply = None
        self.myack = myack

    def join(self, timeout=1):
        """
        Join on this thread.
        """
        super().join(timeout)

    def run(self):
        """
        Wait for data and sends TCP ACKs.
        """
        print('Sniffer: run: start')
        if self.scenario == "pep1":
            sniff(iface=self.interface,
                  filter=f"ip and host {self.destination_ip}",
                  prn=self.keep_echo_reply_with_max_ack,
                  stop_filter=lambda x: self.stop_sniffer_pep1.is_set())
            self.send_ack(self.last_echo_reply)

        # For pep1, we want to ACK all the P+A from Server after having sent the testing sequence
        sniff(iface=self.interface,
              filter=f"ip and host {self.destination_ip}",
              prn=self.send_ack,
              stop_filter=lambda x: self.stop_sniffer.is_set())
        print('Sniffer: run: end')

    def keep_echo_reply_with_max_ack(self, received_packet):
        """
        Update the last observed ACK from the destination (i.e. TCB.RCV_NXT).

        Parameters
        ----------
            received_packet : Scapy packet
                received packet
        """
        print('Sniffer: keep_echo_reply_with_max_ack: start')

        print(
            'Sniffer: keep_echo_reply_with_max_ack: packet["IP"].src == self.destination_ip: ',
            received_packet["IP"].src == self.destination_ip)
        print(
            'Sniffer: keep_echo_reply_with_max_ack: packet["TCP"].dport == self.source_port: ',
            received_packet["TCP"].dport == self.source_port)
        print('Sniffer: keep_echo_reply_with_max_ack: packet["TCP"].ack: ',
              received_packet["TCP"].ack)
        print('Sniffer: keep_echo_reply_with_max_ack: self.myack: ',
              self.myack)

        if received_packet["IP"].src == self.destination_ip and received_packet.haslayer(
                "TCP"
        ) and received_packet["TCP"].dport == self.source_port and received_packet[
                "TCP"].flags == 0x18 and received_packet["TCP"].ack >= self.myack:
            self.myack = received_packet["TCP"].ack
            self.last_echo_reply = received_packet
            #self.index += 1
            print('Sniffer: keep_echo_reply_with_max_ack: myack: ', self.myack)

        print('Sniffer: keep_echo_reply_with_max_ack: end')

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
        print('Sniffer: send_ack: start')

        if received_packet is None:
            return

        ip_layer = received_packet.getlayer("IP")
        if b'\x00' in bytes(received_packet["TCP"].payload):
            bytes_data = bytes(received_packet["TCP"].payload).replace(b'\x00', b'')
        else:
            bytes_data = received_packet["TCP"].payload

        print('Sniffer: send_ack: ip_layer.src: ', ip_layer.src)
        print('Sniffer: send_ack: self.destination_ip: ', self.destination_ip)
        print('Sniffer: send_ack: packet["TCP"].dport: ',
              received_packet["TCP"].dport)
        print('Sniffer: send_ack: self.source_port: ', self.source_port)
        print('Sniffer: send_ack: packet["TCP"].flags == 0x18: ',
              received_packet["TCP"].flags == 0x18)

        print('Sniffer: send_ack: self.index: ', self.index)
        print('Sniffer: send_ack: self.rstack: ',
              self.rstack)

        if ip_layer.src == self.destination_ip and received_packet[
                "TCP"].dport == self.source_port and received_packet[
                    "TCP"].flags == 0x18:
            print(received_packet["TCP"].payload.load.decode("utf-8"))
            self.rstack[
                self.index] = received_packet["TCP"].seq + len(bytes_data)
            #self.rstseq[self.index] = packet["TCP"].ack
            ip_header = IP(src=ip_layer.dst, dst=ip_layer.src)
            tcp_header = TCP(ack=received_packet["TCP"].seq + len(bytes_data),
                      dport=received_packet["TCP"].sport,
                      sport=self.source_port,
                      flags="A",
                      seq=received_packet["TCP"].ack)
            self.myack = received_packet["TCP"].seq + len(bytes_data)
            ack_packet = ip_header / tcp_header
            send(ack_packet)
            self.index += 1
            print('Sniffer: ack sent')

        print('Sniffer: send_ack: end')


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

    def __init__(self, test_index, source_ip, destination_ip, source_port, destination_port, scenario, offset, payload,
                 output_pcap_path, max_offset_before_any_hole):
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
        self.tcb_tcp_seq = 0
        self.myack = 0
        #self.rstseq = [0] * len(offset)
        self.max_offset_before_any_hole = max_offset_before_any_hole + 1 if scenario == 'peos' else max_offset_before_any_hole
        # We use 2*len(offset) because some target send both ACK and echo back to the base.
        self.rstack = [0] * 2 * len(offset)

    def join(self, timeout=1):
        """
        Join on this thread.
        """
        super().join(timeout)

    def run(self):
        """
        Run the test and capture the traffic.
        """
        print('SingleTest: run: start')

        print(
            f"[*] Start capturing traffic for test of index {self.test_index}")
        p = subprocess.Popen([
            "tcpdump", "-U", "-i", "eth1", "-w", self.output_pcap_path, "-nn",
            f"host {self.destination_ip} and port {self.source_port}"
        ],
                             stdout=subprocess.PIPE)

        time.sleep(1)
        self.send_message()
        time.sleep(2)
        self.send_rst()
        print("Sent RST")
        time.sleep(2)
        p.terminate()

        print(f"[*] End capturing traffic for test of index {self.test_index}")
        print('SingleTest: run: end')

    def send_message(self):
        """
        Established the TCP connection and call the function for the test scenario.
        """
        tcp_intial_sequence_number = 10
        tcp = TCP(dport=self.destination_port, sport=self.source_port, flags="S", seq=tcp_intial_sequence_number)
        syn = self.ip_header / tcp
        synack = sr1(syn)

        # In case scapy fail to capture server's reply, we want to reset the communication as properly as possible (i.e. sequence number of RST needs to be in window).
        #self.rstseq[0] = synack["TCP"].ack

        # TODO: add check on SA flag

        time.sleep(0.5)
        self.myack = synack.seq + 1
        self.tcb_tcp_seq = tcp_intial_sequence_number + 1
        tcp_header = TCP(ack=self.myack,
                  dport=self.destination_port,
                  sport=self.source_port,
                  flags="A",
                  seq=self.tcb_tcp_seq)
        ack_packet = self.ip_header / tcp
        send(ack_packet)

        if self.scenario == "peos":
            self.send_peos_sequence()
        elif self.scenario == "pep1":
            self.send_pep1_sequence()
        elif self.scenario == "pep2":
            self.send_pep2_sequence()
        else:
            print("The scenario doesn't exist")
            sys.exit(-1)

    def send_pep2_sequence(self):
        """
        Send the test sequence with the PEP2 scenario.
        """
        for i, offset in enumerate(self.offset):
            # Send payload
            tcp = TCP(ack=self.myack, dport=self.destination_port, sport=self.source_port, flags="PA")
            tcp.seq = self.tcb_tcp_seq + self.offset[i]
            pack = self.ip_header / tcp / self.payload[i]

            # Sniff echo packet to send ACK
            sniffer = Sniffer(self.destination_ip, self.source_port, self.rstack,
                              self.myack, self.scenario)
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
        """
        Send the test sequence with the PEP1 scenario.
        """
        # Sniff echo reply packets
        sniffer = Sniffer(self.destination_ip, self.source_port, self.rstack, 0,
                          self.scenario)
        print("[*] Start sniffing...")
        sniffer.start()

        for i, offset in enumerate(self.offset):
            # Send payload
            time.sleep(0.2)
            tcp = TCP(ack=self.myack, dport=self.destination_port, sport=self.source_port, flags="PA")
            tcp.seq = self.tcb_tcp_seq + self.offset[i]
            pack = self.ip_header / tcp / self.payload[i]
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
        """
        Send the test sequence with the PEOS scenario.
        """
        for i, offset in enumerate(self.offset):
            # Send payload
            time.sleep(0.2)
            tcp = TCP(ack=self.myack, dport=self.destination_port, sport=self.source_port, flags="PA")
            tcp.seq = self.tcb_tcp_seq + self.offset[i] + 1
            pack = self.ip_header / tcp / self.payload[i]
            send(pack)

        # Sniff echo packet after sending all segments
        sniffer = Sniffer(self.destination_ip, self.source_port, self.rstack,
                          self.myack, self.scenario)
        print("[*] Start sniffing...")
        sniffer.start()
        time.sleep(0.5)
        tcp.seq = self.tcb_tcp_seq
        seg0 = "0"
        pack0 = self.ip_header / tcp / seg0
        send(pack0)
        time.sleep(2)
        print("[*] Stop sniffing")
        sniffer.stop_sniffer.set()
        sniffer.join()
        print("[*] After SingleTest.join()")
        self.myack = sniffer.myack

    def send_rst(self):
        """
        Sends a RST.
        """
        print("send_rst: self.tcb_tcp_seq + self.max_offset_before_any_hole: ",
              self.tcb_tcp_seq + self.max_offset_before_any_hole)
        print("send_rst: max(self.rstack): ",
              max(self.rstack))
        rst_packet = self.ip_header / TCP(
            sport=self.source_port,
            dport=self.destination_port,
            flags="RA",
            seq=self.tcb_tcp_seq + self.max_offset_before_any_hole,
            ack=max(self.rstack))
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
    with open(byte_time_sequence_json_path) as file:
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
    offset_l = [p_info['offset'] for i, p_info in test_descr['chunk_c']['bm'].items()]

    multiplied_offset_l = [element * 8 for element in offset_l]

    # Get the payload
    # payload_l = []
    # for _, s_info in test_descr['chunk_c']['bm'].items():
    #     temp = s_info['internet_checksum_s']
    #     payload_l.append(temp)
    payload_l = [s_info['internet_checksum_s'] for i, s_info in test_descr['chunk_c']['bm'].items()]

    # Keep offset/payload in line with temporary position
    temp_pos = test_descr['temporal_position_v']
    # index_l = []
    # for _, i_info in test_descr['chunk_c']['bm'].items():
    #     temp2 = i_info['index']
    #     index_l.append(temp2)
    index_l = [i_info['index'] for i, i_info in test_descr['chunk_c']['bm'].items()]

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
    print("check_hole_in_test_case: payload_offset_d: ", payload_offset_d)
    sorted_payload_offset_d = dict(
        sorted(payload_offset_d.items(), key=lambda x: x[1]))
    print("check_hole_in_test_case: sorted_payload_offset_d: ",
          sorted_payload_offset_d)

    if list(sorted_payload_offset_d.values())[0] != 0:
        return 0

    max_offset_without_hole = list(sorted_payload_offset_d.values())[0] + len(
        list(sorted_payload_offset_d.keys())[0])
    print("check_hole_in_test_case: max_offset_without_hole: ",
          max_offset_without_hole)
    for payload, offset in sorted_payload_offset_d.items():
        # we got a hole
        if offset > max_offset_without_hole:
            print(
                "check_hole_in_test_case: offset > max_offset_wihtout_hole: True"
            )
            print("check_hole_in_test_case: max_offset_without_hole: ",
                  max_offset_without_hole)
            return max_offset_without_hole

        # we update max_offset_wihtout_hole only if current segment finishes after
        if offset + len(payload) > max_offset_without_hole:
            print(
                "check_hole_in_test_case: offset + payload_length > max_offset_without_hole: True"
            )
            max_offset_without_hole = offset + len(payload)

    print("check_hole_in_test_case: max_offset_without_hole: ",
          max_offset_without_hole)
    return max_offset_without_hole


def process(test_index, source_ip, destination_ip, destination_port, byte_time_sequence_json_path,
            output_pcap_path, scenario):
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

    single_test = SingleTest(test_index, source_ip, destination_ip, source_port, destination_port, scenario, offset,
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

    process(test_index, source_ip, destination_ip, destination_port, byte_time_sequence_json_path,
            output_pcap_path, scenario)


if __name__ == "__main__":
    main()
