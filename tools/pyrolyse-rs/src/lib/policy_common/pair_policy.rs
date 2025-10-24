use serde::{Deserialize, Serialize};

use crate::position::pair_overlap::PairOverlap;
use crate::position::payload_mode::PayloadMode;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PairPolicy {
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
    // Both fragments are kept
    Both
}

impl PairPolicy {
    pub fn of_overlap_payload(
        payload_mode: &PayloadMode,
        pair_overlap: &PairOverlap,
        temporal_position_0: u16,
        temporal_position_1: u16,
        payload_option: &Option<String>,
    ) -> PairPolicy {
        debug!("of_overlap_payload: start");

        debug!("of_overlap_payload: payload_option: {:?}", payload_option);

        let overlap_start = pair_overlap.get_start();
        let overlap_end = pair_overlap.get_end();
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
        assert!(overlap_start_n <= overlap_end_n);

        let payload_0 = pair_overlap.get_payload_0();
        let payload_1 = pair_overlap.get_payload_1();
        debug!("of_overlap_payload: payload_0: {:?}", payload_0);
        debug!("of_overlap_payload: payload_1: {:?}", payload_1);

        let policy = match payload_option {
            None => PairPolicy::Ignore,
            Some(payload) => {
                // In case we got several Echo Reply
                // XXX check if it works for triplets
                if payload.len() == 0 {
                    PairPolicy::Both
                }
                // Windows 10 echo often partially answers to sent data.
                // If the end of overlap is located after the end of the payload, it means that the receveived payload is partial.
                else if overlap_end_n as usize > payload.len() {
                    PairPolicy::PartialIgnore
                } else {
                    let overlapping_payload =
                        payload[overlap_start_n as usize..overlap_end_n as usize].to_string();
                    debug!(
                        "of_overlap_payload: overlapping_payload: {:?}",
                        overlapping_payload
                    );
                    if *overlapping_payload == *payload_0 {
                        if temporal_position_0 < temporal_position_1 {
                            PairPolicy::Old
                        } else {
                            PairPolicy::New
                        }
                    } else if *overlapping_payload == *payload_1 {
                        if temporal_position_0 < temporal_position_1 {
                            PairPolicy::New
                        } else {
                            PairPolicy::Old
                        }
                    } else {
                        panic!(
                        "Unexpected payload, we found {}, but it is not equal to either {} or {}. See more detail below:\n{:?}",
                        overlapping_payload, payload_0, payload_1,pair_overlap
                    )
                    }
                }
            }
        };

        debug!("of_overlap_payload: end");

        policy
    }
}
