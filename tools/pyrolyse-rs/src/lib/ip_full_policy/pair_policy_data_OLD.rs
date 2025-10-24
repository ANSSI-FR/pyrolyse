use std::collections::hash_map::Iter;
use std::collections::{BTreeMap, HashMap};
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::Iterator;
use std::str;

use serde::{Deserialize, Serialize, Serializer};

use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceC;
use crate::policy_common::pair_position_policy::PairPositionPolicy;
use crate::misc::pair_time_position::PairTimePosition;
use crate::misc::test_index::TestIndex;
use crate::policy_common::pair_choice::PairChoice;
use crate::policy_common::pair_time_policy::PairTimePolicy;
use crate::position::pair_position_data::PairPositionDataC;
use crate::position::pair_position_data::PairPositionDataD;
use crate::position::payload_mode::PayloadMode;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::reply_payload::reply_payload::ReplyPayloadC;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairPolicyDataD {
    byte_sequence_index: TestIndex,
    relation: AllenIntervalAlgebraRelation,
    pair_time_policy: PairTimePolicy,
    payload_length_correctness: bool,
}

impl PairPolicyDataD {
    pub fn new(
        byte_sequence_index: TestIndex,
        relation: AllenIntervalAlgebraRelation,
        pair_time_policy: PairTimePolicy,
        payload_length_correctness: bool,
    ) -> PairPolicyDataD {
        PairPolicyDataD {
            byte_sequence_index,
            relation,
            pair_time_policy,
            payload_length_correctness,
        }
    }

    pub fn get_relation(&self) -> &AllenIntervalAlgebraRelation {
        &self.relation
    }

    pub fn get_pair_time_policy(&self) -> &PairTimePolicy {
        &self.pair_time_policy
    }

    pub fn get_payload_length_correctness(&self) -> bool {
        self.payload_length_correctness
    }

    fn of_data(
        payload_mode: &PayloadMode,
        temporal_position_v: &[u16],
        payload_byte_length: u16,
        pair_position_data_d: &PairPositionDataD,
        payload_string_option: &Option<Vec<u8>>,
        authorize_overlap_with_no_data: bool,
    ) -> PairPolicyDataD {
        let pair_position = pair_position_data_d.get_pair_position();
        let relation = pair_position.get_allen_interval_algebra_relation();

        let position_policy = PairPositionPolicy::of_data(
            payload_mode,
            temporal_position_v,
            // payload_byte_length,
            pair_position,
            payload_string_option,
            authorize_overlap_with_no_data,
        );

        let relation = position_policy.get_relation();
        let pair_time_policy = position_policy.get_time_policy();

        let payload_length_correctness = match payload_string_option {
            None => false,
            Some(s) => payload_byte_length == s.len() as u16,
        };

        PairPolicyDataD::new(
            pair_position_data_d.get_byte_sequence_index(),
            relation.clone(),
            pair_time_policy.clone(),
            payload_length_correctness,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairPolicyDataC {
    #[serde(serialize_with = "ordered_map")]
    hm: HashMap<TestIndex, PairPolicyDataD>,
}

fn ordered_map<S>(
    value: &HashMap<TestIndex, PairPolicyDataD>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

impl FromIterator<(TestIndex, PairPolicyDataD)> for PairPolicyDataC {
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = (TestIndex, PairPolicyDataD)>,
    {
        Self {
            hm: HashMap::from_iter(iter),
        }
    }
}

impl PairPolicyDataC {
    pub fn len(&self) -> usize {
        self.hm.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hm.is_empty()
    }

    pub fn get_index_total_length(&self) -> u32 {
        (self.hm.len() as u32) * 3
    }

    pub fn of_data(
        payload_mode: &PayloadMode,
        byte_time_sequence_c: &ByteTimeSequenceC<AllenIntervalAlgebraRelation>,
        pair_position_data_c: &PairPositionDataC,
        reply_payload_c: &ReplyPayloadC,
        authorize_overlap_with_no_data: bool,
    ) -> PairPolicyDataC {
        debug!("of_data: start");

        let hm: HashMap<_,_> = pair_position_data_c
            .iter()
            //.map(|(byte_time_sequence_index, pair_overlap_d)| {
            .filter_map(|(byte_time_sequence_index, pair_overlap_d)| 
                match reply_payload_c.get(byte_time_sequence_index) {
                    Some(payload_string_option) => {
                        debug!("\n\n\n");
                        debug!(
                            "of_data: byte_time_sequence_index: {:?}",
                            byte_time_sequence_index
                        );

                        //let payload_string_option = reply_payload_c.get(byte_time_sequence_index).unwrap();
                        let payload_ascii_v_option: Option<Vec<u8>> = match payload_string_option {
                            Some(payload_string) => Some(payload_string.chars().map(|c| c as u8).collect::<Vec<_>>()),
                            None => None
                        };

                        let byte_time_sequence_d =
                            byte_time_sequence_c.get(byte_time_sequence_index).unwrap();
                        let payload_byte_length = match payload_mode {
                            PayloadMode::VariableChecksum1Byte(_) => byte_time_sequence_d.get_simple_payload_byte_length(),
                            PayloadMode::InvariantChecksumFixedLength8Byte(_) => {
                                byte_time_sequence_d.get_internet_checksum_payload_byte_length()
                            }
                            PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(_) 
                            | PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(_) =>  {
                                byte_time_sequence_d.get_invariant_checksum_payload_byte_length()
                            }
                        };
                        let temporal_position_v = byte_time_sequence_d.get_temporal_position_v();

                        debug!(
                            "of_data: byte_sequence_index: {:?}",
                            byte_time_sequence_d.get_byte_sequence_index()
                        );

                        debug!("of_data: pair_overlap_d: {:?}", pair_overlap_d);

                        Some((
                            *byte_time_sequence_index,
                            PairPolicyDataD::of_data(
                                payload_mode,
                                temporal_position_v,
                                payload_byte_length,
                                pair_overlap_d,
                                //payload_string_option,
                                &payload_ascii_v_option,
                                authorize_overlap_with_no_data,
                            ),
                        ))
                    }
                None => None
                })
            .collect();

        debug!("of_data: end");
        PairPolicyDataC { hm }
    }

    pub fn of_hm(hm: HashMap<TestIndex,PairPolicyDataD>) -> PairPolicyDataC {
        PairPolicyDataC { hm }
    }

    pub fn iter(&self) -> Iter<TestIndex, PairPolicyDataD> {
        self.hm.iter()
    }

    pub fn get(&self, test_index: &TestIndex) -> Option<&PairPolicyDataD> {
        self.hm.get(test_index)
    }
}
