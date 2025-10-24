use serde::{Deserialize, Serialize};

use crate::position::pair_overlap::PairOverlap;
use crate::position::payload_mode::PayloadMode;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PairChoice {
    // No reassembly possible (byte relation is B or Bi or M or Mi)
    None,
    // First fragment is kept
    Ignore,
    // First fragment is kept
    PartialIgnore,
    // Overlap is present but reassembled data is from another chunk (probable triple overlap).
    OverlapIgnore,
    // First fragment is kept
    First,
    // Second after is kept
    Second,
    // First and second are kept
    Both,
    // Unexpected data is present
    Bug,
}

impl PairChoice {
    pub fn of_overlap_payload(
        payload_mode: &PayloadMode,
        pair_overlap: &PairOverlap,
        payload_option: &Option<Vec<u8>>,
        authorize_overlap_with_no_data: bool,
    ) -> PairChoice {
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
            None => PairChoice::Ignore,
            Some(payload) => {
                // In case we got several Echo Reply
                // XXX check if it works for triplets
                if payload.len() == 0 {
                    PairChoice::Both
                }
                // Windows 10 echo often partially answers to sent data.
                // If the end of overlap is located after the end of the payload, it means that the receveived payload is partial.
                else if overlap_end_n as usize > payload.len() {
                    PairChoice::PartialIgnore
                } else {
                    let overlapping_payload =
                        payload[overlap_start_n as usize..overlap_end_n as usize].to_vec();
                    debug!(
                        "of_overlap_payload: overlapping_payload: {:?}",
                        overlapping_payload
                    );
                    if *overlapping_payload == *payload_0 {
                        PairChoice::First
                    } else if *overlapping_payload == *payload_1 {
                        PairChoice::Second
                    } else if authorize_overlap_with_no_data {
                        PairChoice::OverlapIgnore
                    } else {
                        PairChoice::Bug
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
}
