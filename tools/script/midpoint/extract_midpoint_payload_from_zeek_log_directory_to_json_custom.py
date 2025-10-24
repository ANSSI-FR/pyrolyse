#!/usr/bin/python
import sys
import os
import json
import argparse
import ast


# TODO check tcp extraction with new patterns

def extract_tcp_payload_from_script(
    line_l_w_payload_v: list,
    nb_starting_character_to_remove: int,
    nb_final_character_to_remove: int,
    ) -> bytes:
    tcp_seq_num_payload_hex_d= {}
    tcp_payload_len_approx_i = 0
    
    for line_l_w_payload in line_l_w_payload_v: 
        print("extract_tcp_payload_from_script: line_l_w_payload: ",str(line_l_w_payload))

        zeek_msg_s = line_l_w_payload.split("\t")[11]
        zeek_msg_d = ast.literal_eval(zeek_msg_s)
        current_tcp_seq_s = zeek_msg_d["seq"]
        current_tcp_payload_s = zeek_msg_d["contents"]

        tcp_payload_len_approx_i += len(current_tcp_payload_s)
    
        entry = tcp_seq_num_payload_hex_d.get(int(current_tcp_seq_s))
        if entry == None:
            tcp_seq_num_payload_hex_d.update( { int(current_tcp_seq_s): [ current_tcp_payload_s ] } )
        elif entry != current_tcp_payload_s:
            new_entry = list(entry)
            new_entry.append(current_tcp_payload_s)
            tcp_seq_num_payload_hex_d.update( { int(current_tcp_seq_s): new_entry } )

    print("extract_tcp_payload_from_script: tcp_seq_num_payload_hex_d: ", tcp_seq_num_payload_hex_d)
    
    # non-ascii characters are hex char (e.g. \xff) 
    tcp_seq_num_payload_b_d = { }
    for tcp_seq,tcp_payload_hex_s_v in tcp_seq_num_payload_hex_d.items():
        print("extract_tcp_payload_from_script: tcp_payload_hex_s_v: ",tcp_payload_hex_s_v)
        tcp_payload_b_s_v = [ ]
        for tcp_payload_hex_s in tcp_payload_hex_s_v:
            print("extract_tcp_payload_from_script: tcp_payload_hex_s: ",tcp_payload_hex_s)
            reconstructed_payload_b = b""
            curr_pos = 0
            # replace non-printable char 
            while curr_pos < len(tcp_payload_hex_s):
                print(tcp_payload_hex_s[curr_pos:curr_pos+2])
                if tcp_payload_hex_s[curr_pos:curr_pos+2] == "\\x":
                    reconstructed_payload_b += bytes.fromhex(tcp_payload_hex_s[curr_pos+2:curr_pos+4])
                    curr_pos += 4
                else:
                    reconstructed_payload_b += bytes(tcp_payload_hex_s[curr_pos], 'utf-8')
                    curr_pos += 1
            print("extract_tcp_payload_from_script: reconstructed_payload_b: ",reconstructed_payload_b)
            tcp_payload_b_s_v.append(reconstructed_payload_b)
        tcp_seq_num_payload_b_d.update({ tcp_seq: tcp_payload_b_s_v})
    
    # Concatenate "sub-payloads" and adding "." substring if there is a hole
    tmp_tcp_payload_b = b"." * tcp_payload_len_approx_i
    for i, (seq, payload_v) in enumerate(tcp_seq_num_payload_b_d.items()):
        if len(payload_v) == 1:
            tmp_tcp_payload_b = tmp_tcp_payload_b[:seq - 1] + payload_v[0] + tmp_tcp_payload_b[seq + len(payload_v[0]) - 1:]
            print("extract_tcp_payload_from_script: tmp_tcp_payload_s", tmp_tcp_payload_b)
        else:
            # different reassembled payload (because different data for same seq number)  
            #return { "is_echo_reply": True, "number": len(payload_v), "payload": "" }
            sys.exit(-1)
    tcp_payload_b = tmp_tcp_payload_b[:seq + len(payload_v[0]) - 1]

    tcp_payload_b_len = len(tcp_payload_b)
    print("extract_tcp_payload_v: tcp_payload_b_len:",tcp_payload_b_len)

    if tcp_payload_b_len < nb_starting_character_to_remove:
        print(f"Not enough extra starting bytes to remove in payload ({tcp_payload_b_len} < {nb_starting_character_to_remove})")
        exit(-1)

    # remove extra bytes
    ## we first remove starting extra bytes
    payload_no_starting_extra_char_b = tcp_payload_b if nb_starting_character_to_remove == 0 else tcp_payload_b[nb_starting_character_to_remove:]
    print("extract_tcp_payload_v: payload_no_starting_extra_char_b: ",payload_no_starting_extra_char_b)

    ## we hypothesize we can remove ending extra bytes
    payload_no_extra_char_b = payload_no_starting_extra_char_b if nb_final_character_to_remove == 0 else payload_no_starting_extra_char_b[:-nb_final_character_to_remove]
    print("extract_tcp_payload_v: payload_no_extra_char_b: ",payload_no_extra_char_b)

    return payload_no_extra_char_b

def extract_tcp_payload_from_signatures(line_l_w_payload_v: list) -> bytes:
    print("extract_tcp_payload_from_signatures: start")
    tcp_payload_b_v = [ bytes(line_l_w_payload.split("\t")[9], 'utf-8') for line_l_w_payload in line_l_w_payload_v ]
    print("extract_tcp_payload_from_signatures: tcp_payload_b_v: ",tcp_payload_b_v)

    tcp_payload_b_v_no_duplicates = list(dict.fromkeys(tcp_payload_b_v)) 
    print("extract_tcp_payload_from_signatures: tcp_payload_b_v_no_duplicates: ",tcp_payload_b_v_no_duplicates)

    # no extra strarting or finiishing char to remove ?
    print("extract_tcp_payload_from_signatures: end")
    return b"".join(tcp_payload_b_v_no_duplicates)


def extract_tcp_payload_v(
    line_l:list,
    nb_starting_character_to_remove: int,
    nb_final_character_to_remove: int,
    test_case_index:int,
):
    print("extract_tcp_payload_v: start")
    line_l_w_payload_v = [ line for line in line_l if "tcp::Tcp_content_From_Client" in line or "Signatures::Sensitive_Signature" in line ]

    if "tcp::Tcp_content_From_Client" in line_l_w_payload_v[0]:
        tcp_payload_b = extract_tcp_payload_from_script(line_l_w_payload_v,nb_starting_character_to_remove,nb_final_character_to_remove)
    elif "Signatures::Sensitive_Signature" in line_l_w_payload_v[0]:
        tcp_payload_b = extract_tcp_payload_from_signatures(line_l_w_payload_v)
    else:
        tcp_payload_b = b""

    # from bytes to ascii
    payload_ascii_s = tcp_payload_b.decode('utf-8','replace')
    print("extract_tcp_payload_v: payload_ascii_s: ",payload_ascii_s)
    
    print("extract_tcp_payload_v: end")
    return { "is_echo_reply": True, "number": 1, "payload": payload_ascii_s }


def extract_ip_payload_v(
    line_l:list,
    nb_starting_character_to_remove: int,
    nb_final_character_to_remove: int,
    test_case_index:int,
):
    print("extract_ip_payload_v: start")
    line_w_payload_v = [ line for line in line_l if "Icmp_Request_Payload_Found" in line or "Signatures::Sensitive_Signature" in line ]
    print("extract_ip_payload_v: line_w_payload_v: ",line_w_payload_v)

    icmp_payload_v = []
    if len(line_w_payload_v) == 0:
        return []

    for line_w_payload in line_w_payload_v:
        print("extract_ip_payload_v: line_w_payload: ",line_w_payload)

        # For script
        if "Icmp_Request_Payload_Found" in line_w_payload:
            icmp_payload_hex_s = line_w_payload.split("\t")[11]
        # For signatures
        else:
            #assert "icmp-first-chunk-piece-AABBCCDD" in line or "icmp6-first-chunk-piece-AABBCCDD" in line 
            #icmp_payload_b = line_w_payload.split("\t")[9].encode('utf-8','replace')
            icmp_payload_hex_s  = line_w_payload.split("\t")[9]

        print("extract_ip_payload_v: icmp_payload_hex_s: ",icmp_payload_hex_s)

        # non-ascii characters are hex char (e.g. \xff) 
        reconstructed_payload_b = b""
        curr_pos = 0
        # replace non-printable char 
        while curr_pos < len(icmp_payload_hex_s):
            print(icmp_payload_hex_s[curr_pos:curr_pos+2])
            if icmp_payload_hex_s[curr_pos:curr_pos+2] == "\\x":
                reconstructed_payload_b += bytes.fromhex(icmp_payload_hex_s[curr_pos+2:curr_pos+4])
                curr_pos += 4
            else:
                reconstructed_payload_b += bytes(icmp_payload_hex_s[curr_pos], 'utf-8')
                curr_pos += 1
        print("extract_ip_payload_v: reconstructed_payload_b: ",reconstructed_payload_b)
        #icmp_payload_b = str(line_w_payload.split("\t")[9], 'utf-8')

        reconstructed_payload_b_len = len(reconstructed_payload_b)
        print("extract_ip_payload_v: payload_len:",reconstructed_payload_b_len)

        if reconstructed_payload_b_len == 0: 
            continue
        
        if reconstructed_payload_b_len < nb_starting_character_to_remove:
            print(f"Not enough extra starting bytes to remove in payload ({reconstructed_payload_b_len} < {nb_starting_character_to_remove})")
            exit(-1)

        # remove extra bytes
        ## we first remove starting extra bytes
        payload_no_starting_extra_char_b = reconstructed_payload_b if nb_starting_character_to_remove == 0 else reconstructed_payload_b[nb_starting_character_to_remove:]
        print("extract_ip_payload_v: payload_no_starting_extra_char_b: ",payload_no_starting_extra_char_b)

        ## we hypothesize we can remove ending extra bytes
        if nb_final_character_to_remove == 0:
            payload_no_extra_char_b = payload_no_starting_extra_char_b 
        else:
            payload_no_extra_char_b = payload_no_starting_extra_char_b[:-nb_final_character_to_remove]
        print("extract_ip_payload_v: payload_no_extra_char_b: ",payload_no_extra_char_b)

        # from bytes to ascii
        payload_ascii_s = payload_no_extra_char_b.decode('utf-8','replace')
        print("extract_ip_payload_v: payload_ascii_s: ",payload_ascii_s)
        
        icmp_payload_v.append(payload_ascii_s)

    payload_no_duplicates_v = list(dict.fromkeys(icmp_payload_v))
    if len(payload_no_duplicates_v) == 1:
        return { "is_echo_reply": True, "number": 1, "payload": payload_no_duplicates_v[0] }
    elif len(payload_no_duplicates_v) > 1:
        return { "is_echo_reply": True, "number": len(payload_no_duplicates_v), "payload": "" }

    return { "is_echo_reply": False, "number": 0, "payload": "" }
    


def extract_data(
        log_path:str, 
        nb_starting_character_to_remove: int,
        nb_final_character_to_remove: int,
        protocol:str, 
  ):
    print("\n\nextract_data: log_path:",log_path)
    print("\n\nextract_data: protocol:",protocol)
    
    test_case_index = extract_test_case_index(log_path)
    print("extract_data: test_case_index: ",test_case_index)

    with open(log_path) as f:
        content = f.readlines()

    if content == []:
        print("extract_data: empty log_path")
        return { "is_echo_reply": False, "number": 0, "payload": "" }
    
    # remove whitespace characters like `\n` at the end of each line
    line_l = [ x.strip() for x in content ]
    
    if protocol == "tcp":
        json_entry = extract_tcp_payload_v(
            line_l, 
            nb_starting_character_to_remove, 
            nb_final_character_to_remove, 
            test_case_index
        )
    elif protocol == "ipv4" or protocol == "ipv6":
        json_entry = extract_ip_payload_v(
            line_l, 
            nb_starting_character_to_remove, 
            nb_final_character_to_remove, 
            test_case_index
        )
    else:
        print("Bad protocol provided")
        exit(-1)

    print("extract_data: json_entry: ",json_entry)
    return json_entry
 

def extract_test_case_index(file_path: str):
    print("extract_test_case_index: start")
    print("extract_test_case_index: file_path: ",str(file_path))
    bn = os.path.basename(file_path)
    print("extract_test_case_index: bn: ",str(bn))
    bn_wo_ext = os.path.splitext(bn)[0]
    print("extract_test_case_index: bn_wo_ext: ",str(bn_wo_ext))
    index = bn_wo_ext.split("_")[-1]
    print("extract_test_case_index: index: ",str(index))
    print("extract_test_case_index: start")
    return int(index)

def process(
    log_directory: str,
    json_output_path: str, 
    protocol:str,
    nb_starting_character_to_remove: int,
    nb_final_character_to_remove: int
):
    print("process: start")
    filename_l = os.listdir(log_directory)
    
    path_l = [ os.path.join(log_directory, filename) for filename in filename_l ]

    # We sort to have a deterministic processing order to ease debugging accross machines.
    path_l.sort()

    index_payload_d = { 
        extract_test_case_index(log_path): 
        extract_data(
            log_path,
            nb_starting_character_to_remove,
            nb_final_character_to_remove,
            protocol,
        ) for log_path in path_l 
    }

    index_i_payload_d = { int(index):payload_d for index,payload_d in index_payload_d.items() }
    data_d = { "hm": index_i_payload_d }

    print("process: data_d: ",data_d)

    with open(json_output_path, 'w') as fp:
        json.dump(data_d, fp, indent=2, sort_keys=True)
    print("process: end")

def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--log-directory", type=str, default="")
    parser.add_argument("-m", "--merged-byte-time-sequence-json-path", type=str, default="")
    parser.add_argument("-j", "--json-path", type=str, default="")
    parser.add_argument("-p", "--protocol", type=str, default="")
    parser.add_argument("-s", "--nb-starting-character-to-remove", type=int, default=0)
    parser.add_argument("-f", "--nb-final-character-to-remove", type=int, default=0)
    parser.add_argument("-pm", "--payload-mode", type=str, choices=['vc1b','icfl8b','icvl8i4','icvl8i6'])
    parser.add_argument("-em", "--extraction-mode", type=str, default="")
    args = parser.parse_args()

    log_directory = args.log_directory
    json_path = args.json_path
    protocol = args.protocol
    nb_starting_character_to_remove = args.nb_starting_character_to_remove
    payload_mode = args.payload_mode
    nb_final_character_to_remove = args.nb_final_character_to_remove

    print('extract_payload_from_zeek_log_directory_to_json: log_directory: "%s"'%(log_directory))
    print('extract_payload_from_zeek_log_directory_to_json: json_path: "%s"'%(json_path))
    print('extract_payload_from_zeek_log_directory_to_json: protocol: "%s"'%(protocol))
    print('extract_payload_from_zeek_log_directory_to_json: nb_starting_character_to_remove: "%s"'%(nb_starting_character_to_remove))
    print('extract_payload_from_zeek_log_directory_to_json: payload_mode: "%s"'%(payload_mode))
    print('extract_payload_from_zeek_log_directory_to_json: nb_final_character_to_remove: "%s"'%(nb_final_character_to_remove))
    
    process(log_directory,
            json_path,
            protocol,
            nb_starting_character_to_remove,
            nb_final_character_to_remove
    )

if __name__ == "__main__":
   main(sys.argv[1:])