use std::fmt;
// use std::ops::Add;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct TestIndex(pub u16);


impl fmt::Display for TestIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }

}

impl From<TestIndex> for u16 {
    // Required method
    fn from(value: TestIndex) -> u16 {
        value.0
    }
}

