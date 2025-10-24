use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum ExtraChunkTime {
    // No extra chunk
    None,
    // Extra chunk sent before in term of time
    Precedes,
    // Extra chunk sent after in term of time
    Follows,
}