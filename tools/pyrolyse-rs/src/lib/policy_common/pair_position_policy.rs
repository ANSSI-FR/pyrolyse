use std::str;

use serde::{Deserialize, Serialize};

use crate::misc::pair_time_position::PairTimePosition;
use crate::policy_common::pair_byte_end_position::PairByteEndPosition;
use crate::policy_common::pair_byte_policy::PairBytePolicy;
use crate::policy_common::pair_byte_start_position::PairByteStartPosition;
use crate::policy_common::pair_choice::PairChoice;
use crate::policy_common::pair_time_policy::PairTimePolicy;
use crate::position::pair_position::PairPosition;
use crate::position::payload_mode::PayloadMode;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PairPositionPolicy {
    relation: AllenIntervalAlgebraRelation,
    pair_time_position: PairTimePosition,
    pair_byte_start_position: PairByteStartPosition,
    pair_byte_end_position: PairByteEndPosition,
    pair_choice: PairChoice,

    time_policy_pair: PairTimePolicy,
    pair_byte_policy: PairBytePolicy,
}

impl PairPositionPolicy {
    pub fn new(
        relation: AllenIntervalAlgebraRelation,
        pair_time_position: PairTimePosition,
        pair_byte_start_position: PairByteStartPosition,
        pair_byte_end_position: PairByteEndPosition,
        pair_choice: PairChoice,

        time_policy_pair: PairTimePolicy,
        pair_byte_policy: PairBytePolicy,
    ) -> PairPositionPolicy {
        PairPositionPolicy {
            relation,
            pair_time_position,
            pair_byte_start_position,
            pair_byte_end_position,
            pair_choice,

            time_policy_pair,
            pair_byte_policy,
        }
    }

    pub fn get_relation(&self) -> &AllenIntervalAlgebraRelation {
        &self.relation
    }

    pub fn get_pair_time_position(&self) -> &PairTimePosition {
        &self.pair_time_position
    }

    pub fn get_pair_choice(&self) -> &PairChoice {
        &self.pair_choice
    }

    pub fn get_time_policy(&self) -> &PairTimePolicy {
        &self.time_policy_pair
    }

    // TODO: merge next two functions

    // pub fn of_data____(
    //     temporal_position_v: &[u16],
    //     // payload_byte_length: u16,
    //     pair_position_data_d: &PairPositionDataD,
    //     payload_string_option: &Option<Vec<u8>>,
    // ) -> PairPositionPolicy {
    //     let pair_position = pair_position_data_d.get_pair_position();

    //     // let payload_length_correctness = match payload_string_option {
    //     //     None => false,
    //     //     Some(s) => payload_byte_length == s.len() as u16,
    //     // };

    //     let temporal_position_0 = temporal_position_v.get(0).unwrap();
    //     let temporal_position_1 = temporal_position_v.get(1).unwrap();

    //     let time_position =
    //         TimePosition::of_temporal_position(*temporal_position_0, *temporal_position_1);

    //     let (byte_relation, policy) = match pair_position {
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

    //     PairPositionPolicy::new(byte_relation, time_position, policy)
    // }

    pub fn of_data(
        payload_mode: &PayloadMode,
        temporal_position_v: &[u16],
        // payload_byte_length: u16,
        pair_position: &PairPosition,
        payload_string_option: &Option<Vec<u8>>,
        authorize_overlap_with_no_data: bool,
    ) -> PairPositionPolicy {
        let temporal_position_0 = temporal_position_v.first().unwrap();
        let temporal_position_1 = temporal_position_v.get(1).unwrap();

        let pair_time_position =
            PairTimePosition::of_temporal_position(*temporal_position_0, *temporal_position_1);

        let pair_byte_start_position = PairByteStartPosition::of_allen_interval_algebra_relation(
            pair_position.get_allen_interval_algebra_relation(),
        );

        let pair_byte_end_position = PairByteEndPosition::of_allen_interval_algebra_relation(
            pair_position.get_allen_interval_algebra_relation(),
        );

        let byte_relation = pair_position.get_allen_interval_algebra_relation();

        let (pair_choice, time_policy_pair, pair_byte_policy) = match pair_position {
            //PairPosition::Disjoint(_) => (
            //    // (*byte_relation).clone(),
            //    PairChoice::None,
            //    PairTimePolicy::None,
            //    PairBytePolicy::None,
//
            //),
            PairPosition::Disjoint(allen_interval_algebra_relation) => {
                // remove if we don't want to take into account M test case ignored
                if (*allen_interval_algebra_relation == AllenIntervalAlgebraRelation::M || *allen_interval_algebra_relation == AllenIntervalAlgebraRelation::Mi) && payload_string_option.is_none() {
                    (PairChoice::Ignore,
                    PairTimePolicy::Ignore,
                    PairBytePolicy::Ignore)
                } else {
                    (PairChoice::None,
                    PairTimePolicy::None,
                    PairBytePolicy::None)
                }
            },
            PairPosition::Overlap(pair_overlap) => {
                // let byte_relation = pair_overlap.get_byte_relation();

                let pair_choice = PairChoice::of_overlap_payload(
                    payload_mode,
                    pair_overlap,
                    payload_string_option,
                    true,
                );

                let time_policy_pair = PairTimePolicy::of_overlap_payload(
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

                // ((*byte_relation).clone(), time_policy_pair, pair_byte_policy)
                (pair_choice, time_policy_pair, pair_byte_policy)
            }
        };

        PairPositionPolicy::new(
            (*byte_relation).clone(),
            pair_time_position,
            pair_byte_start_position,
            pair_byte_end_position,
            pair_choice,
            time_policy_pair,
            pair_byte_policy,
        )
    }
}
