use std::fs;
use std::io;
use std::collections::HashMap;
use std::path::Path;
use hex;

use crate::position::pattern::{PatternD,ChunkBasedPatternC};

pub fn build_snort_suricata_sig_from_pattern_d(
  pattern_d: &PatternD,
  //protocol_s: &str,
  matching_protocol_s: &str,
  reassembly_options_s: &str,
  matching_buffer_s: &str,
  matching_src_ip_range_s: &str,
  matching_dst_ip_range_s: &str,
  matching_src_port_range_s: &str,
  matching_dst_port_range_s: &str,
  rev_s: &str,
  sid_offset: usize
) -> Vec<String> {
  
  let string_v = pattern_d.get_string_v(); 
  let ascii_v_v = pattern_d.get_ascii_v_v(); 
  string_v.iter()
    .enumerate()
    .zip(ascii_v_v.iter())
    .map(|((i,pattern_s),ascii_v)| {
        debug!("build_zeek_sig_from_pattern_d: pattern_s: {}",pattern_s);
        let pattern_hex = hex::encode_upper(ascii_v);
        debug!("build_zeek_sig_from_pattern_d: pattern_hex: {}",pattern_hex);
        let pattern_hex_s = String::from(pattern_hex);
        debug!("build_zeek_sig_from_pattern_d: pattern_hex_s: {}",pattern_hex_s);
      
        let modified_pattern_hex_s_v: Vec<_> = pattern_hex_s
          .chars()
          .step_by(2)
          .enumerate()
          .map(|(i,_c)| {
            format!("{}",pattern_hex_s[i*2..i*2+2].to_string())
          })
          .collect();
        debug!("build_zeek_sig_from_pattern_d: modified_pattern_hex_s_v: {:?}",modified_pattern_hex_s_v);
        
        let modified_pattern_hex_s = modified_pattern_hex_s_v.join(" ");
        debug!("build_zeek_sig_from_pattern_d: modified_pattern_hex_s: {}",modified_pattern_hex_s);

        format!("alert {} {} {} -> {} {} (content: \"|{}|\"; msg:\"{}\"; {} {}; sid: {}; rev: {};)",
          matching_protocol_s,
          matching_src_ip_range_s,
          matching_src_port_range_s,
          matching_dst_ip_range_s,
          matching_dst_port_range_s,
          //pattern_s,
          modified_pattern_hex_s,
          //pattern_s,
          modified_pattern_hex_s, // changed from pattern_s to modified_pattern_hex_s since non-ascii char can cause parsing bugs
          reassembly_options_s,
          matching_buffer_s,
          i + sid_offset,
          rev_s
      )
    })
    .collect()

}

pub fn build_snort_suricata_sig_from_chunk_based_pattern_c(
  chunk_based_pattern_c: &ChunkBasedPatternC,
  //protocol_s: &str,
  matching_protocol_s: &str,
  reassembly_options_s: &str,
  matching_buffer_s: &str,
  matching_src_ip_range_s: &str,
  matching_dst_ip_range_s: &str,
  matching_src_port_range_s: &str,
  matching_dst_port_range_s: &str,
  rev_s: &str,
) -> Vec<String> {
  
  let mut sid_offset = 1;
  let string_v_v: Vec<Vec<_>> = chunk_based_pattern_c
    .iter()
    .map(|(_j,pattern_d)| {
        let string_v = build_snort_suricata_sig_from_pattern_d(
          pattern_d,
          //protocol_s,
          matching_protocol_s,
          reassembly_options_s,
          matching_buffer_s,
          matching_src_ip_range_s,
          matching_dst_ip_range_s,
          matching_src_port_range_s,
          matching_dst_port_range_s,
          rev_s,
          sid_offset
        );
        sid_offset += string_v.len();

        string_v
    })
    .collect();
    
    string_v_v.into_iter().flatten().collect()
}


pub fn build_snort_suricata_sig(
  ids_code_path_s: &str,
  ids_s: &str,
  vc1b_pattern_d: &PatternD,
  icfl8b_pattern_d: &PatternD,
  icvl8i4_pattern_c: &ChunkBasedPatternC,
  icvl8i6_pattern_c: &ChunkBasedPatternC,
) -> io::Result<()> {
  debug!("build_snort_suricata_sig: start");

  let ip_variable_fields_hm: HashMap<&str, HashMap<&str,&str>> = HashMap::from([
    ( "icmp-default", HashMap::from([
        ("matching_protocol", "icmp"), 
        ("reassembly_options", "")
      ])
    ),
    ( "flow-only-frag", HashMap::from([
      ("matching_protocol", "ip"), 
      ("reassembly_options", "flow: only_frag;")
    ])
  )]);
  let tcp_variable_fields_hm: HashMap<&str, HashMap<&str,&str>> = HashMap::from([
    ( "flow-only-stream", HashMap::from([
      ("matching_protocol", "tcp"), 
      ("reassembly_options", "flow: only_stream;")
      ])
  )]);

  let matching_buffer_s = "pkt_data";
  let matching_src_ip_range_s = "[192.168.20.10,192.168.56.0/24,fe80::200:ff:fe02:10]";
  let matching_dst_ip_range_s = "any";
  let matching_src_port_range_s = "any";
  let matching_dst_port_range_s = "any";
  let rev_s = "7";

  let testing_protocol_variable_fields_hm = HashMap::from([
    ("ipv4",&ip_variable_fields_hm), 
    ("ipv6",&ip_variable_fields_hm), 
    ("tcp",&tcp_variable_fields_hm), 
  ]);
  debug!("build_snort_suricata_sig: testing_protocol_variable_fields_hm: {:?}",testing_protocol_variable_fields_hm);

  testing_protocol_variable_fields_hm
    .iter()
    .for_each(|(testing_protocol_s, variable_fields_hm)| {
      let common_filename_part_s = format!("{}_{}",ids_s,testing_protocol_s);

      variable_fields_hm.iter()
        .for_each(|(rule_suffix_name,field_hm)| {
          let matching_protocol_s = field_hm.get("matching_protocol").unwrap();
          let reassembly_options_s = field_hm.get("reassembly_options").unwrap();

          // vc1b chunks
          let vc1b_sig_v = build_snort_suricata_sig_from_pattern_d(
            vc1b_pattern_d,
            //testing_protocol_s,
            matching_protocol_s,
            reassembly_options_s,
            matching_buffer_s,
            matching_src_ip_range_s,
            matching_dst_ip_range_s,
            matching_src_port_range_s,
            matching_dst_port_range_s,
            rev_s,
            1
          );
          let vc1b_sig_path_s = format!("{}/{}_base/conf/{}_s_{}.rules",
            ids_code_path_s,
            ids_s,
            common_filename_part_s, 
            rule_suffix_name
          );
          debug!("vc1b_sig_path_s: {}", vc1b_sig_path_s);
          let simple_sig_path = Path::new(&vc1b_sig_path_s);
          fs::write(simple_sig_path, vc1b_sig_v.join("\n"))
            .expect("Unable to write in file");

          // internet chunk patterns
          let icfl8b_sig_v = build_snort_suricata_sig_from_pattern_d(
            icfl8b_pattern_d,
            //testing_protocol_s,
            matching_protocol_s,
            reassembly_options_s,
            matching_buffer_s,
            matching_src_ip_range_s,
            matching_dst_ip_range_s,
            matching_src_port_range_s,
            matching_dst_port_range_s,
            rev_s,
            1
          );
          let icfl8b_sig_path_s = format!("{}/{}_base/conf/{}_icfl8b_{}.rules",
            ids_code_path_s,
            ids_s,
            common_filename_part_s, 
            rule_suffix_name
          );
          debug!("icfl8b_sig_path_s: {}", icfl8b_sig_path_s);
          let icfl8b_sig_path = Path::new(&icfl8b_sig_path_s);
          fs::write(icfl8b_sig_path, icfl8b_sig_v.join("\n"))
            .expect("Unable to write in file");

          if *testing_protocol_s == "ipv4" || *testing_protocol_s == "tcp" {
            // ipv4 invariant chunk patterns
            let icvl8i4_sig_v = build_snort_suricata_sig_from_chunk_based_pattern_c(
              icvl8i4_pattern_c,
              //testing_protocol_s,
              matching_protocol_s,
              reassembly_options_s,
              matching_buffer_s,
              matching_src_ip_range_s,
              matching_dst_ip_range_s,
              matching_src_port_range_s,
              matching_dst_port_range_s,
              rev_s,
            );
            let icvl8i4_sig_path_s = format!("{}/{}_base/conf/{}_icvl8i4_{}.rules",
              ids_code_path_s,
              ids_s,
              common_filename_part_s, 
              rule_suffix_name
            );
            debug!("icvl8i4_sig_path_s: {}", icvl8i4_sig_path_s);
            let icvl8i4_sig_path = Path::new(&icvl8i4_sig_path_s);
            fs::write(icvl8i4_sig_path, icvl8i4_sig_v.join("\n"))
              .expect("Unable to write in file");
          }

          if *testing_protocol_s == "ipv6" || *testing_protocol_s == "tcp" {
            // ipv6 invariant chunk patterns
            let icvl8i6_sig_v = build_snort_suricata_sig_from_chunk_based_pattern_c(
              icvl8i6_pattern_c,
              //testing_protocol_s,
              matching_protocol_s,
              reassembly_options_s,
              matching_buffer_s,
              matching_src_ip_range_s,
              matching_dst_ip_range_s,
              matching_src_port_range_s,
              matching_dst_port_range_s,
              rev_s,
            );
            let icvl8i6_sig_path_s = format!("{}/{}_base/conf/{}_icvl8i6_{}.rules",
              ids_code_path_s,
              ids_s,
              common_filename_part_s, 
              rule_suffix_name
            );
            debug!("icvl8i6_sig_path_s: {}", icvl8i6_sig_path_s);
            let icvl8i6_sig_path = Path::new(&icvl8i6_sig_path_s);
            fs::write(icvl8i6_sig_path, icvl8i6_sig_v.join("\n"))
              .expect("Unable to write in file");
          }
      });

  });

  debug!("build_snort_suricata_sig: end");
  Ok(())
}


pub fn build_zeek_sig_from_pattern_d(
  pattern_d: &PatternD,
  protocol_s: &str,
  matching_protocol_s: &str,
  matching_src_ip_range_s: &str,
  sid_offset: usize
) -> Vec<String> {

  
  //let string_v = pattern_d.get_string_v(); 
  //string_v.iter()
  //  .enumerate()
  //  .map(|(i,pattern_s)| {
  //    format!(
  //        "signature {}-{} {{ \n\
  //          \tip-proto == {} \n\
  //          \tsrc-ip == {} \n\
  //          \tpayload == /*{}*/ \n\
  //          \tevent \"Found {} using signature from client\" \n\
  //        }} \n\n",
  //        protocol_s,
  //        sid_offset + i,
  //        matching_protocol_s,
  //        matching_src_ip_range_s,
  //        pattern_s,
  //        pattern_s,
  //      )
  //  })
  //  .collect()

  let string_v = pattern_d.get_string_v(); 
  let ascii_v_v = pattern_d.get_ascii_v_v(); 
  string_v.iter()
    .enumerate()
    .zip(ascii_v_v.iter())
    .map(|((i,pattern_s),ascii_v)| {
      debug!("build_zeek_sig_from_pattern_d: pattern_s: {}",pattern_s);
      let pattern_hex = hex::encode_upper(ascii_v);
      debug!("build_zeek_sig_from_pattern_d: pattern_hex: {}",pattern_hex);
      let pattern_hex_s = String::from(pattern_hex);
      debug!("build_zeek_sig_from_pattern_d: pattern_hex_s: {}",pattern_hex_s);
    
      let modified_pattern_hex_s_v: Vec<_> = pattern_hex_s
        .chars()
        .step_by(2)
        .enumerate()
        .map(|(i,_c)| {
          format!("\\x{}",pattern_hex_s[i*2..i*2+2].to_string())
        })
        .collect();
      debug!("build_zeek_sig_from_pattern_d: modified_pattern_hex_s_v: {:?}",modified_pattern_hex_s_v);
      let modified_pattern_hex_s = modified_pattern_hex_s_v.join("");
      debug!("build_zeek_sig_from_pattern_d: modified_pattern_hex_s: {}",modified_pattern_hex_s);

      format!(
          "signature {}-{} {{ \n\
            \tip-proto == {} \n\
            \tsrc-ip == {} \n\
            \tpayload /.*{}.*/ \n\
            \tevent \"Found {} using signature from client\" \n\
          }} \n\n",
          protocol_s,
          sid_offset + i,
          matching_protocol_s,
          matching_src_ip_range_s,
          //pattern_s,
          modified_pattern_hex_s,
          pattern_s,
        )
    })
    .collect()
}

pub fn build_zeek_sig_from_chunk_based_pattern_c(
  chunk_based_pattern_c: &ChunkBasedPatternC,
  protocol_s: &str,
  matching_protocol_s: &str,
  matching_src_ip_range_s: &str,
) -> Vec<String> {
  
  let mut sid_offset = 1;
  let string_v_v: Vec<Vec<_>> = chunk_based_pattern_c
    .iter()
    .map(|(_j,pattern_d)| {
      let string_v = build_zeek_sig_from_pattern_d(
          pattern_d,
          protocol_s,
          matching_protocol_s,
          matching_src_ip_range_s,
          sid_offset
        );
        sid_offset += string_v.len();

        string_v
    })
    .collect();
    
    string_v_v.into_iter().flatten().collect()
}

pub fn build_zeek_sig(
  ids_code_path_s: &str,
  vc1b_pattern_d: &PatternD,
  icfl8b_pattern_d: &PatternD,
  icvl8i4_pattern_c: &ChunkBasedPatternC,
  icvl8i6_pattern_c: &ChunkBasedPatternC,
) -> io::Result<()> {

  debug!("build_zeek_sig: start");

  let matching_src_ip_s = "192.168.20.10,192.168.56.0/24,[fe80::200:ff:fe02:10]";

  let testing_and_matching_protocol_v = vec!(
    ("ipv4","icmp"), 
    ("ipv6","icmp6"), 
    ("tcp","tcp"), 
  );

  testing_and_matching_protocol_v
    .iter()
    .for_each(|(testing_protocol_s,matching_protocol_s)| {
      let common_filename_part_s = format!("zeek_{}",testing_protocol_s);
        
      // simple chunks
      let vc1b_sig_v = build_zeek_sig_from_pattern_d(
        vc1b_pattern_d,
        testing_protocol_s,
        matching_protocol_s,
        matching_src_ip_s,
        1
      );
      let vc1b_sig_path_s = format!("{}/zeek_base/conf/{}_vc1b_{}.sig",
        ids_code_path_s,
        common_filename_part_s, 
        "default"
      );
      debug!("vc1b_sig_path_s: {}", vc1b_sig_path_s);
      let simple_sig_path = Path::new(&vc1b_sig_path_s);
      fs::write(simple_sig_path, vc1b_sig_v.join("\n"))
        .expect("Unable to write in file");

      // internet chunk patterns
      let icfl8b_sig_v = build_zeek_sig_from_pattern_d(
        icfl8b_pattern_d,
        testing_protocol_s,
        matching_protocol_s,
        matching_src_ip_s,
        1
      );
      let icfl8b_sig_path_s = format!("{}/zeek_base/conf/{}_icfl8b_{}.sig",
        ids_code_path_s,
        common_filename_part_s, 
        "default"
      );
      debug!("icfl8b_sig_path_s: {}", icfl8b_sig_path_s);
      let icfl8b_sig_path = Path::new(&icfl8b_sig_path_s);
      fs::write(icfl8b_sig_path, icfl8b_sig_v.join("\n"))
        .expect("Unable to write in file");

      if *testing_protocol_s == "ipv4" || *testing_protocol_s == "tcp" {
        // ipv4 invariant chunk patterns
        let icvl8i4_sig_v = build_zeek_sig_from_chunk_based_pattern_c(
          icvl8i4_pattern_c,
          testing_protocol_s,
          matching_protocol_s,
          matching_src_ip_s,
        );
        let icvl8i4_sig_path_s = format!("{}/zeek_base/conf/{}_icvl8i4_{}.sig",
          ids_code_path_s,
          common_filename_part_s, 
          "default"
        );
        debug!("icvl8i4_sig_path_s: {}", icvl8i4_sig_path_s);
        let icvl8i4_sig_path = Path::new(&icvl8i4_sig_path_s);
        fs::write(icvl8i4_sig_path, icvl8i4_sig_v.join("\n"))
          .expect("Unable to write in file");
      }

      if *testing_protocol_s == "ipv6" || *testing_protocol_s == "tcp" {
        // ipv6 invariant chunk patterns
        let icvl8i6_sig_v = build_zeek_sig_from_chunk_based_pattern_c(
          icvl8i6_pattern_c,
          testing_protocol_s,
          matching_protocol_s,
          matching_src_ip_s,
        );
        let icvl8i6_sig_path_s = format!("{}/zeek_base/conf/{}_icvl8i6_{}.sig",
          ids_code_path_s,
          common_filename_part_s, 
          "default"
        );
        debug!("icvl8i6_sig_path_s: {}", icvl8i6_sig_path_s);
        let icvl8i6_sig_path = Path::new(&icvl8i6_sig_path_s);
        fs::write(icvl8i6_sig_path, icvl8i6_sig_v.join("\n"))
          .expect("Unable to write in file");
      }
    });


  debug!("build_zeek_sig: end");
  Ok(())
  
}

