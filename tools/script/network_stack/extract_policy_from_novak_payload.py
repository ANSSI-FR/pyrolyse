import sys
import json
import argparse
import glob

def process(
      input_json_path: str
  ):
    print("process: start")

    with open(input_json_path) as f:
        d = json.load(f)

    payload_reassembly = d["hm"]["600"]["payload"]
    print(
        f"extract_policy_from_novak: payload_reassembly: {payload_reassembly}"
    )

    if len(payload_reassembly) != 24 * 8:
        print('Bad payload len !')
        return {}
        #exit(-1)

    #print("process: ",payload_reassembly[8:24])
    #print("process: ",payload_reassembly[32:40])
    #print("process: ",payload_reassembly[48:72])
    #print("process: ",payload_reassembly[80:88])
    #print("process: ",payload_reassembly[104:120])
    #print("process: ",payload_reassembly[128:136])
    #print("process: ",payload_reassembly[152:160])
    #print("process: ",payload_reassembly[160:168])
    #print("process: ",payload_reassembly[176:184])
    
    hm = {}
    hm.update({'O': 'o'}) if payload_reassembly[8:24] == "AABBDDCCAACCBBDD" else hm.update({'O': 's'})
    hm.update({'Oi': 'o'}) if payload_reassembly[32:40] == "AACCDDBB" else hm.update({'Oi': 's'})
    hm.update({'Eq': 'o'}) if payload_reassembly[48:72] == "AADDCCBBBBAACCDDBBAADDCC" else hm.update({'Eq': 's'})
    hm.update({'D': 'o'}) if payload_reassembly[80:88] == "BBCCAADD" else hm.update({'D': 's'})
    hm.update({'F': 'o'}) if payload_reassembly[104:120] == "BBCCDDAABBDDAACC" else hm.update({'F': 's'})
    hm.update({'Di': 'o'}) if payload_reassembly[128:136] == "CCAABBDD" else hm.update({'Di': 's'})
    hm.update({'Fi': 'o'}) if payload_reassembly[152:160] == "CCBBDDAA" else hm.update({'Fi': 's'})
    hm.update({'Si': 'o'}) if payload_reassembly[160:168] == "CCDDAABB" else hm.update({'Si': 's'})
    hm.update({'S': 'o'}) if payload_reassembly[176:184] == "DDAABBCC" else hm.update({'S': 's'})

    print("process: hm: ", hm)
    return hm
  
    print("process: end")
    

def main(argv):
    print("extract_policy_from_novak: start")

    parser = argparse.ArgumentParser()
    #parser.add_argument("-p", "--payload-reassembly", type=str, default="")
    parser.add_argument("-i", "--input-dir", type=str, default="")
    parser.add_argument("-p", "--pattern", type=str, default="")
    parser.add_argument("-o", "--output-json-path", type=str, default="")
    args = parser.parse_args()

    #payload_reassembly = args.payload_reassembly
    input_dir = args.input_dir
    pattern = args.pattern
    output_json_path = args.output_json_path
    print(
        f"extract_policy_from_novak: input_dir: {input_dir}"
    )
    print(
        f"extract_policy_from_novak: pattern: {pattern}"
    )
    print(
        f"extract_policy_from_novak: output_json_path: {output_json_path}"
    )

    input_json_file_v = glob.glob(f"{input_dir}/*{pattern}*_payload*.json")
    if len(input_json_file_v) < 1:
        print("No file found")
        exit(-1)

    # check payload reassembly consistency across run
    reassembly_policy_v = [ process(input_json_file) for input_json_file in input_json_file_v ]
    #reassembly_policy_v_no_duplicates = list(set(reassembly_policy_v))
    reassembly_policy_v_no_duplicates = list({json.dumps(reassembly_policy, sort_keys=True) for reassembly_policy in reassembly_policy_v})
    if len(reassembly_policy_v_no_duplicates) > 1:
        print("Inconsistent policies across run")
        exit(-1)

    with open(output_json_path, "w") as write_file:
      json.dump(reassembly_policy_v[0], write_file)

    print("extract_policy_from_novak: end")


if __name__ == "__main__":
    main(sys.argv[1:])