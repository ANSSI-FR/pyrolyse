use serde::{Deserialize, Serialize};

use crate::policy_common::pair_policy_data::PairPolicyDataC;
use crate::minimal_policy::triplet_policy_data::TripletPolicyDataC;
use crate::position::all_position_data::AllPositionData;
use crate::position::payload_mode::PayloadMode;
use crate::reply_payload::reply_payload::ReplyPayloadC;
use crate::byte_time_data::pair_triplet_byte_time_sequence::PairTripletByteTimeSequence;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllPolicy {
    pair_policy_data_c: PairPolicyDataC,
    triplet_policy_data_c: TripletPolicyDataC,
}

impl AllPolicy {
    pub fn new(
        pair_policy_data_c: PairPolicyDataC,
        triplet_policy_data_c: TripletPolicyDataC,
    ) -> AllPolicy {
        AllPolicy {
            pair_policy_data_c,
            triplet_policy_data_c,
        }
    }

    pub fn get_pair_policy_data_c(&self) -> &PairPolicyDataC {
        &self.pair_policy_data_c
    }

    pub fn get_triplet_policy_data_c(&self) -> &TripletPolicyDataC {
        &self.triplet_policy_data_c
    }

    pub fn of_data(
        payload_mode: &PayloadMode,
        pair_triplet_byte_time_sequence: &PairTripletByteTimeSequence,
        all_position_data: &AllPositionData,
        reply_payload_c: &ReplyPayloadC,
        authorize_overlap_with_no_data: bool,
    ) -> AllPolicy {
        debug!("of_data: start");

        let pair_policy_data_c = PairPolicyDataC::of_data(
            payload_mode,
            pair_triplet_byte_time_sequence.get_byte_time_sequence_c_pair(),
            all_position_data.get_pair_position_data_c(),
            reply_payload_c,
            authorize_overlap_with_no_data,
        );

        let triplet_policy_data_c = TripletPolicyDataC::of_data(
            payload_mode,
            pair_triplet_byte_time_sequence.get_byte_time_sequence_c_triplet(),
            all_position_data.get_triplet_position_data_c(),
            reply_payload_c,
            authorize_overlap_with_no_data,
        );

        debug!("of_data: end");
        AllPolicy::new(pair_policy_data_c, triplet_policy_data_c)
    }
}
