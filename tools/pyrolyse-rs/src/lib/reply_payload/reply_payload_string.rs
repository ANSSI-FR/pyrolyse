use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::misc::test_index::TestIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyPayloadStringD {
    is_echo_reply: bool,
    number: usize,
    payload: String,
}

impl ReplyPayloadStringD {
    pub fn get_is_echo_reply(&self) -> &bool {
        &self.is_echo_reply
    }

    pub fn get_payload(&self) -> &String {
        &self.payload
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyPayloadStringC {
    hm: HashMap<TestIndex, ReplyPayloadStringD>,
}

impl ReplyPayloadStringC {
    pub fn new(hm: HashMap<TestIndex, ReplyPayloadStringD>) -> ReplyPayloadStringC {
        ReplyPayloadStringC { hm }
    }

    pub fn get_index(&self, index: &TestIndex) -> Option<&ReplyPayloadStringD> {
        self.hm.get(index)
    }

    pub fn get_hm(&self) -> &HashMap<TestIndex, ReplyPayloadStringD> {
        &self.hm
    }
}
