use serde::{Deserialize, Serialize};

use crate::byte_data::pair_triplet_byte_sequence::PairTripletByteSequence;
use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceC;
use crate::byte_time_data::export_mode::ExportMode;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::relation::relation_triplet::RelationTripletD;
//use crate::misc::invariant_checksum_chunk_pattern::InvariantChecksumChunkPatternC;
use crate::position::pattern::{PatternD,ChunkBasedPatternC};

/// Contains chunks for pair and triplet.
/// This represents both spatial/data and temporal location/relation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairTripletByteTimeSequence {
    byte_time_pair_sequence_c: ByteTimeSequenceC<AllenIntervalAlgebraRelation>,
    byte_time_triplet_sequence_c: ByteTimeSequenceC<RelationTripletD>,
}

impl PairTripletByteTimeSequence {
    pub fn new(
        byte_time_pair_sequence_c: ByteTimeSequenceC<AllenIntervalAlgebraRelation>,
        byte_time_triplet_sequence_c: ByteTimeSequenceC<RelationTripletD>,
    ) -> PairTripletByteTimeSequence {
        PairTripletByteTimeSequence {
            byte_time_pair_sequence_c,
            byte_time_triplet_sequence_c,
        }
    }

    pub fn get_byte_time_sequence_c_pair(
        &self,
    ) -> &ByteTimeSequenceC<AllenIntervalAlgebraRelation> {
        &self.byte_time_pair_sequence_c
    }

    pub fn get_byte_time_sequence_c_triplet(&self) -> &ByteTimeSequenceC<RelationTripletD> {
        &self.byte_time_triplet_sequence_c
    }

    // /// Build possible chunk data pattern as permutations among 4 possible values.
    // /// We use 4 possible values to obtain enough unique combination.
    // /// We use this data size to always have the same checksum (sum of 16 bit words complemented to
    // /// one, cf RFC 791/1071/1141/1624) for all possible permutations.
    // /// TODO: explain more in detail.
    // /// TODO: add explanation with results from consistent relation building OR make it generic
    // /// with a parameter.
    // pub fn build_internet_checksum_chunk_pattern_v() -> Vec<String> {
    //     let chunk_u16_v: Vec<String> = vec![
    //         "AA".to_string(),
    //         "BB".to_string(),
    //         "CC".to_string(),
    //         "DD".to_string(),
    //     ];
    //     let chunk_pattern_v: Vec<String> = chunk_u16_v
    //         .iter()
    //         .permutations(4)
    //         .map(|v| {
    //             let s_v: Vec<String> = v.iter().map(|s| (**s).clone()).collect();
    //             s_v.join("")
    //         })
    //         .collect();
    //     chunk_pattern_v
    // }

    // pub fn build_simple_chunk_pattern_v() -> Vec<String> {
    //     vec![
    //         "A".to_string(),
    //         "B".to_string(),
    //         "C".to_string(),
    //         "D".to_string(),
    //         "E".to_string(),
    //         "F".to_string(),
    //         "G".to_string(),
    //         "H".to_string(),
    //         "I".to_string(),
    //         "J".to_string(),
    //         "K".to_string(),
    //     ]
    // }

    pub fn of_all_byte_sequence(
        simple_chunk_pattern_v: PatternD,
        internet_checksum_chunk_pattern_v: PatternD,
        ipv4_invariant_checksum_chunk_pattern_c: ChunkBasedPatternC,
        ipv6_invariant_checksum_chunk_pattern_c: ChunkBasedPatternC,
        temporal_shuffling: bool,
        all_byte_sequence: &PairTripletByteSequence,
    ) -> PairTripletByteTimeSequence {
        debug!("of_all_sequence: start");

        let export_mode = ExportMode::Isolated;

        // let simple_chunk_pattern_v =
        //     internet_checksum_pattern_generator::build_simple_chunk_pattern_v();
        // let internet_checksum_chunk_pattern_v =
        //     internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();
        // debug!(
        //     "of_all_sequence: internet_checksum_chunk_pattern_v: {:?}",
        //     internet_checksum_chunk_pattern_v
        // );

        let byte_time_pair_sequence_c = ByteTimeSequenceC::<AllenIntervalAlgebraRelation>::of_data_pair(
            &export_mode,
            &simple_chunk_pattern_v,
            &internet_checksum_chunk_pattern_v,
            &ipv4_invariant_checksum_chunk_pattern_c,
            &ipv6_invariant_checksum_chunk_pattern_c,
            temporal_shuffling,
            all_byte_sequence.get_byte_pair_sequence_c(),
        );
                

        // TripletChunkD offset is arbitraly chosen at 100.
        let chunk_index_offset = 100;
        let byte_time_triplet_sequence_c = ByteTimeSequenceC::<RelationTripletD>::of_data_triplet(
            chunk_index_offset,
            &export_mode,
            &simple_chunk_pattern_v,
            &internet_checksum_chunk_pattern_v,
            &ipv4_invariant_checksum_chunk_pattern_c,
            &ipv6_invariant_checksum_chunk_pattern_c,
            temporal_shuffling,
            all_byte_sequence.get_byte_triplet_sequence_c(),
        );

        debug!("of_all_sequence: end");
        PairTripletByteTimeSequence::new(
            // simple_chunk_pattern_v,
            // internet_checksum_chunk_pattern_v,
            byte_time_pair_sequence_c,
            byte_time_triplet_sequence_c,
        )
    }
}
