use serde::{Deserialize, Serialize};

use crate::byte_time_data::pair_triplet_byte_time_sequence::PairTripletByteTimeSequence;
use crate::position::pair_position_data::PairPositionDataC;
use crate::position::payload_mode::PayloadMode;
use crate::position::triplet_position_data::TripletPositionDataC;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllPositionData {
    pair_position_data_c: PairPositionDataC,
    triplet_position_data_c: TripletPositionDataC,
}

impl AllPositionData {
    pub fn new(
        pair_position_data_c: PairPositionDataC,
        triplet_position_data_c: TripletPositionDataC,
    ) -> AllPositionData {
        AllPositionData {
            pair_position_data_c,
            triplet_position_data_c,
        }
    }

    pub fn get_pair_position_data_c(&self) -> &PairPositionDataC {
        &self.pair_position_data_c
    }

    pub fn get_triplet_position_data_c(&self) -> &TripletPositionDataC {
        &self.triplet_position_data_c
    }

    pub fn of_byte_time_sequence(
        payload_mode: &PayloadMode,
        pair_triplet_byte_time_sequence: &PairTripletByteTimeSequence,
    ) -> AllPositionData {
        debug!("of_all_chunk: start");

        debug!("\n\n\n");
        debug!("of_all_chunk: building pair_position_data_c");
        let pair_position_data_c = PairPositionDataC::of_byte_time_sequence_c_pair(
            payload_mode,
            pair_triplet_byte_time_sequence.get_byte_time_sequence_c_pair(),
        );
        debug!("\n\n\n");
        debug!("of_all_chunk: building triplet_position_data_c");
        let triplet_position_data_c = TripletPositionDataC::of_byte_time_sequence_c_triplet(
            payload_mode,
            pair_triplet_byte_time_sequence.get_byte_time_sequence_c_triplet(),
        );

        debug!("of_all_chunk: end");
        AllPositionData::new(pair_position_data_c, triplet_position_data_c)
    }
}
