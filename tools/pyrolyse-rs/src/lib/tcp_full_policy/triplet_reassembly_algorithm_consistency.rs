use serde::{Deserialize, Serialize};

use crate::policy_common::policy_consistency::PolicyConsistency;
use crate::position::payload_mode::PayloadMode;
use crate::policy_common::pair_relation_time_choice::PairRelationTimeChoice;
use crate::misc::interval::{IntervalC,IntervalD};
use crate::misc::pair_time_position::PairTimePosition;
use crate::policy_common::pair_choice::PairChoice;
use crate::byte_time_data::chunk::{ChunkD,ChunkC};
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::relation::relation_triplet::RelationTripletD;
use std::cmp::{min,max};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct TripletReassemblyAlgorithmConsistency {
    qoaimn: PolicyConsistency,
    qoadmn: PolicyConsistency,
    qoaimm: PolicyConsistency,
    qoadmm: PolicyConsistency,
    qoaima: PolicyConsistency,
    qoadma: PolicyConsistency,
}

impl TripletReassemblyAlgorithmConsistency {
    // using vec of (interval,chunk_id), ordered by interval.start
    pub fn of_data(
        payload_mode: &PayloadMode,
        interval_c: &IntervalC,
        chunk_c: &ChunkC,
        pair_relation_time_choice: &PairRelationTimeChoice,
        payload_string_option: &Option<Vec<u8>>,
        //triplet_position_policy: &TripletPositionPolicy, 
        relation_triplet_d: &RelationTripletD, 
    ) -> TripletReassemblyAlgorithmConsistency {
        let qoaimn = Self::build_queued_by_offset_fast_chunk_alteration_no_merging(
            payload_mode,
            interval_c,
            chunk_c,
            pair_relation_time_choice,
            payload_string_option,
            relation_triplet_d,
        );

        let qoadmn = Self::build_queued_by_offset_slow_chunk_alteration_no_merging(
            payload_mode,
            interval_c,
            chunk_c,
            pair_relation_time_choice,
            payload_string_option,
            relation_triplet_d,
        );

        let qoaimm = Self::build_queued_by_offset_meet_merging(
            payload_mode,
            interval_c,
            chunk_c,
            pair_relation_time_choice,
            payload_string_option,
            relation_triplet_d,
            true
        );

        let qoadmm = Self::build_queued_by_offset_meet_merging(
            payload_mode,
            interval_c,
            chunk_c,
            pair_relation_time_choice,
            payload_string_option,
            relation_triplet_d,
            false
        );

        let qoaima = Self::build_queued_by_offset_any_merging(
            payload_mode,
            interval_c,
            chunk_c,
            pair_relation_time_choice,
            payload_string_option,
            relation_triplet_d,
            true
        );

        let qoadma = Self::build_queued_by_offset_any_merging(
            payload_mode,
            interval_c,
            chunk_c,
            pair_relation_time_choice,
            payload_string_option,
            relation_triplet_d,
            false
        );

        TripletReassemblyAlgorithmConsistency {
            qoaimn,
            qoadmn,
            qoaimm,
            qoadmm,
            qoaima,
            qoadma,
        }
    }

    pub fn build_queued_by_offset_fast_chunk_alteration_no_merging(
        payload_mode: &PayloadMode,
        interval_c: &IntervalC,
        chunk_c: &ChunkC,
        pair_relation_time_choice: &PairRelationTimeChoice,
        payload_string_option: &Option<Vec<u8>>,
        //triplet_position_policy: &TripletPositionPolicy, 
        relation_triplet_d: &RelationTripletD, 
    ) -> PolicyConsistency {
        debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: start");
        //let relation_triplet_d = triplet_position_policy.get_relation_triplet();
        debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: relation_triplet_d: {:?}",relation_triplet_d);
        let relation_01 = relation_triplet_d.get_relation_01().clone();
        let _relation_02 = relation_triplet_d.get_relation_02().clone();
        let _relation_12 = relation_triplet_d.get_relation_12().clone();
        
        let original_interval_0 = interval_c.get(&0).unwrap().clone();
        debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: original_interval_0: {:?}",original_interval_0);
        let original_interval_1 = interval_c.get(&1).unwrap().clone();
        debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: original_interval_1: {:?}",original_interval_1);
        let original_interval_2 = interval_c.get(&2).unwrap().clone();
        debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: original_interval_2: {:?}",original_interval_2);

        // handling pair 01
        let r_01_pair_relation_time_choice = pair_relation_time_choice
                                                .get(&(relation_01,PairTimePosition::Before))
                                                .expect("No pair relation time choice"); // always before in the way we design test cases
        let step_01_interval_v = Self::resolve_overlap(
            r_01_pair_relation_time_choice,
            &original_interval_0,
            &original_interval_1,
            0,
            1
        );
        debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: step_01_interval_v: {:?}",step_01_interval_v);
        
        // handling modified chunks 0 and 1 with chunk 2 
        let mut curr_interval_2_o = Some(original_interval_2.clone()); // interval_2 being modified across overlap resolution
        let mut tmp_interval_2_o = curr_interval_2_o.clone(); // interval_2 being modified across overlap resolution
        let mut curr_reassembly_roff = 0; // XXX moove reassembly step afterwards ?
        let factor = payload_mode.get_factor();
        
        let mut reassembled_chunk_interval_no_overlap_v: Vec<(u16,IntervalD)> = Vec::<(u16,IntervalD)>::new();
        
        step_01_interval_v
            .iter()
            .for_each(|(interval_d_o,chunk_id)| {
                match interval_d_o {
                    Some(interval_d) => {
                        match &curr_interval_2_o {
                            Some(curr_interval_2) => {
                                let curr_allen_relation = AllenIntervalAlgebraRelation::of_intervals(interval_d,&curr_interval_2);
                                debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: curr_allen_relation: {:?}",curr_allen_relation);
                                let curr_pair_choice = pair_relation_time_choice
                                                                        .get(&(curr_allen_relation,PairTimePosition::Before))
                                                                        .expect("No pair relation time choice"); // always before in the way we design test cases
                                debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: curr_pair_choice: {:?}",curr_pair_choice);
                                let after_resolution_interval_v = Self::resolve_overlap(
                                    curr_pair_choice,
                                    &interval_d,
                                    &curr_interval_2,
                                    *chunk_id,
                                    2,
                                );
                                debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: after_resolution_interval_v: {:?}",after_resolution_interval_v);
        
                                after_resolution_interval_v
                                    .iter()
                                    .for_each(|(after_resolution_interval_d_o,after_resolution_chunk_id)| {
                                        debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: after_resolution_interval_d_o: {:?}",after_resolution_interval_d_o);
                                        match after_resolution_interval_d_o {
                                            Some(after_resolution_interval_d) => {
                                                debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: after_resolution_chunk_id: {}", after_resolution_chunk_id);
                                                match after_resolution_chunk_id {
                                                    0 | 1 => {
                                                        if after_resolution_interval_d.get_start() * factor == curr_reassembly_roff {
                                                            curr_reassembly_roff = (after_resolution_interval_d.get_end() + 1) * factor;
                                                            debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: curr_reassembly_roff: {}",curr_reassembly_roff);

                                                            reassembled_chunk_interval_no_overlap_v.push((
                                                                *after_resolution_chunk_id,
                                                                after_resolution_interval_d.clone()
                                                            ));
                                                            debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: reassembled_chunk_interval_no_overlap_v: {:?}",reassembled_chunk_interval_no_overlap_v);

                                                        }
                                                    },
                                                    2 => {
                                                        // update reassembly fields only if interval_d (i.e., chunk 0 or 1) end is forward (chunked) chunk 2, otherwise there might be remaining overlap to take care of
                                                        if interval_d.get_end() >= after_resolution_interval_d.get_end() && after_resolution_interval_d.get_start() * factor == curr_reassembly_roff {
                                                            //let end = if interval_d.get_end() == after_resolution_interval_d.get_end() { interval_d.get_end() } else { after_resolution_interval_d.get_end() };  
                                                            let end = after_resolution_interval_d.get_end();  
                                                            debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: end: {}",end);

                                                            curr_reassembly_roff = (end + 1) * factor;
                                                            debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: curr_reassembly_roff: {}",curr_reassembly_roff);

                                                            //let to_insert_interval_d = IntervalD::new(
                                                            //    after_resolution_interval_d.get_start(),
                                                            //    end,
                                                            //);
                                                            //debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: to_insert_interval_d: {:?}",to_insert_interval_d);
                                                            
                                                            reassembled_chunk_interval_no_overlap_v.push((
                                                                *after_resolution_chunk_id,
                                                                after_resolution_interval_d.clone()
                                                            ));
                                                            debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: reassembled_chunk_interval_no_overlap_v: {:?}",reassembled_chunk_interval_no_overlap_v);

                                                        }

                                                        if interval_d.get_end() == after_resolution_interval_d.get_end() {
                                                            // no remaining overlap to take care of, since the chunk queue (i.e., step_01_interval_v) is composed of non overlapping chunks
                                                            tmp_interval_2_o = None
                                                        } else {
                                                            // there might be some remaining overlap to take care of, no alteration of chunk 2
                                                            tmp_interval_2_o = Some(after_resolution_interval_d.clone())
                                                        }
                                                    debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: tmp_interval_2_o: {:?}",tmp_interval_2_o);
                                                },
                                                    _ => panic!("Unexpected chunk id")
                                                }
                                                debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: reassembled_chunk_interval_no_overlap_v: {:?}",reassembled_chunk_interval_no_overlap_v);
                                            }
                                            None => {
                                                if *after_resolution_chunk_id == 2 {
                                                    tmp_interval_2_o = None;
                                                }
                                            }
                                        }
                                    });
                            }
                            None => {
                                debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: curr_interval_2_o: None");
                                tmp_interval_2_o = None;
                                debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: chunk_id: {}",chunk_id);
                                match chunk_id {
                                    0 | 1 => {
                                        if interval_d.get_start() * factor == curr_reassembly_roff {
                                            curr_reassembly_roff = (interval_d.get_end() + 1) * factor;
                                            debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: curr_reassembly_roff: {}",curr_reassembly_roff);
                                            
                                            reassembled_chunk_interval_no_overlap_v.push((
                                                *chunk_id,
                                                interval_d.clone()
                                            ));
                                            debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: reassembled_chunk_interval_no_overlap_v: {:?}",reassembled_chunk_interval_no_overlap_v);

                                        }
                                    },
                                    _ => panic!("Unexpected chunk id"),
                                }
                            }
                        }
                    }
                    None => { }
                }
                curr_interval_2_o = tmp_interval_2_o.clone();
            });
        
        // possible remaining chunk 2 data
        if let Some(curr_interval_2) = curr_interval_2_o {
            if curr_interval_2.get_start() * factor == curr_reassembly_roff {
                reassembled_chunk_interval_no_overlap_v.push((
                    2,
                    curr_interval_2
                ));
                debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: reassembled_chunk_interval_no_overlap_v: {:?}",reassembled_chunk_interval_no_overlap_v);
            }
        }

        // build the reassembled payload
        let reconstructed_payload_ascii_v: Vec<u8> = Self::build_reconstructed_payload_ascii_v(
            chunk_c,
            payload_mode,
            reassembled_chunk_interval_no_overlap_v
        );
        debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: reconstructed_payload_ascii_v: {:?}",reconstructed_payload_ascii_v);
        
        // check consistency between implem reassembled payload and ours
        debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: end");
        Self::check_payload_consistency(
            &reconstructed_payload_ascii_v,
            payload_string_option,
        )
    }

    pub fn build_queued_by_offset_slow_chunk_alteration_no_merging(
        payload_mode: &PayloadMode,
        interval_c: &IntervalC,
        chunk_c: &ChunkC,
        pair_relation_time_choice: &PairRelationTimeChoice,
        payload_string_option: &Option<Vec<u8>>,
        //triplet_position_policy: &TripletPositionPolicy, 
        relation_triplet_d: &RelationTripletD, 
    ) -> PolicyConsistency {
        debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: start");
        //let relation_triplet_d = triplet_position_policy.get_relation_triplet();
        debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: relation_triplet_d: {:?}",relation_triplet_d);
        let relation_01 = relation_triplet_d.get_relation_01().clone();
        let _relation_02 = relation_triplet_d.get_relation_02().clone();
        let _relation_12 = relation_triplet_d.get_relation_12().clone();
        
        let original_interval_0 = interval_c.get(&0).unwrap().clone();
        debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: original_interval_0: {:?}",original_interval_0);
        let original_interval_1 = interval_c.get(&1).unwrap().clone();
        debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: original_interval_1: {:?}",original_interval_1);
        let original_interval_2 = interval_c.get(&2).unwrap().clone();
        debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: original_interval_2: {:?}",original_interval_2);

        // handling pair 01
        let r_01_pair_relation_time_choice = pair_relation_time_choice
                                                .get(&(relation_01,PairTimePosition::Before))
                                                .expect("No pair relation time choice"); // always before in the way we design test cases
        let step_01_interval_v = Self::resolve_overlap(
            r_01_pair_relation_time_choice,
            &original_interval_0,
            &original_interval_1,
            0,
            1
        );
        debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: step_01_interval_v: {:?}",step_01_interval_v);

        // handling interval_v with chunk 2 by reconstructing payload 
        //let mut interval_2_start = 0;
        let mut interval_2_start = original_interval_2.get_start();
        let mut curr_reassembly_roff = 0;
        let factor = payload_mode.get_factor();

        let mut reassembled_chunk_interval_no_overlap_v: Vec<(u16,IntervalD)> = Vec::<(u16,IntervalD)>::new();
        
        step_01_interval_v
            .iter()
            .for_each(|(interval_d_o,chunk_id)| {
                    if let Some(interval_d) = interval_d_o {
                    let curr_allen_relation = AllenIntervalAlgebraRelation::of_intervals(interval_d,&original_interval_2);
                    debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: curr_allen_relation: {:?}",curr_allen_relation);
                    let curr_pair_choice = pair_relation_time_choice
                                                            .get(&(curr_allen_relation,PairTimePosition::Before))
                                                            .expect("No pair relation time choice"); // always before in the way we design test cases
                    debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: curr_pair_choice: {:?}",curr_pair_choice);
                    // resolve overlap with original_interval_2 (that's all the point of this algorithm)
                    let after_resolution_interval_v = Self::resolve_overlap(
                        curr_pair_choice,
                        &interval_d,
                        &original_interval_2,
                        *chunk_id,
                        2,
                    );
                    debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: after_resolution_interval_v: {:?}",after_resolution_interval_v);

                    after_resolution_interval_v
                        .iter()
                        .for_each(|(after_resolution_interval_d_o,after_resolution_chunk_id)| {
                            debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: after_resolution_interval_d_o: {:?}",after_resolution_interval_d_o);
                            if let Some(after_resolution_interval_d) = after_resolution_interval_d_o {
                                debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: after_resolution_chunk_id: {}", after_resolution_chunk_id);
                                match after_resolution_chunk_id {
                                    0 | 1 => {
                                        if after_resolution_interval_d.get_start() * factor == curr_reassembly_roff {
                                            curr_reassembly_roff = (after_resolution_interval_d.get_end() + 1) * factor;
                                            debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: curr_reassembly_roff: {}",curr_reassembly_roff);

                                            reassembled_chunk_interval_no_overlap_v.push((
                                                *after_resolution_chunk_id,
                                                after_resolution_interval_d.clone()
                                            ));
                                            debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: reassembled_chunk_interval_no_overlap_v: {:?}",reassembled_chunk_interval_no_overlap_v);

                                        }
                                    },
                                    2 => {
                                        // update reassembly fields only if interval_d (i.e., chunk 0 or 1) end is forward (chunked) chunk 2, otherwise there might be remaining overlap to take care of
                                        if interval_d.get_end() >= after_resolution_interval_d.get_end() {
                                            // we may have already reassembled forward after_resolution_interval_d's start
                                            let start = max(interval_2_start,after_resolution_interval_d.get_start());
                                            debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: start: {}",start);

                                            if start * factor == curr_reassembly_roff {
                                                let end = after_resolution_interval_d.get_end();
                                                debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: end: {}",end);

                                                curr_reassembly_roff = (end + 1) * factor;
                                                debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: curr_reassembly_roff: {}",curr_reassembly_roff);

                                                let to_insert_interval_d = IntervalD::new(
                                                    start,
                                                    end,
                                                );
                                                debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: to_insert_interval_d: {:?}",to_insert_interval_d);
                                                
                                                reassembled_chunk_interval_no_overlap_v.push((
                                                    *after_resolution_chunk_id,
                                                    to_insert_interval_d.clone()
                                                ));
                                                debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: reassembled_chunk_interval_no_overlap_v: {:?}",reassembled_chunk_interval_no_overlap_v);
                                            }
                                        }

                                        if interval_d.get_end() != after_resolution_interval_d.get_end() {
                                            let curr_reassembly_roff_no_factor = (curr_reassembly_roff / factor) as u16;
                                            
                                            // preventing from reassembling after a hole and moove forward interval_2's start
                                            if curr_reassembly_roff_no_factor >= after_resolution_interval_d.get_start() && curr_reassembly_roff_no_factor <= after_resolution_interval_d.get_end() {
                                                interval_2_start = curr_reassembly_roff_no_factor;
                                                debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: interval_2_start: {:?}",interval_2_start);
                                            }
                                        }
                                    },
                                    _ => panic!("Unexpected chunk id")
                                }
                                debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: reassembled_chunk_interval_no_overlap_v: {:?}",reassembled_chunk_interval_no_overlap_v);
                            }
                        });
                }
            });

        if interval_2_start * factor == curr_reassembly_roff {
            let remaining_interval_2 = IntervalD::new(interval_2_start,original_interval_2.get_end());
            reassembled_chunk_interval_no_overlap_v.push((
                2,
                remaining_interval_2
            ));
            debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: reassembled_chunk_interval_no_overlap_v: {:?}",reassembled_chunk_interval_no_overlap_v);
        }

        // build the reassembled payload
        let reconstructed_payload_ascii_v: Vec<u8> = Self::build_reconstructed_payload_ascii_v(
            chunk_c,
            payload_mode,
            reassembled_chunk_interval_no_overlap_v
        );
        debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: reconstructed_payload_ascii_v: {:?}",reconstructed_payload_ascii_v);
        debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: payload_string_option: {:?}",payload_string_option);

        let reconstructed_payload_ascii: String = reconstructed_payload_ascii_v.iter().map(|a| *a as char).collect();
        debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: reconstructed_payload_ascii: {:?}",reconstructed_payload_ascii);
        
        // check consistency between implem reassembled payload and ours
        debug!("build_queued_by_offset_slow_chunk_alteration_no_merging: end");
        Self::check_payload_consistency(
            &reconstructed_payload_ascii_v,
            payload_string_option,
        )

    }

    fn resolve_overlap(
        pair_choice: &PairChoice,
        original_interval_a: &IntervalD,
        original_interval_b: &IntervalD,
        chunk_a_id: u16,
        chunk_b_id: u16,
    //) -> (Option<Vec<IntervalD>>,Option<Vec<IntervalD>>) {
    ) -> Vec<(Option<IntervalD>,u16)> {
        let (revised_interval_a_v_o,revised_interval_b_v_o) = match pair_choice {
            PairChoice::First => {
            debug!("resolve_overlap: PairChoice::First");
                let intersection_ab = original_interval_a.intersection(&original_interval_b).unwrap();
                debug!("resolve_overlap: intersection_ab: {:?}",intersection_ab);
                let interval_b_v_o = original_interval_b.remove_intersection(&intersection_ab);
                debug!("resolve_overlap: interval_b_v_o: {:?}",interval_b_v_o);

                (
                    Some(vec![original_interval_a.clone()]),
                    interval_b_v_o,
                )
            },
            PairChoice::Second => {
                debug!("resolve_overlap: PairChoice::Second");
                let intersection_ab = original_interval_a.intersection(&original_interval_b).unwrap();
                debug!("resolve_overlap: intersection_ab: {:?}",intersection_ab);
                let interval_a_v_o = original_interval_a.remove_intersection(&intersection_ab);
                debug!("resolve_overlap: interval_a_v_o: {:?}",interval_a_v_o);
                (
                    interval_a_v_o,
                    Some(vec![original_interval_b.clone()]),
                )
            },
            PairChoice::Ignore => {
                // no reply
                debug!("resolve_overlap: PairChoice::Ignore");
                (None,None)
            },
            PairChoice::OverlapIgnore => {
                // triple overlap, data from 3rd chunk is prefered
                debug!("resolve_overlap: PairChoice::OverlapIgnore");
                let intersection_ab = original_interval_a.intersection(&original_interval_b).unwrap();
                let interval_a_v_o = original_interval_a.remove_intersection(&intersection_ab);
                debug!("resolve_overlap: interval_a_v_o: {:?}",interval_a_v_o);
                let interval_b_v_o = original_interval_b.remove_intersection(&intersection_ab);
                debug!("resolve_overlap: interval_b_v_o: {:?}",interval_b_v_o);
                (
                    interval_a_v_o,
                    interval_b_v_o,
                )
            },
            PairChoice::PartialIgnore => {
                // partial reply
                // we don't know which part, so we panic (for now)
                debug!("resolve_overlap: PairChoice::PartialIgnore");
                panic!("Don't know how to handle a PairChoice::PartialIgnore");
            },
            PairChoice::Both => {
                // we got several data for an overlap portion
                // we panic (for now)
                debug!("resolve_overlap: PairChoice::Both");
                panic!("We got several data for an overlap portion");
            },
            PairChoice::Bug => {
                // we got unexpected data 
                // we panic (for now)
                debug!("resolve_overlap: PairChoice::Bug");
                panic!("We got several data for an overlap portion");
            },
            PairChoice::None => {
                // we keep it as it is
                debug!("resolve_overlap: PairChoice::None");
                (
                    Some(vec![original_interval_a.clone()]),
                    Some(vec![original_interval_b.clone()]),
                )
            },
        };

        let interval_v: Vec<(Option<IntervalD>,u16)> = match revised_interval_a_v_o {
            Some(revised_interval_a_v) => {
                match revised_interval_b_v_o {
                    Some(revised_interval_b_v) => {
                        if revised_interval_a_v.len() == 1 {
                            if revised_interval_b_v.len() == 1 {
                                if revised_interval_a_v.first().unwrap().get_start() < revised_interval_b_v.first().unwrap().get_start() {
                                    // new interval_a is before new interval_b 
                                    vec![
                                        (Some(revised_interval_a_v.first().unwrap().clone()),chunk_a_id),
                                        (Some(revised_interval_b_v.first().unwrap().clone()),chunk_b_id),
                                    ]
                                } else {
                                    // new interval_b is before new interval_a 
                                    vec![
                                        (Some(revised_interval_b_v.first().unwrap().clone()),chunk_b_id),
                                        (Some(revised_interval_a_v.first().unwrap().clone()),chunk_a_id),
                                    ]  
                                }
                            } else {
                                // it is not possible that both revised_interval_a_v and revised_interval_b_v have more than 1 item
                                vec![
                                    (Some(revised_interval_b_v.first().unwrap().clone()),chunk_b_id),
                                    (Some(revised_interval_a_v.first().unwrap().clone()),chunk_a_id),
                                    (Some(revised_interval_b_v.get(1).unwrap().clone()),chunk_b_id),
                                ]
                            }
                        } else {
                            // it is not possible that both revised_interval_a_v and revised_interval_b_v have more than 1 item
                            vec![
                                (Some(revised_interval_a_v.first().unwrap().clone()),chunk_a_id),
                                (Some(revised_interval_b_v.first().unwrap().clone()),chunk_b_id),
                                (Some(revised_interval_a_v.get(1).unwrap().clone()),chunk_a_id),
                            ]
                        }
                    },
                    None => {
                        assert!(revised_interval_a_v.len() == 1);
                        vec![
                            (Some(revised_interval_a_v.first().unwrap().clone()),chunk_a_id),
                            (None,chunk_b_id),
                        ]
                    }
                }
            }
            None => {
                match revised_interval_b_v_o {
                    Some(revised_interval_b_v) => {
                        assert!(revised_interval_b_v.len() == 1);
                        vec![
                            (None,chunk_a_id),
                            (Some(revised_interval_b_v.first().unwrap().clone()),chunk_b_id),
                        ]
                    }
                    None => vec![
                        (None,chunk_a_id),
                        (None,chunk_b_id),
                    ]
                }

            }
        };
        //debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: interval_v: {:?}",interval_v);
        //debug!("build_queued_by_offset_fast_chunk_alteration_no_merging: interval_v: {:?}",interval_v);
        interval_v
    }

    pub fn build_queued_by_offset_meet_merging(
        payload_mode: &PayloadMode,
        interval_c: &IntervalC,
        chunk_c: &ChunkC,
        pair_relation_time_choice: &PairRelationTimeChoice,
        payload_string_option: &Option<Vec<u8>>,
        //triplet_position_policy: &TripletPositionPolicy, 
        relation_triplet_d: &RelationTripletD, 
        is_fast: bool
    ) -> PolicyConsistency {
        debug!("build_queued_by_offset_meet_merging: start");
        //let relation_triplet_d = triplet_position_policy.get_relation_triplet();
        debug!("build_queued_by_offset_meet_merging: relation_triplet_d: {:?}",relation_triplet_d);
        let relation_01 = relation_triplet_d.get_relation_01().clone();

        if relation_01 != AllenIntervalAlgebraRelation::M && relation_01 != AllenIntervalAlgebraRelation::Mi {
            let policy_consistency = match is_fast {
                true => Self::build_queued_by_offset_fast_chunk_alteration_no_merging(
                    payload_mode,
                    interval_c,
                    chunk_c,
                    pair_relation_time_choice,
                    payload_string_option,
                    relation_triplet_d, 
                    ),
                false => Self::build_queued_by_offset_slow_chunk_alteration_no_merging(
                    payload_mode,
                    interval_c,
                    chunk_c,
                    pair_relation_time_choice,
                    payload_string_option,
                    relation_triplet_d, 
                )
            };
            return policy_consistency
        }

        let interval_0 = interval_c.get(&0).unwrap().clone();
        debug!("build_queued_by_offset_meet_merging: interval_0: {:?}",interval_0);
        let interval_1 = interval_c.get(&1).unwrap().clone();
        debug!("build_queued_by_offset_meet_merging: interval_1: {:?}",interval_1);
        let interval_2 = interval_c.get(&2).unwrap().clone();
        debug!("build_queued_by_offset_meet_merging: interval_2: {:?}",interval_2);

        // TODO moove part of chunk 01 build elsewhere
        // building 01 chunk
        let chunk_0 = chunk_c.get(&0).unwrap().clone();
        let chunk_1 = chunk_c.get(&1).unwrap().clone();
        let chunk_2 = chunk_c.get(&2).unwrap().clone();

        let does_01_start = chunk_0.get_start() | chunk_1.get_start();
        let offset_01 = min(chunk_0.get_offset(),chunk_1.get_offset());

        let (chunk_left,chunk_right) = if relation_01.is_inverse() {
            debug!("build_queued_by_offset_meet_merging: 1 is left, 0 is right");
            (chunk_1.clone(),chunk_0.clone())
        } else { 
            debug!("build_queued_by_offset_meet_merging: 0 is left, 1 is right");
            (chunk_0.clone(),chunk_1.clone())
        };

        let simple_s_01 = chunk_left.get_simple_s().clone() + &chunk_right.get_simple_s().clone();
        let internet_checksum_s_01 = chunk_left.get_internet_checksum_s().clone() + &chunk_right.get_internet_checksum_s().clone();
        let ipv4_invariant_checksum_s_01 = chunk_left.get_ipv4_invariant_checksum_s().clone() + &chunk_right.get_ipv4_invariant_checksum_s().clone();
        let ipv6_invariant_checksum_s_01 = chunk_left.get_ipv6_invariant_checksum_s().clone() + &chunk_right.get_ipv6_invariant_checksum_s().clone();
        let simple_ascii_v_01 = [chunk_left.get_simple_ascii_v().clone(),chunk_right.get_simple_ascii_v().clone()].concat();
        let internet_checksum_ascii_v_01 = [chunk_left.get_internet_checksum_ascii_v().clone(),chunk_right.get_internet_checksum_ascii_v().clone()].concat();
        let ipv4_invariant_checksum_ascii_v_01 = [chunk_left.get_ipv4_invariant_checksum_ascii_v().clone(),chunk_right.get_ipv4_invariant_checksum_ascii_v().clone()].concat();
        let ipv6_invariant_checksum_ascii_v_01 = [chunk_left.get_ipv6_invariant_checksum_ascii_v().clone(),chunk_right.get_ipv6_invariant_checksum_ascii_v().clone()].concat();

        let chunk_01 = ChunkD::new(
            0,
            does_01_start,
            offset_01,
            simple_s_01,
            simple_ascii_v_01,
            internet_checksum_s_01,
            internet_checksum_ascii_v_01,
            ipv4_invariant_checksum_s_01,
            ipv4_invariant_checksum_ascii_v_01,
            ipv6_invariant_checksum_s_01,
            ipv6_invariant_checksum_ascii_v_01,
        );
        debug!("build_queued_by_offset_meet_merging: chunk_01: {:?}",chunk_01);
        let new_chunk_c = ChunkC::new(
            BTreeMap::from(
                [(0,chunk_01.clone()),
                (2,chunk_2.clone())]
            )
        );

        let start_01 = min(interval_0.get_start(),interval_1.get_start());
        let end_01 = max(interval_0.get_end(),interval_1.get_end());
        let interval_01 = IntervalD::new(start_01,end_01);
        debug!("build_queued_by_offset_meet_merging: interval_01: {:?}",interval_01);
        let relation_01_2 = AllenIntervalAlgebraRelation::of_intervals(&interval_01,&interval_2);
        debug!("build_queued_by_offset_meet_merging: relation_01_2: {:?}",relation_01_2);

        let interval_chunkid_t_v: Vec<(Option<IntervalD>,u16)> = match relation_01_2 {
            AllenIntervalAlgebraRelation::Bi => {
                assert!(chunk_2.get_start());
                vec![(Some(interval_2.clone()),2)]
            },
            AllenIntervalAlgebraRelation::B => {
                assert!(chunk_01.get_start());
                vec![(Some(interval_01.clone()),0)]
            },
            _ => {
                let r_01_2_pair_relation_time_choice = pair_relation_time_choice
                    .get(&(relation_01_2,PairTimePosition::Before))
                    .expect("No pair relation time choice"); // always before in the way we design test cases
                debug!("build_queued_by_offset_meet_merging: r_01_2_pair_relation_time_choice: {:?}",r_01_2_pair_relation_time_choice);

                Self::resolve_overlap(
                    r_01_2_pair_relation_time_choice,
                    &interval_01,
                    &interval_2,
                    0,
                    2
                )
            }
        };
        debug!("build_queued_by_offset_meet_merging: interval_chunkid_t_v: {:?}",interval_chunkid_t_v);

        let reassembled_chunk_interval_no_overlap_v = interval_chunkid_t_v
            .into_iter()
            .filter(|(interval_o,_chunk_id)| interval_o.is_some())
            .map(|(interval_o,chunk_id)| { (chunk_id,interval_o.unwrap()) } )
            .collect();

        let reconstructed_payload_ascii_v = Self::build_reconstructed_payload_ascii_v(
            &new_chunk_c,
            payload_mode,
            reassembled_chunk_interval_no_overlap_v
        );
        debug!("build_queued_by_offset_meet_merging: reconstructed_payload_ascii_v: {:?}",reconstructed_payload_ascii_v);
        debug!("build_queued_by_offset_meet_merging: payload_string_option: {:?}",payload_string_option);


        Self::check_payload_consistency(
            &reconstructed_payload_ascii_v,
            payload_string_option
        )
    }

    pub fn build_queued_by_offset_any_merging(
        payload_mode: &PayloadMode,
        interval_c: &IntervalC,
        chunk_c: &ChunkC,
        pair_relation_time_choice: &PairRelationTimeChoice,
        payload_string_option: &Option<Vec<u8>>,
        //triplet_position_policy: &TripletPositionPolicy, 
        relation_triplet_d: &RelationTripletD, 
        is_fast: bool
    ) -> PolicyConsistency {
        debug!("build_queued_by_offset_any_merging: start");
        //let relation_triplet_d = triplet_position_policy.get_relation_triplet();
        debug!("build_queued_by_offset_any_merging: relation_triplet_d: {:?}",relation_triplet_d);
        let relation_01 = relation_triplet_d.get_relation_01().clone();

        if relation_01 == AllenIntervalAlgebraRelation::B || relation_01 == AllenIntervalAlgebraRelation::Bi {
            let policy_consistency = match is_fast {
                true => Self::build_queued_by_offset_fast_chunk_alteration_no_merging(
                    payload_mode,
                    interval_c,
                    chunk_c,
                    pair_relation_time_choice,
                    payload_string_option,
                    relation_triplet_d, 
                ),
                false => Self::build_queued_by_offset_slow_chunk_alteration_no_merging(
                    payload_mode,
                    interval_c,
                    chunk_c,
                    pair_relation_time_choice,
                    payload_string_option,
                    relation_triplet_d, 
                )
            };
            return policy_consistency
        } else if relation_01 == AllenIntervalAlgebraRelation::M || relation_01 == AllenIntervalAlgebraRelation::Mi {
            return Self::build_queued_by_offset_meet_merging(
                payload_mode,
                interval_c,
                chunk_c,
                pair_relation_time_choice,
                payload_string_option,
                relation_triplet_d,
                is_fast
            )
        }

        let interval_0 = interval_c.get(&0).unwrap().clone();
        debug!("build_queued_by_offset_any_merging: interval_0: {:?}",interval_0);
        let interval_1 = interval_c.get(&1).unwrap().clone();
        debug!("build_queued_by_offset_any_merging: interval_1: {:?}",interval_1);
        let interval_2 = interval_c.get(&2).unwrap().clone();
        debug!("build_queued_by_offset_any_merging: interval_2: {:?}",interval_2);

        // TODO moove chunk 01 build elsewhere ?
        // building 01 chunk
        let _chunk_0 = chunk_c.get(&0).unwrap().clone();
        let _chunk_1 = chunk_c.get(&1).unwrap().clone();
        let chunk_2 = chunk_c.get(&2).unwrap().clone();

        // handling pair 01
        let r_01_pair_relation_time_choice = pair_relation_time_choice
                                                .get(&(relation_01,PairTimePosition::Before))
                                                .expect("No pair relation time choice"); // always before in the way we design test cases
        
        // TODO handle other specific pair time choice ?
        let (new_chunk_c,interval_chunkid_t_v): (ChunkC,Vec<(Option<IntervalD>,u16)>) = if *r_01_pair_relation_time_choice == PairChoice::Ignore {
            let bm = BTreeMap::from([(2,chunk_2.clone())]);
            let new_chunk_c = ChunkC::new(bm);
            let interval_chunkid_t_v = if chunk_2.get_start() {
                    vec![(Some(interval_2),2)]
            } else {
                vec![(None,2)]
            };
            (new_chunk_c,interval_chunkid_t_v)
        } else {
            let chunk_01_interval_v = Self::resolve_overlap(
                r_01_pair_relation_time_choice,
                &interval_0,
                &interval_1,
                0,
                1
            );

            let chunk_01 = Self::merge_chunk_01_after_overlap_resolution(
                chunk_01_interval_v,
                chunk_c,
                payload_mode,
            );
            debug!("build_queued_by_offset_any_merging: chunk_01: {:?}",chunk_01);
            let new_chunk_c = ChunkC::new(
                BTreeMap::from(
                    [(0,chunk_01.clone()),
                    (2,chunk_2.clone())]
                )
            );

            let start_01 = min(interval_0.get_start(),interval_1.get_start());
            let end_01 = max(interval_0.get_end(),interval_1.get_end());
            let interval_01 = IntervalD::new(start_01,end_01);
            debug!("build_queued_by_offset_any_merging: interval_01: {:?}",interval_01);
            let relation_01_2 = AllenIntervalAlgebraRelation::of_intervals(&interval_01,&interval_2);
            debug!("build_queued_by_offset_any_merging: relation_01_2: {:?}",relation_01_2);

            let interval_chunkid_t_v = match relation_01_2 {
                AllenIntervalAlgebraRelation::Bi => {
                    assert!(chunk_2.get_start());
                    vec![(Some(interval_2.clone()),2)]
                },
                AllenIntervalAlgebraRelation::B => {
                    assert!(chunk_01.get_start());
                    vec![(Some(interval_01.clone()),0)]
                },
                _ => {
                    let r_01_2_pair_relation_time_choice = pair_relation_time_choice
                        .get(&(relation_01_2,PairTimePosition::Before))
                        .expect("No pair relation time choice"); // always before in the way we design test cases
                    debug!("build_queued_by_offset_any_merging: r_01_2_pair_relation_time_choice: {:?}",r_01_2_pair_relation_time_choice);

                    Self::resolve_overlap(
                        r_01_2_pair_relation_time_choice,
                        &interval_01,
                        &interval_2,
                        0,
                        2
                    )
                }
            };
            (new_chunk_c,interval_chunkid_t_v)
        };
        debug!("build_queued_by_offset_any_merging: interval_chunkid_t_v: {:?}",interval_chunkid_t_v);

        let reassembled_chunk_interval_no_overlap_v = interval_chunkid_t_v
            .into_iter()
            .filter(|(interval_o,_chunk_id)| interval_o.is_some())
            .map(|(interval_o,chunk_id)| { (chunk_id,interval_o.unwrap()) } )
            .collect();

        let reconstructed_payload_ascii_v = Self::build_reconstructed_payload_ascii_v(
            &new_chunk_c,
            payload_mode,
            reassembled_chunk_interval_no_overlap_v
        );
        debug!("build_queued_by_offset_any_merging: reconstructed_payload_ascii_v: {:?}",reconstructed_payload_ascii_v);
        debug!("build_queued_by_offset_any_merging: payload_string_option: {:?}",payload_string_option);


        Self::check_payload_consistency(
            &reconstructed_payload_ascii_v,
            payload_string_option
        )
    }

    pub fn build_reconstructed_payload_ascii_v(
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode,
        reassembled_chunk_interval_no_overlap_v: Vec<(u16,IntervalD)>
    ) -> Vec<u8> {
        let factor = payload_mode.get_factor();

        reassembled_chunk_interval_no_overlap_v
            .iter()
            .map(|(chunk_id,interval_d)| {
                let chunk = chunk_c.get(&chunk_id).unwrap();
                let chunk_payload_u8_v = chunk.get_chunk_pattern_ascii_v(payload_mode);
                let chunk_offset = chunk.get_offset();

                let start = (interval_d.get_start() - chunk_offset) * factor;
                debug!("build_reconstructed_payload_ascii_v: start: {}",start);
                let end = (interval_d.get_end() - chunk_offset + 1) * factor;
                debug!("build_reconstructed_payload_ascii_v: end: {}",end);

                chunk_payload_u8_v[start as usize..end as usize].to_vec()
            })
            .collect::<Vec<Vec<u8>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>()
    }

    pub fn check_payload_consistency(
        reconstructed_payload_ascii_v: &Vec<u8>,
        payload_string_option: &Option<Vec<u8>>,
    ) -> PolicyConsistency {
        match payload_string_option {
            Some(payload_string) => {
                if payload_string == reconstructed_payload_ascii_v {
                    PolicyConsistency::Consistent
                } else {
                    PolicyConsistency::NotConsistent
                }
            }
            None => {
                if reconstructed_payload_ascii_v.len() == 0 {
                    PolicyConsistency::Consistent
                } else {
                    PolicyConsistency::NotConsistent
                }
            }
        }
    }

    pub fn merge_chunk_01_after_overlap_resolution(
        chunk_01_interval_v: Vec<(Option<IntervalD>,u16)>,
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode
    ) -> ChunkD {

        let chunk_0 = chunk_c.get(&0).unwrap();
        let chunk_1 = chunk_c.get(&1).unwrap();
        let does_01_start = chunk_0.get_start() | chunk_1.get_start();
        let offset_01 = min(chunk_0.get_offset(),chunk_1.get_offset());

        let mut simple_s_01 = String::new();
        let mut internet_checksum_s_01 = String::new();
        let mut ipv4_invariant_checksum_s_01 = String::new();
        let mut ipv6_invariant_checksum_s_01 = String::new();
        let mut simple_ascii_v_01 = Vec::<u8>::new();
        let mut internet_checksum_ascii_v_01 = Vec::<u8>::new();
        let mut ipv4_invariant_checksum_ascii_v_01 = Vec::<u8>::new();
        let mut ipv6_invariant_checksum_ascii_v_01 = Vec::<u8>::new();

        let factor = payload_mode.get_factor();

        chunk_01_interval_v
            .into_iter()
            .filter(|(interval_d_o,_)| interval_d_o.is_some() )
            .for_each(|(interval_d_o,chunk_id)| {
                let interval_d = interval_d_o.unwrap();
                let chunk = chunk_c.get(&chunk_id).unwrap();
                
                let chunk_simple_s = chunk.get_simple_s();
                let chunk_internet_checksum_s = chunk.get_internet_checksum_s();
                let chunk_ipv4_invariant_checksum_s = chunk.get_ipv4_invariant_checksum_s();
                let chunk_ipv6_invariant_checksum_s = chunk.get_ipv6_invariant_checksum_s();
                let chunk_simple_ascii_v = chunk.get_simple_ascii_v();
                let chunk_internet_checksum_ascii_v = chunk.get_internet_checksum_ascii_v();
                let chunk_ipv4_invariant_checksum_ascii_v = chunk.get_ipv4_invariant_checksum_ascii_v();
                let chunk_ipv6_invariant_checksum_ascii_v = chunk.get_ipv6_invariant_checksum_ascii_v();

                let chunk_offset = chunk.get_offset();
                let start_simple = interval_d.get_start() - chunk_offset;
                debug!("merge_chunk_01_after_overlap_resolution: start_simple: {}",start_simple);
                let start = (interval_d.get_start() - chunk_offset) * factor;
                debug!("merge_chunk_01_after_overlap_resolution: start: {}",start);
                let end_simple = interval_d.get_end() - chunk_offset + 1;
                debug!("merge_chunk_01_after_overlap_resolution: end_simple: {}",end_simple);
                let end = (interval_d.get_end() - chunk_offset + 1) * factor;
                debug!("merge_chunk_01_after_overlap_resolution: end: {}",end);

                simple_s_01.push_str(&chunk_simple_s[start_simple as usize..end_simple as usize]);
                internet_checksum_s_01.push_str(&chunk_internet_checksum_s[start as usize..end as usize]);
                ipv4_invariant_checksum_s_01.push_str(&chunk_ipv4_invariant_checksum_s[start as usize..end as usize]);
                ipv6_invariant_checksum_s_01.push_str(&chunk_ipv6_invariant_checksum_s[start as usize..end as usize]);

                simple_ascii_v_01.extend(chunk_simple_ascii_v[start_simple as usize..end_simple as usize].to_vec());
                internet_checksum_ascii_v_01.extend(chunk_internet_checksum_ascii_v[start as usize..end as usize].to_vec());
                ipv4_invariant_checksum_ascii_v_01.extend(chunk_ipv4_invariant_checksum_ascii_v[start as usize..end as usize].to_vec());
                ipv6_invariant_checksum_ascii_v_01.extend(chunk_ipv6_invariant_checksum_ascii_v[start as usize..end as usize].to_vec());
            });

        ChunkD::new(
            0,
            does_01_start,
            offset_01,
            simple_s_01,
            simple_ascii_v_01,
            internet_checksum_s_01,
            internet_checksum_ascii_v_01,
            ipv4_invariant_checksum_s_01,
            ipv4_invariant_checksum_ascii_v_01,
            ipv6_invariant_checksum_s_01,
            ipv6_invariant_checksum_ascii_v_01,
        )
    }

}