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


def matchPairRelation(diction, dict_no, subdict):
    for i in dict_no:
        if diction[i]['temporal_position_v'] == [0, 1]:
            #print(pairs[i]['pair_position_policy']['relation'])
            #print(sys.version)
            match diction[i]['pair_position_policy']['relation'].split():
                case ["Eq"]:
                    subdict["Eq"] = diction[i]['pair_position_policy']['pair_time_policy']
                case ["B"]:
                    subdict["B"] = diction[i]['pair_position_policy']['pair_time_policy']
                case ["Bi"]:
                    subdict["Bi"] = diction[i]['pair_position_policy']['pair_time_policy']
                case ["M"]:
                    subdict["M"] = diction[i]['pair_position_policy']['pair_time_policy']
                case ["Mi"]:
                    subdict["Mi"] = diction[i]['pair_position_policy']['pair_time_policy']
                case ["O"]:
                    subdict["O"] = diction[i]['pair_position_policy']['pair_time_policy']
                case ["Oi"]:
                    subdict["Oi"] = diction[i]['pair_position_policy']['pair_time_policy']
                case ["S"]:
                    subdict["S"] = diction[i]['pair_position_policy']['pair_time_policy']
                case ["Si"]:
                    subdict["Si"] = diction[i]['pair_position_policy']['pair_time_policy']
                case ["D"]:
                    subdict["D"] = diction[i]['pair_position_policy']['pair_time_policy']
                case ["Di"]:
                    subdict["Di"] = diction[i]['pair_position_policy']['pair_time_policy']
                case ["F"]:
                    subdict["F"] = diction[i]['pair_position_policy']['pair_time_policy']
                case ["Fi"]:
                    subdict["Fi"] = diction[i]['pair_position_policy']['pair_time_policy']        
                case _:
                    pass
    return subdict

path = os.path.abspath(os.getcwd()) + "/" + sys.argv[1]

with open(path) as f:
      data = json.load(f)

relation_list = ["Eq", "B", "Bi", "M", "Mi", "O", "Oi", "S", "Si", "D", "Di", "F", "Fi"]
pair_dict = {key:[] for key in relation_list}
pairs = data['pair_position_policy_data_c']['hm']
pair_no = list(pairs.keys())

pair_dict = matchPairRelation(pairs, pair_no, pair_dict)
print(pair_dict)
dfPair = pd.DataFrame(pair_dict, index=[0])
print(dfPair)

triplets = data['triplet_position_policy_data_c']['hm']
# #print(triplets)
triplets_no = list(triplets.keys())
relation_all = []
policy_01 = []
policy_02 = []
policy_12 =[]
trip_policy = []
# print(triplets_no)
for i in triplets_no:
    if triplets[i]['temporal_position_v'] == [0, 1, 2]:
        relation_all.append(triplets[i]['triplet_position_policy_012']['relation_triplet']['relation_01'] + triplets[i]['triplet_position_policy_012']['relation_triplet']['relation_02'] +triplets[i]['triplet_position_policy_012']['relation_triplet']['relation_12'])
        trip_policy.append(triplets[i]['triplet_position_policy_012']['triplet_policy'])
        
        addTripPairPolicy(policy_01, triplets[i]['triplet_pair_position_policy_01'])
        addTripPairPolicy(policy_02, triplets[i]['triplet_pair_position_policy_02'])
        addTripPairPolicy(policy_12, triplets[i]['triplet_pair_position_policy_12'])
        # policy_01.append(dict_depth(triplets[i]['triplet_pair_position_policy_01']))
        # policy_02.append(dict_depth(triplets[i]['triplet_pair_position_policy_02']))
        # policy_12.append(dict_depth(triplets[i]['triplet_pair_position_policy_12']))

dfTriplets = pd.DataFrame(
    {'Index': triplets_no,
     'Relation': relation_all,
     'Pair_time_policy_01': policy_01,
     'Pair_time_policy_02': policy_02,
     'Pair_time_policy_12': policy_12,
     'Triplet policy' : trip_policy
    })

print(dfTriplets)
csvPath = path.replace('tcp_segmentation_policy_0_complicated', 'triplet_policy_0_complicated.csv')
csvPath2 = path.replace('tcp_segmentation_policy_0_complicated', 'pair_policy_0_complicated.csv')
dfPair.to_csv(csvPath2,index = False, header=True)
dfTriplets.to_csv(csvPath,index = False, header=True)

# print(triplet_list)
# triplets_dict = matchRelation(triplets, triplets_no, triplets_dict)
# print(triplets_dict)
# print("Eq: %d" % len(list(triplets_dict["Eq"])))
# print("B: %d" % len(list(triplets_dict["B"])))
# print("Bi: %d" %  len(list(triplets_dict["Bi"])))
# print("M: %d" % len(list(triplets_dict["M"])))
# print("Mi: %d" % len(list(triplets_dict["Mi"])))
# print("O: %d" % len(list(triplets_dict["O"])))
# print("Oi: %d" %  len(list(triplets_dict["Oi"])))
# print("S: %d" % len(list(triplets_dict["S"])))
# print("Si: %d" % len(list(triplets_dict["Si"])))
# print("D: %d" % len(list(triplets_dict["D"])))
# print("Di: %d" % len(list(triplets_dict["Di"])))
# print("F: %d" % len(list(triplets_dict["F"])))
# print("Fi: %d" % len(list(triplets_dict["Fi"])))

# print(set(triplets_dict.values()))
# triplets_index = list(range(39))
# dfTri = pd.DataFrame(triplets_dict, index=triplets_index)

#print(dfTri)
# pair_list = [(sub[1], sub[0]) for sub in list(pair_dict.items())]
# print(pair_list)
# with open('test.csv', 'w') as f:
#     writer = csv.writer(f , lineterminator='\n')
#     for tup in pair_list:
#         writer.writerow(tup)
    
# with open('test.csv', 'w') as f:
#     for key in pair_dict.keys():
#         f.write("%s,%s\n"%(key,pair_dict[key]))