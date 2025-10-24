#!/usr/bin/python

# This script retrieve some meta information on icmp OS policies 
# from files ip_fragmentation_pexxx_policy_complicated.json and
# ip_fragmentation_icmp_pexxx_payload.json

import sys
import os
from scapy.all import IP,ICMP,rdpcap,PacketList,bytes_hex
import json
import argparse
import glob
import itertools
import hashlib
from math import comb
import numpy as np 
import matplotlib.pyplot as plt 
import matplotlib.ticker as mtick

allen_relations = [
    "M", "Mi", "D", "Di", "S", "Si", "F", "Fi", "Eq", "B", "Bi", "O", "Oi"
]

non_overlapping_allen_relations = [
    "M", "Mi", "B", "Bi"
]


def get_scenarii_name_complicated(json_path):
    return json_path.split('_')[3]

def get_scenarii_name_payload(json_path):
    return json_path.split('_')[4]

def get_complicated_path_from_scenario_name(scenario, complicated_files):
    for complicated_file in complicated_files:
        if get_scenarii_name_complicated(complicated_file) == scenario:
            return complicated_file

def get_payload_path_from_scenario_name(scenario, payload_files):
    for payload_file in payload_files:
        if get_scenarii_name_payload(payload_file) == scenario:
            return payload_file 

def get_meta_info_on_consistent_pair_position_policy(triplets_tests):
    consistency_pair_policies = {} 

    for test_number in triplets_tests.keys():
        policy_01 = triplets_tests[test_number]['consistent_pair_position_policy_01_first_second']
        policy_02 = triplets_tests[test_number]['consistent_pair_position_policy_02_first_second']
        policy_12 = triplets_tests[test_number]['consistent_pair_position_policy_12_first_second']

        consistency_pair_policies[policy_01] = consistency_pair_policies.get(policy_01, 0) + 1
        consistency_pair_policies[policy_02] = consistency_pair_policies.get(policy_02, 0) + 1
        consistency_pair_policies[policy_12] = consistency_pair_policies.get(policy_12, 0) + 1

    return consistency_pair_policies


def get_info_on_relation_between_inconsistencies_and_icmpEchoReply(triplets_tests, payload):
    consistency_pair_policies_per_relation = {} 
    consistency_pair_policies_per_test = {} 

    for test_number in triplets_tests.keys():
        policy_01 = triplets_tests[test_number]['consistent_pair_position_policy_01_first_second']
        policy_02 = triplets_tests[test_number]['consistent_pair_position_policy_02_first_second']
        policy_12 = triplets_tests[test_number]['consistent_pair_position_policy_12_first_second']
    
        at_least_one_not_consistent_with_reply = False
        all_relation_consistent = True

        if payload['hm'][test_number]['is_echo_reply']:
            if policy_01 == "NotConsistent" or policy_01 == "NotConsistentPeoLike":
                consistency_pair_policies_per_relation['NotConsistentWithReply'] = consistency_pair_policies_per_relation.get('NotConsistentWithReply', 0) + 1
                at_least_one_not_consistent_with_reply = True
                all_relation_consistent = False
            if policy_02 == "NotConsistent" or policy_02 == "NotConsistentPeoLike":
                consistency_pair_policies_per_relation['NotConsistentWithReply'] = consistency_pair_policies_per_relation.get('NotConsistentWithReply', 0) + 1
                at_least_one_not_consistent_with_reply = True
                all_relation_consistent = False
            if policy_12 == "NotConsistent" or policy_12 == "NotConsistentPeoLike":
                consistency_pair_policies_per_relation['NotConsistentWithReply'] = consistency_pair_policies_per_relation.get('NotConsistentWithReply', 0) + 1
                at_least_one_not_consistent_with_reply = True
                all_relation_consistent = False

            if at_least_one_not_consistent_with_reply:
                consistency_pair_policies_per_test['number_of_tests_NotConsistentWithReply'] = consistency_pair_policies_per_test.get('number_of_tests_NotConsistentWithReply', 0) + 1

            if all_relation_consistent:
                consistency_pair_policies_per_test['Test_fully_consistent'] = consistency_pair_policies_per_test.get('Test_fully_consistent', 0) + 1
        
    return consistency_pair_policies_per_relation, consistency_pair_policies_per_test

def get_info_on_relation_between_inconsistencies_and_icmpEchoReply_v2(triplets_tests, payload):
    consistency_pair_policies_per_relation = {} 
    consistency_pair_policies_per_test = {} 

    for test_number in triplets_tests.keys():
        policy_01 = triplets_tests[test_number]['consistent_pair_position_policy_01_first_second']
        policy_02 = triplets_tests[test_number]['consistent_pair_position_policy_02_first_second']
        policy_12 = triplets_tests[test_number]['consistent_pair_position_policy_12_first_second']
    
        at_least_one_not_consistent_with_reply = False
        all_relation_consistent = True

        if payload['hm'][test_number]['is_echo_reply']:
            if policy_01 == "NotConsistent" or policy_01 == "NotConsistentPeoLike":
                consistency_pair_policies_per_relation['NotConsistentWithReply'] = consistency_pair_policies_per_relation.get('NotConsistentWithReply', 0) + 1
                at_least_one_not_consistent_with_reply = True
                all_relation_consistent = False
            if policy_02 == "NotConsistent" or policy_02 == "NotConsistentPeoLike":
                consistency_pair_policies_per_relation['NotConsistentWithReply'] = consistency_pair_policies_per_relation.get('NotConsistentWithReply', 0) + 1
                at_least_one_not_consistent_with_reply = True
                all_relation_consistent = False
            if policy_12 == "NotConsistent" or policy_12 == "NotConsistentPeoLike":
                consistency_pair_policies_per_relation['NotConsistentWithReply'] = consistency_pair_policies_per_relation.get('NotConsistentWithReply', 0) + 1
                at_least_one_not_consistent_with_reply = True
                all_relation_consistent = False

            if at_least_one_not_consistent_with_reply:
                consistency_pair_policies_per_test['number_of_tests_NotConsistentWithReply'] = consistency_pair_policies_per_test.get('number_of_tests_NotConsistentWithReply', 0) + 1

            if all_relation_consistent:
                consistency_pair_policies_per_test['Test_fully_consistent'] = consistency_pair_policies_per_test.get('Test_fully_consistent', 0) + 1
        
    return consistency_pair_policies_per_relation, consistency_pair_policies_per_test



def get_consistency_and_no_consistency_per_relation(triplets_tests, payload):
    current_qualitative_result_not_consistent = {allen_relation: 0 for allen_relation in allen_relations}
    current_qualitative_result_not_consistent_with_reply = {allen_relation: 0 for allen_relation in allen_relations}
    current_qualitative_result_consistent = {allen_relation: 0 for allen_relation in allen_relations}
    current_qualitative_result_consistent_with_reply = {allen_relation: 0 for allen_relation in allen_relations}
    
    for test_number in triplets_tests.keys():

        policy_01 = triplets_tests[test_number]['consistent_pair_position_policy_01_first_second']
        policy_02 = triplets_tests[test_number]['consistent_pair_position_policy_02_first_second']
        policy_12 = triplets_tests[test_number]['consistent_pair_position_policy_12_first_second']

        if 'triplet_position_policy_012' in triplets_tests[test_number] and 'relation_triplet' in triplets_tests[test_number]['triplet_position_policy_012']:
            relation_01 = triplets_tests[test_number]['triplet_position_policy_012']['relation_triplet']['relation_01']
            relation_02 = triplets_tests[test_number]['triplet_position_policy_012']['relation_triplet']['relation_02']
            relation_12 = triplets_tests[test_number]['triplet_position_policy_012']['relation_triplet']['relation_12']

            if payload['hm'][test_number]['is_echo_reply']:
                is_echo_reply = True
            else:
                is_echo_reply = False

            if policy_01 == "NotConsistent" or policy_01 == "NotConsistentPeoLike":
                current_qualitative_result_not_consistent[relation_01] = current_qualitative_result_not_consistent.get(relation_01, 0) + 1
                if is_echo_reply:
                    current_qualitative_result_not_consistent_with_reply[relation_01] = current_qualitative_result_not_consistent_with_reply.get(relation_01, 0) + 1
            elif policy_01 == "Consistent":
                current_qualitative_result_consistent[relation_01] = current_qualitative_result_consistent.get(relation_01, 0) + 1
                if is_echo_reply:
                    current_qualitative_result_consistent_with_reply[relation_01] = current_qualitative_result_consistent_with_reply.get(relation_01, 0) + 1

            if policy_02 == "NotConsistent" or policy_02 == "NotConsistentPeoLike":
                current_qualitative_result_not_consistent[relation_02] = current_qualitative_result_not_consistent.get(relation_02, 0) + 1
                if payload['hm'][test_number]['is_echo_reply']:
                    current_qualitative_result_not_consistent_with_reply[relation_02] = current_qualitative_result_not_consistent_with_reply.get(relation_02, 0) + 1
            elif policy_02 == "Consistent":
                current_qualitative_result_consistent[relation_02] = current_qualitative_result_consistent.get(relation_02, 0) + 1
                if is_echo_reply:
                    current_qualitative_result_consistent_with_reply[relation_02] = current_qualitative_result_consistent_with_reply.get(relation_02, 0) + 1

            if policy_12 == "NotConsistent" or policy_02 == "NotConsistentPeoLike":
                current_qualitative_result_not_consistent[relation_12] = current_qualitative_result_not_consistent.get(relation_12, 0) + 1
                if payload['hm'][test_number]['is_echo_reply']:
                    current_qualitative_result_not_consistent_with_reply[relation_12] = current_qualitative_result_not_consistent_with_reply.get(relation_12, 0) + 1
            elif policy_12 == "Consistent":
                current_qualitative_result_consistent[relation_12] = current_qualitative_result_consistent.get(relation_12, 0) + 1
                if is_echo_reply:
                    current_qualitative_result_consistent_with_reply[relation_12] = current_qualitative_result_consistent_with_reply.get(relation_12, 0) + 1
    return current_qualitative_result_consistent, current_qualitative_result_consistent_with_reply, current_qualitative_result_not_consistent, current_qualitative_result_not_consistent_with_reply

def get_info_on_relation_position_and_icmpEchoReply(triplets_tests, tests_with_ICMP_echo_reply):
    relations_01 = {allen_relation: 0 for allen_relation in allen_relations}
    relations_02 = {allen_relation: 0 for allen_relation in allen_relations}
    relations_12 = {allen_relation: 0 for allen_relation in allen_relations}

    for test_number in tests_with_ICMP_echo_reply:
        # we only want triplets
        if int(test_number) >= 100: 
            relation_01 = triplets_tests[test_number]['triplet_position_policy_012']['relation_triplet']['relation_01']
            relation_02 = triplets_tests[test_number]['triplet_position_policy_012']['relation_triplet']['relation_02']
            relation_12 = triplets_tests[test_number]['triplet_position_policy_012']['relation_triplet']['relation_12']
    
            relations_01[relation_01] = relations_01.get(relation_01, 0) + 1
            relations_02[relation_02] = relations_02.get(relation_02, 0) + 1
            relations_12[relation_12] = relations_12.get(relation_12, 0) + 1
    
    return relations_01, relations_02, relations_12

def get_tests_with_ICMP_echo_reply(payload):
    tests = []
    for test_number in payload['hm'].keys():
        if payload['hm'][test_number]['is_echo_reply']:
            tests.append(test_number)
    return tests

def get_relation_consistency_with_isolated_pair(json_complicated_files, json_payload_files):
    result_per_scenarii = {
        "pep" : {},
        "peos" : {},
        "peoe" : {},
        "peose" : {},
        "peoes" : {}
    }

    for json_path in json_complicated_files:
        scenarii_str = get_scenarii_name_complicated(json_path)

        current_qualitative_result_consistency_percentages = {allen_relation: '/' for allen_relation in allen_relations}

        with open(json_path, 'r') as json_file:
            data = json.load(json_file)
        triplets_tests = data['triplet_position_policy_data_c']['hm']

        with open(get_payload_path_from_scenario_name(get_scenarii_name_complicated(json_path),json_payload_files), 'r') as json_file:
            payload = json.load(json_file)

        # get meta info on consistent_pair_position_policy field from complicated files
        current_quantitative_result_per_relation_without_ICMP_EchoReply = get_meta_info_on_consistent_pair_position_policy(triplets_tests)
        
        # get info on relation between pairs inconsistencies and is_echo_reply   
        current_quantitative_result_per_relation_with_ICMP_EchoReply, current_quantitative_result_per_test = get_info_on_relation_between_inconsistencies_and_icmpEchoReply(triplets_tests, payload)

        # get consistency per relation (e.g. 'M': 16)
        current_qualitative_result_consistent, current_qualitative_result_consistent_with_reply,  current_qualitative_result_not_consistent, current_qualitative_result_not_consistent_with_reply = get_consistency_and_no_consistency_per_relation(triplets_tests, payload)

        tests_with_ICMP_echo_reply = get_tests_with_ICMP_echo_reply(payload)
        # for test for which we get a reply, get relation_01,relation_02 et relation_12
        relations_01_with_ICMP_EchoReply, relations_02_with_ICMP_EchoReply, relations_12_with_ICMP_EchoReply = get_info_on_relation_position_and_icmpEchoReply(triplets_tests, tests_with_ICMP_echo_reply)
            
        result_per_scenarii[scenarii_str] = dict({"global_consistency_per_relation":current_quantitative_result_per_relation_without_ICMP_EchoReply})
        result_per_scenarii[scenarii_str].update(dict({"global_consistency_per_relation_with_ICMP_EchoReply":current_quantitative_result_per_relation_with_ICMP_EchoReply}))
        result_per_scenarii[scenarii_str].update(dict({"global_consistency_per_test_with_ICMP_EchoReply":current_quantitative_result_per_test}))
        
        result_per_scenarii[scenarii_str].update(dict({"relations_01_with_ICMP_EchoReply":relations_01_with_ICMP_EchoReply}))
        result_per_scenarii[scenarii_str].update(dict({"relations_02_with_ICMP_EchoReply":relations_02_with_ICMP_EchoReply}))
        result_per_scenarii[scenarii_str].update(dict({"relations_12_with_ICMP_EchoReply":relations_12_with_ICMP_EchoReply}))
       
        # calculate percentages per scenario
        if result_per_scenarii[scenarii_str]["global_consistency_per_relation"].values():
            sum = 0
            if 'Consistent' in result_per_scenarii[scenarii_str]["global_consistency_per_relation"]: 
                sum += result_per_scenarii[scenarii_str]["global_consistency_per_relation"]['Consistent']

            if 'NotConsistent' in result_per_scenarii[scenarii_str]["global_consistency_per_relation"]: 
                sum += result_per_scenarii[scenarii_str]["global_consistency_per_relation"]['NotConsistent']

            if 'NotConsistentPeoLike' in result_per_scenarii[scenarii_str]["global_consistency_per_relation"]: 
                sum += result_per_scenarii[scenarii_str]["global_consistency_per_relation"]['NotConsistentPeoLike']

            result_per_scenarii[scenarii_str].update({"global_consistency_percentage":(result_per_scenarii[scenarii_str]["global_consistency_per_relation"]['Consistent']/sum)*100})
        else:
            result_per_scenarii[scenarii_str].append({"global_consistency_percentage":0.0})
        
        # calculate percentages of consistencies per relation
        for r in current_qualitative_result_consistent.keys():
            sum = current_qualitative_result_consistent[r] + current_qualitative_result_not_consistent[r]
            if sum != 0:
                current_qualitative_result_consistency_percentages.update({r: (current_qualitative_result_consistent[r]/sum)*100}) 
        
        result_per_scenarii[scenarii_str].update({"per_relation_consistency":current_qualitative_result_consistent})
        result_per_scenarii[scenarii_str].update({"per_relation_consistency_with_reply":current_qualitative_result_consistent_with_reply})
        result_per_scenarii[scenarii_str].update({"per_relation_non_consistency":current_qualitative_result_not_consistent})
        result_per_scenarii[scenarii_str].update({"per_relation_non_consistency_with_reply":current_qualitative_result_not_consistent_with_reply})
        result_per_scenarii[scenarii_str].update({"per_relation_consistency_percentages":current_qualitative_result_consistency_percentages})

    return result_per_scenarii 

def get_echo_reply_info(json_payload_files):
    result_per_scenarii = {
        "pep" : {},
        "peos" : {},
        "peoe" : {},
        "peose" : {},
        "peoes" : {}
    }

    for json_path in json_payload_files:
        scenarii_str = get_scenarii_name_payload(json_path)

        current_result = {True: 0, False: 0}
        with open(json_path, 'r') as json_file:
            data = json.load(json_file)
        triplets_tests = data['hm']
            
        for test_number in triplets_tests.keys():
            is_echo_reply = triplets_tests[test_number]['is_echo_reply']

            current_result[is_echo_reply] = current_result.get(is_echo_reply, 0) + 1

        result_per_scenarii[scenarii_str] = current_result
    
    result_percentages = []
    for scenarii in result_per_scenarii.keys():
        sum = 0
        for result in result_per_scenarii[scenarii].values():
            sum += result
        if result_per_scenarii[scenarii].values():
            result_percentages.append((list(result_per_scenarii[scenarii].values())[0]/sum)*100)
        else:
            result_percentages.append(0.0)

    return result_per_scenarii, result_percentages

# return groups of scenarios that have same policies
def get_groups_of_policies(json_payload_files):
    groups = {}

    for json_path in json_payload_files:
        current_file_hash = str(hashlib.md5(open(json_path,'rb').read()).hexdigest())

        try:            
            groups[current_file_hash].append(str(json_path))
        except:
            groups[current_file_hash] = [str(json_path)]
    
    return groups.values()

def get_policy_modification_accross_scenario(json_payload_files):
    result = {}
    scenarios = []

    groups = get_groups_of_policies(json_payload_files)
    representative_scenario = [group[0] for group in groups]

    for json_path in json_payload_files:
        scenarii_str = get_scenarii_name_payload(json_path)
        if scenarii_str not in scenarios:
            scenarios.append(scenarii_str)

    permutations = itertools.combinations(representative_scenario, 2)

    for permutation in permutations:
        with open(permutation[0], 'r') as json_file:
            data_file_1 = json.load(json_file)

        with open(permutation[1], 'r') as json_file:
            data_file_2 = json.load(json_file)
    
        scenarii_0_str = get_scenarii_name_payload(permutation[0])
        scenarii_1_str = get_scenarii_name_payload(permutation[1])

        triplets_0_tests = data_file_1['hm']
        triplets_1_tests = data_file_2['hm']

        if triplets_0_tests.keys() == triplets_1_tests.keys():
            divergences = [i
                for i in triplets_0_tests.keys() if ((triplets_0_tests[i]["is_echo_reply"] ==  triplets_1_tests[i]["is_echo_reply"]) and (triplets_0_tests[i]["payload"] != triplets_1_tests[i]["payload"]))
            ]
            
        else:
            print(f"++ Scenario {scenarii_0_str} and {scenarii_1_str} do not have the same tests indexes")

        result.update({f'{scenarii_0_str}_{scenarii_1_str}': divergences})
    
    result_percentages = [(len(r)/422)*100 for r in result.values()]

    return result, result_percentages

def get_percentage_of_responses_involving_overlaps(json_complicated_files, json_payload_files):
    groups = get_groups_of_policies(json_complicated_files)
    representative_scenario = [group[0] for group in groups]

    non_response = ["Ignore", "None"]
    tests_with_response_without_overlaps = {}
    tests_with_response_and_overlaps = {}
    
    for json_complicated_file_name in json_complicated_files:
        with open(json_complicated_file_name, 'r') as json_complicated_file:
            data_complicated_file = json.load(json_complicated_file)

        payload_file_name = get_payload_path_from_scenario_name(get_scenarii_name_complicated(json_complicated_file_name), json_payload_files)

        with open(payload_file_name, 'r') as json_payload_file:
            data_payload_file = json.load(json_payload_file)

        pairs_complicated = data_complicated_file['pair_position_policy_data_c']['hm']
        triplets_complicated = data_complicated_file['triplet_position_policy_data_c']['hm']
        payloads = data_payload_file['hm']

        current_tests_with_response_and_overlaps = []
        current_tests_with_response_without_overlaps = []

        for pair_index in pairs_complicated.keys():
            pair_content = pairs_complicated[pair_index]

            # verify that we get a response from that test
            if payloads[pair_index]['is_echo_reply']:
                # verify that the relation is an overlap
                if pair_content['pair_position_policy']['relation'] not in non_overlapping_allen_relations:
                    current_tests_with_response_and_overlaps.append(pair_index)
                else:
                    current_tests_with_response_without_overlaps.append(pair_index)
        
        for triplet_index in triplets_complicated.keys():
            triplet_content = triplets_complicated[triplet_index]
            
            # verify that we get a response from that test
            if payloads[triplet_index]['is_echo_reply']:
                # verify that at least one relation is an overlap
                if triplet_content['triplet_position_policy_012']['relation_triplet']['relation_01'] not in non_overlapping_allen_relations or triplet_content['triplet_position_policy_012']['relation_triplet']['relation_02'] not in non_overlapping_allen_relations or triplet_content['triplet_position_policy_012']['relation_triplet']['relation_12'] not in non_overlapping_allen_relations:
                    current_tests_with_response_and_overlaps.append(triplet_index)
                else:
                    current_tests_with_response_without_overlaps.append(triplet_index)
        
        tests_with_response_and_overlaps.update({get_scenarii_name_complicated(json_complicated_file_name):current_tests_with_response_and_overlaps})
        tests_with_response_without_overlaps.update({get_scenarii_name_complicated(json_complicated_file_name):current_tests_with_response_without_overlaps})

    return tests_with_response_and_overlaps, tests_with_response_without_overlaps


def write_in_file(groups, relation_consistency_between_pairs_within_triplets, echo_reply_info, policy_modification, output_file):
    with open(output_file, 'w') as o_file:
        o_file.write("\scriptsize\n")
        o_file.write("\\begin{table}[H]\n")
        o_file.write("\setlength\\tabcolsep{4pt}\n")
        o_file.write("\\begin{tabular}{c l c c c c c}\n")
        o_file.write("\\toprule\n")
        o_file.write("Groupe & Scénario & \multicolumn{2}{c}{\makecell{Cohérence entre paires \\\ au sein d'un triplet}} & \multicolumn{2}{c}{\makecell{Nombre de réponses \\\quand réponse}} &")
        
        #if len(groups) > 2:
        #    o_file.write("\multicolumn{")
        #    o_file.write(str(comb(len(groups),2)))
        #    o_file.write("}{c}{\makecell{Modification de payload \\\inter-scénario}} \\\n")
        #else:
        #    o_file.write("\makecell{Modification de payload \\\inter-scénario} \\\n")  
        o_file.write("\makecell{Modification de payload \\\inter-scénario} \\\n")      
        
        o_file.write("\midrule\n")
        o_file.write("& & True & False & True & False & 1 vs 2 \\\n")

        #if len(groups) > 2:
        #    o_file.write("\multicolumn{")
        #    for i in range(comb(len(groups),2)):
        #        o_file.write("& "
        #    o_file.write("}{c}{\makecell{Modification de payload \\\inter-scénario}} \\\n")
        #else:
        #    o_file.write("\makecell{Modification de payload \\\inter-scénario} \\\n")  
        
        o_file.write("\midrule\n")

        for i,group in enumerate(groups):
            if len(group) > 1:
                o_file.write("\multirow{")
                o_file.write(len(group))
                o_file.write("}{*}{\\makecell{")
                o_file.write(str(i+1))
                o_file.write("}} & ")
                o_file.write(get_scenarii_name_payload(group[0]))
                o_file.write(" & \multirow{")
                o_file.write(len(group))
                o_file.write("}{*}{")
                o_file.write(len(group))

        
        o_file.write("")
        o_file.write("")
        o_file.write("")
        o_file.write("")
        o_file.write("")
        o_file.write("")
        o_file.write("\\bottomrule\n")
        o_file.write("\end{tabular}\n")
        o_file.write("\caption{Hello}\n")
        o_file.write("\end{table}\n")

def plot_consistency_between_pairs(os_directory, relation_consistency_between_pairs_within_triplets,groups_str):

    X = []
    Y = []
    for group in relation_consistency_between_pairs_within_triplets.keys():
        current_Y = []
        if relation_consistency_between_pairs_within_triplets[group] != {}:
            X = list(relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency_percentages"].keys())

            for k,v in relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency_percentages"].items():
                if v != '/':
                    current_Y.append(100 - float(v))
                else:
                    X.remove(k)

            Y.append(current_Y)

    X_axis = np.arange(len(X))
    width = 0.25

    for i,(y,group) in enumerate(zip(Y,groups_str)):
        plt.bar(X_axis + 0.2*i, y, 0.2, label = group)

    plt.ylim([0,100])
    plt.xticks(X_axis + width/2, X)
    plt.xlabel("Relations")
    plt.ylabel("%")
    plt.title(f"% of inconsistencies per relation and per scenario - {os_directory}")
    plt.legend()
    #plt.show()
    plt.savefig(f'../{os_directory}/relation_inconsistencies.pdf')

def plot_consistencies_inconsistencies_between_pairs_with_reply(os_directory, relation_consistency_between_pairs_within_triplets,groups_str):

    X = []
    Y1 = []
    Y2 = []
    for group in relation_consistency_between_pairs_within_triplets.keys():
        current_Y1 = []
        current_Y2 = []
        if relation_consistency_between_pairs_within_triplets[group] != {}:
            X = list(relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency_with_reply"].keys())

            for k in relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency_with_reply"].keys():

                #if not relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency_with_reply"].get(k) and not relation_consistency_between_pairs_within_triplets[group]["per_relation_non_consistency_with_reply"].get(k):
                #    X.remove(k)
                #    continue
                #
                if relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency_with_reply"].get(k):
                    current_Y1.append(relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency_with_reply"].get(k))
                else:
                    current_Y1.append(0)

                if relation_consistency_between_pairs_within_triplets[group]["per_relation_non_consistency_with_reply"].get(k):
                    current_Y2.append(relation_consistency_between_pairs_within_triplets[group]["per_relation_non_consistency_with_reply"].get(k))
                else:
                    current_Y2.append(0)

            Y1.append(current_Y1)
            Y2.append(current_Y2)


    X_axis = np.arange(len(X))
    width = 0.3

    color_list = ['royalblue', 'darkorange', 'forestgreen', 'r']
    y_max = 0

    for i,(y1,y2,group) in enumerate(zip(Y1,Y2,groups_str)):

        plt.bar(X_axis + 0.2*i , y1, width = 0.2, label = group, alpha=0.9, color = color_list[i])

        plt.bar(X_axis + 0.2*i, y2, width = 0.2, bottom=y1, alpha=0.5, color = color_list[i])
        
        for yy1, yy2 in zip(y1, y2):
            if yy1 + yy2 > y_max:
                y_max = yy1 + yy2
        

    plt.ylim([0,y_max + 5])
    plt.xticks(X_axis +  width/2, X)
    plt.xlabel("Relations")
    plt.ylabel("Count")
    plt.title(f"Number of consistencies and inconsistencies with reply per relation and per scenario - {os_directory}")
    plt.legend()
    #plt.show()
    plt.savefig(f'../{os_directory}/relation_consistencies_and_inconsistencies_with_reply.pdf', bbox_inches='tight',dpi=100)


def plot_consistencies_inconsistencies_between_pairs(os_directory, relation_consistency_between_pairs_within_triplets,groups_str):

    X = []
    Y1 = []
    Y2 = []

    for group in relation_consistency_between_pairs_within_triplets.keys():
        current_Y1 = []
        current_Y2 = []
        if relation_consistency_between_pairs_within_triplets[group] != {}:
            X = list(relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency"].keys())

            for k in relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency"].keys():

                if not relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency"].get(k) and not relation_consistency_between_pairs_within_triplets[group]["per_relation_non_consistency"].get(k):
                    X.remove(k)
                    continue
                
                if relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency"].get(k):
                    current_Y1.append(relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency"].get(k))
                else:
                    current_Y1.append(0)

                if relation_consistency_between_pairs_within_triplets[group]["per_relation_non_consistency"].get(k):
                    current_Y2.append(relation_consistency_between_pairs_within_triplets[group]["per_relation_non_consistency"].get(k))
                else:
                    current_Y2.append(0)

            Y1.append(current_Y1)
            Y2.append(current_Y2)


    X_axis = np.arange(len(X))
    width = 0.3

    color_list = ['royalblue', 'darkorange', 'forestgreen', 'r']
    y_max = 0

    for i,(y1,y2,group) in enumerate(zip(Y1,Y2,groups_str)):
        plt.bar(X_axis + 0.2*i , y1, width = 0.2, label = group, alpha=0.9, color = color_list[i])
        plt.bar(X_axis + 0.2*i, y2, width = 0.2, bottom=y1, alpha=0.5, color = color_list[i])
        
        for yy1, yy2 in zip(y1, y2):
            if yy1 + yy2 > y_max:
                y_max = yy1 + yy2
        

    plt.ylim([0,y_max + 2])
    plt.xticks(X_axis +  width/2, X)
    plt.xlabel("Relations")
    plt.ylabel("Count")
    plt.title(f"Number of consistencies and inconsistencies per relation and per scenario - {os_directory}")
    plt.legend()
    #plt.show()
    plt.savefig(f'../{os_directory}/relation_consistencies_and_inconsistencies.pdf', bbox_inches='tight',dpi=100)


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-d", "--os_directory", type=str, default="")
    args = parser.parse_args()

    os_directory = args.os_directory
    json_complicated_files = []
    json_payload_files = []
    relation_consistency_between_pairs_within_triplets = {}
    echo_reply_info = {}

    json_complicated_files = glob.glob(f"../{os_directory}/ip_fragmentation_*_policy_complicated.json")
    json_payload_files = glob.glob(f"../{os_directory}/ip_fragmentation_icmp_*_payload.json")

    
    output_meta_info_file = open(f'../{os_directory}/meta_info.txt','w')

    output_meta_info_file.write('os_directory: "%s" \n\n'%(os_directory))
    output_meta_info_file.write('json_complicated_files: "%s" \n\n'%(json_complicated_files))
    output_meta_info_file.write('json_payload_files: "%s" \n\n'%(json_payload_files))
    
    groups = get_groups_of_policies(json_payload_files)
    representative_scenario_payload = [group[0] for group in groups]
    representative_scenario_complicated = []
    for group in groups:
        for json_complicated_file in json_complicated_files:
            if get_scenarii_name_payload(group[0]) == get_scenarii_name_complicated(json_complicated_file):
                representative_scenario_complicated.append(json_complicated_file)
    
    groups_str = []
    for group in groups:
        group_str = [get_scenarii_name_payload(scenarii) for scenarii in group]
        groups_str.append(group_str)
    
    output_meta_info_file.write(f'Groups of scenario: {groups_str} \n\n')
    
    relation_consistency_between_pairs_within_triplets = get_relation_consistency_with_isolated_pair(representative_scenario_complicated, representative_scenario_payload)
    
    output_meta_info_file.write(f'Relation consistency between pairs within triplets:\n') 
    for group in relation_consistency_between_pairs_within_triplets.keys():
        if relation_consistency_between_pairs_within_triplets[group] != {}:
            output_meta_info_file.write(f'+ {group}\n')
            output_meta_info_file.write(f'\t- global consistency per relation: {relation_consistency_between_pairs_within_triplets[group]["global_consistency_per_relation"]}\n')
            output_meta_info_file.write(f'\t- global consistency percentage: {relation_consistency_between_pairs_within_triplets[group]["global_consistency_percentage"]}\n')

            output_meta_info_file.write(f'\t- global consistency per relation (EchoReply is true): {relation_consistency_between_pairs_within_triplets[group]["global_consistency_per_relation_with_ICMP_EchoReply"]}\n')
            output_meta_info_file.write(f'\t- global consistency per test: {relation_consistency_between_pairs_within_triplets[group]["global_consistency_per_test_with_ICMP_EchoReply"]}\n')

            output_meta_info_file.write(f'\t- number of time that there is this relation between 0 and 1 with ICMP echo reply: {relation_consistency_between_pairs_within_triplets[group]["relations_01_with_ICMP_EchoReply"]}\n')
            output_meta_info_file.write(f'\t- number of time that there is this relation between 0 and 2 with ICMP echo reply: {relation_consistency_between_pairs_within_triplets[group]["relations_02_with_ICMP_EchoReply"]}\n')
            output_meta_info_file.write(f'\t- number of time that there is this relation between 1 and 2 with ICMP echo reply: {relation_consistency_between_pairs_within_triplets[group]["relations_12_with_ICMP_EchoReply"]}\n')

            output_meta_info_file.write(f'\t- per_relation_consistency: {relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency"]}\n')
            output_meta_info_file.write(f'\t- per_relation_consistency_with_reply: {relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency_with_reply"]}\n')
            output_meta_info_file.write(f'\t- per_relation_non_consistency: {relation_consistency_between_pairs_within_triplets[group]["per_relation_non_consistency"]}\n')
            output_meta_info_file.write(f'\t- per_relation_non_consistency_with_reply: {relation_consistency_between_pairs_within_triplets[group]["per_relation_non_consistency_with_reply"]}\n')
            output_meta_info_file.write(f'\t- per_relation_consistency_percentages: {relation_consistency_between_pairs_within_triplets[group]["per_relation_consistency_percentages"]} \n')
    output_meta_info_file.write("\n")

    echo_reply_info = get_echo_reply_info(representative_scenario_payload)
    output_meta_info_file.write(f'Tests for which we get replies:\n')
    output_meta_info_file.write(f'+++ number {echo_reply_info[0]}\n')
    output_meta_info_file.write(f'+++ percentage {echo_reply_info[1]} \n\n')

    info_on_responses_involving_overlaps =  get_percentage_of_responses_involving_overlaps(representative_scenario_complicated, representative_scenario_payload)

    for scenario in info_on_responses_involving_overlaps[0].keys():
        output_meta_info_file.write(f"+++ {scenario}\n")
        output_meta_info_file.write(f"\t- Number of tests receiving a response with at least one overlap : {len(info_on_responses_involving_overlaps[0].get(scenario))} ({round(len(info_on_responses_involving_overlaps[0].get(scenario))/(len(info_on_responses_involving_overlaps[0].get(scenario)) + len(info_on_responses_involving_overlaps[1].get(scenario))),2) * 100}%)\n")
        output_meta_info_file.write(f"\t- Number of tests receiving a response without any overlap : {len(info_on_responses_involving_overlaps[1].get(scenario))} ({round(len(info_on_responses_involving_overlaps[1].get(scenario))/(len(info_on_responses_involving_overlaps[0].get(scenario)) + len(info_on_responses_involving_overlaps[1].get(scenario))),2) * 100 }%)\n")
        output_meta_info_file.write(f"\t- Tests receiving a response with at least one overlap : {list(info_on_responses_involving_overlaps[0].get(scenario))}\n")
        output_meta_info_file.write(f"\t- Tests receiving a response without any overlap : {list(info_on_responses_involving_overlaps[1].get(scenario))}\n")

    policy_modification = get_policy_modification_accross_scenario(representative_scenario_payload)
    output_meta_info_file.write(f"\n\nTests for which 1/ both groups repond with 1 ICMP Echo Reply and 2/ payload is different (e.g. AABBCCDD vs AABBDDCC):\n")
    output_meta_info_file.write(f'+++ tests_id {policy_modification[0]}\n')
    output_meta_info_file.write(f'+++ percentage {policy_modification[1]} \n\n')

    plot_consistency_between_pairs(os_directory, relation_consistency_between_pairs_within_triplets, groups_str)
    
    plt.clf()
    plot_consistencies_inconsistencies_between_pairs(os_directory, relation_consistency_between_pairs_within_triplets, groups_str)
    
    plt.clf()
    plot_consistencies_inconsistencies_between_pairs_with_reply(os_directory, relation_consistency_between_pairs_within_triplets, groups_str)

    output_meta_info_file.close()

    # writing in json file 
    json_dict = {}

    for scenario in info_on_responses_involving_overlaps[0].keys():
        # write one entry for all the scenario
        for group in groups_str:
            if scenario in group:
                for scenario_from_group in group:
                    json_dict_tests = {
                        "test_for_which_we_get_a_response" : [],
                        "test_for_which_we_get_a_response_with_at_east_one_overlap": [],
                        "test_for_which_we_get_a_response_without_any_overlap": []
                    }
                    json_dict_tests.update({"test_for_which_we_get_a_response_with_at_east_one_overlap":list(info_on_responses_involving_overlaps[0].get(scenario))})
                    json_dict_tests.update({"test_for_which_we_get_a_response_without_any_overlap":list(info_on_responses_involving_overlaps[1].get(scenario))})
                    json_dict_tests.update({"test_for_which_we_get_a_response":list(info_on_responses_involving_overlaps[0].get(scenario)) + list(info_on_responses_involving_overlaps[1].get(scenario))})

                    json_dict.update({scenario_from_group:json_dict_tests})
                break

    # Serializing json
    json_object = json.dumps(json_dict, indent=4)

    with open(f'../{os_directory}/meta_info.json', "w") as outfile:
        outfile.write(json_object)

if __name__ == "__main__":
    main(sys.argv[1:])
