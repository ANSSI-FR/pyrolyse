use serde::{Deserialize, Serialize, Serializer};
use std::collections::hash_map::Iter;
use std::collections::{BTreeMap, HashMap};
use std::iter::Iterator;
use std::iter::FromIterator;

use std::net::IpAddr;
use std::net::IpAddr::{V4,V6};
use pnet::util::checksum; 
use crate::itertools::Itertools;
use hex::FromHex;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatternD {
  ascii_v_v: Vec<Vec<u8>>,
  string_v: Vec<String>
}

impl PatternD {

  pub fn new() -> PatternD {
    PatternD {
      ascii_v_v: Vec::<Vec<u8>>::new(),
      string_v: Vec::<String>::new(),
    }
  }

  //pub fn build_simple_chunk_pattern_v() -> PatternD {
  pub fn of_simple_chunk_pattern_v() -> PatternD {
    let string_v = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
        "E".to_string(),
        "F".to_string(),
        "G".to_string(),
        "H".to_string(),
        "I".to_string(),
        "J".to_string(),
        "K".to_string(),
    ];

    let ascii_v_v: Vec<Vec<u8>> = string_v
        .iter()
        .map(|s| s.clone().into_bytes() )
        .collect();

    PatternD { 
      ascii_v_v,
      string_v
    }
  }

  //pub fn build_internet_checksum_chunk_pattern_v() -> PatternD {
  pub fn of_internet_checksum_chunk_pattern_v() -> PatternD {
    // XXX check if there is not a fastest way
    let chunk_u16_v: Vec<String> = vec![
        "AA".to_string(),
        "BB".to_string(),
        "CC".to_string(),
        "DD".to_string(),
    ];
    let string_v: Vec<String> = chunk_u16_v
        .iter()
        .permutations(4)
        .map(|v| {
            let s_v: Vec<String> = v.iter().map(|s| (**s).clone()).collect();
            s_v.join("")
        })
        .collect();

    let ascii_v_v: Vec<Vec<u8>> = string_v
        .iter()
        .map(|s| s.clone().into_bytes() )
        .collect();

    PatternD { 
      ascii_v_v,
      string_v
    }
  } 

  pub fn len(&self) -> usize {
    assert!(self.ascii_v_v.len() == self.string_v.len());
    self.ascii_v_v.len()
  }

  //pub fn get_ascii_v(&self, index: &u16) -> Option<&Vec<u8>> {
  //  let index_us = *index as usize;
  //  match index_us {
  //    n if (0..=self.ascii_v_v.len() - 1).contains(&n) => Some(&self.ascii_v_v[index_us]),
  //    _ => None
  //  }
  //}

  pub fn get_ascii_v_v(&self) -> &Vec<Vec<u8>> {
    &self.ascii_v_v
  }

  pub fn get_string_v(&self) -> &Vec<String> {
    &self.string_v
  }

  //pub fn to_fake_payload_v(&self) -> Vec<u8> {
  //    (self.0).clone().into_iter().flatten().collect::<Vec<u8>>()
  //}
  
  pub fn to_fake_payload_ascii_v(&self) -> Vec<u8> {
    self.ascii_v_v.clone().into_iter().flatten().collect::<Vec<u8>>()
  }

  pub fn to_fake_payload_string(&self) -> String {
    self.string_v.clone().join("")
}

}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// Hashmap key is the chunk id and value are its corresponding patterns
// Hashmap u16::MAX key correspond to End extra chunk
pub struct ChunkBasedPatternC {
    #[serde(serialize_with = "ordered_map")]
    hm: HashMap<u16, PatternD>,
}

fn ordered_map<S>(
  value: &HashMap<u16, PatternD>,
  serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let ordered: BTreeMap<_, _> = value.iter().collect();
  ordered.serialize(serializer)
}

impl FromIterator<(u16, PatternD)> for ChunkBasedPatternC {
  fn from_iter<U>(iter: U) -> Self
  where
      U: IntoIterator<Item = (u16, PatternD)>,
  {
      Self {
          hm: HashMap::from_iter(iter),
      }
  }
}

impl ChunkBasedPatternC {
  pub fn len(&self) -> usize {
      self.hm.len()
  }

  pub fn iter(&self) -> Iter<u16, PatternD> {
    self.hm.iter()
  }

  pub fn is_empty(&self) -> bool {
      self.hm.is_empty()
  }

  pub fn get(&self, index: &u16) -> Option<&PatternD> {
      self.hm.get(index)
  }

  pub fn get_end_chunk_pattern_d(&self) -> Option<&PatternD> {
    self.hm.get(&u16::MAX)
  }

  pub fn new() -> ChunkBasedPatternC {
    ChunkBasedPatternC { hm: HashMap::new() }
  }

  pub fn of_invariant_checksum(
    chunk_number: u16,
    pattern_number: u16,
    ip_addr: IpAddr, // TODO: replace by bool or enum IpAddr type v4 or v6
  ) -> ChunkBasedPatternC {
    // We ensure that pattern id composed with 3 chunk id-related digits and 3 pattern offset-related digits 
    let max_allowed_chunk_number = 999;
    let max_allowed_pattern_number = 999;
    assert!(chunk_number <= max_allowed_chunk_number && pattern_number <= max_allowed_pattern_number);

    // the pattern "contribution" to the overall checksum is null
    let pattern_offset_v: Vec<_> = (0..pattern_number).collect();
    debug!("of_data: pattern_offset_v: {:?}", pattern_offset_v);


    let ffff_u8_v = <[u8; 2]>::from_hex("ffff").unwrap();
    debug!("of_data: ffff_u8_v: {:?}", ffff_u8_v);
    let ffff_u16 = ((ffff_u8_v[0] as u16) << 8) | ffff_u8_v[1] as u16;
    debug!("of_data: ffff_u16: {:?}", ffff_u16);
    let fff7_u8_v = <[u8; 2]>::from_hex("fff7").unwrap();
    debug!("of_data: fff7_u8_v: {:?}", fff7_u8_v);
    let fff7_u16 = ((fff7_u8_v[0] as u16) << 8) | fff7_u8_v[1] as u16;
    debug!("of_data: fff7_u16: {:?}", fff7_u16);

    let mut hm: HashMap<u16,PatternD> = (0..chunk_number).map(|chunk_id| {
        let chunk_id_s = format!("{:0>3}", chunk_id);
        debug!("of_data: chunk_id_s: {:?}", chunk_id_s);

        let (ascii_v_v,string_v): (Vec<Vec<u8>>,Vec<String>) = pattern_offset_v
          .iter()
          .map(|pattern_offset| {
            //let pattern_offset_s = format!("{:0>3}", pattern_offset);
            //let six_byte_pattern_s = format!("{}{}", chunk_id_s, pattern_offset_s);
            //let six_byte_pattern_b = six_byte_pattern_s.as_bytes();
//
            //let curr_checksum = checksum(six_byte_pattern_b,usize::MAX);
//
            //let payload_length_correction_in_checksum = match ip_addr {
            //  V4(_) => 0,
            //  V6(_) => 8
            //};
            //let two_byte_checksum_correction_u16: u16 = if target_checksum + payload_length_correction_in_checksum > curr_checksum {
            //   target_checksum + payload_length_correction_in_checksum - curr_checksum
            //} else { curr_checksum - target_checksum - payload_length_correction_in_checksum };
//
            //let two_byte_checksum_correction_u8: &[u8] = &two_byte_checksum_correction_u16.to_be_bytes();
            //debug!("of_data: two_byte_checksum_correction_u8: {:?}", two_byte_checksum_correction_u8);
            //debug!("of_data: two_byte_checksum_correction_u8.len(): {}", two_byte_checksum_correction_u8.len());
//
            //let ascii_v: Vec<u8> = [six_byte_pattern_b, two_byte_checksum_correction_u8].concat();
            //debug!("of_data: ascii_v: {:?}", ascii_v);
            //
            //let string: String = ascii_v.iter().map(|a| *a as char).collect();

            let pattern_offset_s = format!("{:0>3}", pattern_offset);
            let six_byte_pattern_s = format!("{}{}", chunk_id_s, pattern_offset_s);
            let six_byte_pattern_b = six_byte_pattern_s.as_bytes();

            // To get the ones' complement sum of a variable, we use !pnet::util::checksum() (i.e. the ones' complement of the ones' complement of the ones' complement sum)  
            let pattern_id_ones_complement_u16 = !checksum(six_byte_pattern_b,usize::MAX);

            let two_byte_checksum_correction_u16: u16 = match ip_addr {
                V4(_) => ffff_u16 - pattern_id_ones_complement_u16,
                V6(_) => {
                  let r_u32: u32 = fff7_u16 as u32 + ffff_u16 as u32 - pattern_id_ones_complement_u16 as u32;
                  let r_u8_v = [(r_u32 >> 24) as u8, (r_u32 >> 16) as u8, (r_u32 >> 8) as u8, r_u32 as u8];
                  !checksum(&r_u8_v,usize::MAX)
                } 
            };
            
            let two_byte_checksum_correction_u8: &[u8] = &two_byte_checksum_correction_u16.to_be_bytes();
            debug!("of_data: two_byte_checksum_correction_u8: {:?}", two_byte_checksum_correction_u8);
            debug!("of_data: two_byte_checksum_correction_u8.len(): {}", two_byte_checksum_correction_u8.len());

            let ascii_v: Vec<u8> = [six_byte_pattern_b, two_byte_checksum_correction_u8].concat();
            debug!("of_data: ascii_v: {:?}", ascii_v);
            
            let string: String = ascii_v.iter().map(|a| *a as char).collect();

            (ascii_v,string)
          })
          .unzip();
        
        (
            chunk_id,
            PatternD {
              ascii_v_v,
              string_v
            }
        )
    })
    .collect();

    // adding End chunk pattern
    let six_first_bytes_s: String = "TheEnd".to_string();
    let six_first_bytes_b: &[u8] = six_first_bytes_s.as_bytes();
    // To get the ones' complement sum of a variable, we use !pnet::util::checksum() (i.e. the ones' complement of the ones' complement of the ones' complement sum)  
    let pattern_id_ones_complement_u16 = !checksum(six_first_bytes_b,usize::MAX);
    let two_last_bytes_u16: u16 = match ip_addr {
        V4(_) => ffff_u16 - pattern_id_ones_complement_u16,
        V6(_) => {
          let r_u32: u32 = fff7_u16 as u32 + ffff_u16 as u32 - pattern_id_ones_complement_u16 as u32;
          let r_u8_v = [(r_u32 >> 24) as u8, (r_u32 >> 16) as u8, (r_u32 >> 8) as u8, r_u32 as u8];
          !checksum(&r_u8_v,usize::MAX)
        } 
    };

    let end_ascii_v: Vec<u8> = [six_first_bytes_b, &two_last_bytes_u16.to_be_bytes()].concat();
    let end_string_v: Vec<String> = vec![end_ascii_v.iter().map(|a| *a as char).collect::<String>()];
    let end_ascii_v_v: Vec<Vec<u8>> = vec![end_ascii_v];
    let end_pattern_d = PatternD {
        ascii_v_v: end_ascii_v_v,
        string_v: end_string_v
    };
    hm.insert(u16::MAX, end_pattern_d);

    
    ChunkBasedPatternC { hm }
  }

  //pub fn to_fake_payload_v(
  //    &self,
  //) -> Vec<u8> {
  //    let payload_v_v: Vec<Vec<u8>> = self.hm
  //      .iter()
  //      .map(|(_,pattern_v)| { pattern_v.0.first().unwrap().clone() })
  //      .collect();
  //    
  //    payload_v_v.into_iter().flatten().collect::<Vec<u8>>()
  //}

  pub fn to_fake_payload_ascii_v(
    &self,
  ) -> Vec<u8> {
      let any_pattern_d = self
        .get(&0)
        .unwrap();

      let fake_v = any_pattern_d.ascii_v_v.clone().into_iter().flatten().collect::<Vec<u8>>();
      
      debug!("to_fake_payload_ascii_v: fake_v: {:?}", fake_v);
      fake_v
  }

  pub fn to_fake_payload_string(
    &self,
  ) -> String {
      self.hm
        .iter()
        .map(|(_,pattern_v)| { pattern_v.string_v.first().unwrap().clone() })
        .collect::<Vec<_>>()
        .join("")
  }

}