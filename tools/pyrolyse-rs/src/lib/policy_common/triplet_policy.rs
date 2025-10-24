use serde::{Deserialize, Serialize};

use crate::position::{payload_mode::PayloadMode, triplet_overlap::TripletOverlap};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum TripletPolicy {
    // No triple overlap
    None,
    // All fragments are dropped
    Ignore,
    // Old fragment is kept
    Old,
    // Middle fragment is kept
    Middle,
    // New fragment is kept
    New,
    // Multiple fragments are kept
    Multiple,
    // Unexpected data is present
    Bug,
}

impl TripletPolicy {
    pub fn of_temporal_position(
        temporal_position_v: Vec<u16>,
        reassembled_index: usize,
    ) -> TripletPolicy {
        assert_eq!(temporal_position_v.len(), 3);
        assert!(reassembled_index <= 2);
        match temporal_position_v[reassembled_index] {
            0 => TripletPolicy::Old,
            1 => TripletPolicy::Middle,
            2 => TripletPolicy::New,
            _ => panic!("Unepxected temporal position outside of [0,2]"),
        }
    }

    pub fn of_overlap_payload(
        payload_mode: &PayloadMode,
        triplet_overlap: &TripletOverlap,
        temporal_position_v: Vec<u16>,
        payload_option: &Option<Vec<u8>>,
    ) -> TripletPolicy {
        debug!("of_overlap_payload: start");

        let overlap_start = triplet_overlap.get_start();
        let overlap_end = triplet_overlap.get_end();
        debug!(
            "of_overlap_payload: pair_overlap: {:?} -> {:?}",
            overlap_start, overlap_end
        );

        //let factor = match payload_mode {
        //    // Each pattern contains a single character.
        //    PayloadMode::VariableChecksum1Byte(_) => 1,
        //    // Each pattern contains 8 characters.
        //    PayloadMode::InvariantChecksumFixedLength8Byte(_) 
        //    | PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(_) 
        //    | PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(_) => 8,
        //};
        let factor = payload_mode.get_factor();
        let overlap_start_n = overlap_start * factor;
        let overlap_end_n = (overlap_end + 1) * factor;
        debug!(
            "of_overlap_payload: overlap n: {:?} -> {:?}",
            overlap_start_n, overlap_end_n
        );

        let payload_0 = triplet_overlap.get_payload_0();
        let payload_1 = triplet_overlap.get_payload_1();
        let payload_2 = triplet_overlap.get_payload_2();

        let policy = match payload_option {
            None => TripletPolicy::Ignore,
            Some(payload) => {
                if payload.len() == 0 {
                    return TripletPolicy::Multiple;
                }
                let overlapping_payload =
                    payload[overlap_start_n as usize..overlap_end_n as usize].to_vec();
                if *overlapping_payload == *payload_0 {
                    TripletPolicy::of_temporal_position(temporal_position_v, 0)
                } else if *overlapping_payload == *payload_1 {
                    TripletPolicy::of_temporal_position(temporal_position_v, 1)
                } else if *overlapping_payload == *payload_2 {
                    TripletPolicy::of_temporal_position(temporal_position_v, 2)
                } else {
                    TripletPolicy::Bug
                    //    panic!(
                //    "Unexpected payload, we found {:?}, but it is not equal to either {:?} or {:?}. See more detail below:\n{:?}",
                //    overlapping_payload, payload_0, payload_1,pair_overlap
                //)
                }
            }
        };

        debug!("of_overlap_payload: start");

        policy
    }
}
