use serde::{Deserialize, Serialize};

use crate::position::pair_overlap::PairOverlap;
use crate::position::payload_mode::PayloadMode;
use crate::position::triplet_pair_position_data::TripletPairPositionData;
use crate::position::pair_position::PairPosition;
use crate::policy_common::pair_choice::PairChoice;
use crate::policy_common::policy_consistency::PolicyConsistency;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum TimePolicyTripletResidualPair {
    // No reassembly possible (byte relation is B or Bi)
    None,
    // Both fragments are dropped
    Ignore,
    // some data is present but nothing for overlap
    PartialIgnore,
    // Old fragment is kept
    Old,
    // New fragment is kept
    New,
    // Unexpected data is present
    Bug,
}

impl TimePolicyTripletResidualPair {
    pub fn of_overlap_payload(
        payload_mode: &PayloadMode,
        pair_overlap: &PairOverlap,
        temporal_position_0: u16,
        temporal_position_1: u16,
        payload_option: &Option<Vec<u8>>,
        authorize_overlap_with_no_data: bool,
    ) -> TimePolicyTripletResidualPair {
        debug!("of_overlap_payload: start");

        debug!("of_overlap_payload: payload_option: {:?}", payload_option);

        let overlap_start = pair_overlap.get_start();
        let overlap_end = pair_overlap.get_end();
        debug!(
            "of_overlap_payload: pair_overlap: {:?} -> {:?}",
            overlap_start, overlap_end
        );

        let factor = payload_mode.get_factor();

        let overlap_start_n = overlap_start * factor;
        let overlap_end_n = (overlap_end + 1) * factor;
        debug!(
            "of_overlap_payload: overlap n: {:?} -> {:?}",
            overlap_start_n, overlap_end_n
        );
        assert!(overlap_start_n <= overlap_end_n);

        let payload_0 = pair_overlap.get_payload_0();
        let payload_1 = pair_overlap.get_payload_1();
        debug!("of_overlap_payload: payload_0: {:?}", payload_0);
        debug!("of_overlap_payload: payload_1: {:?}", payload_1);

        let policy = match payload_option {
            None => TimePolicyTripletResidualPair::Ignore,
            Some(payload) => {
                // In case we got several Echo Reply for IP protocols - we panic
                if payload.len() == 0 {
                    //TimePolicyTripletResidualPair::Both
                    panic!("got data from multiple chunks at the same position")
                }
                // Windows 10 echo often partially answers to sent data.
                // If the end of overlap is located after the end of the payload, it means that the receveived payload is partial.
                else if overlap_end_n as usize > payload.len() {
                    TimePolicyTripletResidualPair::PartialIgnore
                } else {
                    let overlapping_payload =
                        payload[overlap_start_n as usize..overlap_end_n as usize].to_vec();
                    debug!(
                        "of_overlap_payload: overlapping_payload: {:?}",
                        overlapping_payload
                    );
                    if *overlapping_payload == *payload_0 {
                        if temporal_position_0 < temporal_position_1 {
                            TimePolicyTripletResidualPair::Old
                        } else {
                            TimePolicyTripletResidualPair::New
                        }
                    } else if *overlapping_payload == *payload_1 {
                        if temporal_position_0 < temporal_position_1 {
                            TimePolicyTripletResidualPair::New
                        } else {
                            TimePolicyTripletResidualPair::Old
                        }
                    } else if authorize_overlap_with_no_data {
                        //TimePolicyTripletResidualPair::OverlapIgnore
                        TimePolicyTripletResidualPair::None
                    } else {
                        TimePolicyTripletResidualPair::Bug
                        //    panic!(
                    //    "Unexpected payload, we found {:?}, but it is not equal to either {:?} or {:?}. See more detail below:\n{:?}",
                    //    overlapping_payload, payload_0, payload_1,pair_overlap
                    //)
                    }
                }
            }
        };

        debug!("of_overlap_payload: end");

        policy
    }

    pub fn of_pair_position(
        payload_mode: &PayloadMode,
        temporal_position_0: &u16,
        temporal_position_1: &u16,
        pair_position: &PairPosition,
        payload_string_option: &Option<Vec<u8>>,
        authorize_overlap_with_no_data: bool,
    ) -> TimePolicyTripletResidualPair {
        match pair_position {
            PairPosition::Disjoint(_) => TimePolicyTripletResidualPair::None,
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

    pub fn of_data(
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
                TimePolicyTripletResidualPair::of_pair_position(
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
                    TimePolicyTripletResidualPair::of_pair_position(
                        payload_mode,
                        temporal_position_0,
                        temporal_position_1,
                        pair_position_first,
                        payload_string_option,
                        authorize_overlap_with_no_data,
                    );
                let pair_time_policy_second =
                    TimePolicyTripletResidualPair::of_pair_position(
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

    pub fn extract_policy_consitency(
        &self,
        pair_time_policy: &PairChoice,
    ) -> PolicyConsistency {

        let consistent_policy_bool = match pair_time_policy {
            PairChoice::None => *self == TimePolicyTripletResidualPair::None, 
            PairChoice::Ignore => *self == TimePolicyTripletResidualPair::Ignore || *self == TimePolicyTripletResidualPair::None,
            PairChoice::First => { *self == TimePolicyTripletResidualPair::Old || *self == TimePolicyTripletResidualPair::None },
            PairChoice::Second => { *self == TimePolicyTripletResidualPair::New || *self == TimePolicyTripletResidualPair::None },
            //PairChoice::PartialIgnore => *self == TimePolicyTripletResidualPair::PartialIgnore || *self == TimePolicyTripletResidualPair::None,
            PairChoice::PartialIgnore => panic!("this case should be impossible"),
            PairChoice::OverlapIgnore => panic!("this case should be impossible"),
            PairChoice::Both =>  panic!("what to do ?"),
            PairChoice::Bug => panic!("what to do?")
        };

        if consistent_policy_bool {
            return PolicyConsistency::Consistent
        } else {
            return PolicyConsistency::NotConsistent

        }

    }

}
