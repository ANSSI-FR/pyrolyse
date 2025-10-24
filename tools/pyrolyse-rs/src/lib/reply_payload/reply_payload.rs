use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::misc::test_index::TestIndex;
use crate::reply_payload::reply_payload_hex::ReplyPayloadHexC;
use crate::reply_payload::reply_payload_string::ReplyPayloadStringC;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyPayloadC {
    hm: HashMap<TestIndex, Option<String>>,
}

impl ReplyPayloadC {
    pub fn new(hm: HashMap<TestIndex, Option<String>>) -> ReplyPayloadC {
        ReplyPayloadC { hm }
    }

    pub fn get(&self, index: &TestIndex) -> Option<&Option<String>> {
        self.hm.get(index)
    }

    pub fn contains_key(&self, index: &TestIndex) -> bool {
        self.hm.contains_key(index)
    }

    pub fn of_replay_payload_hex(reply_payload_hex_c: ReplyPayloadHexC) -> ReplyPayloadC {
        let hm = reply_payload_hex_c
            .get_hm()
            .iter()
            .map(|(index, reply_payload_hex_d)| {
                let o = if *reply_payload_hex_d.get_is_echo_reply() {
                    debug!("of_data: reply_payload_hex_d: {:?}", reply_payload_hex_d);

                    let s_v = reply_payload_hex_d
                        .get_payload()
                        .chars()
                        .collect::<Vec<char>>()
                        .chunks(2)
                        .map(|c| c.iter().collect::<String>())
                        .collect::<Vec<String>>();
                    debug!("of_data: s_v: {:?}", s_v);

                    let u8_map = s_v.iter().map(|s| u8::from_str_radix(s, 16).unwrap());
                    debug!("of_data: u8_v: {:?}", u8_map);

                    // TODO: remove this
                    // We remove padding with 0.
                    let u8_v_wo_padding =
                        u8_map.into_iter().filter(|u| *u != 0).collect::<Vec<u8>>();
                    debug!("of_data: u8_v_wo_padding: {:?}", u8_v_wo_padding);

                    let payload = String::from_utf8(u8_v_wo_padding).unwrap();
                    debug!("of_data: payload: {:?}", payload);

                    Some(payload)
                } else {
                    None
                };

                (*index, o)
            })
            .collect();
        ReplyPayloadC::new(hm)
    }

    pub fn of_replay_payload_string(reply_payload_string_c: ReplyPayloadStringC) -> ReplyPayloadC {
        let hm = reply_payload_string_c
            .get_hm()
            .iter()
            .map(|(index, reply_payload_string_d)| {
                let o = if *reply_payload_string_d.get_is_echo_reply() {
                    // debug!("of_data: reply_payload_hex_d: {:?}", reply_payload_hex_d);

                    // let s_v = reply_payload_hex_d
                    //     .get_payload()
                    //     .chars()
                    //     .collect::<Vec<char>>()
                    //     .chunks(2)
                    //     .map(|c| c.iter().collect::<String>())
                    //     .collect::<Vec<String>>();
                    // debug!("of_data: s_v: {:?}", s_v);

                    // let u8_map = s_v.iter().map(|s| u8::from_str_radix(s, 16).unwrap());
                    // debug!("of_data: u8_v: {:?}", u8_map);

                    // let payload = String::from_utf8(u8_v_wo_padding).unwrap();
                    // debug!("of_data: payload: {:?}", payload);

                    Some(reply_payload_string_d.get_payload().clone())
                } else {
                    None
                };

                (*index, o)
            })
            .collect();
        ReplyPayloadC::new(hm)
    }
}
