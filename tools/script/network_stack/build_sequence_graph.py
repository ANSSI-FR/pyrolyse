import os
import matplotlib.pyplot as plt
import json

#print(os.getcwd()) 
with open('../test_data/byte_time_sequence.json') as f:
  data = json.load(f)


directory = os.path.abspath(os.getcwd()) 
path = os.path.join(directory, "sequence_graph_new_testcases")
print(path)
try:
    #original_umask = os.umask(0)
    #os.makedirs(path, mode=0o777, exist_ok = True)
    os.system("sudo mkdir '%s'" % path)
    print("Directory '%s' created successfully" % "sequence_graph_nodup")
except OSError as error:
    print("Directory '%s' can not be created" % "sequence_graph_nodup")
    print(error)

pairs = data['byte_time_pair_sequence_c']
triplets = data['byte_time_triplet_sequence_c']

pairs_no = list(pairs['hm'].keys())
#print(pairs_no)
for i in pairs_no:
  
  slice_data = pairs['hm'][i]['chunk_c']['bm']
  #print(slice_data)
  offset = []
  for p_id, p_info in slice_data.items():
      res = p_info['offset']
      offset.append(res)
  multiplied_offset = [element * 8 for element in offset]
  #print(multiplied_offset)

  #paylen = pairs['hm'][i]['payload_byte_length']
  #print(paylen)

  payload = []
  for s_id, s_info in slice_data.items():
      temp = s_info['internet_checksum_s']
      payload.append(temp)
  temp_pos = pairs['hm'][i]['temporal_position_v']

  # example data
  x =  multiplied_offset
  # print("x coordinates: ", x)
  y = temp_pos

  error = [len(i) for i in payload]
  # print("error: ", error)
  xerror = [(0, 0), error]

  fig = plt.figure()
  bar = plt.errorbar(x, y, xerr=xerror, fmt=',' )
  #print(enumerate(bar))
  #plt.title('Demonstration')
  plt.xticks([0, 8, 16, 24, 32, 40, 48])
  plt.yticks([-1, 0, 1, 2, 3],('Start', 'First', 'Second', 'Third', 'Finish'))
  plt.xlabel("Sequence Number")
  for j in range(2):
    plt.text(x[j],y[j]+0.1, payload[j])
  graph_name = path +"/sequence_pair_" + i +".png"
  print(graph_name)
  plt.savefig(graph_name)


triplets_no = list(triplets['hm'].keys())
for j in triplets_no:
  slice_data = triplets['hm'][j]['chunk_c']['bm']
  #paylen = triplets['hm'][j]['payload_byte_length']

  #get the offset
  offset = []
  for p_id, p_info in slice_data.items():
      res = p_info['offset']
      offset.append(res)

  multiplied_offset = [element * 8 for element in offset]

  # get the payload
  payload = []
  for s_id, s_info in slice_data.items():
      temp = s_info['internet_checksum_s']
      payload.append(temp)

  #Keep offset/payload in line with temporary position
  temp_pos = triplets['hm'][j]['temporal_position_v']
  
  x =  multiplied_offset
  # print("x coordinates: ", x)
  y = temp_pos
  # print("temp pos: ", temp_pos)
  # print("payload: ", payload)
  # example error bar values that vary with x-position
  error = [len(i) for i in payload]
  # print("error: ", error)
  xerror = [(0, 0, 0), error]

  fig = plt.figure()
  bar = plt.errorbar(x, y, xerr=xerror, fmt=',' )
  # print(enumerate(bar))
  #plt.title('Demonstration')
  plt.xticks([0, 8, 16, 24, 32, 40, 48])
  plt.yticks([-1, 0, 1, 2, 3],('Start', 'First', 'Second', 'Third', 'Finish'))
  plt.xlabel("Sequence Number")
  for w in range(3):
    plt.text(x[w],y[w]+0.1, payload[w])
  graph_name = path +"/sequence_triplet_" + j +".png"
  print(graph_name)
  plt.savefig(graph_name)
  plt.close('all')