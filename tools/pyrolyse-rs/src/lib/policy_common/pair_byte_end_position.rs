use serde::{Deserialize, Serialize};

// use crate::position::pair_overlap::PairOverlap;
// use crate::position::payload_mode::PayloadMode;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PairByteEndPosition {
    // Chunk starts Before
    ByteEndBefore,
    // Chunk ends same
    ByteEndSame,
    // Chunk starts after
    ByteEndAfter,
}

impl PairByteEndPosition {
    // pub fn of_allen_interval_algebra_relation(
    //     allen_interval_algebra_relation: &AllenIntervalAlgebraRelation,
    // ) -> PairByteEndPosition {
    //     if allen_interval_algebra_relation.end_before() {
    //         PairByteEndPosition::ByteEndBefore
    //     } else {
    //         PairByteEndPosition::ByteEndBefore
    //     }
    // }

    pub fn of_allen_interval_algebra_relation(
        allen_interval_algebra_relation: &AllenIntervalAlgebraRelation,
    ) -> PairByteEndPosition {
        match allen_interval_algebra_relation {
            AllenIntervalAlgebraRelation::Eq => PairByteEndPosition::ByteEndSame,
            AllenIntervalAlgebraRelation::M => PairByteEndPosition::ByteEndBefore,
            AllenIntervalAlgebraRelation::Mi => PairByteEndPosition::ByteEndAfter,
            AllenIntervalAlgebraRelation::B => PairByteEndPosition::ByteEndBefore,
            AllenIntervalAlgebraRelation::Bi => PairByteEndPosition::ByteEndAfter,
            AllenIntervalAlgebraRelation::O => PairByteEndPosition::ByteEndBefore,
            AllenIntervalAlgebraRelation::Oi => PairByteEndPosition::ByteEndAfter,
            AllenIntervalAlgebraRelation::S => PairByteEndPosition::ByteEndBefore,
            AllenIntervalAlgebraRelation::Si => PairByteEndPosition::ByteEndAfter,
            AllenIntervalAlgebraRelation::D => PairByteEndPosition::ByteEndBefore,
            AllenIntervalAlgebraRelation::Di => PairByteEndPosition::ByteEndAfter,
            AllenIntervalAlgebraRelation::F => PairByteEndPosition::ByteEndSame,
            AllenIntervalAlgebraRelation::Fi => PairByteEndPosition::ByteEndSame,
        }
    }
}
