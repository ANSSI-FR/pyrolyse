use std::cmp::max;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::Iterator;
use std::net::IpAddr;
use std::path::Path;

use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::ip::IpNextHeaderProtocols;
use serde::Serialize;

use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceC;
use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceD;
use crate::byte_time_data::chunk::ChunkC;
use crate::misc::interval::IntervalC;
use crate::misc::ip_addr_fragmentation_trait_impl::IpAddrForFragmentationTesting;
use crate::misc::pcap_utils;
use crate::misc::policy_evaluation::PolicyEvaluation;
use crate::misc::test_index::TestIndex;
use crate::misc::test_target::TestTarget;
//use crate::misc::internet_checksum_pattern_generator;
use crate::misc::test_more_chunk;
use crate::pcap::testing_packet::TestingPacket;
use crate::position::payload_mode::PayloadMode;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::relation::relation_container::RelationContainer;
use crate::relation::relation_triplet::RelationTripletD;

// TODO: factorize with IcmpPacket

#[derive(Debug, Clone)]
pub struct TestingTraceD<I: Into<IpAddr>> {
    v: Vec<TestingPacket<I>>,
}

impl<I: Debug + Copy + Into<IpAddr> + IpAddrForFragmentationTesting> TestingTraceD<I> {
    pub fn new(v: Vec<TestingPacket<I>>) -> TestingTraceD<I> {
        TestingTraceD { v }
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&TestingPacket<I>> {
        self.v.get(index)
    }

    // fn extract_chunk_data(temporal_position_sl: &[u16], chunk_d: &ChunkD) -> (u16, u16) {
    //     let chunk_index = chunk_d.get_index();
    //     let temporal_position = temporal_position_sl.get(chunk_index as usize).unwrap();
    //     (chunk_index, *temporal_position)
    // }

    pub fn testing_packet_v_of_data(
        //internet_checksum_chunk_pattern_sl: &[u8],
        test_target: &TestTarget<I>,
        policy_evaluation: &PolicyEvaluation,
        temporal_position_sl: &[u16],
        header_data_sl: &[u8],
        test_index: TestIndex,
        // sequence_index: u16,
        chunk_c: &ChunkC,
        interval_c: &IntervalC,
        payload_mode: &PayloadMode,
    ) -> Vec<TestingPacket<I>> {
        debug!("testing_packet_v_of_data: start");
        let more_chunk_v = test_more_chunk::get_test_case_more_chunk_v(
            policy_evaluation,
            temporal_position_sl,
            interval_c,
        );
        let factor = payload_mode.get_factor() as usize;

        let chunk_data_hm = match *policy_evaluation {
            PolicyEvaluation::ProgressiveAllFinishing
            | PolicyEvaluation::ProgressiveOldestFinishing
            | PolicyEvaluation::ProgressiveMidFinishing
            | PolicyEvaluation::ProgressiveNewestFinishing
            | PolicyEvaluation::ProgressiveOldestNewestFinishing
            | PolicyEvaluation::ProgressiveOldestMidFinishing
            | PolicyEvaluation::ProgressiveMidNewestFinishing
            | PolicyEvaluation::ProgressiveAllStarting
            | PolicyEvaluation::ProgressiveOldestStarting
            | PolicyEvaluation::ProgressiveMidStarting
            | PolicyEvaluation::ProgressiveNewestStarting
            | PolicyEvaluation::ProgressiveOldestNewestStarting
            | PolicyEvaluation::ProgressiveOldestMidStarting
            | PolicyEvaluation::ProgressiveMidNewestStarting => {
                let hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap();
                        let more_chunk = more_chunk_v[*i as usize];
                        let mut offset = chunk_d.get_offset();
                        // header content is in the payload of fragment with offset 0
                        let data = if offset != 0 {
                            offset += 1;
                            chunk_d.get_chunk_pattern_ascii_v(payload_mode)
                        } else {
                            vec![
                                header_data_sl.to_vec(),
                                chunk_d.get_chunk_pattern_ascii_v(payload_mode),
                            ]
                            .concat()
                        };

                        (
                            *i,
                            (chunk_index, *temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();
                hm
            }
            PolicyEvaluation::OnceEndFollows => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap();
                        let more_chunk = more_chunk_v[*i as usize];
                        let mut offset = chunk_d.get_offset();
                        // header content is in the payload of fragment with offset 0
                        let data = if offset != 0 {
                            offset += 1;
                            chunk_d.get_chunk_pattern_ascii_v(payload_mode)
                        } else {
                            vec![
                                header_data_sl.to_vec(),
                                chunk_d.get_chunk_pattern_ascii_v(payload_mode),
                            ]
                            .concat()
                        };

                        (
                            *i,
                            (chunk_index, *temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();
                let chunk_nb = chunk_c.len() as u16;

                // We process the ending chunk offset as the biggest offset + data length of all chunks.
                let ending_chunk_offset = hm.iter().fold(
                    0,
                    |acc, (_i, (_chunk_index, _temporal_position, _more_chunk, offset, data))| {
                        // let offset = chunk_d.get_offset();
                        let data_length = data.len();
                        //let offset_after_chunk = offset + (data_length / 8) as u16;
                        let offset_after_chunk = offset + (data_length / factor) as u16;

                        // debug!(
                        //     "icmp_packet_v_of_data: {}: {} {}",
                        //     chunk_index,
                        //     offset,
                        //     data_length,
                        // );

                        max(acc, offset_after_chunk)
                    },
                );

                // We insert the tail chunk at the end
                let ending_chunk_index = chunk_nb;
                let ending_chunk_temporal_position = chunk_nb;
                //let internet_checksum_chunk_pattern_v =
                //    internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();
                //let ending_chunk_payload = internet_checksum_chunk_pattern_v
                //    [internet_checksum_chunk_pattern_v.len() - 1]
                //    .as_bytes()
                //    .to_vec();
                let ending_chunk_payload = Self::get_ending_chunk_payload(
                    payload_mode,
                    //chunk_nb - 1,
                    //ending_chunk_offset - 1,
                );

                hm.insert(
                    ending_chunk_index,
                    (
                        ending_chunk_index,
                        ending_chunk_temporal_position,
                        false,
                        ending_chunk_offset,
                        ending_chunk_payload,
                    ),
                );

                hm
            }
            PolicyEvaluation::OnceEndPrecedes => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap() + 1;
                        let more_chunk = more_chunk_v[*i as usize];
                        let mut offset = chunk_d.get_offset();
                        // header content is in the payload of fragment with offset 0
                        let data = if offset != 0 {
                            offset += 1;
                            chunk_d.get_chunk_pattern_ascii_v(payload_mode)
                        } else {
                            vec![
                                header_data_sl.to_vec(),
                                chunk_d.get_chunk_pattern_ascii_v(payload_mode),
                            ]
                            .concat()
                        };

                        (
                            *i,
                            (chunk_index, temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                // We process the ending chunk offset as the biggest offset + data length of all chunks.
                let ending_chunk_offset = hm.iter().fold(
                    0,
                    |acc, (_i, (_chunk_index, _temporal_position, _more_chunk, offset, data))| {
                        // let offset = chunk_d.get_offset();
                        let data_length = data.len();
                        //let offset_after_chunk = offset + (data_length / 8) as u16;
                        let offset_after_chunk = offset + (data_length / factor) as u16;

                        // debug!(
                        //     "icmp_packet_v_of_data: {}: {} {}",
                        //     chunk_index,
                        //     offset,
                        //     data_length,
                        // );

                        max(acc, offset_after_chunk)
                    },
                );
                let chunk_nb = chunk_c.len() as u16;

                // We insert the tail chunk at the begining.
                let ending_chunk_index = chunk_nb;
                let ending_chunk_temporal_position = 0;
                //let internet_checksum_chunk_pattern_v =
                //    internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();
                //let ending_chunk_payload = internet_checksum_chunk_pattern_v
                //    [internet_checksum_chunk_pattern_v.len() - 1]
                //    .as_bytes()
                //    .to_vec();
                let ending_chunk_payload = Self::get_ending_chunk_payload(
                    payload_mode,
                    //chunk_nb - 1,
                    //ending_chunk_offset - 1,
                );

                hm.insert(
                    ending_chunk_index,
                    (
                        ending_chunk_index,
                        ending_chunk_temporal_position,
                        false,
                        ending_chunk_offset,
                        ending_chunk_payload,
                    ),
                );
                hm
            }
            PolicyEvaluation::OnceStartPrecedesAllFinishing
            | PolicyEvaluation::OnceStartPrecedesOldestFinishing
            | PolicyEvaluation::OnceStartPrecedesMidFinishing
            | PolicyEvaluation::OnceStartPrecedesNewestFinishing
            | PolicyEvaluation::OnceStartPrecedesOldestNewestFinishing
            | PolicyEvaluation::OnceStartPrecedesOldestMidFinishing
            | PolicyEvaluation::OnceStartPrecedesMidNewestFinishing
            | PolicyEvaluation::OnceStartPrecedesAllStarting
            | PolicyEvaluation::OnceStartPrecedesOldestStarting
            | PolicyEvaluation::OnceStartPrecedesMidStarting
            | PolicyEvaluation::OnceStartPrecedesNewestStarting
            | PolicyEvaluation::OnceStartPrecedesOldestNewestStarting
            | PolicyEvaluation::OnceStartPrecedesOldestMidStarting
            | PolicyEvaluation::OnceStartPrecedesMidNewestStarting => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        // We shift temporal position by one to leave space for the header at the beginning.
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap() + 1;
                        let data = chunk_d.get_chunk_pattern_ascii_v(payload_mode);
                        let more_chunk = more_chunk_v[*i as usize];
                        let offset = chunk_d.get_offset() + 1;
                        (
                            *i,
                            (chunk_index, temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                let chunk_nb = chunk_c.len() as u16;

                // We insert the header chunk at the beginning.
                let starting_chunk_index = chunk_nb;
                let starting_chunk_temporal_position = 0;

                hm.insert(
                    starting_chunk_index,
                    (
                        starting_chunk_index,
                        starting_chunk_temporal_position,
                        true,
                        0,
                        header_data_sl.to_vec(),
                    ),
                );
                hm
            }
            PolicyEvaluation::OnceStartFollowsAllFinishing
            | PolicyEvaluation::OnceStartFollowsOldestFinishing
            | PolicyEvaluation::OnceStartFollowsMidFinishing
            | PolicyEvaluation::OnceStartFollowsNewestFinishing
            | PolicyEvaluation::OnceStartFollowsOldestNewestFinishing
            | PolicyEvaluation::OnceStartFollowsOldestMidFinishing
            | PolicyEvaluation::OnceStartFollowsMidNewestFinishing
            | PolicyEvaluation::OnceStartFollowsAllStarting
            | PolicyEvaluation::OnceStartFollowsOldestStarting
            | PolicyEvaluation::OnceStartFollowsMidStarting
            | PolicyEvaluation::OnceStartFollowsNewestStarting
            | PolicyEvaluation::OnceStartFollowsOldestNewestStarting
            | PolicyEvaluation::OnceStartFollowsOldestMidStarting
            | PolicyEvaluation::OnceStartFollowsMidNewestStarting => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap();
                        let more_chunk = more_chunk_v[*i as usize];
                        let offset = chunk_d.get_offset() + 1;
                        let data = chunk_d.get_chunk_pattern_ascii_v(payload_mode);

                        (
                            *i,
                            (chunk_index, *temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();
                let chunk_nb = chunk_c.len() as u16;

                // We insert the header chunk at the end.
                let starting_chunk_index = chunk_nb;
                let starting_chunk_temporal_position = chunk_nb;

                hm.insert(
                    starting_chunk_index,
                    (
                        starting_chunk_index,
                        starting_chunk_temporal_position,
                        true,
                        0,
                        header_data_sl.to_vec(),
                    ),
                );
                hm
            }
            PolicyEvaluation::OnceStartPrecedesEndFollows => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap() + 1;
                        // More_chunk is set to true because there will be a new last chunk.
                        let more_chunk = more_chunk_v[*i as usize];
                        let offset = chunk_d.get_offset() + 1;
                        let data = chunk_d.get_chunk_pattern_ascii_v(payload_mode);

                        (
                            *i,
                            (chunk_index, temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                // We process the ending chunk offset as the biggest offset + data length of all chunks.
                let ending_chunk_offset = hm.iter().fold(
                    0,
                    |acc, (_i, (_chunk_index, _temporal_position, _more_chunk, offset, data))| {
                        // let offset = chunk_d.get_offset();
                        let data_length = data.len();
                        //let offset_after_chunk = offset + (data_length / 8) as u16;
                        let offset_after_chunk = offset + (data_length / factor) as u16;

                        // debug!(
                        //     "icmp_packet_v_of_data: {}: {} {}",
                        //     chunk_index,
                        //     offset,
                        //     data_length,
                        // );

                        max(acc, offset_after_chunk)
                    },
                );

                // debug!(
                //     "icmp_packet_v_of_data: ending_chunk_offset: {}",
                //     ending_chunk_offset
                // );

                let chunk_nb = chunk_c.len() as u16;

                // We insert the header chunk at the beginning.
                let starting_chunk_index = chunk_nb;
                let starting_chunk_temporal_position = 0;
                let starting_chunk_payload = header_data_sl.to_vec();

                // We insert the tail chunk at the end.
                let ending_chunk_index = chunk_nb + 1;
                let ending_chunk_temporal_position = chunk_nb + 1;
                //let internet_checksum_chunk_pattern_v =
                //    internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();
                //let ending_chunk_payload = internet_checksum_chunk_pattern_v
                //    [internet_checksum_chunk_pattern_v.len() - 1]
                //    .as_bytes()
                //    .to_vec();
                let ending_chunk_payload = Self::get_ending_chunk_payload(
                    payload_mode,
                    //chunk_nb - 1,
                    //ending_chunk_offset - 1,
                );

                hm.insert(
                    starting_chunk_index,
                    (
                        starting_chunk_index,
                        starting_chunk_temporal_position,
                        true,
                        0,
                        starting_chunk_payload,
                    ),
                );
                hm.insert(
                    ending_chunk_index,
                    (
                        ending_chunk_index,
                        ending_chunk_temporal_position,
                        false,
                        ending_chunk_offset,
                        ending_chunk_payload,
                    ),
                );
                hm
            }
            PolicyEvaluation::OnceStartFollowsEndFollows => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap();
                        // More_chunk is set to true because there will be a new last chunk.
                        let more_chunk = more_chunk_v[*i as usize];
                        let offset = chunk_d.get_offset() + 1;
                        let data = chunk_d.get_chunk_pattern_ascii_v(payload_mode);

                        (
                            *i,
                            (chunk_index, *temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                // We process the ending chunk offset as the biggest offset + data length of all chunks.
                let ending_chunk_offset = hm.iter().fold(
                    0,
                    |acc, (_i, (_chunk_index, _temporal_position, _more_chunk, offset, data))| {
                        // let offset = chunk_d.get_offset();
                        let data_length = data.len();
                        //let offset_after_chunk = offset + (data_length / 8) as u16;
                        let offset_after_chunk = offset + (data_length / factor) as u16;

                        // debug!(
                        //     "icmp_packet_v_of_data: {}: {} {}",
                        //     chunk_index,
                        //     offset,
                        //     data_length,
                        // );

                        max(acc, offset_after_chunk)
                    },
                );

                // debug!(
                //     "icmp_packet_v_of_data: ending_chunk_offset: {}",
                //     ending_chunk_offset
                // );

                let chunk_nb = chunk_c.len() as u16;

                // We insert the header chunk just before the end.
                let starting_chunk_index = chunk_nb;
                let starting_chunk_temporal_position = chunk_nb;
                let starting_chunk_payload = header_data_sl.to_vec();

                // We insert the tail chunk at the end.
                let ending_chunk_index = chunk_nb + 1;
                let ending_chunk_temporal_position = chunk_nb + 1;
                //let internet_checksum_chunk_pattern_v =
                //    internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();
                //let ending_chunk_payload = internet_checksum_chunk_pattern_v
                //    [internet_checksum_chunk_pattern_v.len() - 1]
                //    .as_bytes()
                //    .to_vec();
                let ending_chunk_payload = Self::get_ending_chunk_payload(
                    payload_mode,
                    //chunk_nb - 1,
                    //ending_chunk_offset - 1,
                );

                hm.insert(
                    starting_chunk_index,
                    (
                        starting_chunk_index,
                        starting_chunk_temporal_position,
                        true,
                        0,
                        starting_chunk_payload,
                    ),
                );
                hm.insert(
                    ending_chunk_index,
                    (
                        ending_chunk_index,
                        ending_chunk_temporal_position,
                        false,
                        ending_chunk_offset,
                        ending_chunk_payload,
                    ),
                );
                hm
            }
            PolicyEvaluation::OnceEndFollowsStartFollows => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap();
                        // More_chunk is set to true because there will be a new last chunk.
                        let more_chunk = more_chunk_v[*i as usize];
                        let offset = chunk_d.get_offset() + 1;
                        let data = chunk_d.get_chunk_pattern_ascii_v(payload_mode);

                        (
                            *i,
                            (chunk_index, *temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                // We process the ending chunk offset as the biggest offset + data length of all chunks.
                let ending_chunk_offset = hm.iter().fold(
                    0,
                    |acc, (_i, (_chunk_index, _temporal_position, _more_chunk, offset, data))| {
                        // let offset = chunk_d.get_offset();
                        let data_length = data.len();
                        //let offset_after_chunk = offset + (data_length / 8) as u16;
                        let offset_after_chunk = offset + (data_length / factor) as u16;

                        // debug!(
                        //     "icmp_packet_v_of_data: {}: {} {}",
                        //     chunk_index,
                        //     offset,
                        //     data_length,
                        // );

                        max(acc, offset_after_chunk)
                    },
                );

                // debug!(
                //     "icmp_packet_v_of_data: ending_chunk_offset: {}",
                //     ending_chunk_offset
                // );

                let chunk_nb = chunk_c.len() as u16;

                // We insert the header chunk at the end.
                let starting_chunk_index = chunk_nb + 1;
                let starting_chunk_temporal_position = chunk_nb + 1;
                let starting_chunk_payload = header_data_sl.to_vec();

                // We insert the tail chunk just before the end.
                let ending_chunk_index = chunk_nb;
                let ending_chunk_temporal_position = chunk_nb;
                //let internet_checksum_chunk_pattern_v =
                //    internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();
                //let ending_chunk_payload = internet_checksum_chunk_pattern_v
                //    [internet_checksum_chunk_pattern_v.len() - 1]
                //    .as_bytes()
                //    .to_vec();
                let ending_chunk_payload = Self::get_ending_chunk_payload(
                    payload_mode,
                    //chunk_nb - 1,
                    //ending_chunk_offset - 1,
                );

                hm.insert(
                    starting_chunk_index,
                    (
                        starting_chunk_index,
                        starting_chunk_temporal_position,
                        true,
                        0,
                        starting_chunk_payload,
                    ),
                );
                hm.insert(
                    ending_chunk_index,
                    (
                        ending_chunk_index,
                        ending_chunk_temporal_position,
                        false,
                        ending_chunk_offset,
                        ending_chunk_payload,
                    ),
                );
                hm
            }
            PolicyEvaluation::OnceEndPrecedesStartFollows => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap() + 1;
                        // More_chunk is set to true because there will be a new last chunk.
                        let more_chunk = more_chunk_v[*i as usize];
                        let offset = chunk_d.get_offset() + 1;
                        let data = chunk_d.get_chunk_pattern_ascii_v(payload_mode);

                        (
                            *i,
                            (chunk_index, temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                // We process the ending chunk offset as the biggest offset + data length of all chunks.
                let ending_chunk_offset = hm.iter().fold(
                    0,
                    |acc, (_i, (_chunk_index, _temporal_position, _more_chunk, offset, data))| {
                        // let offset = chunk_d.get_offset();
                        let data_length = data.len();
                        //let offset_after_chunk = offset + (data_length / 8) as u16;
                        let offset_after_chunk = offset + (data_length / factor) as u16;

                        // debug!(
                        //     "icmp_packet_v_of_data: {}: {} {}",
                        //     chunk_index,
                        //     offset,
                        //     data_length,
                        // );

                        max(acc, offset_after_chunk)
                    },
                );

                // debug!(
                //     "icmp_packet_v_of_data: ending_chunk_offset: {}",
                //     ending_chunk_offset
                // );

                let chunk_nb = chunk_c.len() as u16;

                // We insert the header chunk at the end.
                let starting_chunk_index = chunk_nb + 1;
                let starting_chunk_temporal_position = chunk_nb + 1;
                let starting_chunk_payload = header_data_sl.to_vec();

                // We insert the tail chunk at the begining.
                let ending_chunk_index = chunk_nb;
                let ending_chunk_temporal_position = 0;
                //let internet_checksum_chunk_pattern_v =
                //    internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();
                //let ending_chunk_payload = internet_checksum_chunk_pattern_v
                //    [internet_checksum_chunk_pattern_v.len() - 1]
                //    .as_bytes()
                //    .to_vec();
                let ending_chunk_payload = Self::get_ending_chunk_payload(
                    payload_mode,
                    //chunk_nb - 1,
                    //ending_chunk_offset - 1,
                );

                hm.insert(
                    starting_chunk_index,
                    (
                        starting_chunk_index,
                        starting_chunk_temporal_position,
                        true,
                        0,
                        starting_chunk_payload,
                    ),
                );
                hm.insert(
                    ending_chunk_index,
                    (
                        ending_chunk_index,
                        ending_chunk_temporal_position,
                        false,
                        ending_chunk_offset,
                        ending_chunk_payload,
                    ),
                );
                hm
            }
            PolicyEvaluation::OnceStartPrecedesEndPrecedes => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap() + 2;
                        // More_chunk is set to true because there will be a new last chunk.
                        let more_chunk = more_chunk_v[*i as usize];
                        let offset = chunk_d.get_offset() + 1;
                        let data = chunk_d.get_chunk_pattern_ascii_v(payload_mode);

                        (
                            *i,
                            (chunk_index, temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                // We process the ending chunk offset as the biggest offset + data length of all chunks.
                let ending_chunk_offset = hm.iter().fold(
                    0,
                    |acc, (_i, (_chunk_index, _temporal_position, _more_chunk, offset, data))| {
                        // let offset = chunk_d.get_offset();
                        let data_length = data.len();
                        let offset_after_chunk = offset + (data_length / factor) as u16;

                        // debug!(
                        //     "icmp_packet_v_of_data: {}: {} {}",
                        //     chunk_index,
                        //     offset,
                        //     data_length,
                        // );

                        max(acc, offset_after_chunk)
                    },
                );

                // debug!(
                //     "icmp_packet_v_of_data: ending_chunk_offset: {}",
                //     ending_chunk_offset
                // );

                let chunk_nb = chunk_c.len() as u16;

                // We insert the header chunk at the begining.
                let starting_chunk_index = chunk_nb;
                let starting_chunk_temporal_position = 0;
                let starting_chunk_payload = header_data_sl.to_vec();

                // We insert the tail chunk just after the begining.
                let ending_chunk_index = chunk_nb + 1;
                let ending_chunk_temporal_position = 1;
                //let internet_checksum_chunk_pattern_v =
                //    internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();
                //let ending_chunk_payload = internet_checksum_chunk_pattern_v
                //    [internet_checksum_chunk_pattern_v.len() - 1]
                //    .as_bytes()
                //    .to_vec();
                let ending_chunk_payload = Self::get_ending_chunk_payload(
                    payload_mode,
                    //chunk_nb - 1,
                    //ending_chunk_offset - 1,
                );

                hm.insert(
                    starting_chunk_index,
                    (
                        starting_chunk_index,
                        starting_chunk_temporal_position,
                        true,
                        0,
                        starting_chunk_payload,
                    ),
                );
                hm.insert(
                    ending_chunk_index,
                    (
                        ending_chunk_index,
                        ending_chunk_temporal_position,
                        false,
                        ending_chunk_offset,
                        ending_chunk_payload,
                    ),
                );
                hm
            }
            PolicyEvaluation::OnceEndPrecedesStartPrecedes => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap() + 2;
                        // More_chunk is set to true because there will be a new last chunk.
                        let more_chunk = more_chunk_v[*i as usize];
                        let offset = chunk_d.get_offset() + 1;
                        let data = chunk_d.get_chunk_pattern_ascii_v(payload_mode);

                        (
                            *i,
                            (chunk_index, temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                // We process the ending chunk offset as the biggest offset + data length of all chunks.
                let ending_chunk_offset = hm.iter().fold(
                    0,
                    |acc, (_i, (_chunk_index, _temporal_position, _more_chunk, offset, data))| {
                        // let offset = chunk_d.get_offset();
                        let data_length = data.len();
                        //let offset_after_chunk = offset + (data_length / 8) as u16;
                        let offset_after_chunk = offset + (data_length / factor) as u16;

                        // debug!(
                        //     "icmp_packet_v_of_data: {}: {} {}",
                        //     chunk_index,
                        //     offset,
                        //     data_length,
                        // );

                        max(acc, offset_after_chunk)
                    },
                );

                // debug!(
                //     "icmp_packet_v_of_data: ending_chunk_offset: {}",
                //     ending_chunk_offset
                // );

                let chunk_nb = chunk_c.len() as u16;

                // We insert the header chunk at the begining.
                let starting_chunk_index = chunk_nb + 1;
                let starting_chunk_temporal_position = 1;
                let starting_chunk_payload = header_data_sl.to_vec();

                // We insert the tail chunk just after the begining.
                let ending_chunk_index = chunk_nb;
                let ending_chunk_temporal_position = 0;
                //let internet_checksum_chunk_pattern_v =
                //    internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();
                //let ending_chunk_payload = internet_checksum_chunk_pattern_v
                //    [internet_checksum_chunk_pattern_v.len() - 1]
                //    .as_bytes()
                //    .to_vec();
                let ending_chunk_payload = Self::get_ending_chunk_payload(
                    payload_mode,
                    //chunk_nb - 1,
                    //ending_chunk_offset - 1,
                );

                hm.insert(
                    starting_chunk_index,
                    (
                        starting_chunk_index,
                        starting_chunk_temporal_position,
                        true,
                        0,
                        starting_chunk_payload,
                    ),
                );
                hm.insert(
                    ending_chunk_index,
                    (
                        ending_chunk_index,
                        ending_chunk_temporal_position,
                        false,
                        ending_chunk_offset,
                        ending_chunk_payload,
                    ),
                );
                hm
            }
        };

        let mut index_position_testing_packet_v = chunk_data_hm
            .iter()
            .map(
                |(_, (chunk_index, temporal_position, more_chunk, offset, data))| {
                    // .map(|chunk_d| {
                    // debug!("testing_packet_v_of_data: chunk_d: {:?}", chunk_d);
                    debug!("testing_packet_v_of_data: chunk_index: {:?}", chunk_index);

                    // let chunk_index = chunk_d.get_index();
                    // let temporal_position = temporal_position_sl.get(chunk_index as usize).unwrap();

                    debug!(
                        "testing_packet_v_of_data: chunk_d data ({:?}): {:?}",
                        //data.len() / 8,
                        data.len() / factor,
                        data
                    );
                    debug!(
                        "testing_packet_v_of_data: chunk_d data hex ({:?}): {:x?}",
                        //data.len() / 8,
                        data.len() / factor,
                        data
                    );

                    // // If this chunk starts the pattern:
                    // let data = if chunk_d.get_start() {
                    //     // We add the ICMP header in front, and use chunk data after.
                    //     let fragment_data_tmp_v = chunk_d.get_internet_checksum_ascii_v().clone();
                    //     [icmp_header, &fragment_data_tmp_v].concat()
                    // } else {
                    //     // otherwise, we only use data.
                    //     chunk_d.get_internet_checksum_ascii_v().clone()
                    // };
                    // This is especially important if fragment 0 and 1 start at the same
                    // position.

                    // debug!("generate_isolated_pcap: data: {:?}", data);
                    // debug!(
                    //     "testing_packet_v_of_data: data ({:?}): {:x?}",
                    //     data.len() / 8,
                    //     data
                    // );

                    // debug!("generate_isolated_pcap: \n\n\n\n");
                    (
                        *chunk_index,
                        *temporal_position,
                        TestingPacket::new(
                            // test_target.macaddr_src,
                            // test_target.macaddr_dst,
                            // test_target.ip_src,
                            // test_target.ip_dst,
                            test_target.clone(),
                            test_index,
                            *more_chunk,
                            // TODO: clean this
                            *offset,
                            // test_index,
                            // sequence_index,
                            data.to_vec(),
                        ),
                    )
                },
            )
            .collect::<Vec<(u16, u16, TestingPacket<I>)>>();

        debug!(
            "testing_packet_v_of_data: index_position_testing_packet_v: {:?}",
            index_position_testing_packet_v
        );
        index_position_testing_packet_v.sort_by_key(|tuple| {
            let (_, temporal_position, _) = tuple;
            *temporal_position
        });
        debug!(
            "testing_packet_v_of_data: index_position_testing_packet_v: {:?}",
            index_position_testing_packet_v
        );

        let testing_packet_v = index_position_testing_packet_v
            .into_iter()
            .map(|(_, _, icmp_packet)| icmp_packet)
            .collect();

        debug!("testing_packet_v_of_data: end");

        testing_packet_v
    }

    pub fn of_data_byte_time_sequence_d<Rc: Debug + Clone + Serialize + RelationContainer>(
        //internet_checksum_chunk_pattern_sl: &[u8],
        test_target: &TestTarget<I>,
        policy_evaluation: &PolicyEvaluation,
        header_data_sl: &[u8],
        // test_index: u16,
        byte_time_sequence_d: &ByteTimeSequenceD<Rc>,
        payload_mode: &PayloadMode,
    ) -> TestingTraceD<I> {
        debug!("of_data_byte_time_sequence_d: start");

        debug!(
            "of_data_byte_time_sequence_d: byte_time_sequence_d: {:?}",
            byte_time_sequence_d
        );

        let test_index = byte_time_sequence_d.get_byte_sequence_index();
        let temporal_position_v = byte_time_sequence_d.get_temporal_position_v();
        let chunk_c = byte_time_sequence_d.get_chunk_c();
        let interval_c = byte_time_sequence_d.get_interval_c();

        // let sequence_index = byte_time_sequence_d.get_byte_sequence_index();

        let testing_packet_v = TestingTraceD::testing_packet_v_of_data(
            //internet_checksum_chunk_pattern_sl,
            test_target,
            policy_evaluation,
            temporal_position_v,
            header_data_sl,
            test_index,
            // sequence_index,
            chunk_c,
            interval_c,
            payload_mode,
        );

        debug!("of_data_byte_time_sequence_d: end");

        TestingTraceD::new(testing_packet_v)
    }

    pub fn export(
        &self,
        payload_protocol: IpNextHeaderProtocol,
        // test_target: &TestTarget,
        pcap_directory_path: &Path,
        // icmp_header: &Vec<u8>,
        test_index_offset: TestIndex,
        test_index: TestIndex,
        // chunk_c: &ChunkC,
        // temporal_position_v: &Vec<u32>,
    ) {
        debug!("export: start");
        // debug!(
        //     "export_chunk_c: temporal_position_v: {:?}",
        //     temporal_position_v
        // );

        // let output_path_s = format!("{}/test_{}.pcap",pcap_directory_path, test_index);
        // let output_path = Path::new(&output_path_s);

        // let output_path = PathBuf::new();
        // output_path.push(pcap_directory_path);
        // output_path.push(format!("test_{}.pcap",test_index));
        let output_path = pcap_directory_path.join(format!("test_{}.pcap", test_index));

        // let output_path_s = format!("{}/test_{}.pcap",pcap_directory_path, test_index);
        debug!("export: output_path: {:?}", output_path);

        let mut output_file = if output_path
            .to_str()
            .expect("Could not convert output_path to string")
            == "-"
        {
            Box::new(io::stdout())
        } else {
            let file = File::create(output_path).unwrap();
            Box::new(file) as Box<dyn io::Write>
        };

        debug!("export: pcap_write_header");
        pcap_utils::pcap_write_header(&mut output_file, 1500).unwrap();

        // index_position_ethernet_data_v.iter().enumerate().for_each(
        self.v.iter().enumerate().for_each(|(i, testing_packet)| {
            let ts_sec = i as u32;
            let ts_usec = 0;

            assert_eq!(test_index, testing_packet.get_test_index());

            let ip_id = test_index_offset.0 + test_index.0;

            let data = match payload_protocol {
                IpNextHeaderProtocols::Icmp => {
                    testing_packet.build_ethernet_pdu_data_v_for_icmp(ip_id)
                }
                IpNextHeaderProtocols::Udp => {
                    testing_packet.build_ethernet_pdu_data_v_for_udp(ip_id)
                }
                _ => panic!("Provided payload protocol not implemented"),
            };

            let written =
                pcap_utils::pcap_write_packet(&mut output_file, ts_sec, ts_usec, &data).unwrap();
            debug!("export: written: {}", written);
        });
        debug!("export: end");
    }

    pub fn get_ending_chunk_payload(
        payload_mode: &PayloadMode,
        //chunk_nb: u16,
        //offset: u16,
    ) -> Vec<u8> {
        match payload_mode {
            PayloadMode::VariableChecksum1Byte(pattern_d)
            | PayloadMode::InvariantChecksumFixedLength8Byte(pattern_d) => {
                let ascii_v_v = pattern_d.get_ascii_v_v();
                ascii_v_v[(pattern_d.len() - 1) as usize].clone()
            }
            PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(chunk_based_pattern_c)
            | PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(chunk_based_pattern_c) => {
                let pattern_d = chunk_based_pattern_c.get_end_chunk_pattern_d().unwrap();
                pattern_d.get_ascii_v_v()[0].clone()
                //ascii_v_v[0].clone()
                //pattern_d.get(&offset).unwrap().clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use crate::byte_data::byte_sequence::ByteSequenceD;
    use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceD;
    use crate::byte_time_data::chunk::ChunkC;
    use crate::byte_time_data::chunk::ChunkD;
    use crate::byte_time_data::export_mode::ExportMode;
    //use crate::misc::internet_checksum_pattern_generator;
    use crate::misc::interval::IntervalC;
    use crate::misc::interval::IntervalD;
    use crate::misc::test_index::TestIndex;
    use crate::position::pattern::{ChunkBasedPatternC, PatternD};
    use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;

    #[test]
    fn pair_o() {
        let relation = AllenIntervalAlgebraRelation::O;
        let interval_c = IntervalC::new(
            vec![
                // IntervalD::new(0, false, 0, 1),
                // IntervalD::new(1, false, 1, 2),
                (0, IntervalD::new(0, 1)),
                (1, IntervalD::new(1, 2)),
            ]
            .into_iter()
            .collect(),
        );

        let index_offset = 0;
        let data_offset = 0;
        let pair_sequence_d =
            ByteSequenceD::of_data(&relation, &interval_c, index_offset, data_offset);

        let sequence_index = TestIndex(0);
        let simple_chunk_pattern_v = PatternD::of_simple_chunk_pattern_v();
        let internet_checksum_chunk_pattern_v = PatternD::of_internet_checksum_chunk_pattern_v();
        let ipaddr = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));
        let ipv4_invariant_checksum_chunk_pattern_c =
            ChunkBasedPatternC::of_invariant_checksum(100, 100, ipaddr);
        let ipv6_invariant_checksum_chunk_pattern_c =
            ChunkBasedPatternC::of_invariant_checksum(100, 100, ipaddr);
        let temporal_position_v = vec![0, 1];
        let byte_time_pair_sequence_d = ByteTimeSequenceD::<AllenIntervalAlgebraRelation>::of_data(
            sequence_index,
            &ExportMode::Isolated,
            &simple_chunk_pattern_v,
            &internet_checksum_chunk_pattern_v,
            &ipv4_invariant_checksum_chunk_pattern_c,
            &ipv6_invariant_checksum_chunk_pattern_c,
            &pair_sequence_d,
            &temporal_position_v,
        );

        let simple_payload_0 = "AB".to_string();
        let simple_payload_1 = "CD".to_string();
        let internet_checksum_payload_0 = "AABBCCDDAABBDDCC".to_string();
        let internet_checksum_payload_1 = "AACCBBDDAACCDDBB".to_string();
        let ipv4_invariant_checksum_payload_0 = "000000oo000001on".to_string();
        let ipv4_invariant_checksum_payload_1 = "001001nn001002nm".to_string();
        let ipv6_invariant_checksum_payload_0 = "000000oo000001on".to_string();
        let ipv6_invariant_checksum_payload_1 = "001001nn001002nm".to_string();
        let simple_payload_byte_length = 3;
        let internet_checksum_payload_byte_length = 24;
        let invariant_checksum_payload_byte_length = 24;
        let chunk_c = ChunkC::new(
            vec![
                (
                    0,
                    ChunkD::new(
                        0,
                        true,
                        // true,
                        0,
                        simple_payload_0.clone(),
                        simple_payload_0.as_bytes().to_vec(),
                        internet_checksum_payload_0.clone(),
                        internet_checksum_payload_0.as_bytes().to_vec(),
                        ipv4_invariant_checksum_payload_0.clone(),
                        ipv4_invariant_checksum_payload_0.as_bytes().to_vec(),
                        ipv6_invariant_checksum_payload_0.clone(),
                        ipv6_invariant_checksum_payload_0.as_bytes().to_vec(),
                    ),
                ),
                (
                    1,
                    ChunkD::new(
                        1,
                        false,
                        // false,
                        1,
                        simple_payload_1.clone(),
                        simple_payload_1.as_bytes().to_vec(),
                        internet_checksum_payload_1.clone(),
                        internet_checksum_payload_1.as_bytes().to_vec(),
                        ipv4_invariant_checksum_payload_1.clone(),
                        ipv4_invariant_checksum_payload_1.as_bytes().to_vec(),
                        ipv6_invariant_checksum_payload_1.clone(),
                        ipv6_invariant_checksum_payload_1.as_bytes().to_vec(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );
        let byte_time_pair_sequence_d_ref = ByteTimeSequenceD::<AllenIntervalAlgebraRelation>::new(
            sequence_index,
            relation,
            interval_c,
            chunk_c,
            temporal_position_v,
            simple_payload_byte_length,
            internet_checksum_payload_byte_length,
            invariant_checksum_payload_byte_length,
        );

        assert_eq!(byte_time_pair_sequence_d, byte_time_pair_sequence_d_ref);
    }
}

#[derive(Debug, Clone)]
pub struct TestingTraceC<I: Into<IpAddr>> {
    hm: HashMap<TestIndex, TestingTraceD<I>>,
}

impl<I: Into<IpAddr> + IpAddrForFragmentationTesting> FromIterator<(TestIndex, TestingTraceD<I>)>
    for TestingTraceC<I>
{
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = (TestIndex, TestingTraceD<I>)>,
    {
        Self {
            hm: HashMap::from_iter(iter),
        }
    }
}

impl<I: Debug + Copy + Into<IpAddr> + IpAddrForFragmentationTesting> TestingTraceC<I> {
    pub fn len(&self) -> usize {
        self.hm.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hm.is_empty()
    }

    pub fn get(&self, index: &TestIndex) -> Option<&TestingTraceD<I>> {
        self.hm.get(index)
    }

    pub fn iter(&self) -> Iter<TestIndex, TestingTraceD<I>> {
        self.hm.iter()
    }

    /// Build an ICMP header.
    pub fn build_icmp_header_data_v(
        policy_evaluation: &PolicyEvaluation,
        payload_mode: &PayloadMode,
        internet_checksum_chunk_pattern_sl: &[u8],
        // test_index_offset: u16,
        // test_index: u16,
        // sequence_index: u16,
        icmp_identifier: u16,
        icmp_sequence_number: u16,
        interval_c: &IntervalC,
        ip_src: I,
        ip_dst: I,
    ) -> Vec<u8> {
        debug!("build_icmp_header_data_v: start");
        // We build a fake packet to obtain a valid header.
        // Get the total payload length from generated relations
        let total_length = interval_c.get_total_length();
        // We increase by one if we add another interval/chunk at the end.
        let chunk_nb_for_checksum = match policy_evaluation {
            PolicyEvaluation::OnceEndFollows => total_length + 1,
            PolicyEvaluation::OnceEndPrecedes => total_length + 1,
            PolicyEvaluation::OnceStartFollowsEndFollows => total_length + 1,
            PolicyEvaluation::OnceEndFollowsStartFollows => total_length + 1,
            PolicyEvaluation::OnceStartPrecedesEndFollows => total_length + 1,
            PolicyEvaluation::OnceEndPrecedesStartFollows => total_length + 1,
            PolicyEvaluation::OnceStartPrecedesEndPrecedes => total_length + 1,
            PolicyEvaluation::OnceEndPrecedesStartPrecedes => total_length + 1,
            _ => total_length,
        };

        // Get the data for this total length
        //let pattern_v = &internet_checksum_chunk_pattern_sl[0..chunk_nb_for_checksum as usize];
        //debug!("build_icmp_header_data_v: pattern_v: {:?}",pattern_v);
        //let data = pattern_v.join("").as_bytes().to_vec();
        let factor = payload_mode.get_factor();
        let data: &[u8] = match payload_mode {
            PayloadMode::VariableChecksum1Byte(_)
            | PayloadMode::InvariantChecksumFixedLength8Byte(_) => {
                &internet_checksum_chunk_pattern_sl[0..(chunk_nb_for_checksum * factor) as usize]
            }
            PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(_)
            | PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(_) => &[],
        };

        // Build the ICMP packet with this (and thus correct checksum)
        let icmp_pdu_data_v = I::build_icmp_pdu_data_v(
            icmp_identifier,
            icmp_sequence_number,
            &data,
            &ip_src,
            &ip_dst,
        );
        // Extract the ICMP header (type, code, identifier, sequence number) for the first
        // fragmentation block (it corresponds to a fragmentation offset of one, 8bytes)
        let icmp_header = &icmp_pdu_data_v[0..8].to_vec();

        debug!("build_icmp_header_data_v: end");
        (*icmp_header).clone()
    }

    /// Build an UDP header.
    pub fn build_udp_header_data_v(
        policy_evaluation: &PolicyEvaluation,
        payload_mode: &PayloadMode,
        internet_checksum_chunk_pattern_sl: &[u8],
        udp_src_port: u16,
        // test_index: u16,
        // sequence_index: u16,
        interval_c: &IntervalC,
        ip_src: I,
        ip_dst: I,
    ) -> Vec<u8> {
        // We build a fake packet to obtain a valid header.
        // Get the total payload length from generated relations
        let total_length = interval_c.get_total_length();
        // We increase by one if we add another interval/chunk at the end.
        let chunk_nb_for_checksum = match policy_evaluation {
            PolicyEvaluation::OnceEndFollows => total_length + 1,
            PolicyEvaluation::OnceEndPrecedes => total_length + 1,
            PolicyEvaluation::OnceStartFollowsEndFollows => total_length + 1,
            PolicyEvaluation::OnceEndFollowsStartFollows => total_length + 1,
            PolicyEvaluation::OnceStartPrecedesEndFollows => total_length + 1,
            PolicyEvaluation::OnceEndPrecedesStartFollows => total_length + 1,
            PolicyEvaluation::OnceStartPrecedesEndPrecedes => total_length + 1,
            PolicyEvaluation::OnceEndPrecedesStartPrecedes => total_length + 1,
            _ => total_length,
        };

        // Get the data for this total length
        //let pattern_v = &internet_checksum_chunk_pattern_sl[0..chunk_nb_for_checksum as usize];
        //debug!("build_icmp_header_data_v: pattern_v: {:?}",pattern_v);
        //let data = pattern_v.join("").as_bytes().to_vec();
        let factor = payload_mode.get_factor();
        //let data = (&internet_checksum_chunk_pattern_sl[0..(chunk_nb_for_checksum * factor) as usize]).to_vec();
        let data: &[u8] = match payload_mode {
            PayloadMode::VariableChecksum1Byte(_)
            | PayloadMode::InvariantChecksumFixedLength8Byte(_) => {
                &internet_checksum_chunk_pattern_sl[0..(chunk_nb_for_checksum * factor) as usize]
            }
            PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(_)
            | PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(_) => &[],
        };

        // Build the ICMP packet with this (and thus correct checksum)
        // let udp_packet_data = I::build_udp_data(test_index, sequence_index, &data);
        let udp_pdu_data_v = I::build_udp_pdu_data_v(udp_src_port, &data, &ip_src, &ip_dst);
        // Extract the UDPP header (src port, dst port, length, checksum) for the first
        // fragmentation block (it corresponds to a fragmentation offset of one, 8bytes)
        let udp_header_data_v = &udp_pdu_data_v[0..8].to_vec();

        (*udp_header_data_v).clone()
    }

    pub fn of_data_byte_time_pair_sequence_c(
        // internet_checksum_chunk_pattern_v: &Vec<String>,
        test_target: &TestTarget<I>,
        policy_evaluation: &PolicyEvaluation,
        // export_mode: &ExportMode,
        internet_checksum_chunk_pattern_sl: &[u8],
        test_index_offset: TestIndex,
        byte_time_pair_sequence_c: &ByteTimeSequenceC<AllenIntervalAlgebraRelation>,
        payload_mode: &PayloadMode,
    ) -> TestingTraceC<I> {
        debug!("of_data: start");

        // debug!("of_triplet_sequence_c: building position permutation");
        // let temporal_position_v_v = std::ops::Range { start: 0, end: 2 }
        //     .permutations(2)
        //     .collect::<Vec<Vec<u32>>>();
        // debug!(
        //     "of_pair_sequence_c: temporal_position_v_v: {:?}",
        //     temporal_position_v_v
        // );

        let mut tuple_v: Vec<(&TestIndex, &ByteTimeSequenceD<AllenIntervalAlgebraRelation>)> =
            byte_time_pair_sequence_c.iter().collect();
        tuple_v.sort_by_key(|(k, _v)| *k);

        debug!("of_data: building v");
        let map = tuple_v.iter().map(|(test_index, byte_time_sequence_d)| {
            debug!("of_data: sequence_index: {:?}", test_index);

            let udp_src_port = test_index_offset.0 + test_index.0;

            assert_eq!(**test_index, byte_time_sequence_d.get_byte_sequence_index());

            let udp_header_data_v = TestingTraceC::build_udp_header_data_v(
                policy_evaluation,
                payload_mode,
                internet_checksum_chunk_pattern_sl,
                udp_src_port,
                // test_index_offset,
                // **test_index,
                // byte_time_sequence_d.get_byte_sequence_index(),
                byte_time_sequence_d.get_interval_c(),
                test_target.ip_addr_src,
                test_target.ip_addr_dst,
            );

            (
                **test_index,
                TestingTraceD::of_data_byte_time_sequence_d(
                    //internet_checksum_chunk_pattern_sl,
                    test_target,
                    policy_evaluation,
                    &udp_header_data_v,
                    // **test_index,
                    byte_time_sequence_d,
                    payload_mode,
                ),
            )
        });

        debug!("of_data: building hm");
        let hm = map.into_iter().collect();

        debug!("of_data: end");
        TestingTraceC { hm }
    }

    pub fn of_data_byte_time_triplet_sequence_c(
        // internet_checksum_chunk_pattern_v: &Vec<String>,
        test_target: &TestTarget<I>,
        policy_evaluation: &PolicyEvaluation,
        internet_checksum_chunk_pattern_sl: &[u8],
        test_index_offset: TestIndex,
        byte_time_triplet_sequence_c: &ByteTimeSequenceC<RelationTripletD>,
        payload_mode: &PayloadMode,
    ) -> TestingTraceC<I> {
        debug!("of_data_byte_time_triplet_sequence_c: start");

        // debug!("of_triplet_sequence_c: building position permutation");
        // let temporal_position_v_v = std::ops::Range { start: 0, end: 2 }
        //     .permutations(2)
        //     .collect::<Vec<Vec<u32>>>();
        // debug!(
        //     "of_pair_sequence_c: temporal_position_v_v: {:?}",
        //     temporal_position_v_v
        // );

        let mut tuple_v: Vec<(&TestIndex, &ByteTimeSequenceD<RelationTripletD>)> =
            byte_time_triplet_sequence_c.iter().collect();
        tuple_v.sort_by_key(|(k, _v)| *k);

        // TEST TO REMOVE
        tuple_v.remove(0);
        tuple_v.remove(0);
        tuple_v.remove(0);

        debug!("of_data_byte_time_triplet_sequence_c: building v");
        let map = tuple_v
            .iter()
            .map(|(test_index, byte_time_triplet_sequence_d)| {
                debug!(
                    "of_data_byte_time_triplet_sequence_c: sequence_index: {:?}",
                    test_index
                );

                let udp_src_port = test_index_offset.0 + test_index.0;

                let udp_header_data_v = TestingTraceC::build_udp_header_data_v(
                    policy_evaluation,
                    payload_mode,
                    internet_checksum_chunk_pattern_sl,
                    udp_src_port,
                    // test_index_offset,
                    // **test_index,
                    // byte_time_triplet_sequence_d.get_byte_sequence_index(),
                    byte_time_triplet_sequence_d.get_interval_c(),
                    test_target.ip_addr_src,
                    test_target.ip_addr_dst,
                );

                (
                    **test_index,
                    TestingTraceD::of_data_byte_time_sequence_d(
                        //internet_checksum_chunk_pattern_sl,
                        test_target,
                        policy_evaluation,
                        &udp_header_data_v,
                        // **test_index,
                        byte_time_triplet_sequence_d,
                        payload_mode,
                    ),
                )
            });

        debug!("of_data_byte_time_triplet_sequence_c: building hm");
        let hm = map.into_iter().collect();

        debug!("of_data_byte_time_triplet_sequence_c: end");
        TestingTraceC { hm }
    }

    pub fn export(
        &self,
        payload_protocol: IpNextHeaderProtocol,
        pcap_directory_path: &Path,
        test_index_offset: TestIndex,
    ) {
        debug!("export: start");
        self.hm.iter().for_each(|(test_index, testing_trace_d)| {
            debug!("export: test_index: {:?}", test_index);
            testing_trace_d.export(
                payload_protocol,
                pcap_directory_path,
                test_index_offset,
                *test_index,
            )
        });
        debug!("export: end");
    }

    // pub fn export(
    //     &self,
    //     test_target: &TestTarget,
    //     pcap_directory_path: &Path,
    //     icmp_header: &Vec<u8>,
    //     test_index: u32,
    //     // chunk_c: &ChunkC,
    //     temporal_position_v: &Vec<u32>,
    // ) {
    //     debug!("export_chunk_c: start");
    //
    //     let index_data_v = self.hm.iter().map(|i, icmp_trace_d| {
    //         icmp_trace_d.to_data
    //     }).collect::<Vec<(u16, Vec<u8>)>>();
    //
    //     // let output_path_s = format!("{}/test_{}.pcap",pcap_directory_path, test_index);
    //     // let output_path = Path::new(&output_path_s);
    //
    //     // let output_path = PathBuf::new();
    //     // output_path.push(pcap_directory_path);
    //     // output_path.push(format!("test_{}.pcap",test_index));
    //     let output_path = pcap_directory_path.join(format!("test_{}.pcap", test_index));
    //
    //     // let output_path_s = format!("{}/test_{}.pcap",pcap_directory_path, test_index);
    //     debug!("export_chunk_c: output_path: {:?}", output_path);
    //
    //     let mut output_file = if output_path
    //         .to_str()
    //         .expect("Could not convert output_path to string")
    //         == "-"
    //     {
    //         Box::new(io::stdout())
    //     } else {
    //         let file = File::create(output_path).unwrap();
    //         Box::new(file) as Box<dyn io::Write>
    //     };
    //
    //     debug!("export_chunk_c: pcap_write_header");
    //     icmp_echo_request_generation_generic::pcap_write_header(&mut output_file, 1500 as usize).unwrap();
    //
    //     index_position_ethernet_data_v.iter().enumerate().for_each(
    //         |(i, (_index, _position, ethernet_data))| {
    //             let ts_sec = i as u32;
    //             let ts_usec = 0;
    //             let written = icmp_echo_request_generation_generic::pcap_write_packet(
    //                 &mut output_file,
    //                 ts_sec,
    //                 ts_usec,
    //                 &ethernet_data,
    //             )
    //             .unwrap();
    //             debug!("export_chunk_c: written: {}", written);
    //         },
    //     );
    //     debug!("export_chunk_c: end");
    // }
}
