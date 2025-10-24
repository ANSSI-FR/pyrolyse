use std::collections::hash_map::Iter;
use std::collections::{BTreeMap, HashMap};
use std::collections::hash_map::Keys;
use std::collections::hash_map::Values;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::Iterator;
use std::str;

use serde::{Deserialize, Serialize, Serializer};

use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceC;
use crate::misc::test_index::TestIndex;
use crate::position::payload_mode::PayloadMode;
use crate::position::triplet_position_data::TripletPositionDataC;
use crate::position::triplet_position_data::TripletPositionDataD;
use crate::relation::relation_triplet::RelationTripletD;
use crate::reply_payload::reply_payload::ReplyPayloadC;
use crate::position::triplet_position::TripletPosition;
use crate::policy_common::time_policy_triplet_residual_pair::TimePolicyTripletResidualPair;
use crate::policy_common::triplet_policy::TripletPolicy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripletPolicyDataD {
    byte_sequence_index: TestIndex,

    relation_triplet: RelationTripletD,

    time_policy_triplet_triple: TripletPolicy,

    // TODO: add answer policy: None, UntilHole, All

    // Between 0 and 1
    time_policy_triplet_residual_01: TimePolicyTripletResidualPair,

    // Between 0 and 2
    time_policy_triplet_residual_02: TimePolicyTripletResidualPair,

    // Between 1 and 2
    time_policy_triplet_residual_12: TimePolicyTripletResidualPair,

    payload_length_correctness: bool,
}

impl TripletPolicyDataD {
    pub fn new(
        byte_sequence_index: TestIndex,

        relation_triplet: RelationTripletD,

        time_policy_triplet_triple: TripletPolicy,

        time_policy_triplet_residual_01: TimePolicyTripletResidualPair,
        time_policy_triplet_residual_02: TimePolicyTripletResidualPair,
        time_policy_triplet_residual_12: TimePolicyTripletResidualPair,

        payload_length_correctness: bool,
    ) -> TripletPolicyDataD {
        TripletPolicyDataD {
            byte_sequence_index,

            relation_triplet,

            time_policy_triplet_triple,

            time_policy_triplet_residual_01,
            time_policy_triplet_residual_02,
            time_policy_triplet_residual_12,

            payload_length_correctness,
        }
    }

    pub fn get_byte_sequence_index(&self) -> &TestIndex {
        &self.byte_sequence_index
    }

    pub fn get_time_policy_triplet_residual_01(&self) -> &TimePolicyTripletResidualPair {
        &self.time_policy_triplet_residual_01
    }

    pub fn get_time_policy_triplet_residual_02(&self) -> &TimePolicyTripletResidualPair {
        &self.time_policy_triplet_residual_02
    }

    pub fn get_time_policy_triplet_residual_12(&self) -> &TimePolicyTripletResidualPair {
        &self.time_policy_triplet_residual_12
    }

    pub fn get_payload_length_correctness(&self) -> bool {
        self.payload_length_correctness
    }


    fn of_data(
        payload_mode: &PayloadMode,
        temporal_position_v: &[u16],
        payload_byte_length: u16,
        triplet_position_data_d: &TripletPositionDataD,
        payload_string_option: &Option<Vec<u8>>,
        authorize_overlap_with_no_data: bool,
    ) -> TripletPolicyDataD {
        debug!("of_data: start");

        debug!("of_data: building triplet_position_policy_012");
        let triplet_position_012 = triplet_position_data_d.get_triplet_position_012();

        let temporal_position_0 = temporal_position_v.first().unwrap();
        let temporal_position_1 = temporal_position_v.get(1).unwrap();
        let temporal_position_2 = temporal_position_v.get(2).unwrap();
        let temporal_position_v = vec![
            *temporal_position_0,
            *temporal_position_1,
            *temporal_position_2,
        ];

        let (relation_triplet, time_policy_triplet_triple) = match triplet_position_012 {
            TripletPosition::Disjoint(triplet_relation_d) => {
                ((*triplet_relation_d).clone(), TripletPolicy::None)
            }
            TripletPosition::Overlap(triplet_overlap) => {
                let triplet_relation_d = triplet_overlap.get_relation_triplet_d();

                let time_policy_triplet_triple = TripletPolicy::of_overlap_payload(
                    payload_mode,
                    triplet_overlap,
                    temporal_position_v.clone(),
                    payload_string_option,
                );

                ((*triplet_relation_d).clone(), time_policy_triplet_triple)
            }
        };

        let triplet_pair_position_data_01 =
            triplet_position_data_d.get_triplet_pair_position_data_01();
        let triplet_pair_position_data_02 =
            triplet_position_data_d.get_triplet_pair_position_data_02();
        let triplet_pair_position_data_12 =
            triplet_position_data_d.get_triplet_pair_position_data_12();

        let temporal_position_0 = temporal_position_v.first().unwrap();
        let temporal_position_1 = temporal_position_v.get(1).unwrap();
        let temporal_position_2 = temporal_position_v.get(2).unwrap();

        debug!("of_data: building time_policy_triplet_residual_01");
        // 01
        let time_policy_triplet_residual_01 = TimePolicyTripletResidualPair::of_data(
            payload_mode,
            temporal_position_0,
            temporal_position_1,
            triplet_pair_position_data_01,
            payload_string_option,
            authorize_overlap_with_no_data,
        );

        debug!("of_data: building time_policy_triplet_residual_02");
        // 02
        let time_policy_triplet_residual_02 = TimePolicyTripletResidualPair::of_data(
            payload_mode,
            temporal_position_0,
            temporal_position_2,
            triplet_pair_position_data_02,
            payload_string_option,
            authorize_overlap_with_no_data,
        );

        debug!("of_data: building time_policy_triplet_residual_12");
        // 12
        let time_policy_triplet_residual_12 = TimePolicyTripletResidualPair::of_data(
            payload_mode,
            temporal_position_1,
            temporal_position_2,
            triplet_pair_position_data_12,
            payload_string_option,
            authorize_overlap_with_no_data,
        );

        let payload_length_correctness = match payload_string_option {
            None => false,
            Some(s) => payload_byte_length == s.len() as u16,
        };

        debug!("of_data: end");

        TripletPolicyDataD::new(
            triplet_position_data_d.get_byte_sequence_index(),
            relation_triplet,
            time_policy_triplet_triple,
            time_policy_triplet_residual_01,
            time_policy_triplet_residual_02,
            time_policy_triplet_residual_12,
            payload_length_correctness,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripletPolicyDataC {
    #[serde(serialize_with = "ordered_map")]
    hm: HashMap<TestIndex, TripletPolicyDataD>,
}

fn ordered_map<S>(
    value: &HashMap<TestIndex, TripletPolicyDataD>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

impl FromIterator<(TestIndex, TripletPolicyDataD)> for TripletPolicyDataC {
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = (TestIndex, TripletPolicyDataD)>,
    {
        Self {
            hm: HashMap::from_iter(iter),
        }
    }
}

impl TripletPolicyDataC {
    pub fn len(&self) -> usize {
        self.hm.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hm.is_empty()
    }

    pub fn get_index_total_length(&self) -> u32 {
        (self.hm.len() as u32) * 3
    }

    pub fn iter(&self) -> Iter<TestIndex, TripletPolicyDataD> {
        self.hm.iter()
    }

    pub fn get(&self,test_index: &TestIndex) -> Option<&TripletPolicyDataD> {
        self.hm.get(test_index)
    }

    pub fn keys(&self) -> Keys<TestIndex, TripletPolicyDataD> {
        self.hm.keys()
    }

    pub fn values(&self) -> Values<TestIndex, TripletPolicyDataD> {
        self.hm.values()
    }

    pub fn of_data(
        payload_mode: &PayloadMode,
        byte_time_sequence_c: &ByteTimeSequenceC<RelationTripletD>,
        triplet_position_data_c: &TripletPositionDataC,
        reply_payload_c: &ReplyPayloadC,
        authorize_overlap_with_no_data: bool,
    ) -> TripletPolicyDataC {
        debug!("of_data: start");

        let hm: HashMap<_,_> = triplet_position_data_c
            .iter()
            //.map(|(byte_time_sequence_index, pair_overlap_d)| {
            .filter_map(|(byte_time_sequence_index, triplet_position_data_d)| 
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

                        Some((
                            *byte_time_sequence_index,
                            TripletPolicyDataD::of_data(
                                payload_mode,
                                temporal_position_v,
                                payload_byte_length,
                                triplet_position_data_d,
                                //&payload_string_option,
                                &payload_ascii_v_option,
                                authorize_overlap_with_no_data,
                            ),
                        ))
                    }
                None => None
                })
            .collect();

        debug!("of_data: end");

        TripletPolicyDataC { hm }
    }

    pub fn of_hm(
        hm: HashMap<TestIndex,TripletPolicyDataD> 
    ) -> TripletPolicyDataC {
       TripletPolicyDataC { hm }
    }
}
