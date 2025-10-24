use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::Iterator;

use serde::{Deserialize, Serialize};

use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceC;
use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceD;
use crate::misc::test_index::TestIndex;
use crate::position::pair_position::PairPosition;
// use crate::position::pair_position_data::PairPositionDataD;
use crate::position::payload_mode::PayloadMode;
use crate::position::triplet_pair_position_data::TripletPairPositionData;
use crate::position::triplet_position::TripletPosition;
use crate::relation::relation_triplet::RelationTripletD;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripletPositionDataD {
    byte_sequence_index: TestIndex,

    // pair_position_data_01: PairPositionDataD,
    // pair_position_data_02: PairPositionDataD,
    // pair_position_data_12: PairPositionDataD,
    pair_position_01: PairPosition,
    pair_position_02: PairPosition,
    pair_position_12: PairPosition,

    triplet_position_012: TripletPosition,
    triplet_pair_position_data_01: TripletPairPositionData,
    triplet_pair_position_data_02: TripletPairPositionData,
    triplet_pair_position_data_12: TripletPairPositionData,
}

impl TripletPositionDataD {
    pub fn new(
        byte_sequence_index: TestIndex,

        // pair_position_data_01: PairPositionDataD,
        // pair_position_data_02: PairPositionDataD,
        // pair_position_data_12: PairPositionDataD,
        pair_position_01: PairPosition,
        pair_position_02: PairPosition,
        pair_position_12: PairPosition,

        triplet_position_012: TripletPosition,
        triplet_pair_position_data_01: TripletPairPositionData,
        triplet_pair_position_data_02: TripletPairPositionData,
        triplet_pair_position_data_12: TripletPairPositionData,
    ) -> TripletPositionDataD {
        TripletPositionDataD {
            byte_sequence_index,

            pair_position_01,
            pair_position_02,
            pair_position_12,

            triplet_position_012,
            triplet_pair_position_data_01,
            triplet_pair_position_data_02,
            triplet_pair_position_data_12,
        }
    }

    fn of_byte_time_sequence_d_triplet(
        payload_mode: &PayloadMode,
        byte_time_sequence_d_triplet: &ByteTimeSequenceD<RelationTripletD>,
    ) -> TripletPositionDataD {
        debug!("of_byte_time_sequence_d_triplet: start");

        let chunk_c = byte_time_sequence_d_triplet.get_chunk_c();
        let chunk_0 = chunk_c.get(&0).unwrap();
        let chunk_1 = chunk_c.get(&1).unwrap();
        let chunk_2 = chunk_c.get(&2).unwrap();

        let interval_c = byte_time_sequence_d_triplet.get_interval_c();
        let interval_0 = interval_c.get(&0).unwrap();
        let interval_1 = interval_c.get(&1).unwrap();
        let interval_2 = interval_c.get(&2).unwrap();

        let relation_triplet = byte_time_sequence_d_triplet.get_rc();
        let relation_01 = relation_triplet.get_relation_01();
        let relation_02 = relation_triplet.get_relation_02();
        let relation_12 = relation_triplet.get_relation_12();

        let pair_position_01 = PairPosition::of_relation_interval_chunk(
            payload_mode,
            relation_01,
            interval_0,
            interval_1,
            chunk_0,
            chunk_1,
        );
        // let pair_position_data_d_01 = PairPositionDataD::new(0, pair_position_d_01);

        let pair_position_02 = PairPosition::of_relation_interval_chunk(
            payload_mode,
            relation_02,
            interval_0,
            interval_2,
            chunk_0,
            chunk_2,
        );
        // let pair_position_data_d_02 = PairPositionDataD::new(0, pair_position_d_02);

        let pair_position_12 = PairPosition::of_relation_interval_chunk(
            payload_mode,
            relation_12,
            interval_1,
            interval_2,
            chunk_1,
            chunk_2,
        );
        // let pair_position_data_d_12 = PairPositionDataD::new(0, pair_position_d_12);

        debug!("of_byte_time_sequence_d_triplet: building triplet_position_012");
        let triplet_position_012 = TripletPosition::of_byte_time_sequence_d_triplet(
            payload_mode,
            byte_time_sequence_d_triplet,
        );

        debug!("of_byte_time_sequence_d_triplet: building triplet_pair_position_data_01");
        let triplet_pair_position_data_01 =
            TripletPairPositionData::of_triplet_position_relation_interval_chunk(
                payload_mode,
                &triplet_position_012,
                byte_time_sequence_d_triplet.get_rc().get_relation_01(),
                interval_0,
                interval_1,
                chunk_0,
                chunk_1,
            );

        debug!("of_byte_time_sequence_d_triplet: building triplet_pair_position_data_02");
        let triplet_pair_position_data_02 =
            TripletPairPositionData::of_triplet_position_relation_interval_chunk(
                payload_mode,
                &triplet_position_012,
                byte_time_sequence_d_triplet.get_rc().get_relation_02(),
                interval_0,
                interval_2,
                chunk_0,
                chunk_2,
            );

        debug!("of_byte_time_sequence_d_triplet: building triplet_pair_position_data_12");
        let triplet_pair_position_data_12 =
            TripletPairPositionData::of_triplet_position_relation_interval_chunk(
                payload_mode,
                &triplet_position_012,
                byte_time_sequence_d_triplet.get_rc().get_relation_12(),
                interval_1,
                interval_2,
                chunk_1,
                chunk_2,
            );

        debug!("of_byte_time_sequence_d_triplet: end");
        debug!("\n\n\n");

        TripletPositionDataD::new(
            byte_time_sequence_d_triplet.get_byte_sequence_index(),
            // pair_position_data_d_01,
            // pair_position_data_d_02,
            // pair_position_data_d_12,
            pair_position_01,
            pair_position_02,
            pair_position_12,
            triplet_position_012,
            triplet_pair_position_data_01,
            triplet_pair_position_data_02,
            triplet_pair_position_data_12,
        )
    }

    pub fn get_byte_sequence_index(&self) -> TestIndex {
        self.byte_sequence_index
    }

    // pub fn get_pair_position_data_01(&self) -> &PairPositionDataD {
    //     &self.pair_position_data_01
    // }

    // pub fn get_pair_position_data_02(&self) -> &PairPositionDataD {
    //     &self.pair_position_data_02
    // }

    // pub fn get_pair_position_data_12(&self) -> &PairPositionDataD {
    //     &self.pair_position_data_12
    // }

    pub fn get_pair_position_01(&self) -> &PairPosition {
        &self.pair_position_01
    }

    pub fn get_pair_position_02(&self) -> &PairPosition {
        &self.pair_position_02
    }

    pub fn get_pair_position_12(&self) -> &PairPosition {
        &self.pair_position_12
    }

    pub fn get_relation_triplet_d(&self) -> &RelationTripletD {
        self.triplet_position_012.get_relation_triplet_d()
    }

    pub fn get_triplet_position_012(&self) -> &TripletPosition {
        &self.triplet_position_012
    }

    pub fn get_triplet_pair_position_data_01(&self) -> &TripletPairPositionData {
        &self.triplet_pair_position_data_01
    }

    pub fn get_triplet_pair_position_data_02(&self) -> &TripletPairPositionData {
        &self.triplet_pair_position_data_02
    }

    pub fn get_triplet_pair_position_data_12(&self) -> &TripletPairPositionData {
        &self.triplet_pair_position_data_12
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripletPositionDataC {
    hm: HashMap<TestIndex, TripletPositionDataD>,
}

impl FromIterator<(TestIndex, TripletPositionDataD)> for TripletPositionDataC {
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = (TestIndex, TripletPositionDataD)>,
    {
        Self {
            hm: HashMap::from_iter(iter),
        }
    }
}

impl TripletPositionDataC {
    pub fn len(&self) -> usize {
        self.hm.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hm.is_empty()
    }

    pub fn get_index_total_length(&self) -> u32 {
        (self.hm.len() as u32) * 3
    }

    pub fn iter(&self) -> Iter<TestIndex, TripletPositionDataD> {
        self.hm.iter()
    }

    pub fn of_byte_time_sequence_c_triplet(
        payload_mode: &PayloadMode,
        byte_time_sequence_c: &ByteTimeSequenceC<RelationTripletD>,
    ) -> TripletPositionDataC {
        debug!("of_byte_time_sequence_c_triplet: start");

        let hm = byte_time_sequence_c
            .iter()
            .map(|(index, byte_time_sequence_d)| {
                debug!("of_byte_time_sequence_c_triplet: index: {:?}", index);
                (
                    *index,
                    TripletPositionDataD::of_byte_time_sequence_d_triplet(
                        payload_mode,
                        byte_time_sequence_d,
                    ),
                )
            })
            .collect();

        debug!("of_byte_time_sequence_c_triplet: end");
        TripletPositionDataC { hm }
    }
}
