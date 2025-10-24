use serde::{Deserialize, Serialize};

use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceD;
use crate::byte_time_data::chunk::ChunkD;
use crate::misc::interval::IntervalD;
use crate::position::overlap::Overlap;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::relation::relation_triplet::RelationTripletD;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Position {
    Disjoint(AllenIntervalAlgebraRelation),
    Overlap(Overlap),
}

impl Position {
    pub fn of_relation_interval_chunk(
        byte_relation: AllenIntervalAlgebraRelation,
        interval_0: &IntervalD,
        interval_1: &IntervalD,
        chunk_0: &ChunkD,
        chunk_1: &ChunkD,
    ) -> Position {
        if interval_0.overlap(interval_1) {
            Position::Overlap(Overlap::of_relation_interval_chunk(
                byte_relation,
                interval_0,
                interval_1,
                chunk_0,
                chunk_1,
            ))
        } else {
            // Sanity check
            assert!(
                byte_relation == AllenIntervalAlgebraRelation::B
                    || byte_relation == AllenIntervalAlgebraRelation::Bi
            );
            Position::Disjoint(byte_relation)
        }
    }

    pub fn of_byte_time_sequence_d_triplet(
        byte_time_sequence_d: &ByteTimeSequenceD<RelationTripletD>,
    ) -> (Position, Position, Position) {
        let chunk_c = byte_time_sequence_d.get_chunk_c();
        let chunk_0 = chunk_c.get(0).unwrap();
        let chunk_1 = chunk_c.get(1).unwrap();
        let chunk_2 = chunk_c.get(2).unwrap();

        let interval_c = byte_time_sequence_d.get_interval_c();
        let interval_0 = interval_c.get(0).unwrap();
        let interval_1 = interval_c.get(1).unwrap();
        let interval_2 = interval_c.get(2).unwrap();

        let overlap_01 = Position::of_relation_interval_chunk(
            byte_time_sequence_d.get_rc().get_relation_01().clone(),
            interval_0,
            interval_1,
            chunk_0,
            chunk_1,
        );

        let overlap_02 = Position::of_relation_interval_chunk(
            byte_time_sequence_d.get_rc().get_relation_02().clone(),
            interval_0,
            interval_2,
            chunk_0,
            chunk_2,
        );

        let overlap_12 = Position::of_relation_interval_chunk(
            byte_time_sequence_d.get_rc().get_relation_12().clone(),
            interval_1,
            interval_2,
            chunk_1,
            chunk_2,
        );

        (overlap_01, overlap_02, overlap_12)
    }
}
