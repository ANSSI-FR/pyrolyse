use itertools::Itertools;
use std::iter::Iterator;
use itertools::iproduct;

use std::net::IpAddr;

use std::collections::HashMap;

use crate::position::pattern::{PatternD,ChunkBasedPatternC};

// use serde::{Deserialize, Serialize};

// use crate::byte_data::pair_triplet_byte_sequence::PairTripletByteSequence;
// use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceC;
// use crate::byte_time_data::export_mode::ExportMode;
// use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
// use crate::relation::relation_triplet::RelationTripletD;

/// Build possible chunk data pattern as permutations among 4 possible values.
/// We use 4 possible values to obtain enough unique combination.
/// We use this data size to always have the same checksum (sum of 16 bit words complemented to
/// one, cf RFC 791/1071/1141/1624) for all possible permutations.
/// TODO: explain more in detail.
/// TODO: add explanation with results from consistent relation building OR make it generic
/// with a parameter.
//pub fn build_internet_checksum_chunk_pattern_v() -> Vec<String> {
//    let chunk_u16_v: Vec<String> = vec![
//        "AA".to_string(),
//        "BB".to_string(),
//        "CC".to_string(),
//        "DD".to_string(),
//    ];
//    let chunk_pattern_v: Vec<String> = chunk_u16_v
//        .iter()
//        .permutations(4)
//        .map(|v| {
//            let s_v: Vec<String> = v.iter().map(|s| (**s).clone()).collect();
//            s_v.join("")
//        })
//        .collect();
//    chunk_pattern_v
//}
//
//pub fn build_simple_chunk_pattern_v() -> Vec<String> {
//    vec![
//        "A".to_string(),
//        "B".to_string(),
//        "C".to_string(),
//        "D".to_string(),
//        "E".to_string(),
//        "F".to_string(),
//        "G".to_string(),
//        "H".to_string(),
//        "I".to_string(),
//        "J".to_string(),
//        "K".to_string(),
//    ]
//}
//
//pub fn build_invariant_checksum_chunk_pattern_c(
//    chunk_number: u16,
//    pattern_number: u16,
//    target_checksum: u16,
//    ip_addr: IpAddr
//) -> InvariantChecksumChunkPatternC {
//    InvariantChecksumChunkPatternC::of_data(
//        chunk_number,
//        pattern_number,
//        target_checksum,
//        ip_addr
//    )
//} 

//pub fn build_invariant_ipv4_checksum_chunk_pattern_d(
//    chunk_number: u16,
//    pattern_number: u16
//) -> HashMap<u16,Vec<String>> {
//
//    assert!(chunk_number <= 999 && pattern_number <= 999);
//
//    let reference_checksum = 0;
//    let pattern_id_v: Vec<String> = (0..pattern_number).collect::<Vec<_>>().iter().map(|i| format!("{:0>3}", i) ).collect();
//    debug!("build_invariant_ipv4_checksum_chunk_pattern_d: pattern_id_v: {:?}", pattern_id_v);
//
//    let hm: HashMap<u16,Vec<String>> = (0..chunk_number).map(|chunk_id| {
//        let chunk_id_s = format!("{:0>3}", chunk_id);
//        debug!("build_invariant_ipv4_checksum_chunk_pattern_d: chunk_id_s: {:?}", chunk_id_s);
//
//        let chunk_pattern_payload_v: Vec<_> = pattern_id_v.iter().map(|pattern_id_s| {
//            let chunk_pattern_s = format!("{}{}", chunk_id_s, pattern_id_s);
//            let chunk_pattern_b = chunk_pattern_s.as_bytes();
//            
//            let curr_checksum = checksum(chunk_pattern_b,0);
//            let checksum_correction_bytes_i = curr_checksum - reference_checksum;
//            let checksum_correction_bytes_v: Vec<u8> = checksum_correction_bytes_i.to_be_bytes().to_vec();
//
//            let internet_checksum_s = checksum_correction_bytes_v.iter().map(|a| char::from(*a).to_string()).collect::<Vec<_>>().join("");
//
//            let full_pattern = format!("{}{}", chunk_pattern_s, internet_checksum_s);
//            debug!("build_invariant_ipv4_checksum_chunk_pattern_d: full_pattern: {:?}", full_pattern);
//            full_pattern
//        })
//        .collect();
//        debug!("build_invariant_ipv4_checksum_chunk_pattern_d: chunk_pattern_payload_v: {:?}", chunk_pattern_payload_v);
//
//        (
//            chunk_id,
//            chunk_pattern_payload_v
//        )
//    })
//    .collect();
//    //let chunk_id_v: Vec<String> = (0..chunk_number).collect::<Vec<_>>().iter().map(|i| format!("{:0>3}", i) ).collect();
//    //debug!("build_invariant_ipv4_checksum_chunk_pattern_d: chunk_id_v: {:?}", chunk_id_v);
//    //let pattern_id_v: Vec<String> = (0..pattern_number).collect::<Vec<_>>().iter().map(|i| format!("{:0>3}", i) ).collect();
//    //debug!("build_invariant_ipv4_checksum_chunk_pattern_d: pattern_id_v: {:?}", pattern_id_v);
////
//    //let product: Vec<String> = iproduct!(chunk_id_v, pattern_id_v)
//    //    .map(|(a, b)| format!("{}{}", a, b))
//    //    .collect();
//    //debug!("build_invariant_ipv4_checksum_chunk_pattern_d: product: {:?}", product);
//
//    hm
//}
//
//pub fn build_invariant_ipv6_checksum_chunk_pattern_d(
//    chunk_number: u16,
//    pattern_number: u16
//) -> HashMap<u16,Vec<String>> {
//
//    assert!(chunk_number <= 999 && pattern_number <= 999);
//
//    let reference_checksum = 0;
//    //let pattern_id_v: Vec<String> = (0..pattern_number).collect::<Vec<_>>().iter().map(|i| format!("{:0>3}", i) ).collect();
//    //debug!("build_invariant_ipv6_checksum_chunk_pattern_d: pattern_id_v: {:?}", pattern_id_v);
//    let pattern_id_v: Vec<_> = (0..pattern_number).collect();
//    debug!("build_invariant_ipv6_checksum_chunk_pattern_d: pattern_id_v: {:?}", pattern_id_v);
//
//
//    let hm: HashMap<u16,Vec<String>> = (0..chunk_number).map(|chunk_id| {
//        let chunk_id_s = format!("{:0>3}", chunk_id);
//        debug!("build_invariant_ipv6_checksum_chunk_pattern_d: chunk_id_s: {:?}", chunk_id_s);
//
//        let chunk_pattern_payload_v: Vec<_> = pattern_id_v.iter().map(|pattern_id| {
//            let pattern_id_s = format!("{:0>3}", pattern_id);
//            let chunk_pattern_s = format!("{}{}", chunk_id_s, pattern_id_s);
//            let chunk_pattern_b = chunk_pattern_s.as_bytes();
//            
//            let curr_checksum = checksum(chunk_pattern_b,0);
//            let checksum_correction_bytes_i = curr_checksum - reference_checksum + (8 * pattern_id) as u16;
//            let checksum_correction_bytes_v: Vec<u8> = checksum_correction_bytes_i.to_be_bytes().to_vec();
//
//            let internet_checksum_s = checksum_correction_bytes_v.iter().map(|a| char::from(*a).to_string()).collect::<Vec<_>>().join("");
//
//            let full_pattern = format!("{}{}", chunk_pattern_s, internet_checksum_s);
//            debug!("build_invariant_ipv6_checksum_chunk_pattern_d: full_pattern: {:?}", full_pattern);
//            full_pattern
//        })
//        .collect();
//        debug!("build_invariant_ipv6_checksum_chunk_pattern_d: chunk_pattern_payload_v: {:?}", chunk_pattern_payload_v);
//
//        (
//            chunk_id,
//            chunk_pattern_payload_v
//        )
//    })
//    .collect();
//    //let chunk_id_v: Vec<String> = (0..chunk_number).collect::<Vec<_>>().iter().map(|i| format!("{:0>3}", i) ).collect();
//    //debug!("build_invariant_ipv4_checksum_chunk_pattern_d: chunk_id_v: {:?}", chunk_id_v);
//    //let pattern_id_v: Vec<String> = (0..pattern_number).collect::<Vec<_>>().iter().map(|i| format!("{:0>3}", i) ).collect();
//    //debug!("build_invariant_ipv4_checksum_chunk_pattern_d: pattern_id_v: {:?}", pattern_id_v);
////
//    //let product: Vec<String> = iproduct!(chunk_id_v, pattern_id_v)
//    //    .map(|(a, b)| format!("{}{}", a, b))
//    //    .collect();
//    //debug!("build_invariant_ipv4_checksum_chunk_pattern_d: product: {:?}", product);
//
//    hm
//}
//
// pub fn generate() -> Vec<String> {
//     let simple_chunk_pattern_v = build_simple_chunk_pattern_v();
//     let internet_checksum_chunk_pattern_v =
//         build_internet_checksum_chunk_pattern_v();

//     internet_checksum_chunk_pattern_v
// }


pub fn build_internet_checksum_chunk_pattern_v() -> PatternD {
    // XXX check if there is not a fastest way
    let chunk_u16_v: Vec<String> = vec![
        "AA".to_string(),
        "BB".to_string(),
        "CC".to_string(),
        "DD".to_string(),
    ];
    let chunk_pattern_v: Vec<String> = chunk_u16_v
        .iter()
        .permutations(4)
        .map(|v| {
            let s_v: Vec<String> = v.iter().map(|s| (**s).clone()).collect();
            s_v.join("")
        })
        .collect();

    let chunk_pattern_v_v: Vec<Vec<u8>> = chunk_pattern_v
        .into_iter()
        .map(|s| s.clone().into_bytes() )
        .collect();

    PatternD(chunk_pattern_v_v)
}

pub fn build_simple_chunk_pattern_v() -> PatternD {
    let chunk_pattern_v = vec![
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

    let chunk_pattern_v_v: Vec<Vec<u8>> = chunk_pattern_v
        .iter()
        .map(|s| s.clone().into_bytes() )
        .collect();

    PatternD(chunk_pattern_v_v)
    
}

pub fn build_invariant_checksum_chunk_pattern_c(
    chunk_number: u16,
    pattern_number: u16,
    //target_checksum: u16,
    ip_addr: IpAddr
) -> ChunkBasedPatternC {
    ChunkBasedPatternC::of_data(
        chunk_number,
        pattern_number,
        //target_checksum,
        ip_addr
    )
} 