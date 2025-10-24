use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum PolicyConsistency {
    Na,
    Consistent,
    NotConsistent,
    NotConsistentPeoLike
}
