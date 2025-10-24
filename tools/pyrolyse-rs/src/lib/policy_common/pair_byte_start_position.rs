use serde::{Deserialize, Serialize};

// use crate::position::pair_overlap::PairOverlap;
// use crate::position::payload_mode::PayloadMode;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PairByteStartPosition {
    // Chunk starts Before
    ByteStartBefore,
    // Chunk starts same
    ByteStartSame,
    // Chunk starts after
    ByteStartAfter,
}

impl PairByteStartPosition {
    // pub fn of_allen_interval_algebra_relation(
    //     allen_interval_algebra_relation: &AllenIntervalAlgebraRelation,
    // ) -> PairByteStartPosition {
    //     if allen_interval_algebra_relation.start_before() {
    //         PairByteStartPosition::ByteStartBefore
    //     } else {
    //         PairByteStartPosition::ByteStartAfter
    //     }
    // }

    pub fn of_allen_interval_algebra_relation(
        allen_interval_algebra_relation: &AllenIntervalAlgebraRelation,
    ) -> PairByteStartPosition {
        match allen_interval_algebra_relation {
            AllenIntervalAlgebraRelation::Eq => PairByteStartPosition::ByteStartSame,
            AllenIntervalAlgebraRelation::M => PairByteStartPosition::ByteStartBefore,
            AllenIntervalAlgebraRelation::Mi => PairByteStartPosition::ByteStartAfter,
            AllenIntervalAlgebraRelation::B => PairByteStartPosition::ByteStartBefore,
            AllenIntervalAlgebraRelation::Bi => PairByteStartPosition::ByteStartAfter,
            AllenIntervalAlgebraRelation::O => PairByteStartPosition::ByteStartBefore,
            AllenIntervalAlgebraRelation::Oi => PairByteStartPosition::ByteStartAfter,
            AllenIntervalAlgebraRelation::S => PairByteStartPosition::ByteStartSame,
            AllenIntervalAlgebraRelation::Si => PairByteStartPosition::ByteStartSame,
            AllenIntervalAlgebraRelation::D => PairByteStartPosition::ByteStartAfter,
            AllenIntervalAlgebraRelation::Di => PairByteStartPosition::ByteStartBefore,
            AllenIntervalAlgebraRelation::F => PairByteStartPosition::ByteStartAfter,
            AllenIntervalAlgebraRelation::Fi => PairByteStartPosition::ByteStartBefore,
        }
    }
}
