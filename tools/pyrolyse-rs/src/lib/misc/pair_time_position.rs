use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum PairTimePosition {
    Before,
    After,
}

impl PairTimePosition {
    pub fn of_temporal_position(
        temporal_position_0: u16,
        temporal_position_1: u16,
    ) -> PairTimePosition {
        if temporal_position_0 < temporal_position_1 {
            PairTimePosition::Before
        } else {
            PairTimePosition::After
        }
    }
}
