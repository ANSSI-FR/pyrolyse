use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::Iterator;

use serde::{Deserialize, Serialize};

use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceC;
use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceD;
use crate::position::pair_position::PairPosition;
use crate::misc::test_index::TestIndex;
use crate::position::payload_mode::PayloadMode;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairPositionDataD {
    byte_sequence_index: TestIndex,
    pair_position: PairPosition,
}

impl PairPositionDataD {
    pub fn new(byte_sequence_index: TestIndex, pair_position: PairPosition) -> PairPositionDataD {
        PairPositionDataD {
            byte_sequence_index,
            pair_position,
        }
    }

    pub fn get_byte_sequence_index(&self) -> TestIndex {
        self.byte_sequence_index
    }

    pub fn get_pair_position(&self) -> &PairPosition {
        &self.pair_position
    }

    fn of_byte_time_sequence_d_pair(
        payload_mode: &PayloadMode,
        byte_time_sequence_d: &ByteTimeSequenceD<AllenIntervalAlgebraRelation>,
    ) -> PairPositionDataD {
        let chunk_c = byte_time_sequence_d.get_chunk_c();
        let chunk_0 = chunk_c.get(&0).unwrap();
        let chunk_1 = chunk_c.get(&1).unwrap();

        let interval_c = byte_time_sequence_d.get_interval_c();
        let interval_0 = interval_c.get(&0).unwrap();
        let interval_1 = interval_c.get(&1).unwrap();

        let position = PairPosition::of_relation_interval_chunk(
            payload_mode,
            &byte_time_sequence_d.get_rc(),
            interval_0,
            interval_1,
            chunk_0,
            chunk_1,
        );

        PairPositionDataD::new(byte_time_sequence_d.get_byte_sequence_index(), position)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairPositionDataC {
    hm: HashMap<TestIndex, PairPositionDataD>,
}

impl FromIterator<(TestIndex, PairPositionDataD)> for PairPositionDataC {
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = (TestIndex, PairPositionDataD)>,
    {
        Self {
            hm: HashMap::from_iter(iter),
        }
    }
}

impl PairPositionDataC {
    pub fn len(&self) -> usize {
        self.hm.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hm.is_empty()
    }
    pub fn get_index_total_length(&self) -> u32 {
        (self.hm.len() as u32) * 3
    }

    pub fn of_byte_time_sequence_c_pair(
        payload_mode: &PayloadMode,
        byte_time_sequence_c: &ByteTimeSequenceC<AllenIntervalAlgebraRelation>,
    ) -> PairPositionDataC {
        debug!("of_byte_time_sequence_c_pair: start");

        let hm = byte_time_sequence_c
            .iter()
            .map(|(index, byte_time_sequence_d_pair)| {
                debug!("of_byte_time_sequence_c_pair: index: {:?}", index);
                (
                    *index,
                    PairPositionDataD::of_byte_time_sequence_d_pair(
                        payload_mode,
                        byte_time_sequence_d_pair,
                    ),
                )
            })
            .collect();

        debug!("of_byte_time_sequence_c_pair: end");
        PairPositionDataC { hm }
    }

    pub fn iter(&self) -> Iter<TestIndex, PairPositionDataD> {
        self.hm.iter()
    }
}
