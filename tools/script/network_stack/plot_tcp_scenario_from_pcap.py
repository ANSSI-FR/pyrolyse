#!/usr/bin/env python3

import sys
import os
import argparse
import matplotlib.pyplot as plt
import matplotlib
from scapy.all import IP, TCP, rdpcap 
import glob
import pathlib
import numpy as np
from enum import Enum

# TODO add icvl8i4 and icvl8i6 payload mode extraction ?

#data_chunks_color_v = ['orange', 'green', 'gray', 'cyan','olive', 
#                      'deepskyblue', 'coral', 'purple', 
#                      'deepskyblue', 'coral', 'purple',
#                      'orange', 'green', 'gray', 'cyan','olive'] # list for Novak sequence 
data_chunks_color_v = ['orange', 'green', 'gray', 'cyan','olive']
start_color = 'blue' 
end_color = 'red' 

# TCP flag
PSH = 0x08

extra_chunk_length = 8
#payload_extra_chunk_after_test_case_v = ["DDCCBBAA", "TheEnd×í", "TheEnd×å"]
protocol_factor = 8


class PacketType(Enum):
    StartChunk = 1
    TestCaseChunk = 2
    EndChunk = 3


class ScenarioPlots():
    def __init__(
        self,
        test_case_directory_scenario: str,
        output_directory: str,
        with_payload: bool
        ):

        self.test_case_directory_scenario = test_case_directory_scenario
        self.output_directory = output_directory
        self.with_payload = with_payload
        self.scenario = ''

    
    def run(self):
        print("run: start")
        print("run: self.test_case_directory_scenario: ", self.test_case_directory_scenario)
        self.scenario = os.path.basename(self.test_case_directory_scenario).split('_')[1]

        # creating output dir for that scenario
        #current_output_directory = f"{self.output_directory}/plot_{self.protocol}_{self.scenario}"
        current_output_directory = f"{self.output_directory}/plot_{self.scenario}"
        print("run: current_output_directory: ", current_output_directory)
        pathlib.Path(current_output_directory).mkdir(parents=True, exist_ok=True)

        # go throw all pcap files from test_case_directory_scenario and retrieve necessary info to build the plot
        test_case_pcap_file_v = glob.glob(f"{self.test_case_directory_scenario}/*.pcap")

        for test_case_pcap_file in test_case_pcap_file_v: 
            test_index = os.path.basename(test_case_pcap_file).split('.')[0].split('_')[1]

            seq_start_v, seq_end_v, payload_v, packet_type_v = self.retrieve_info_from_pcap(test_case_pcap_file)

            output_file = f"{current_output_directory}/test_{test_index}_{self.scenario}.pdf"
            print("run: output_file: ", output_file)
            self.build_graph(output_file,seq_start_v,seq_end_v,payload_v,packet_type_v)

        print("run: end")

    def build_graph(self, output_file,seq_start_v,seq_end_v,payload_v,packet_type_v):
        print("build_graph: start")
        t_chunk_v = [chunk_index for chunk_index in range(len(seq_start_v))]

        # chunk colors
        plt_color_v = []
        glob_color_index = 0 

        for chunk_index in range(len(seq_start_v)):
            if packet_type_v[chunk_index] == PacketType.TestCaseChunk:
                plt_color_v.append(data_chunks_color_v[glob_color_index])
                glob_color_index += 1
            elif packet_type_v[chunk_index] == PacketType.StartChunk:
                plt_color_v.append(start_color)
            elif packet_type_v[chunk_index] == PacketType.EndChunk:
                plt_color_v.append(end_color)
        
                # update initial packets fields values to ease plot  
        modified_offset_start_v = []
        modified_offset_end_v = []
        modified_payload_v = []
        modified_packet_type_v = []
        modified_t_chunk_v = []
        modified_plt_color_v = []

        for chunk_index in range(len(seq_start_v)):
            # print("build_graph: chunk_index: ", chunk_index)
            for i in range((seq_end_v[chunk_index] - seq_start_v[chunk_index])):
                # print("build_graph: i: ", i)
                modified_offset_start_v.append((seq_start_v[chunk_index] + i))
                modified_offset_end_v.append((seq_start_v[chunk_index] + i + 1))
                modified_payload_v.append(payload_v[chunk_index][i])
                modified_packet_type_v.append(packet_type_v[chunk_index])
                modified_t_chunk_v.append(t_chunk_v[chunk_index])
                modified_plt_color_v.append(plt_color_v[chunk_index])

        # print("retrieve_info_from_pcap: modified_offset_start_v: ", modified_offset_start_v)
        # print("retrieve_info_from_pcap: modified_offset_end_v: ", modified_offset_end_v)
        # print("retrieve_info_from_pcap: modified_payload_v: ", modified_payload_v)
        # print("retrieve_info_from_pcap: modified_packet_type_v: ", modified_packet_type_v)  

        #plt.figure(figsize=(12, 3.8))
        #plt.figure(figsize=(12, 3))

        plt.figure(figsize=(12, 1.7))
        plt.rcParams["font.family"] = "monospace"
        #plt.rcParams["font.monospace"] = ["FreeMono"]
        plt.rcParams['font.monospace'] = ['DejaVu Sans Mono', 'Bitstream Vera Sans Mono', 'Computer Modern Typewriter', 'Andale Mono', 'Nimbus Mono L', 'Courier New', 'Courier', 'Fixed', 'Terminal', 'monospace']

        fontsize = self.compute_font_size(len(packet_type_v))
        #fontsize = 20
        font = {'size': fontsize}
        matplotlib.rc('font', **font)

        hlines = plt.hlines(modified_t_chunk_v, modified_offset_start_v,
                modified_offset_end_v, modified_plt_color_v,
                linewidth=3)

        # adding payload  
        if self.with_payload:       
            for i in range(len(modified_t_chunk_v)):
                plt.text(modified_offset_start_v[i],
                    #modified_t_chunk_v[i] + 0.05,
                    modified_t_chunk_v[i] + 0.1,
                    modified_payload_v[i])

        # remove useless axis
        ax = plt.gca()
        ax.spines['top'].set_visible(False)
        ax.spines['right'].set_visible(False)

        # y axis
        y_ticks_label = [ f"t$_{ {i} }$" for i in range(max(modified_t_chunk_v) + 1) ]
        #y_ticks_label = [] # novak
        #for i in range(max(modified_t_chunk_v) + 1):
        #    if i % 5 == 0:
        #        y_ticks_label.append(f"t$_{ {i} }$")
        #    else:
        #        y_ticks_label.append("")
        
        y_ticks_label_all = [''] + y_ticks_label + ['']
        y_ticks_index = [ i for i in range(-1,len(y_ticks_label_all) - 1,1) ]
        #y_ticks_index = [ i for i in range(-1,(len(y_ticks_label_all) - 1) * 5,1) ]
        plt.yticks(y_ticks_index, y_ticks_label_all)

        ax = plt.gca()
        ax.set_ylim(bottom = -0.5)

        plt.ylabel("Time")

        # x axis
        plt.xlabel("Sequence number")
        ax.xaxis.set_major_locator(matplotlib.ticker.MaxNLocator(integer=True))
        
        # store plot
        plt.savefig(output_file, bbox_inches='tight')
        print("build_graph: end")

    def compute_font_size(
        self,
        chunk_number
    ):
        if chunk_number == 2:
            return 30
        elif chunk_number == 3:
            return 22
        elif chunk_number == 4:
            return 22
        elif chunk_number == 5:
            return 16
        elif chunk_number == 6:
            # Shankar model
            return 6
        else:
            # Novak model
            return 4

    
    def retrieve_info_from_pcap(self, 
                                retrieve_info_from_pcap):
        print("retrieve_info_from_pcap: start")
        packets = rdpcap(retrieve_info_from_pcap)
        seq_start_v = []
        seq_end_v = []
        payload_v = []
        packet_type_v = []

        ip_client = packets[0][IP].src
        data_packet_from_client_v = [ packet for packet in packets if packet[IP].src == ip_client and packet[TCP].flags & PSH]

        initial_sequence_number = packets[0][TCP].seq

        for data_packet in data_packet_from_client_v:
            expected_payload_ven = data_packet[IP].len - data_packet[IP].ihl * 4
            ip_payload_ven = len(bytes(data_packet[TCP]))
            extra_bytes = ip_payload_ven - expected_payload_ven
            if extra_bytes != 0:
                payload_bytes = bytes(data_packet[TCP].payload)[:-(extra_bytes)]
                print(f"extract_data_tcp: extra_bytes: {extra_bytes}")
            else:
                payload_bytes = bytes(data_packet[TCP].payload)
            print(f"extract_data_tcp: payload_bytes: {payload_bytes}")

            start = data_packet[TCP].seq - initial_sequence_number
            print(f"extract_data_tcp: start: {start}")
            end = start + len(payload_bytes)
            print(f"extract_data_tcp: end: {end}")
            seq_start_v.append(start)
            seq_end_v.append(end)

            # Extract payload
            if len(payload_bytes
                   ) != 0:
                print(
                    f"extract_data_tcp: destination port: {data_packet[TCP].dport}")
                payload_str = payload_bytes.decode('UTF8','replace')
                print(
                    f"extract_data_tcp: type(payload_str): {type(payload_str)}"
                )
                if payload_str == "0":
                    if data_packet[TCP].seq == initial_sequence_number + 1:
                        packet_type_v.append(PacketType.StartChunk)
                    else:
                        packet_type_v.append(PacketType.EndChunk)
                    payload_v.append(' ')
                else:
                    packet_type_v.append(PacketType.TestCaseChunk)
                    payload_v.append(payload_str)

        return seq_start_v, seq_end_v, payload_v, packet_type_v 


def main(argv):
    print("main: start")
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--test-case-directory-scenario", type=str, default="")
    parser.add_argument("-o", "--output-directory", type=str, default="")
    parser.add_argument("-p", "--with-payload", action=argparse.BooleanOptionalAction)
    args = parser.parse_args()

    test_case_directory_scenario = args.test_case_directory_scenario
    output_directory = args.output_directory
    with_payload = args.with_payload
    print("test_case_directory_scenario: ", test_case_directory_scenario)
    print("output_directory: ", output_directory)
    print("with_payload: ", with_payload)
    #print("protocol: ", protocol)

    scenario_plots = ScenarioPlots(test_case_directory_scenario,
                                output_directory,
                               with_payload
    )
    scenario_plots.run()

    print("main: end")


if __name__ == "__main__":
    main(sys.argv[1:])
