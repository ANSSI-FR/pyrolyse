use std::str;

use serde::{Deserialize, Serialize};

use crate::misc::pair_time_position::PairTimePosition;
use crate::policy_common::pair_byte_end_position::PairByteEndPosition;
use crate::policy_common::pair_byte_policy::PairBytePolicy;
use crate::policy_common::pair_byte_start_position::PairByteStartPosition;
use crate::policy_common::pair_choice::PairChoice;
use crate::policy_common::residual_pair_time_policy::TimePolicyTripletResidualPair;
use crate::position::pair_position::PairPosition;
use crate::position::payload_mode::PayloadMode;
use crate::position::triplet_pair_position_data::TripletPairPositionData;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;

//#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
//pub struct ResidualPairPolicy {
//    residual_pair_policy: TimePolicyTripletResidualPair,
//}

//impl ResidualPairPolicy {
//    pub fn new(
//        residual_pair_policy: TimePolicyTripletResidualPair,
//    ) -> ResidualPairPolicy {
//        ResidualPairPolicy {
//            residual_pair_policy,
//        }
//    }
//
//    pub fn get_time_policy(&self) -> &TimePolicyTripletResidualPair {
//        &self.residual_pair_policy
//    }

pub fn build_pair_time_policy(
    payload_mode: &PayloadMode,
    temporal_position_0: &u16,
    temporal_position_1: &u16,
    pair_position: &PairPosition,
    payload_string_option: &Option<Vec<u8>>,
    authorize_overlap_with_no_data: bool,
) -> TimePolicyTripletResidualPair {
    match pair_position {
        PairPosition::Disjoint(_) => (
            // (*byte_relation).clone(),
            TimePolicyTripletResidualPair::None
        ),
        PairPosition::Overlap(pair_overlap) => {
            TimePolicyTripletResidualPair::of_overlap_payload(
                payload_mode,
                pair_overlap,
                *temporal_position_0,
                *temporal_position_1,
                payload_string_option,
                authorize_overlap_with_no_data,
            )
        }
    }
}

pub fn build_residual_pair_time_policy(
    payload_mode: &PayloadMode,
    temporal_position_0: &u16,
    temporal_position_1: &u16,
    // pair_position_bt_option: &Option<PairPosition>,
    // pair_position_at_option: &Option<PairPosition>,
    triplet_pair_position_data: &TripletPairPositionData,
    payload_string_option: &Option<Vec<u8>>,
    authorize_overlap_with_no_data: bool,
) -> TimePolicyTripletResidualPair {
    match triplet_pair_position_data {
        TripletPairPositionData::None => TimePolicyTripletResidualPair::None,
        TripletPairPositionData::One(pair_position) => {
            build_pair_time_policy(
                    payload_mode,
                    temporal_position_0,
                    temporal_position_1,
                    pair_position,
                    payload_string_option,
                    authorize_overlap_with_no_data,
                )
        }
        TripletPairPositionData::BeforeAfter(triplet_double_pair_position) => {
            let pair_position_first = triplet_double_pair_position.get_pair_position_first();
            let pair_position_second = triplet_double_pair_position.get_pair_position_second();
            let pair_time_policy_first =
                build_pair_time_policy(
                    payload_mode,
                    temporal_position_0,
                    temporal_position_1,
                    pair_position_first,
                    payload_string_option,
                    authorize_overlap_with_no_data,
                );
            let pair_time_policy_second =
                build_pair_time_policy(
                    payload_mode,
                    temporal_position_0,
                    temporal_position_1,
                    pair_position_second,
                    payload_string_option,
                    authorize_overlap_with_no_data,
                );
            assert_eq!(pair_time_policy_first,pair_time_policy_second);
            pair_time_policy_first
        }
    }
}
