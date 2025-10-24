use std::str;

use serde::{Deserialize, Serialize};

use intervals_general::bound_pair::BoundPair;
use intervals_general::interval::Interval;

use crate::byte_time_data::chunk::ChunkD;
use crate::misc::interval::IntervalD;
use crate::position::pair_overlap::PairOverlap;
use crate::position::pair_position::PairPosition;
use crate::position::payload_mode::PayloadMode;
use crate::position::triplet_position::TripletPosition;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripletDoublePairPosition {
    pair_position_first: PairPosition,
    pair_position_second: PairPosition,
}

impl TripletDoublePairPosition {
    pub fn new(
        pair_position_first: PairPosition,
        pair_position_second: PairPosition,
    ) -> TripletDoublePairPosition {
        TripletDoublePairPosition {
            pair_position_first,
            pair_position_second,
        }
    }

    pub fn get_pair_position_first(&self) -> &PairPosition {
        &self.pair_position_first
    }

    pub fn get_pair_position_second(&self) -> &PairPosition {
        &self.pair_position_second
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TripletPairPositionData {
    // Everything is in the triplet overlap
    None,
    // Pair overlap overflow after or before triplet overlap
    One(PairPosition),
    // Pair overlap overflow after and before triplet overlap
    BeforeAfter(TripletDoublePairPosition),
}

impl TripletPairPositionData {
    pub fn of_triplet_position_relation_interval_chunk(
        payload_mode: &PayloadMode,
        triplet_position: &TripletPosition,
        byte_relation: &AllenIntervalAlgebraRelation,
        interval_0: &IntervalD,
        interval_1: &IntervalD,
        chunk_0: &ChunkD,
        chunk_1: &ChunkD,
    ) -> TripletPairPositionData {
        debug!("of_byte_time_sequence_d_triplet: start");

        let r = match triplet_position {
            TripletPosition::Disjoint(_) => {
                TripletPairPositionData::One(PairPosition::of_relation_interval_chunk(
                    payload_mode,
                    byte_relation,
                    interval_0,
                    interval_1,
                    chunk_0,
                    chunk_1,
                ))
            }
            TripletPosition::Overlap(triplet_overlap) => {
                debug!("of_byte_time_sequence_d_triplet: triplet overlap");

                debug!("of_byte_time_sequence_d_triplet: building pair_position_base");
                let pair_position_base = PairPosition::of_relation_interval_chunk(
                    payload_mode,
                    byte_relation,
                    interval_0,
                    interval_1,
                    chunk_0,
                    chunk_1,
                );
                debug!("of_byte_time_sequence_d_triplet: building pair_overlap");
                let pair_overlap = match pair_position_base {
                    PairPosition::Disjoint(_) => {
                        panic!("Inconsistency: TripletOverlap but no PairOverlap")
                    }
                    PairPosition::Overlap(pair_overlap) => pair_overlap,
                };
                debug!("of_byte_time_sequence_d_triplet: building pair_overlap_interval");
                let pair_overlap_interval = if pair_overlap.get_start() == pair_overlap.get_end() {
                    Interval::Singleton {
                        at: pair_overlap.get_start(),
                    }
                } else {
                    Interval::Closed {
                        bound_pair: BoundPair::new(
                            pair_overlap.get_start(),
                            pair_overlap.get_end(),
                        )
                        .ok_or("invalid BoundPair")
                        .unwrap(),
                    }
                };
                debug!(
                    "of_byte_time_sequence_d_triplet: pair_overlap_interval: {:?}",
                    pair_overlap_interval
                );

                let triplet_overlap_interval =
                    if triplet_overlap.get_start() == triplet_overlap.get_end() {
                        Interval::Singleton {
                            at: triplet_overlap.get_start(),
                        }
                    } else {
                        Interval::Closed {
                            bound_pair: BoundPair::new(
                                triplet_overlap.get_start(),
                                triplet_overlap.get_end(),
                            )
                            .ok_or("invalid BoundPair")
                            .unwrap(),
                        }
                    };
                let triplet_overlap_interval_complement_v: Vec<_> =
                    triplet_overlap_interval.complement().collect();
                assert!(triplet_overlap_interval_complement_v.len() <= 2);

                match triplet_overlap_interval_complement_v.len() {
                    0 => panic!("We found an empty complement to an interval that is finite!"),
                    1 => {
                        panic!("We found a complement with only one element to an interval that is finite!")
                    }
                    2 => {
                        debug!("of_byte_time_sequence_d_triplet: 2 elements in triplet overlap complement");

                        let triplet_overlap_interval_complement_bt =
                            triplet_overlap_interval_complement_v[0];
                        let triplet_overlap_interval_complement_at =
                            triplet_overlap_interval_complement_v[1];
                        debug!("of_byte_time_sequence_d_triplet: triplet_overlap_interval_complement_bt: {:?}",triplet_overlap_interval_complement_bt);
                        debug!("of_byte_time_sequence_d_triplet: triplet_overlap_interval_complement_at: {:?}",triplet_overlap_interval_complement_at);

                        let pair_overlap_minus_triplet_interval_tmp_bt: Interval<u16> =
                            triplet_overlap_interval_complement_bt
                                .intersect(&pair_overlap_interval);
                        let pair_overlap_minus_triplet_interval_tmp_at: Interval<u16> =
                            triplet_overlap_interval_complement_at
                                .intersect(&pair_overlap_interval);

                        let pair_overlap_minus_triplet_interval_bt_option = match pair_overlap_minus_triplet_interval_tmp_bt {
                            Interval::Closed{bound_pair} => Some(IntervalD::new(*bound_pair.left(),*bound_pair.right())),
                            Interval::RightHalfOpen{ bound_pair } => {
                                assert!(*bound_pair.left() < *bound_pair.right());
                                Some(IntervalD::new(*bound_pair.left(),*bound_pair.right()-1))
                            },
                            Interval::Singleton{at} => Some(IntervalD::new(at,at)),
                            Interval::Empty => None,
                            _ => panic!("Unexpected type in pair_overlap_minus_triplet_interval_tmp: {:?}", 
                                        pair_overlap_minus_triplet_interval_tmp_bt)
                        };
                        let pair_overlap_minus_triplet_interval_at_option = match pair_overlap_minus_triplet_interval_tmp_at {
                            Interval::Closed{bound_pair} => Some(IntervalD::new(*bound_pair.left(),*bound_pair.right())),
                            Interval::LeftHalfOpen{ bound_pair } => {
                                assert!(*bound_pair.left() < *bound_pair.right());
                                Some(IntervalD::new(*bound_pair.left()+1,*bound_pair.right()))
                            },
                            Interval::Singleton{at} => Some(IntervalD::new(at,at)),
                            Interval::Empty => None,
                            _ => panic!("Unexpected type in pair_overlap_minus_triplet_interval_tmp: {:?}", 
                            pair_overlap_minus_triplet_interval_tmp_at)
                        };

                        // Note: using pair_overlap_minus_triplet_interval_0 twice in both calls is working
                        // because this function extract payload from chunks using embedded chunk offsets.
                        match pair_overlap_minus_triplet_interval_bt_option {
                            None => match pair_overlap_minus_triplet_interval_at_option {
                                None => TripletPairPositionData::None,
                                Some(pair_overlap_minus_triplet_interval_at) => {
                                    TripletPairPositionData::One(PairPosition::Overlap(
                                        PairOverlap::of_relation_interval_chunk(
                                            payload_mode,
                                            byte_relation,
                                            &pair_overlap_minus_triplet_interval_at,
                                            &pair_overlap_minus_triplet_interval_at,
                                            chunk_0,
                                            chunk_1,
                                        ),
                                    ))
                                }
                            },
                            Some(pair_overlap_minus_triplet_interval_bt) => {
                                let pp =
                                    PairPosition::Overlap(PairOverlap::of_relation_interval_chunk(
                                        payload_mode,
                                        byte_relation,
                                        &pair_overlap_minus_triplet_interval_bt,
                                        &pair_overlap_minus_triplet_interval_bt,
                                        chunk_0,
                                        chunk_1,
                                    ));

                                match pair_overlap_minus_triplet_interval_at_option {
                                    None => TripletPairPositionData::One(pp),
                                    Some(pair_overlap_minus_triplet_interval_at) => {
                                        TripletPairPositionData::BeforeAfter(
                                            TripletDoublePairPosition::new(
                                                PairPosition::Overlap(
                                                    PairOverlap::of_relation_interval_chunk(
                                                        payload_mode,
                                                        byte_relation,
                                                        &pair_overlap_minus_triplet_interval_at,
                                                        &pair_overlap_minus_triplet_interval_at,
                                                        chunk_0,
                                                        chunk_1,
                                                    ),
                                                ),
                                                pp,
                                            ),
                                        )
                                    }
                                }
                            }
                        }
                    }
                    _ => panic!(
                        "We found more than than two intervals that complement a single interval!"
                    ),
                }
            }
        };

        debug!("of_byte_time_sequence_d_triplet: end");

        r
    }
}
