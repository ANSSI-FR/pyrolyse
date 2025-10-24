use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use std::iter::Iterator;
use std::collections::btree_map::Keys;
use std::cmp::max;

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::misc::interval::IntervalC;
use crate::misc::interval::IntervalD;
use crate::position::pattern::{PatternD,ChunkBasedPatternC};
use crate::position::payload_mode::PayloadMode;
//use std::net::Ipv4Addr;
//use std::net::Ipv6Addr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Getters)]
pub struct ChunkD {
    index: u16,
    start: bool,
    // Used for the more_fragment field of IP fragmentation.
    //more_chunk: bool,
    /// Originally encoded on 13bits
    offset: u16,
    // Payload data with letters
    simple_s: String,
    // Payload data as ASCII code for letters
    simple_ascii_v: Vec<u8>,
    // Payload data with letters
    internet_checksum_s: String,
    // Payload data as ASCII code for letters
    internet_checksum_ascii_v: Vec<u8>,
    // Payload data with letters
    ipv4_invariant_checksum_s: String,
    // Payload data as ASCII code for letters
    ipv4_invariant_checksum_ascii_v: Vec<u8>,
    // Payload data with letters
    ipv6_invariant_checksum_s: String,
    // Payload data as ASCII code for letters
    ipv6_invariant_checksum_ascii_v: Vec<u8>,
}

impl ChunkD {
    pub fn new(
        index: u16,
        start: bool,
        offset: u16,
        simple_s: String,
        simple_ascii_v: Vec<u8>,
        internet_checksum_s: String,
        internet_checksum_ascii_v: Vec<u8>,
        ipv4_invariant_checksum_s: String,
        ipv4_invariant_checksum_ascii_v: Vec<u8>,
        ipv6_invariant_checksum_s: String,
        ipv6_invariant_checksum_ascii_v: Vec<u8>,
    ) -> ChunkD {
        ChunkD {
            index,
            start,
            offset,
            simple_s,
            simple_ascii_v,
            internet_checksum_s,
            internet_checksum_ascii_v,
            ipv4_invariant_checksum_s,
            ipv4_invariant_checksum_ascii_v,
            ipv6_invariant_checksum_s,
            ipv6_invariant_checksum_ascii_v,
        }
    }

    pub fn of_data(
        index: u16,
        simple_chunk_pattern_sl: &[u8],
        internet_checksum_chunk_pattern_sl: &[u8],
        internet_invariant_ipv4_checksum_chunk_pattern_sl: &[u8],
        internet_invariant_ipv6_checksum_chunk_pattern_sl: &[u8],
        chunk_pattern_offset: u16,
        interval_d: &IntervalD,
    ) -> ChunkD {
        // XXX introduce an offset multiplier
        let start = interval_d.get_start() == 0;

        let offset = interval_d.get_start();
        let interval_duration = interval_d.get_duration();

        //let simple_pattern_v = &simple_chunk_pattern_sl[chunk_pattern_offset as usize
        //    ..chunk_pattern_offset as usize + interval_duration as usize];
        //let simple_s = simple_pattern_v.join("");
        //let simple_ascii_v = simple_s.as_bytes().to_vec();
        let simple_ascii_v = (&simple_chunk_pattern_sl[chunk_pattern_offset as usize
        ..chunk_pattern_offset as usize + interval_duration as usize]).to_vec();
        let simple_s = String::from_utf8(simple_ascii_v.clone()).unwrap();
        debug!("of_data: simple_s: {:?}", simple_s);

        //let internet_checksum_pattern_v = &internet_checksum_chunk_pattern_sl[chunk_pattern_offset
        //    as usize
        //    ..chunk_pattern_offset as usize + interval_duration as usize];
        //let internet_checksum_s = internet_checksum_pattern_v.join("");
        //let internet_checksum_ascii_v = internet_checksum_s.as_bytes().to_vec();
        let internet_checksum_ascii_v = (&internet_checksum_chunk_pattern_sl[(chunk_pattern_offset * 8)
            as usize
            ..(chunk_pattern_offset * 8) as usize + (interval_duration * 8) as usize]).to_vec();
        // considering byte slice is a valid string slice
        let internet_checksum_s = String::from_utf8(internet_checksum_ascii_v.clone()).unwrap();
        debug!("of_data: internet_checksum_s: {:?}", internet_checksum_s);
        
        //let internet_invariant_ipv4_checksum_chunk_pattern_v = &internet_invariant_ipv4_checksum_chunk_pattern_sl[chunk_pattern_offset
        //    as usize
        //    ..chunk_pattern_offset as usize + interval_duration as usize];
        //debug!("of_data: internet_invariant_ipv4_checksum_chunk_pattern_v.len(): {}", internet_invariant_ipv4_checksum_chunk_pattern_v.len());
        //debug!("of_data: internet_invariant_ipv4_checksum_chunk_pattern_v: {:?}", internet_invariant_ipv4_checksum_chunk_pattern_v);
        //let ipv4_invariant_checksum_s = internet_invariant_ipv4_checksum_chunk_pattern_v.join("");
        //debug!("of_data: ipv4_invariant_checksum_s: {:?}", ipv4_invariant_checksum_s);
        //debug!("of_data: ipv4_invariant_checksum_s.len(): {}", ipv4_invariant_checksum_s.len());
        //let ipv4_invariant_checksum_ascii_v = ipv4_invariant_checksum_s.as_bytes().to_vec();
        //debug!("of_data: ipv4_invariant_checksum_ascii_v: {:?}", ipv4_invariant_checksum_ascii_v);
        let ipv4_invariant_checksum_ascii_v = (&internet_invariant_ipv4_checksum_chunk_pattern_sl[(offset * 8)
            as usize
            ..(offset * 8) as usize + (interval_duration * 8) as usize]).to_vec();
        let ipv4_invariant_checksum_s = ipv4_invariant_checksum_ascii_v.iter().map(|a| *a as char).collect();
        debug!("of_data: ipv4_invariant_checksum_s: {:?}", ipv4_invariant_checksum_s);

        //let internet_invariant_ipv6_checksum_chunk_pattern_v = &internet_invariant_ipv6_checksum_chunk_pattern_sl[chunk_pattern_offset
        //    as usize
        //    ..chunk_pattern_offset as usize + interval_duration as usize];
        //let ipv6_invariant_checksum_s = internet_invariant_ipv6_checksum_chunk_pattern_v.join("");
        //let ipv6_invariant_checksum_ascii_v = ipv6_invariant_checksum_s.as_bytes().to_vec();
        let ipv6_invariant_checksum_ascii_v = (&internet_invariant_ipv6_checksum_chunk_pattern_sl[(offset * 8)
            as usize
            ..(offset * 8) as usize + (interval_duration * 8) as usize]).to_vec();
        let ipv6_invariant_checksum_s = ipv6_invariant_checksum_ascii_v.iter().map(|a| *a as char).collect();
        debug!("of_data: ipv6_invariant_checksum_s: {:?}", ipv6_invariant_checksum_s);

        ChunkD::new(
            index,
            start,
            offset,
            simple_s,
            simple_ascii_v,
            internet_checksum_s,
            internet_checksum_ascii_v,
            ipv4_invariant_checksum_s,
            ipv4_invariant_checksum_ascii_v,
            ipv6_invariant_checksum_s,
            ipv6_invariant_checksum_ascii_v,
        )
    }

    pub fn get_index(&self) -> u16 {
        self.index
    }

    pub fn get_start(&self) -> bool {
        self.start
    }

    pub fn get_offset(&self) -> u16 {
        self.offset
    }

    pub fn get_simple_s(&self) -> String {
        self.simple_s.clone()
    }

    pub fn get_simple_ascii_v(&self) -> &Vec<u8> {
        &self.simple_ascii_v
    }

    pub fn get_internet_checksum_s(&self) -> String {
        self.internet_checksum_s.clone()
    }

    pub fn get_internet_checksum_ascii_v(&self) -> &Vec<u8> {
        &self.internet_checksum_ascii_v
    }

    pub fn get_ipv4_invariant_checksum_s(&self) -> String {
        self.ipv4_invariant_checksum_s.clone()
    }

    pub fn get_ipv4_invariant_checksum_ascii_v(&self) -> &Vec<u8> {
        &self.ipv4_invariant_checksum_ascii_v
    }

    pub fn get_ipv6_invariant_checksum_s(&self) -> String {
        self.ipv6_invariant_checksum_s.clone()
    }

    pub fn get_ipv6_invariant_checksum_ascii_v(&self) -> &Vec<u8> {
        &self.ipv6_invariant_checksum_ascii_v
    }

    pub fn get_chunk_pattern_ascii_v(
        &self,
        payload_mode: &PayloadMode 
    ) -> Vec<u8> {
        match payload_mode {
            PayloadMode::VariableChecksum1Byte(_) => self.simple_ascii_v.clone(),
            PayloadMode::InvariantChecksumFixedLength8Byte(_) => self.internet_checksum_ascii_v.clone(),
            PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(_) => self.ipv4_invariant_checksum_ascii_v.clone(),
            PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(_) => self.ipv6_invariant_checksum_ascii_v.clone(),
        }
    }

    // TODO: rename to get_offset_after_chunk?
    pub fn get_ending_offset(&self) -> u16 {
        let payload_len: u16 = (self.internet_checksum_ascii_v.len() as u16) / 8 ;
        self.offset + payload_len
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChunkC {
    bm: BTreeMap<u16, ChunkD>,
}

impl ChunkC {
    pub fn new(bm: BTreeMap<u16, ChunkD>) -> ChunkC {
        ChunkC { bm }
    }

    pub fn is_empty(&self) -> bool {
        self.bm.is_empty()
    }

    pub fn len(&self) -> usize {
        self.bm.len()
    }

    pub fn iter(&self) -> Iter<u16, ChunkD> {
        self.bm.iter()
    }

    pub fn get(&self, index: &u16) -> Option<&ChunkD> {
        self.bm.get(index)
    }

    pub fn keys(&self) -> Keys<u16, ChunkD> {
        self.bm.keys()
    }

    //pub fn set_more_fragment_v(
    //    more_fragment_mode: &MoreFragmentMode,
    //    interval_c: &IntervalC,
    //    temporal_position_sl: &[u16],
    //) -> Option<Vec<bool>> {
    //    match more_fragment_mode {
    //        MoreFragmentMode::RightmostFinishLeftOlder => {
    //            let last_data_index = interval_c.get_last_data_index();
    //            // We build the vector with index of interval who contain the last byte.
    //            let mut i_is_last_v = interval_c
    //            .iter()
    //            .filter(|(_index, interval_d)| interval_d.get_end() == last_data_index)
    //            .map(|(i, _)| *i as u32)
    //            .collect::<Vec<u32>>();
    //            debug!("more_fragment_v: i_is_last_v: {:?}", i_is_last_v);
    //            // If two interval are located at the end, we take the last one (temporally).
    //            // We sort the vector of final interval index using their temporal position.
    //            i_is_last_v.sort_by_key(|i| temporal_position_sl[*i as usize]);
    //            debug!("more_fragment_v: i_is_last_v sorted: {:?}", i_is_last_v);
    //            // The index without more fragment (MF) flag is the last one.
    //            let i_wo_mf = i_is_last_v[i_is_last_v.len() - 1];
    //            debug!("more_fragment_v: i_wo_mf: {:?}", i_wo_mf);
//
    //            // More fragment (MF) is always set except...
    //            let mut more_fragment_v = vec![true; interval_c.len()];
    //            // for the previously processed index.
    //            more_fragment_v[i_wo_mf as usize] = false;
    //            debug!("more_fragment_v: more_fragment_v: {:?}", more_fragment_v);
    //            Some(more_fragment_v)
    //        },
    //        MoreFragmentMode::First => {
    //            Some([ vec![false], vec![true; interval_c.len()-1]].concat())
    //            // let mut more_fragment_v = [vec![false; interval_c.len() - 1]].concat();
    //            // more_fragment_v.insert(0, true);
    //            // more_fragment_v
    //        },
    //        MoreFragmentMode::Second => {
    //            let mut more_fragment_v = vec![true; interval_c.len() - 1];
    //            more_fragment_v.insert(1, false);
    //            Some(more_fragment_v)
    //        },
    //        MoreFragmentMode::Third => {
    //            if interval_c.len() < 3 {
    //                eprintln!("More fragment mode impossible for pair test cases");
    //                exit(-1)
    //            };
    //            let mut more_fragment_v = vec![true; interval_c.len() - 1];
    //            more_fragment_v.insert(2,false);
    //            Some(more_fragment_v)
    //        },
    //        MoreFragmentMode::FirstSecond => {
    //            let mut more_fragment_v = vec![true; interval_c.len() - 2];
    //            more_fragment_v.insert(0, false);
    //            more_fragment_v.insert(1, false);
    //            Some(more_fragment_v)
    //        },
    //        MoreFragmentMode::FirstThird => {
    //            if interval_c.len() < 3 {
    //                eprintln!("More fragment mode impossible for pair test cases");
    //                exit(-1)
    //            };
    //            let mut more_fragment_v = vec![true; interval_c.len() - 2];
    //            more_fragment_v.insert(0, false);
    //            more_fragment_v.insert(2, false);
    //            Some(more_fragment_v)
    //        },
    //        MoreFragmentMode::SecondThird => {
    //            if interval_c.len() < 3 {
    //                eprintln!("More fragment mode impossible for pair test cases");
    //                exit(-1)
    //            };
    //            let mut more_fragment_v = vec![true; interval_c.len() - 2];
    //            more_fragment_v.insert(1, false);
    //            more_fragment_v.insert(2, false);
    //            Some(more_fragment_v)
    //        },
    //        MoreFragmentMode::FirstSecondThird => {
    //            if interval_c.len() < 3 {
    //                eprintln!("More fragment mode impossible for pair test cases");
    //                exit(-1)
    //            };
    //            let mut more_fragment_v = vec![true; interval_c.len() - 3];
    //            more_fragment_v.insert(1, false);
    //            more_fragment_v.insert(2, false);
    //            more_fragment_v.insert(3, false);
    //            Some(more_fragment_v)
    //        },
    //        MoreFragmentMode::All => {
    //            Some(vec![false; interval_c.len()])
    //        },
    //        MoreFragmentMode::Last => {
    //            let mut more_fragment_v = vec![true; interval_c.len() - 1];
    //            more_fragment_v.push(false);
    //            Some(more_fragment_v)
    //        },
    //        MoreFragmentMode::NoMoreFragmentField => {
    //            None
    //        },
    //    }
//
    //}

    pub fn of_data(
        simple_chunk_pattern_sl: &PatternD,
        internet_checksum_chunk_pattern_sl: &PatternD,
        ipv4_invariant_checksum_chunk_pattern_c: &ChunkBasedPatternC,
        ipv6_invariant_checksum_chunk_pattern_c: &ChunkBasedPatternC,
        interval_c: &IntervalC,
        //temporal_position_sl: &[u16],
    ) -> ChunkC {
        debug!("of_interval_c: start");
        debug!("of_interval_c: interval_c: {:?}", interval_c);

        let mut interval_v = interval_c.iter().collect::<Vec<_>>();
        interval_v.sort();
        debug!("of_interval_c: interval_v: {:?}", interval_v);

        // Build interval total length
        let length_v: Vec<u16> = interval_v
            .iter()
            .map(|(_index, interval_d)| interval_d.get_duration())
            .collect();
        // debug!(
        //     "of_interval_c: length_v ({}): {:?}",
        //     length_v.len(),
        //     length_v
        // );

        // Build offset
        let mut data_offset_v_tmp = length_v.iter().fold(vec![], |mut acc, length| {
            let offset: u16 = if acc.is_empty() {
                *length
            } else {
                acc[acc.len() - 1] + length
            };
            acc.append(&mut vec![offset]);
            acc
        });
        // debug!(
        //     "of_interval_c: data_offset_v_tmp ({}): {:?}",
        //     data_offset_v_tmp.len(),
        //     data_offset_v_tmp
        // );

        // Adding 0 for initial null offset
        let mut pattern_offset_v = vec![0];
        pattern_offset_v.append(&mut data_offset_v_tmp);
        // Removing last offset
        debug!(
            "of_interval_c: pattern_offset_v ({}): {:?}",
            pattern_offset_v.len(),
            pattern_offset_v
        );

        // let v = pattern_offset_v
        //     .iter()
        //     .zip(interval_c.iter())
        let bm: BTreeMap<_, _> = izip!(&pattern_offset_v, interval_v)
            .map(|(pattern_offset, (index, interval_d))| {
                //let ipv4_v = ipv4_invariant_checksum_chunk_pattern_c.get(index).unwrap().clone().into_iter().flatten().collect::<Vec<u8>>();
                //let ipv6_v = ipv6_invariant_checksum_chunk_pattern_c.get(index).unwrap().clone().into_iter().flatten().collect::<Vec<u8>>();
                let ipv4_pattern_d = ipv4_invariant_checksum_chunk_pattern_c.get(index).unwrap();
                let ipv6_pattern_d = ipv6_invariant_checksum_chunk_pattern_c.get(index).unwrap();
                (
                    *index,
                    ChunkD::of_data(
                        *index,
                        &simple_chunk_pattern_sl.to_fake_payload_ascii_v(),
                        &internet_checksum_chunk_pattern_sl.to_fake_payload_ascii_v(),
                        &ipv4_pattern_d.to_fake_payload_ascii_v(),
                        &ipv6_pattern_d.to_fake_payload_ascii_v(),
                        *pattern_offset,
                        interval_d,
                    ),
                )
            })
            .collect();
        // v.sort_by_key(|chunk_d| chunk_d.index);

        debug!("of_interval_c: end");

        ChunkC::new(bm)
    }

    pub fn get_ending_chunk_offset(
        &self
    ) -> u16 {
        // We process the ending chunk offset as the biggest offset + data length of all chunks.
        let ending_chunk_offset = self.bm.iter().fold(
            0,
            //|acc, (_i, (_chunk_index, _temporal_position, _more_chunk, offset, data))| {
            |acc, (_i, chunk_d)| {
                // let offset = chunk_d.get_offset();
                let data_length = chunk_d.get_internet_checksum_s().len();
                let offset_after_chunk = chunk_d.get_offset() + (data_length / 8) as u16;

                    //debug!(
                    //    "icmp_packet_v_of_data:  {} {}",
                    //    //chunk_index,
                    //    offset,
                    //    data_length,
                    //);

                max(acc, offset_after_chunk)
            },
        );
        debug!(
            "icmp_packet_v_of_data: ending_chunk_offset: {}",
            ending_chunk_offset
        );
        ending_chunk_offset
    }

}
