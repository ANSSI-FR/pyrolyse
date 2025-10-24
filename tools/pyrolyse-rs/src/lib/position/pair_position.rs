use serde::{Deserialize, Serialize};

use crate::byte_time_data::chunk::ChunkD;
use crate::misc::interval::IntervalD;
use crate::position::pair_overlap::PairOverlap;
use crate::position::payload_mode::PayloadMode;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;

/// Relative position of two chunks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PairPosition {
    Disjoint(AllenIntervalAlgebraRelation),
    Overlap(PairOverlap),
}

impl PairPosition {
    pub fn get_allen_interval_algebra_relation(&self) -> &AllenIntervalAlgebraRelation {
        match self {
            PairPosition::Disjoint(r) => r,
            PairPosition::Overlap(pair_overlap) => pair_overlap.get_byte_relation(),
        }
    }

    pub fn of_relation_interval_chunk(
        payload_mode: &PayloadMode,
        byte_relation: &AllenIntervalAlgebraRelation,
        interval_0: &IntervalD,
        interval_1: &IntervalD,
        chunk_0: &ChunkD,
        chunk_1: &ChunkD,
    ) -> PairPosition {
        if interval_0.intersect(interval_1) {
            PairPosition::Overlap(PairOverlap::of_relation_interval_chunk(
                payload_mode,
                byte_relation,
                interval_0,
                interval_1,
                chunk_0,
                chunk_1,
            ))
        } else {
            // Sanity check
            debug!("No interscetion for byte_relation {:?}", byte_relation);
            assert!(
                *byte_relation == AllenIntervalAlgebraRelation::B
                    || *byte_relation == AllenIntervalAlgebraRelation::Bi
                    || *byte_relation == AllenIntervalAlgebraRelation::M
                    || *byte_relation == AllenIntervalAlgebraRelation::Mi
            );
            PairPosition::Disjoint((*byte_relation).clone())
        }
    }
}
