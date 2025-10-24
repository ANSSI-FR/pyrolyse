#!/usr/bin/python

# This script retrieve some meta information on icmp OS policies 
# from file ip_fragmentation_icmp_pexxx_payload.json

import sys
import os
import json
import argparse
import glob
import itertools
import hashlib
from math import comb
import numpy as np 
import matplotlib.pyplot as plt 
import matplotlib.ticker as mtick
from sklearn.feature_extraction.text import TfidfVectorizer

allen_relations = [
    "M", "Mi", "D", "Di", "S", "Si", "F", "Fi", "Eq", "B", "Bi", "O", "Oi"
]

def get_scenarii_name_payload(json_path):
    return json_path.split('_')[4]

def get_OS_name_payload(json_path):
    return json_path.split('/')[1]

def get_OS_name_meta_info(json_path):
    print(json_path)
    return json_path.split('/')[1]

def calculate_exact_similarities_between_lists(l1,l2):
    res = 0
    for m1, m2 in zip(l1,l2):
        if m1 == m2:
            res += 1   

    return res/len(l1) 

# compare the payload between scenario of OSes
def compare_OSes_payload(os_directories_payload_files):
    scenarios = [get_scenarii_name_payload(scenario) for scenario in list(os_directories_payload_files.values())[0]] 
    similarities = {}
    for i,scenario in enumerate(scenarios):
        payload_files = [list(os_directories_payload_files.values())[j][i] for j,os in enumerate(os_directories_payload_files.keys())]
        similarities_per_scenario = []

        # get combinations 
        combinations = itertools.combinations(payload_files,2)

        for combination in combinations: 
            json_contents = [json.load(open(f)) for f in combination]
            json_content = []

            for i, content in enumerate(json_contents):
                temporary = []
                for key,value in content['hm'].items():
                    if value['payload'] != '':
                        temporary.append(value['payload'])
                    else:
                        temporary.append('/')
                json_content.append(temporary)

            similarities_per_scenario.append(calculate_exact_similarities_between_lists(json_content[0], json_content[1]))

        similarities.update({scenario:similarities_per_scenario})

    return similarities

# compare test IDs between scenario of OSes
def compare_OSes_tests(os_directories_meta_info_files):
    # get json content
    json_content = {os:json.load(open(meta_file[0])) for os,meta_file in os_directories_meta_info_files.items()} 

    scenarios = [scenario for scenario in json_content[next(iter(json_content))].keys()] 
    
    similarities = {}

    fields_to_compare = ['test_for_which_we_get_a_response', 'test_for_which_we_get_a_response_with_at_east_one_overlap', 'test_for_which_we_get_a_response_without_any_overlap']

    field_similarities = {}

    for field in fields_to_compare:
        #print(field)
        similarities = {}

        for i,scenario in enumerate(scenarios):
            similarities_per_scenario = []

            # get combinations 
            combinations = itertools.combinations(os_directories_meta_info_files.keys(),2)

            for combination in combinations: 
                concatenate_test_for_which_we_get_a_response = json_content[combination[0]][scenario][field] + json_content[combination[1]][scenario][field]

                similarities_test_for_which_we_get_a_response = [test for test in concatenate_test_for_which_we_get_a_response if test in json_content[combination[0]][scenario][field] and test in json_content[combination[1]][scenario][field]]

                similarity = round(len(similarities_test_for_which_we_get_a_response) / len(concatenate_test_for_which_we_get_a_response),2)

                similarities_per_scenario.append(similarity)
            
            similarities.update({scenario:similarities_per_scenario})
                
        field_similarities.update({field:similarities})
    
    return field_similarities

def print_similarities(similarities, os_directories):
    for scenarii in similarities.keys():
            matrix_results = []
            j = 0
            current_index = 0

            for j,os in enumerate(os_directories):
                added_cell = 0
                while added_cell < j+1:
                    if j == added_cell:
                        matrix_results.append('1.00')
                    else:
                        matrix_results.append('.....')
                    added_cell += 1

                while current_index + 1 <= len(similarities.get(scenarii)) and added_cell < (len(os_directories)):
                    matrix_results.append(round(similarities.get(scenarii)[current_index],3))
                    current_index += 1
                    added_cell += 1

            shape = ( len(os_directories), len(os_directories) )  
            print('+++ ' + scenarii)      
            print(np.array(matrix_results).reshape(shape))
            print('\n')

def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-d", "--os_directories", nargs='+', default=[])
    args = parser.parse_args()

    # Similarities of OSes' reassembly policies - per scenario
    os_directories = args.os_directories
    os_directories_payload_files = {}
    
    for os in os_directories:
        os_directories_payload_files.update({os:glob.glob(f"../{os}/ip_fragmentation_icmp_*_payload.json")})

    # compare 2 à 2
    if len(os_directories_payload_files.keys()) > 1:
        similarities = compare_OSes_payload(os_directories_payload_files)

    print(f"OS comparison : \n \t\t{os_directories}")
    for os in os_directories:
        print(f'{os}')
    print('\n\n')
    
    print(f"- Payload comparison : \n")
    print_similarities(similarities, os_directories)
    
    # Similarities of OSes' tests IDs - per scenario
    os_directories_meta_info_json_files = {}
    
    for os in os_directories:
        os_directories_meta_info_json_files.update({os:glob.glob(f"../{os}/meta_info.json")})

    # compare 2 à 2
    if len(os_directories_meta_info_json_files.keys()) > 1:
        field_similarities = compare_OSes_tests(os_directories_meta_info_json_files)
    
    for k,similarities in field_similarities.items():
        print(f"-- {k} : \n")
        print_similarities(similarities, os_directories)
        
    
if __name__ == "__main__":
    main(sys.argv[1:])
