use serde::{Deserialize, Serialize};

use crate::byte_time_data::pair_triplet_byte_time_sequence::PairTripletByteTimeSequence;
use crate::position::pair_position::PairPositionC;
use crate::position::triplet_position::TripletPositionC;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllPosition {
    pair_position_c: PairPositionC,
    triplet_position_c: TripletPositionC,
}

impl AllPosition {
    pub fn new(
        pair_position_c: PairPositionC,
        triplet_position_c: TripletPositionC,
    ) -> AllPosition {
        AllPosition {
            pair_position_c,
            triplet_position_c,
        }
    }

    pub fn get_pair_position_c(&self) -> &PairPositionC {
        &self.pair_position_c
    }

    pub fn get_triplet_position_c(&self) -> &TripletPositionC {
        &self.triplet_position_c
    }

    pub fn of_byte_time_sequence(
        pair_triplet_byte_time_sequence: &PairTripletByteTimeSequence,
    ) -> AllPosition {
        debug!("of_all_chunk: start");

        let pair_position_c = PairPositionC::of_byte_time_sequence_c_pair(
            &pair_triplet_byte_time_sequence.get_byte_time_sequence_c_pair(),
        );
        let triplet_position_c = TripletPositionC::of_byte_time_sequence_c_triplet(
            &pair_triplet_byte_time_sequence.get_byte_time_sequence_c_triplet(),
        );

        debug!("of_all_chunk: end");
        AllPosition::new(pair_position_c, triplet_position_c)
    }
}
