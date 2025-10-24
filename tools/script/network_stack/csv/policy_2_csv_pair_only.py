import os,sys
import json, csv
import pandas as pd

def dict_depth(dic, level = 1):
       
    str_dic = str(dic)
    counter = 0
    for i in str_dic:
        if i == "{":
            counter += 1
    return(counter)

def addTripPairPolicy(policy_n, subdict):
    depth = dict_depth(subdict)
    if depth == 0 :
        policy_n.append(subdict)
    elif depth == 3:
        policy_n.append(subdict['One']['pair_position_policy']['pair_time_policy'])
    elif depth == 4:
        if subdict['BeforeAfter']['pair_position_policy_first']['pair_time_policy'] == subdict['BeforeAfter']['pair_position_policy_second']['pair_time_policy']:
            policy_n.append(subdict['BeforeAfter']['pair_position_policy_first']['pair_time_policy']+subdict['BeforeAfter']['pair_position_policy_second']['pair_time_policy'])
        else:
            policy_n.append("Conflict")
    else:
        pass


path = os.path.abspath(os.getcwd()) + "/" + sys.argv[1]

with open(path) as f:
      data = json.load(f)


pairs = data['pair_position_policy_data_c']['hm']
pair_no = list(pairs.keys())
scenario = []
pair_byte_start_position = []
pair_byte_end_position = []
pair_choice = []
relation_all = []

for i in pair_no:
    if pairs[i]['temporal_position_v'] == [0, 1]:
        pair_byte_start_position.append(pairs[i]['pair_position_policy']['pair_byte_start_position'])
        pair_byte_end_position.append(pairs[i]['pair_position_policy']['pair_byte_end_position'])
        pair_choice.append(pairs[i]['pair_position_policy']['pair_choice'])
        relation_all.append(pairs[i]['pair_position_policy']['relation'])



dfPair = pd.DataFrame(
    {'relation': relation_all,
     'pair_byte_start_position': pair_byte_start_position,
     'pair_byte_end_position': pair_byte_end_position,
     'pair_choice': pair_choice,
    })

dfPair['scenario'] = sys.argv[2]
dfPair.insert(0, 'scenario', dfPair.pop('scenario'))
print(dfPair)

curr_time = 'pair_sc' + sys.argv[2] + '.csv'
directory = os.path.abspath(os.getcwd()) 
csvpath = os.path.join(directory, curr_time)

print(csvpath)
dfPair.to_csv(csvpath,index = False, header=True)

