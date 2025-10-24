#!/usr/bin/python

import sys
import os
import json
import argparse
import glob

# 3 cases of inconsistencies:
# - 1/ Pair overlap resolution for a relation r is New (resp. Old) and triplet overlap resolution for r is Old (resp. New);
# - 2/ Pair overlap resolution for a relation r is Ignore and triplet overlap resolution for r is Old or New;
# - 3/ Pair overlap resolution for a relation r is Old or New and triplet overlap resolution for r is Ignore.
#   - a/ no matter the triplet
#   - b/ triplets without hole

# How to classify the inconsistency? 
# -> Compare "pair_time_policy" fields from pair and triplet  

triplet_positions = ['01', '02', '12']

def check_for_hole(triplet_test):
    return True if sum(1 for position in triplet_positions if triplet_test['triplet_position_policy_012']['relation_triplet'][f'relation_{position}'] in ['B', 'Bi']) > 1 else False

def get_inconsistency_with_position_and_pair_time_policy(test_case_data, pairs_to_inspect_d, position, pair_time_policy_to_find):
    relation = test_case_data['triplet_position_policy_012']['relation_triplet'][f'relation_{position}']
    
    if relation not in pairs_to_inspect_d or test_case_data[f'triplet_pair_position_policy_{position}'] == 'None':
        return False 
    
    # no triple overlap            
    if list(test_case_data[f'triplet_pair_position_policy_{position}'].keys()) == ['One'] and test_case_data[f'triplet_pair_position_policy_{position}']['One']['pair_position_policy']['pair_time_policy'] in pair_time_policy_to_find:
        return True
    
    # triple overlap 
    if list(test_case_data[f'triplet_pair_position_policy_{position}'].keys()) == ['BeforeAfter'] and (test_case_data[f'triplet_pair_position_policy_{position}']['BeforeAfter']['pair_position_policy_first']['pair_time_policy'] in pair_time_policy_to_find and test_case_data[f'triplet_pair_position_policy_{position}']['BeforeAfter']['pair_position_policy_second']['pair_time_policy'] in pair_time_policy_to_find):
        return True
    
    return False


def get_inconsistencies_of_type_1(pair_tests, triplet_tests):
    # build dictionary that contain relations with response and associated pair_position_policy 
    pairs_to_inspect_d = { pair_tests[test_index]['pair_position_policy']['relation']:pair_tests[test_index]['pair_position_policy']['pair_time_policy']  for test_index in pair_tests.keys() if pair_tests[test_index]['pair_position_policy']['pair_time_policy'] == 'Old' or pair_tests[test_index]['pair_position_policy']['pair_time_policy'] == 'New' } 

    print('get_inconsistencies_of_type_1: pairs_to_inspect_d: ',pairs_to_inspect_d)

    # look for test triplets in which with a response (i.e. pair_time_policy = Old or New) 
    results = {}
    for test_index in triplet_tests.keys():
        is_type_1_inconsistency = False
        
        for position in triplet_positions:
            # set pair_time_policy value Old or New to find if it is an inconsistent relation 
            relation_position = triplet_tests[test_index]['triplet_position_policy_012']['relation_triplet'][f'relation_{position}']
            pair_time_policy_to_find = ['Old'] if pairs_to_inspect_d.get(relation_position) == 'New' else ['New'] 
            
            if (is_type_1_inconsistency := get_inconsistency_with_position_and_pair_time_policy(triplet_tests[test_index], list(pairs_to_inspect_d.keys()), position, pair_time_policy_to_find)):
                break

        results.update({ test_index: is_type_1_inconsistency })
        
    return results

def get_inconsistencies_of_type_2(pair_tests, triplet_tests):
    # build dictionary that contain relations with no response 
    pairs_to_inspect_d = [pair_tests[test_index]['pair_position_policy']['relation'] for test_index in pair_tests.keys() if pair_tests[test_index]['pair_position_policy']['pair_time_policy'] == 'Ignore' ] 

    print('get_inconsistencies_of_type_2: pairs_to_inspect_d: ',pairs_to_inspect_d)

    # look for triplet tests with a reassembly for all relations to inspect
    results = {}
    pair_time_policy_to_find = ['Old', 'New'] 

    for test_index in triplet_tests.keys():
        is_type_2_inconsistency = False
 
        for position in triplet_positions:
            if (is_type_2_inconsistency := get_inconsistency_with_position_and_pair_time_policy(triplet_tests[test_index], pairs_to_inspect_d, position, pair_time_policy_to_find)):
                break

        results.update({ test_index: is_type_2_inconsistency })
        
    return results

def get_inconsistencies_of_type_3_a(pair_tests, triplet_tests):
    # build dictionary that contain relations with response and associated pair_position_policy 
    pairs_to_inspect_d = { pair_tests[test_index]['pair_position_policy']['relation']:pair_tests[test_index]['pair_position_policy']['pair_time_policy']  for test_index in pair_tests.keys() if pair_tests[test_index]['pair_position_policy']['pair_time_policy'] == 'Old' or pair_tests[test_index]['pair_position_policy']['pair_time_policy'] == 'New' } 

    print('get_inconsistencies_of_type_3_a: pairs_to_inspect_d: ',pairs_to_inspect_d)

    # look for triplet tests with no reassembly for all relations to inspect
    results = {}
    pair_time_policy_to_find = ['Ignore'] 

    for test_index in triplet_tests.keys():
        is_type_3_inconsistency = False
 
        for position in triplet_positions:
            if (is_type_3_inconsistency := get_inconsistency_with_position_and_pair_time_policy(triplet_tests[test_index], list(pairs_to_inspect_d.keys()), position, pair_time_policy_to_find)):
                break

        results.update({ test_index: is_type_3_inconsistency })
        
    return results

def get_inconsistencies_of_type_3_b(pair_tests, triplet_tests):
    # build dictionary that contain relations with response and associated pair_position_policy 
    pairs_to_inspect_d = { pair_tests[test_index]['pair_position_policy']['relation']:pair_tests[test_index]['pair_position_policy']['pair_time_policy']  for test_index in pair_tests.keys() if pair_tests[test_index]['pair_position_policy']['pair_time_policy'] == 'Old' or pair_tests[test_index]['pair_position_policy']['pair_time_policy'] == 'New' } 

    print('get_inconsistencies_of_type_3_b: pairs_to_inspect_d: ',pairs_to_inspect_d)

    # look for triplet tests with no reassembly for all relations to inspect
    results = {}
    pair_time_policy_to_find = ['Ignore'] 

    for test_index in triplet_tests.keys():
        if check_for_hole(triplet_tests.get(test_index)):
            #print('get_inconsistencies_of_type_3_b: hole detected for test: ', test_index)
            results.update({ test_index: False })
            continue

        is_type_3_inconsistency = False

        for position in triplet_positions:
            if (is_type_3_inconsistency := get_inconsistency_with_position_and_pair_time_policy(triplet_tests[test_index], list(pairs_to_inspect_d.keys()), position, pair_time_policy_to_find)):
                break

        results.update({ test_index: is_type_3_inconsistency })
        
    return results

def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-d", "--os_directory_path", type=str, default="")
    parser.add_argument("-p", "--protocol", type=str, default="")
    args = parser.parse_args()

    os_directory_path = args.os_directory_path
    protocol = args.protocol
    json_complicated_files = []

    json_complicated_files = glob.glob(f"{os_directory_path}/{protocol}_*_policy_complicated.json")
    print("main: json_complicated_files: ",json_complicated_files)

    for json_complicated_file in json_complicated_files: 
        scenario = os.path.basename(json_complicated_file).split('_')[1]
        print('main: processing scenario: ', scenario)

        with open(json_complicated_file, 'r') as json_file:
            data = json.load(json_file)
    
        pair_tests = data['pair_position_policy_data_c']['hm']
        triplet_tests = data['triplet_position_policy_data_c']['hm']

        inconsistencies_of_type_1 = get_inconsistencies_of_type_1(pair_tests, triplet_tests)
        inconsistencies_of_type_2 = get_inconsistencies_of_type_2(pair_tests, triplet_tests)
        inconsistencies_of_type_3_a = get_inconsistencies_of_type_3_a(pair_tests, triplet_tests)
        inconsistencies_of_type_3_b = get_inconsistencies_of_type_3_b(pair_tests, triplet_tests)

        final_json_content = { test_index: 
            { "has_inconsistency_of_type_1": inconsistencies_of_type_1.get(test_index), 
            "has_inconsistency_of_type_2": inconsistencies_of_type_2.get(test_index),
            "has_inconsistency_of_type_3_a": inconsistencies_of_type_3_a.get(test_index),
            "has_inconsistency_of_type_3_b": inconsistencies_of_type_3_b.get(test_index)
            } 
            for test_index in inconsistencies_of_type_1.keys() }

        json_object = json.dumps(final_json_content, indent=4)

        output_filename = f'{protocol}_{scenario}_consistencies.json'
        output_json_scenario_path = os_directory_path + '/' + output_filename 
        print('main: writing results in : ', output_json_scenario_path)
        with open(output_json_scenario_path, "w") as output_file:
            output_file.write(json_object)


        print('++ # inconsistencies_of_type_1: ', sum(1 for value in inconsistencies_of_type_1.values() if value))
        print('++ \% inconsistencies_of_type_1: ', sum(1 for value in inconsistencies_of_type_1.values() if value)/len(inconsistencies_of_type_1))

        print('++ # inconsistencies_of_type_2: ', sum(1 for value in inconsistencies_of_type_2.values() if value))
        print('++ \% inconsistencies_of_type_2: ', sum(1 for value in inconsistencies_of_type_2.values() if value)/len(inconsistencies_of_type_2))

        print('++ # inconsistencies_of_type_3_a: ', sum(1 for value in inconsistencies_of_type_3_a.values() if value))
        print('++ \% inconsistencies_of_type_3_a: ', sum(1 for value in inconsistencies_of_type_3_a.values() if value)/len(inconsistencies_of_type_3_a))

        print('++ # inconsistencies_of_type_3_b: ', sum(1 for value in inconsistencies_of_type_3_b.values() if value))
        print('++ \% inconsistencies_of_type_3_b: ', sum(1 for value in inconsistencies_of_type_3_b.values() if value)/len(inconsistencies_of_type_3_b))

        


if __name__ == "__main__":
    main(sys.argv[1:])
