use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum PairIgnoreInterpretation {
    FlowAllChunksDrop,
    PairAllChunksDrop,
    PairNewestChunkDrop,
    //PairOldestChunkDrop
}
