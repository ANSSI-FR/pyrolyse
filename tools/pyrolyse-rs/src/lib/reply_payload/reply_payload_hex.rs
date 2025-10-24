use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::misc::test_index::TestIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyPayloadHexD {
    is_echo_reply: bool,
    number: usize,
    payload: String,
}

impl ReplyPayloadHexD {
    pub fn get_is_echo_reply(&self) -> &bool {
        &self.is_echo_reply
    }

    pub fn get_payload(&self) -> &String {
        &self.payload
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyPayloadHexC {
    hm: HashMap<TestIndex, ReplyPayloadHexD>,
}

impl ReplyPayloadHexC {
    pub fn new(hm: HashMap<TestIndex, ReplyPayloadHexD>) -> ReplyPayloadHexC {
        ReplyPayloadHexC { hm }
    }

    pub fn get_index(&self, index: &TestIndex) -> Option<&ReplyPayloadHexD> {
        self.hm.get(index)
    }

    pub fn get_hm(&self) -> &HashMap<TestIndex, ReplyPayloadHexD> {
        &self.hm
    }
}
