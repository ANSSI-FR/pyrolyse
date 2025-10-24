use std::str;

use serde::{Deserialize, Serialize};

use crate::misc::pair_time_position::PairTimePosition;
use crate::policy_common::pair_byte_end_position::PairByteEndPosition;
use crate::policy_common::pair_byte_policy::PairBytePolicy;
use crate::policy_common::pair_byte_start_position::PairByteStartPosition;
use crate::policy_common::pair_choice::PairChoice;
use crate::policy_common::pair_time_policy::PairTimePolicy;
use crate::position::pair_position::PairPosition;
// use crate::position::pair_position_data::PairPositionDataD;
use crate::minimal_policy::residual_pair_policy::ResidualPairPolicy;
// use crate::complicated_policy::pair_relation_policy::PairRelationPolicy;
use crate::position::triplet_pair_position_data::TripletPairPositionData;
// use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::position::payload_mode::PayloadMode;


//#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
//pub struct TripletSinglePairPolicy {
//    residual_pair_policy: ResidualPairPolicy,
//}
//
//impl TripletSinglePairPolicy {
//    pub fn new(
//        residual_pair_policy: ResidualPairPolicy,
//    ) -> TripletSinglePairPolicy {
//        TripletSinglePairPolicy {
//            residual_pair_policy,
//        }
//    }
//
//
//    pub fn get_residual_pair_policy(&self) -> &ResidualPairPolicy {
//        &self.residual_pair_policy
//    }
//}
//
//#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
//pub struct TripletDoublePairPolicy {
//    residual_pair_policy_first: ResidualPairPolicy,
//    residual_pair_policy_second: ResidualPairPolicy,
//}
//
//impl TripletDoublePairPolicy {
//    pub fn new(
//        residual_pair_policy_first: ResidualPairPolicy,
//        residual_pair_policy_second: ResidualPairPolicy,
//    ) -> TripletDoublePairPolicy {
//        TripletDoublePairPolicy {
//            residual_pair_policy_first,
//            residual_pair_policy_second,
//        }
//    }
//
//    pub fn get_residual_pair_policy_first(&self) -> &ResidualPairPolicy {
//        &self.residual_pair_policy_first
//    }
//
//    pub fn get_residual_pair_policy_second(&self) -> &ResidualPairPolicy {
//        &self.residual_pair_policy_second
//    }
//}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
//pub enum TripletPairPolicy {
//    // Everything is in the triplet overlap
//    None,
//    // Pair overlap overflow after or before triplet overlap
//    One(TripletSinglePairPolicy),
//    // Pair overlap overflow after and before triplet overlap
//    BeforeAfter(TripletDoublePairPolicy),
//}
pub struct TripletPairPolicy {
    residual_pair_policy: ResidualPairPolicy,
}

impl TripletPairPolicy {
    fn build_pair_policy(
        payload_mode: &PayloadMode,
        temporal_position_0: &u16,
        temporal_position_1: &u16,
        pair_position: &PairPosition,
        payload_string_option: &Option<Vec<u8>>,
        authorize_overlap_with_no_data: bool,
    ) -> ResidualPairPolicy {
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
                let residual_pair_policy = ResidualPairPolicy::new(
                    PairTimePolicy::None,
                );
                residual_pair_policy
            }
            PairPosition::Overlap(pair_overlap) => {
                let pair_time_policy = PairTimePolicy::of_overlap_payload(
                    payload_mode,
                    pair_overlap,
                    *temporal_position_0,
                    *temporal_position_1,
                    payload_string_option,
                    authorize_overlap_with_no_data,
                );

                let residual_pair_policy = ResidualPairPolicy::new(
                    pair_time_policy,
                );

                // ((*byte_relation).clone(), time_position, policy, consistent)
                residual_pair_policy
            }
        }
    }

    //pub fn of_data_option__(
    //    payload_mode: &PayloadMode,
    //    temporal_position_0: &u16,
    //    temporal_position_1: &u16,
    //    pair_position_bt_option: &Option<PairPosition>,
    //    pair_position_at_option: &Option<PairPosition>,
    //    payload_string_option: &Option<Vec<u8>>,
    //    authorize_overlap_with_no_data: bool,
    //) -> TripletPairPolicy {
    //    match pair_position_bt_option {
    //        None => TripletPairPolicy::None,
    //        Some(pair_position_bt) => {
    //            let residual_pair_policy_bt =
    //                TripletPairPolicy::build_pair_policy(
    //                    payload_mode,
    //                    temporal_position_0,
    //                    temporal_position_1,
    //                    pair_position_bt,
    //                    payload_string_option,
    //                    authorize_overlap_with_no_data,
    //                );
//
    //            match pair_position_at_option {
    //                None => {
    //                    TripletPairPolicy::One(TripletSinglePairPolicy::new(
    //                        residual_pair_policy_bt,
    //                    ))
    //                }
    //                Some(pair_position_at) => {
    //                    let residual_pair_policy_at =
    //                        TripletPairPolicy::build_pair_policy(
    //                            payload_mode,
    //                            temporal_position_0,
    //                            temporal_position_1,
    //                            pair_position_at,
    //                            payload_string_option,
    //                            authorize_overlap_with_no_data,
    //                        );
//
    //                    TripletPairPolicy::BeforeAfter(
    //                        TripletDoublePairPolicy::new(
    //                            residual_pair_policy_bt,
    //                            residual_pair_policy_at,
    //                        ),
    //                    )
    //                }
    //            }
    //        }
    //    }
    //}

    pub fn of_data_option(
        payload_mode: &PayloadMode,
        temporal_position_0: &u16,
        temporal_position_1: &u16,
        pair_position_bt_option: &Option<PairPosition>,
        pair_position_at_option: &Option<PairPosition>,
        payload_string_option: &Option<Vec<u8>>,
        authorize_overlap_with_no_data: bool,
    ) -> TripletPairPolicy {
        match pair_position_bt_option {
            None => TripletPairPolicy { residual_pair_policy: ResidualPairPolicy::new(PairTimePolicy::None) },
            Some(pair_position_bt) => {
                let residual_pair_policy_bt =
                    TripletPairPolicy::build_pair_policy(
                        payload_mode,
                        temporal_position_0,
                        temporal_position_1,
                        pair_position_bt,
                        payload_string_option,
                        authorize_overlap_with_no_data,
                    );

                match pair_position_at_option {
                    None => {
                        TripletPairPolicy { residual_pair_policy: residual_pair_policy_bt }
                    }
                    Some(pair_position_at) => {
                        let residual_pair_policy_at =
                            TripletPairPolicy::build_pair_policy(
                                payload_mode,
                                temporal_position_0,
                                temporal_position_1,
                                pair_position_at,
                                payload_string_option,
                                authorize_overlap_with_no_data,
                            );

                        assert_eq!(residual_pair_policy_bt,residual_pair_policy_at);

                        TripletPairPolicy { residual_pair_policy: residual_pair_policy_bt }
                    }
                }
            }
        }
    }

    //pub fn of_data__(
    //    payload_mode: &PayloadMode,
    //    temporal_position_0: &u16,
    //    temporal_position_1: &u16,
    //    // pair_position_bt_option: &Option<PairPosition>,
    //    // pair_position_at_option: &Option<PairPosition>,
    //    triplet_pair_position_data: &TripletPairPositionData,
    //    payload_string_option: &Option<Vec<u8>>,
    //    authorize_overlap_with_no_data: bool,
    //) -> TripletPairPolicy {
    //    match triplet_pair_position_data {
    //        TripletPairPositionData::None => TripletPairPolicy::None,
    //        TripletPairPositionData::One(pair_position) => {
    //            let residual_pair_policy_bt =
    //                TripletPairPolicy::build_pair_policy(
    //                    payload_mode,
    //                    temporal_position_0,
    //                    temporal_position_1,
    //                    pair_position,
    //                    payload_string_option,
    //                    authorize_overlap_with_no_data,
    //                );
//
    //            TripletPairPolicy::One(TripletSinglePairPolicy::new(
    //                residual_pair_policy_bt,
    //            ))
    //        }
    //        TripletPairPositionData::BeforeAfter(triplet_double_pair_position) => {
    //            let pair_position_first = triplet_double_pair_position.get_pair_position_first();
    //            let pair_position_second = triplet_double_pair_position.get_pair_position_second();
//
    //            let residual_pair_policy_first =
    //                TripletPairPolicy::build_pair_policy(
    //                    payload_mode,
    //                    temporal_position_0,
    //                    temporal_position_1,
    //                    pair_position_first,
    //                    payload_string_option,
    //                    authorize_overlap_with_no_data,
    //                );
//
    //            let residual_pair_policy_second =
    //                TripletPairPolicy::build_pair_policy(
    //                    payload_mode,
    //                    temporal_position_0,
    //                    temporal_position_1,
    //                    pair_position_second,
    //                    payload_string_option,
    //                    authorize_overlap_with_no_data,
    //                );
//
    //            TripletPairPolicy::BeforeAfter(TripletDoublePairPolicy::new(
    //                residual_pair_policy_first,
    //                residual_pair_policy_second,
    //            ))
    //        }
    //    }
    //}

    pub fn of_data(
        payload_mode: &PayloadMode,
        temporal_position_0: &u16,
        temporal_position_1: &u16,
        // pair_position_bt_option: &Option<PairPosition>,
        // pair_position_at_option: &Option<PairPosition>,
        triplet_pair_position_data: &TripletPairPositionData,
        payload_string_option: &Option<Vec<u8>>,
        authorize_overlap_with_no_data: bool,
    ) -> TripletPairPolicy {
        match triplet_pair_position_data {
            TripletPairPositionData::None => TripletPairPolicy { residual_pair_policy: ResidualPairPolicy::new(PairTimePolicy::None) },
            TripletPairPositionData::One(pair_position) => {
                let residual_pair_policy_bt =
                    TripletPairPolicy::build_pair_policy(
                        payload_mode,
                        temporal_position_0,
                        temporal_position_1,
                        pair_position,
                        payload_string_option,
                        authorize_overlap_with_no_data,
                    );

                TripletPairPolicy { residual_pair_policy: residual_pair_policy_bt }
            }
            TripletPairPositionData::BeforeAfter(triplet_double_pair_position) => {
                let pair_position_first = triplet_double_pair_position.get_pair_position_first();
                let pair_position_second = triplet_double_pair_position.get_pair_position_second();

                let residual_pair_policy_first =
                    TripletPairPolicy::build_pair_policy(
                        payload_mode,
                        temporal_position_0,
                        temporal_position_1,
                        pair_position_first,
                        payload_string_option,
                        authorize_overlap_with_no_data,
                    );

                let residual_pair_policy_second =
                    TripletPairPolicy::build_pair_policy(
                        payload_mode,
                        temporal_position_0,
                        temporal_position_1,
                        pair_position_second,
                        payload_string_option,
                        authorize_overlap_with_no_data,
                    );

                assert_eq!(residual_pair_policy_first,residual_pair_policy_second);

                TripletPairPolicy { residual_pair_policy: residual_pair_policy_first }
            }
        }
    }

    //pub fn get_residual_pair_policy__(&self) -> &ResidualPairPolicy {
    //    match self {
    //        TripletPairPolicy::One(triplet_single_pair_policy) => {
    //            triplet_single_pair_policy.get_residual_pair_policy()
    //        }
    //        TripletPairPolicy::BeforeAfter(triplet_double_pair_policy) => {
    //            // sanity check
    //            let residual_pair_policy_first = triplet_double_pair_policy.get_residual_pair_policy_first();
    //            let residual_pair_policy_second = triplet_double_pair_policy.get_residual_pair_policy_second();
//
    //            assert!(residual_pair_policy_first == residual_pair_policy_second);
    //            
    //            triplet_double_pair_policy.get_residual_pair_policy_first()
    //        }
    //        TripletPairPolicy::None => 
    //        panic!("No residual_pair_policy for triplet_residual_pair_policy = None")
    //    }
    //}

    pub fn get_residual_pair_policy(&self) -> &ResidualPairPolicy {
        return &self.residual_pair_policy
    }
}
