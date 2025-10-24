use std::str;

use serde::{Deserialize, Serialize};

use crate::misc::pair_time_position::PairTimePosition;
use crate::policy_common::pair_byte_end_position::PairByteEndPosition;
use crate::policy_common::pair_byte_policy::PairBytePolicy;
use crate::policy_common::pair_byte_start_position::PairByteStartPosition;
use crate::policy_common::pair_choice::PairChoice;
use crate::policy_common::pair_time_policy::PairTimePolicy;
use crate::position::pair_position::PairPosition;
use crate::tcp_full_policy::pair_position_policy::PairPositionPolicy;
use crate::policy_common::pair_relation_time_choice::PairRelationTimeChoice;
use crate::position::triplet_pair_position_data::TripletPairPositionData;
use crate::position::payload_mode::PayloadMode;

//use super::policy_consistency::PolicyConsistency;
use crate::policy_common::policy_consistency::PolicyConsistency;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TripletSinglePairPolicyConsistency {
    pair_position_policy: PairPositionPolicy,
    policy_consistency_with_isolated_pair: PolicyConsistency,
}

impl TripletSinglePairPolicyConsistency {
    pub fn new(
        pair_position_policy: PairPositionPolicy,
        policy_consistency_with_isolated_pair: PolicyConsistency,
    ) -> TripletSinglePairPolicyConsistency {
        TripletSinglePairPolicyConsistency {
            pair_position_policy,
            policy_consistency_with_isolated_pair,
        }
    }

    pub fn get_policy_consistency_with_isolated_pair(&self, is_peo_like: bool) -> PolicyConsistency {
        if is_peo_like {
            match self.policy_consistency_with_isolated_pair {
                PolicyConsistency::NotConsistent => PolicyConsistency::NotConsistentPeoLike,
                _ => self.policy_consistency_with_isolated_pair.clone(),
            }
        } else {
            self.policy_consistency_with_isolated_pair.clone()
        }
    }

    pub fn get_pair_position_policy(&self) -> &PairPositionPolicy {
        &self.pair_position_policy
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TripletDoublePairPolicyConsistency {
    pair_position_policy_first: PairPositionPolicy,
    policy_consistency_with_isolated_pair_first: PolicyConsistency,
    pair_position_policy_second: PairPositionPolicy,
    policy_consistency_with_isolated_pair_second: PolicyConsistency,
}

impl TripletDoublePairPolicyConsistency {
    pub fn new(
        pair_position_policy_first: PairPositionPolicy,
        policy_consistency_with_isolated_pair_first: PolicyConsistency,
        pair_position_policy_second: PairPositionPolicy,
        policy_consistency_with_isolated_pair_second: PolicyConsistency,
    ) -> TripletDoublePairPolicyConsistency {
        TripletDoublePairPolicyConsistency {
            pair_position_policy_first,
            policy_consistency_with_isolated_pair_first,
            pair_position_policy_second,
            policy_consistency_with_isolated_pair_second,
        }
    }

    /// Extract policy consistency for doublae pair overlap in triplet.
    /// This is consistent if policies before and after triple overlap are
    /// consistent and if they are consistent with the isolated pair
    pub fn extract_policy_consitency(&self) -> PolicyConsistency {
        let bt_at_consistent = self.pair_position_policy_first.get_time_policy()
            == self.pair_position_policy_second.get_time_policy();

        if bt_at_consistent {
            self.policy_consistency_with_isolated_pair_first.clone()
        } else {
            PolicyConsistency::NotConsistent
        }
    }

    pub fn get_pair_position_policy_first(&self) -> &PairPositionPolicy {
        &self.pair_position_policy_first
    }

    pub fn get_pair_position_policy_second(&self) -> &PairPositionPolicy {
        &self.pair_position_policy_second
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TripletPairPositionPolicy {
    // Everything is in the triplet overlap
    None,
    // Pair overlap overflow after or before triplet overlap
    One(TripletSinglePairPolicyConsistency),
    // Pair overlap overflow after and before triplet overlap
    BeforeAfter(TripletDoublePairPolicyConsistency),
}

impl TripletPairPositionPolicy {
    fn build_pair_policy(
        payload_mode: &PayloadMode,
        pair_relation_time_choice: &PairRelationTimeChoice,
        temporal_position_0: &u16,
        temporal_position_1: &u16,
        pair_position: &PairPosition,
        payload_string_option: &Option<Vec<u8>>,
        authorize_overlap_with_no_data: bool,
    ) -> (PairPositionPolicy, PolicyConsistency) {
        let pair_time_position =
            PairTimePosition::of_temporal_position(*temporal_position_0, *temporal_position_1);

        let pair_byte_start_position = PairByteStartPosition::of_allen_interval_algebra_relation(
            pair_position.get_allen_interval_algebra_relation(),
        );

        let pair_byte_end_position = PairByteEndPosition::of_allen_interval_algebra_relation(
            pair_position.get_allen_interval_algebra_relation(),
        );

        match pair_position {
            PairPosition::Disjoint(byte_relation) => {
                let pair_position_policy = PairPositionPolicy::new(
                    (*byte_relation).clone(),
                    pair_time_position,
                    pair_byte_start_position,
                    pair_byte_end_position,
                    PairChoice::None,
                    PairTimePolicy::None,
                    PairBytePolicy::None,
                );
                (pair_position_policy, PolicyConsistency::Na)
            }
            PairPosition::Overlap(pair_overlap) => {
                let byte_relation = pair_overlap.get_byte_relation();

                let pair_choice = PairChoice::of_overlap_payload(
                    payload_mode,
                    pair_overlap,
                    payload_string_option,
                    authorize_overlap_with_no_data,
                );

                let pair_time_policy = PairTimePolicy::of_overlap_payload(
                    payload_mode,
                    pair_overlap,
                    *temporal_position_0,
                    *temporal_position_1,
                    payload_string_option,
                    authorize_overlap_with_no_data,
                );

                let pair_byte_policy = PairBytePolicy::of_overlap_payload(
                    payload_mode,
                    pair_overlap,
                    // *temporal_position_0,
                    // *temporal_position_1,
                    payload_string_option,
                    true,
                );

                let consistent = if pair_choice
                    == *pair_relation_time_choice
                        .get(&((*byte_relation).clone(), pair_time_position.clone()))
                        .unwrap()
                {
                    PolicyConsistency::Consistent
                } else {
                    PolicyConsistency::NotConsistent
                };

                let pair_position_policy = PairPositionPolicy::new(
                    (*byte_relation).clone(),
                    pair_time_position,
                    pair_byte_start_position,
                    pair_byte_end_position,
                    pair_choice,
                    pair_time_policy,
                    pair_byte_policy,
                );

                // ((*byte_relation).clone(), time_position, policy, consistent)
                (pair_position_policy, consistent)
            }
        }
    }

    pub fn of_data_option(
        payload_mode: &PayloadMode,
        pair_relation_time_choice: &PairRelationTimeChoice,
        temporal_position_0: &u16,
        temporal_position_1: &u16,
        pair_position_bt_option: &Option<PairPosition>,
        pair_position_at_option: &Option<PairPosition>,
        payload_string_option: &Option<Vec<u8>>,
        authorize_overlap_with_no_data: bool,
    ) -> TripletPairPositionPolicy {
        match pair_position_bt_option {
            None => TripletPairPositionPolicy::None,
            Some(pair_position_bt) => {
                let (pair_position_policy_bt, consistent_with_pair_bt) =
                    TripletPairPositionPolicy::build_pair_policy(
                        payload_mode,
                        pair_relation_time_choice,
                        temporal_position_0,
                        temporal_position_1,
                        pair_position_bt,
                        payload_string_option,
                        authorize_overlap_with_no_data,
                    );

                match pair_position_at_option {
                    None => {
                        TripletPairPositionPolicy::One(TripletSinglePairPolicyConsistency::new(
                            pair_position_policy_bt,
                            consistent_with_pair_bt,
                        ))
                    }
                    Some(pair_position_at) => {
                        let (pair_position_policy_at, consistent_with_pair_at) =
                            TripletPairPositionPolicy::build_pair_policy(
                                payload_mode,
                                pair_relation_time_choice,
                                temporal_position_0,
                                temporal_position_1,
                                pair_position_at,
                                payload_string_option,
                                authorize_overlap_with_no_data,
                            );

                        TripletPairPositionPolicy::BeforeAfter(
                            TripletDoublePairPolicyConsistency::new(
                                pair_position_policy_bt,
                                consistent_with_pair_bt,
                                pair_position_policy_at,
                                consistent_with_pair_at,
                            ),
                        )
                    }
                }
            }
        }
    }

    pub fn of_data(
        payload_mode: &PayloadMode,
        pair_relation_time_choice: &PairRelationTimeChoice,
        temporal_position_0: &u16,
        temporal_position_1: &u16,
        // pair_position_bt_option: &Option<PairPosition>,
        // pair_position_at_option: &Option<PairPosition>,
        triplet_pair_position_data: &TripletPairPositionData,
        payload_string_option: &Option<Vec<u8>>,
        authorize_overlap_with_no_data: bool,
    ) -> TripletPairPositionPolicy {
        match triplet_pair_position_data {
            TripletPairPositionData::None => TripletPairPositionPolicy::None,
            TripletPairPositionData::One(pair_position) => {
                let (pair_position_policy_bt, consistent_with_pair_bt) =
                    TripletPairPositionPolicy::build_pair_policy(
                        payload_mode,
                        pair_relation_time_choice,
                        temporal_position_0,
                        temporal_position_1,
                        pair_position,
                        payload_string_option,
                        authorize_overlap_with_no_data,
                    );

                TripletPairPositionPolicy::One(TripletSinglePairPolicyConsistency::new(
                    pair_position_policy_bt,
                    consistent_with_pair_bt,
                ))
            }
            TripletPairPositionData::BeforeAfter(triplet_double_pair_position) => {
                let pair_position_first = triplet_double_pair_position.get_pair_position_first();
                let pair_position_second = triplet_double_pair_position.get_pair_position_second();

                let (pair_position_policy_first, consistent_with_pair_first) =
                    TripletPairPositionPolicy::build_pair_policy(
                        payload_mode,
                        pair_relation_time_choice,
                        temporal_position_0,
                        temporal_position_1,
                        pair_position_first,
                        payload_string_option,
                        authorize_overlap_with_no_data,
                    );

                let (pair_position_policy_second, consistent_with_pair_second) =
                    TripletPairPositionPolicy::build_pair_policy(
                        payload_mode,
                        pair_relation_time_choice,
                        temporal_position_0,
                        temporal_position_1,
                        pair_position_second,
                        payload_string_option,
                        authorize_overlap_with_no_data,
                    );

                TripletPairPositionPolicy::BeforeAfter(TripletDoublePairPolicyConsistency::new(
                    pair_position_policy_first,
                    consistent_with_pair_first,
                    pair_position_policy_second,
                    consistent_with_pair_second,
                ))
            }
        }
    }

    pub fn extract_policy_consitency(&self, is_peo_like: bool) -> PolicyConsistency {
        match self {
            TripletPairPositionPolicy::None => PolicyConsistency::Na,
            // TripletPairPositionPolicy::One(_) => PolicyConsistency::Na,
            TripletPairPositionPolicy::One(triplet_single_pair_policy_consistency) => {
                triplet_single_pair_policy_consistency.get_policy_consistency_with_isolated_pair(is_peo_like)
            },
            TripletPairPositionPolicy::BeforeAfter(triplet_double_pair_policy_consistency
                // pair_position_policy_bt,
                // consistent_with_pair_bt,
                // pair_position_policy_at,
                // consistent_with_pair_at,
            ) => {
                // if pair_position_policy_bt.get_policy() == pair_position_policy_at.get_policy() {
                //     PolicyConsistency::Consistent
                // } else {
                //     PolicyConsistency::NotConsistent
                // }
                assert!(!is_peo_like);
                triplet_double_pair_policy_consistency.extract_policy_consitency()
            }
        }
    }

    //     let (pair_position_policy_bt, consistent_with_pair_bt) =
    //         TripletPairPositionPolicy::build_pair_policy(
    //             pair_relation_policy,
    //             temporal_position_0,
    //             temporal_position_1,
    //             pair_position_bt,
    //             payload_string_option,
    //         );
    //     let (pair_position_policy_01_at, consistent_with_pair_01_at) =
    //         TripletPairPositionPolicy::build_pair_policy(
    //             pair_relation_policy,
    //             temporal_position_0,
    //             temporal_position_1,
    //             pair_position_at,
    //             payload_string_option,
    //         );

    //     let time_position =
    //         TimePosition::of_temporal_position(*temporal_position_0, *temporal_position_1);

    //     let (byte_relation, pair_policy) = match pair_position {
    //         PairPosition::Disjoint(byte_relation) => ((*byte_relation).clone(), PairPolicy::None),
    //         PairPosition::Overlap(overlap) => {
    //             let byte_relation = overlap.get_byte_relation();

    //             let pair_policy = PairPolicy::of_overlap_payload(
    //                 &overlap,
    //                 *temporal_position_0,
    //                 *temporal_position_1,
    //                 payload_string_option,
    //             );

    //             ((*byte_relation).clone(), pair_policy)
    //         }
    //     };

    //     PairPositionPolicy::new(byte_relation, time_position, pair_policy)
    // }

    pub fn get_pair_position_policy(&self) -> &PairPositionPolicy {
        match self {
            TripletPairPositionPolicy::One(triplet_single_pair_policy_consistency) => {
                triplet_single_pair_policy_consistency.get_pair_position_policy()
            }
            TripletPairPositionPolicy::BeforeAfter(triplet_double_pair_policy_consistency) => {
                // sanity check
                let pair_position_policy_first = triplet_double_pair_policy_consistency.get_pair_position_policy_first();
                let pair_position_policy_second = triplet_double_pair_policy_consistency.get_pair_position_policy_second();

                assert!(pair_position_policy_first == pair_position_policy_second);
                
                triplet_double_pair_policy_consistency.get_pair_position_policy_first()
            }
            TripletPairPositionPolicy::None => 
            panic!("No pair_position_policy for triplet_pair_position_policy = None")
        }
    }
}
