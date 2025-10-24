#!/usr/bin/env python3

import sys
import argparse
import matplotlib.pyplot as plt
import matplotlib
from scapy.all import *
import glob
import pathlib
import numpy as np
from enum import Enum

#data_chunks_color_v = ['orange', 'green', 'gray', 'cyan','olive', 
#                      'deepskyblue', 'coral', 'purple', 
#                      'deepskyblue', 'coral', 'purple',
#                      'orange', 'green', 'gray', 'cyan','olive'] # list for Novak sequence 
data_chunks_color_v = ['orange', 'green', 'gray', 'cyan','olive']
header_color_v = ['blue', 'deepskyblue', 'navy'] # list in case we have several headers
mf_bit_unset_color_v = ['red','coral','mistyrose'] # list in case we have several frag with mf bit unset
header_and_mf_and_data_color_v = ['purple', 'orchid', 'rebeccapurple'] # list in case we have several frag with mf bit unset

icmp_header_length = 8
payload_extra_chunk_after_test_case_v = ["DDCCBBAA", "TheEnd×í", "TheEnd×å"]
protocol_factor = 8


class PacketType(Enum):
    HeaderOnly = 1
    DataOnly = 2
    MfUnsetOnly = 3
    HeaderAndData = 4
    MfUnsetAndData = 5
    HeaderAndMfUnsetAndData = 6


class ScenarioPlots():
    def __init__(
        self,
        test_case_directory_scenario: str,
        output_directory: str,
        #protocol: str
        with_payload: bool
        ):

        self.test_case_directory_scenario = test_case_directory_scenario
        self.output_directory = output_directory
        #self.protocol = protocol
        self.with_payload = with_payload
        self.scenario = ''

    
    def run(self):
        print("run: start")
        print("run: self.test_case_directory_scenario: ", self.test_case_directory_scenario)
        self.scenario = os.path.basename(self.test_case_directory_scenario).split('_')[2]

        # creating output dir for that scenario
        #current_output_directory = f"{self.output_directory}/plot_{self.protocol}_{self.scenario}"
        current_output_directory = f"{self.output_directory}/plot_{self.scenario}"
        print("run: current_output_directory: ", current_output_directory)
        pathlib.Path(current_output_directory).mkdir(parents=True, exist_ok=True)

        # go throw all pcap files from test_case_directory_scenario and retrieve necessary info to build the plot
        test_case_pcap_file_v = glob.glob(f"{self.test_case_directory_scenario}/*.pcap")

        for test_case_pcap_file in test_case_pcap_file_v: 
            test_index = os.path.basename(test_case_pcap_file).split('.')[0].split('_')[1]

            offset_start_v, offset_end_v, payload_v, packet_type_v = self.retrieve_info_from_pcap(test_case_pcap_file)

            output_file = f"{current_output_directory}/test_{test_index}_{self.scenario}.pdf"
            print("run: output_file: ", output_file)
            self.build_graph(output_file,offset_start_v,offset_end_v,payload_v,packet_type_v)

        print("run: end")

    def build_graph(self, output_file,offset_start_v,offset_end_v,payload_v,packet_type_v):
        print("build_graph: start")
        # protocol_factor = 1 if protocol == "ip" else 8
        #protocol_factor = 8

        t_chunk_v = [chunk_index for chunk_index in range(len(offset_start_v))]

        # chunk colors
        plt_color = []
        glob_color_index, h_color_index, mf_color_index, h_and_mf_color_index = 0, 0, 0, 0 

        for chunk_index in range(len(offset_start_v)):
            if packet_type_v[chunk_index] == PacketType.DataOnly:
                plt_color.append(data_chunks_color_v[glob_color_index])
                glob_color_index += 1
            elif packet_type_v[chunk_index] == PacketType.HeaderOnly or packet_type_v[chunk_index] == PacketType.HeaderAndData:
                plt_color.append(header_color_v[h_color_index])
                h_color_index += 1
            elif packet_type_v[chunk_index] == PacketType.MfUnsetOnly or packet_type_v[chunk_index] == PacketType.MfUnsetAndData:
                plt_color.append(mf_bit_unset_color_v[mf_color_index])
                mf_color_index += 1
            elif packet_type_v[chunk_index] == PacketType.HeaderAndMfUnsetAndData:
                plt_color.append(header_and_mf_and_data_color_v[h_and_mf_color_index])
                h_and_mf_color_index += 1
        
        # update initial packets fields values to ease plot  
        modified_offset_start_v = []
        modified_offset_end_v = []
        modified_payload_v = []
        modified_packet_type_v = []
        modified_t_chunk_v = []
        modified_plt_color_v = []

        for chunk_index in range(len(offset_start_v)):
            # print("build_graph: chunk_index: ", chunk_index)
            for i in range((offset_end_v[chunk_index] - offset_start_v[chunk_index]) * 8):
                # print("build_graph: i: ", i)
                modified_offset_start_v.append((offset_start_v[chunk_index] * 8 + i) / protocol_factor )
                modified_offset_end_v.append((offset_start_v[chunk_index] * 8 + i + 1) / protocol_factor)
                modified_payload_v.append(payload_v[chunk_index][i])
                modified_packet_type_v.append(packet_type_v[chunk_index])
                modified_t_chunk_v.append(t_chunk_v[chunk_index])
                modified_plt_color_v.append(plt_color[chunk_index])

        # print("retrieve_info_from_pcap: modified_offset_start_v: ", modified_offset_start_v)
        # print("retrieve_info_from_pcap: modified_offset_end_v: ", modified_offset_end_v)
        # print("retrieve_info_from_pcap: modified_payload_v: ", modified_payload_v)
        # print("retrieve_info_from_pcap: modified_packet_type_v: ", modified_packet_type_v)  

        #plt.figure(figsize=(12, 3.8))
        #plt.figure(figsize=(12, 3))

        plt.figure(figsize=(12, 1.7))
        #font.monospace: ['DejaVu Sans Mono', 'Bitstream Vera Sans Mono', 'Computer Modern Typewriter', 'Andale Mono', 'Nimbus Mono L', 'Courier New', 'Courier', 'Fixed', 'Terminal', 'monospace']
        plt.rcParams["font.family"] = "monospace"
        #print("plt.rcParams['font.monospace'] = ",plt.rcParams["font.monospace"])
        plt.rcParams['font.monospace'] = ['DejaVu Sans Mono', 'Bitstream Vera Sans Mono', 'Computer Modern Typewriter', 'Andale Mono', 'Nimbus Mono L', 'Courier New', 'Courier', 'Fixed', 'Terminal', 'monospace']

        #fontsize = self.compute_font_size(len(packet_type_v))
        fontsize = self.compute_font_size(len(packet_type_v),max(modified_offset_end_v))
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
        plt.xlabel("Byte offset")
        ax.xaxis.set_major_locator(matplotlib.ticker.MaxNLocator(integer=True))
        
        # store plot
        plt.savefig(output_file, bbox_inches='tight')
        print("build_graph: end")

    #def compute_font_size(
    #    self,
    #    chunk_number
    #):
    #    if chunk_number == 2:
    #        return 30
    #    elif chunk_number == 3:
    #        return 22
    #    elif chunk_number == 4:
    #        return 22
    #    elif chunk_number == 5:
    #        return 16
    #    elif chunk_number == 6:
    #        # Shankar model
    #        return 6
    #    else:
    #        # Novak model
    #        return 4
    
    def compute_font_size(
        self,
        chunk_number,
        ending_offset
    ):
        if chunk_number == 2:
            return 32 - 2*ending_offset
        elif chunk_number == 3:
            return 26 - 2*ending_offset
        elif chunk_number == 4:
            return 26 - 2*ending_offset
        elif chunk_number == 5:
            return 21 - 2*ending_offset
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
        offset_start_v = []
        offset_end_v = []
        payload_v = []
        packet_type_v = []

        for packet in packets:

            len_ip_header = (packet[IP].ihl * 4)
            len_ip_packet = packet[IP].len
            len_payload_ip = len_ip_packet - len_ip_header
            payload_offset_start = packet[IP].frag
            payload_offset_end = payload_offset_start + (len_payload_ip // 8)

            offset_start_v.append(packet[IP].frag)
            offset_end_v.append(payload_offset_end)

            if packet[IP].frag == 0:
                # packet contains ICMP header
                if len_payload_ip == 8:
                    # packet contains only ICMP header
                    payload_v.append(' ' * icmp_header_length)
                    packet_type_v.append(PacketType.HeaderOnly)
                else:
                    payload_v.append(str(' ' * icmp_header_length + packet[ICMP].payload.load.decode('UTF8','replace')))
                    if packet[IP].flags == 0x00:
                        # packet contains header and has MF bit unset
                        packet_type_v.append(PacketType.HeaderAndMfUnsetAndData)
                    else:
                        packet_type_v.append(PacketType.HeaderAndData)
            elif packet[IP].payload.load in payload_extra_chunk_after_test_case_v:
                # replace extra chunk sent after with empty payload
                payload_v.append(' ' * protocol_factor)
                packet_type_v.append(PacketType.MfUnsetOnly)
            else:
                payload_v.append(packet[IP].payload.load.decode('UTF8','replace'))
                if packet[IP].flags == 0x00:
                    packet_type_v.append(PacketType.MfUnsetAndData)
                else:
                    packet_type_v.append(PacketType.DataOnly)

        # print("retrieve_info_from_pcap: offset_start_v: ", offset_start_v)
        # print("retrieve_info_from_pcap: offset_end_v: ", offset_end_v)
        # print("retrieve_info_from_pcap: payload_v: ", payload_v)
        # print("retrieve_info_from_pcap: packet_type_v: ", packet_type_v)
        # print("retrieve_info_from_pcap: end")

        return offset_start_v, offset_end_v, payload_v, packet_type_v 


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
