use itertools::Itertools;
use std::collections::hash_map::Iter;
use std::collections::{BTreeMap, HashMap};
use std::collections::hash_map::Keys;
use std::collections::hash_map::Values;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::Iterator;

use serde::{Deserialize, Serialize, Serializer};

use crate::byte_data::byte_sequence::ByteSequenceC;
use crate::byte_data::byte_sequence::ByteSequenceD;
use crate::byte_time_data::chunk::ChunkC;
use crate::byte_time_data::export_mode::ExportMode;
use crate::misc::interval::IntervalC;
use crate::misc::test_index::TestIndex;
//use crate::misc::invariant_checksum_chunk_pattern::InvariantChecksumChunkPatternC;
use crate::position::pattern::{PatternD,ChunkBasedPatternC};
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::relation::relation_container::RelationContainer;
use crate::relation::relation_triplet::RelationTripletD;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByteTimeSequenceD<Rc> {
    byte_sequence_index: TestIndex,
    rc: Rc,
    interval_c: IntervalC,
    chunk_c: ChunkC,
    temporal_position_v: Vec<u16>,
    simple_payload_byte_length: u16,
    internet_checksum_payload_byte_length: u16,
    invariant_checksum_payload_byte_length: u16,
}

impl<Rc: Clone + Serialize + RelationContainer> ByteTimeSequenceD<Rc> {
    pub fn new(
        byte_sequence_index: TestIndex,
        rc: Rc,
        interval_c: IntervalC,
        chunk_c: ChunkC,
        temporal_position_v: Vec<u16>,
        simple_payload_byte_length: u16,
        internet_checksum_payload_byte_length: u16,
        invariant_checksum_payload_byte_length: u16,
    ) -> ByteTimeSequenceD<Rc> {
        ByteTimeSequenceD {
            byte_sequence_index,
            rc,
            interval_c,
            chunk_c,
            temporal_position_v,
            simple_payload_byte_length,
            internet_checksum_payload_byte_length,
            invariant_checksum_payload_byte_length,
        }
    }

    pub fn get_byte_sequence_index(&self) -> TestIndex {
        self.byte_sequence_index
    }

    pub fn get_rc(&self) -> Rc {
        self.rc.clone()
    }

    pub fn get_relation_s(&self) -> String {
        self.rc.to_sparq_string()
    }

    pub fn get_chunk_c(&self) -> &ChunkC {
        &self.chunk_c
    }

    pub fn get_interval_c(&self) -> &IntervalC {
        &self.interval_c
    }

    pub fn get_simple_payload_byte_length(&self) -> u16 {
        self.simple_payload_byte_length
    }

    pub fn get_internet_checksum_payload_byte_length(&self) -> u16 {
        self.internet_checksum_payload_byte_length
    }

    pub fn get_invariant_checksum_payload_byte_length(&self) -> u16 {
        self.invariant_checksum_payload_byte_length
    }

    pub fn get_temporal_position_v(&self) -> &Vec<u16> {
        &self.temporal_position_v
    }

    // No union inside intervals_general
    // fn hole_present_intervals_general(interval_c: &IntervalC) -> bool {
    //     let interval_v = interval_c
    //         .iter()
    //         .map(|(_, interval_d)| {
    //             if interval_d.get_start() == interval_d.get_end() {
    //                 Interval::Singleton {
    //                     at: interval_d.get_start(),
    //                 }
    //             } else {
    //                 Interval::Closed {
    //                     bound_pair: BoundPair::new(
    //                         interval_d.get_start(),
    //                         interval_d.get_end(),
    //                     )
    //                     .ok_or("invalid BoundPair")
    //                     .unwrap(),
    //                 }
    //             }
    //         })
    //         .collect::<Vec<_>>();

    //         let union  =
    //         match interval_v.len() {
    //             2 => interval_v[0].union(interval_v[1]),
    //             3 => interval_v[0].union(interval_v[1]).union(interval_v[2])
    //             _ => panic!()
    //         };

    // }

    // union unimplemented inside portion
    // fn hole_present_portion(interval_c: &IntervalC) -> bool {
    //     let interval_v = interval_c
    //         .iter()
    //         .map(|(_, interval_d)| {
    //             if interval_d.get_start() == interval_d.get_end() {
    //                 Portion::singleton(interval_d.get_start())
    //             } else {
    //                 Portion::closed(interval_d.get_start(), interval_d.get_end())
    //             }
    //         })
    //         .collect::<Vec<_>>();

    //     let union = match interval_v.len() {
    //         2 => interval_v[0].union(interval_v[1]),
    //         3 => interval_v[0].union(interval_v[1]).union(interval_v[2]),
    //         _ => panic!(),
    //     };

    //     debug!("hole_present: union: {:?}", union);

    //     panic!("toto");

    //     true
    // }

    // pub fn of_data_pair__(
    //     sequence_index: u16,
    //     export_mode: &ExportMode,
    //     simple_chunk_pattern_sl: &[String],
    //     internet_checksum_chunk_pattern_sl: &[String],
    //     byte_sequence_d: &ByteSequenceD<AllenIntervalAlgebraRelation>,
    //     temporal_position_sl: &[u16],
    // ) -> ByteTimeSequenceD<AllenIntervalAlgebraRelation> {
    //     let interval_c = match export_mode {
    //         ExportMode::Isolated => byte_sequence_d.get_base_interval_c().clone(),
    //         ExportMode::Concatenated => byte_sequence_d.get_global_interval_c().clone(),
    //     };

    //     let chunk_c = ChunkC::of_data(
    //         simple_chunk_pattern_sl,
    //         internet_checksum_chunk_pattern_sl,
    //         &interval_c,
    //         temporal_position_sl,
    //     );

    //     let simple_payload_byte_length = interval_c.get_total_length();
    //     let internet_checksum_payload_byte_length = interval_c.get_total_length() * 8;

    //     // let hole_present = hole_present(interval_c);

    //     ByteTimeSequenceD::new(
    //         sequence_index,
    //         byte_sequence_d.get_rc().to_v()[0].clone(),
    //         interval_c,
    //         chunk_c,
    //         temporal_position_sl.to_vec(),
    //         simple_payload_byte_length,
    //         internet_checksum_payload_byte_length,
    //     )
    // }

    // pub fn of_data_triplet__(
    //     sequence_index: u16,
    //     export_mode: &ExportMode,
    //     simple_chunk_pattern_sl: &[String],
    //     internet_checksum_chunk_pattern_sl: &[String],
    //     byte_triplet_sequence_d: &ByteSequenceD<RelationTripletD>,
    //     temporal_position_sl: &[u16],
    // ) -> ByteTimeSequenceD<RelationTripletD> {
    //     let interval_c = match export_mode {
    //         ExportMode::Isolated => byte_triplet_sequence_d.get_base_interval_c().clone(),
    //         ExportMode::Concatenated => byte_triplet_sequence_d.get_global_interval_c().clone(),
    //     };

    //     let chunk_c = ChunkC::of_data(
    //         simple_chunk_pattern_sl,
    //         internet_checksum_chunk_pattern_sl,
    //         &interval_c,
    //         temporal_position_sl,
    //     );

    //     let simple_payload_byte_length = interval_c.get_total_length();
    //     let internet_checksum_payload_byte_length = interval_c.get_total_length() * 8;

    //     ByteTimeSequenceD::new(
    //         sequence_index,
    //         // byte_triplet_sequence_d.get_rc().get_relation_01().clone(),
    //         // byte_triplet_sequence_d.get_rc().get_relation_02().clone(),
    //         // byte_triplet_sequence_d.get_rc().get_relation_12().clone(),
    //         (*byte_triplet_sequence_d.get_rc()).clone(),
    //         interval_c,
    //         chunk_c,
    //         temporal_position_sl.to_vec(),
    //         simple_payload_byte_length,
    //         internet_checksum_payload_byte_length,
    //     )
    // }

    pub fn of_data(
        sequence_index: TestIndex,
        export_mode: &ExportMode,
        simple_chunk_pattern_sl: &PatternD,
        internet_checksum_chunk_pattern_sl: &PatternD,
        ipv4_invariant_checksum_chunk_pattern_c: &ChunkBasedPatternC,
        ipv6_invariant_checksum_chunk_pattern_c: &ChunkBasedPatternC,
        byte_sequence_d_triplet: &ByteSequenceD<Rc>,
        temporal_position_sl: &[u16],
    ) -> ByteTimeSequenceD<Rc> {
        let interval_c = match export_mode {
            ExportMode::Isolated => byte_sequence_d_triplet.get_base_interval_c().clone(),
            ExportMode::Concatenated => byte_sequence_d_triplet.get_global_interval_c().clone(),
        };

        let chunk_c = ChunkC::of_data(
            simple_chunk_pattern_sl,
            internet_checksum_chunk_pattern_sl,
            ipv4_invariant_checksum_chunk_pattern_c,
            ipv6_invariant_checksum_chunk_pattern_c,
            &interval_c,
            //temporal_position_sl,
        );

        let simple_payload_byte_length = interval_c.get_total_length();
        let internet_checksum_payload_byte_length = interval_c.get_total_length() * 8;
        let invariant_checksum_payload_byte_length = interval_c.get_total_length() * 8;

        ByteTimeSequenceD::new(
            sequence_index,
            // byte_triplet_sequence_d.get_rc().get_relation_01().clone(),
            // byte_triplet_sequence_d.get_rc().get_relation_02().clone(),
            // byte_triplet_sequence_d.get_rc().get_relation_12().clone(),
            (*byte_sequence_d_triplet.get_rc()).clone(),
            interval_c,
            chunk_c,
            temporal_position_sl.to_vec(),
            simple_payload_byte_length,
            internet_checksum_payload_byte_length,
            invariant_checksum_payload_byte_length,
        )
    }

    pub fn build_sparq_constraint_string(&self, i_init: u32) -> String {
        // format!(
        //     "(i{} {} i{})",
        //     i_init,
        //     self.byte_relation.to_sparq_string(),
        //     i_init + 2,
        // )
        self.rc.build_sparq_constraint_string(i_init)
    }

    // pub fn newer_include_older(&self) -> bool {
    //     debug!("newer_include_older: start");
    //     let chunk_0_temporal_position = self.temporal_position_v.get(0).unwrap();
    //     let chunk_1_temporal_position = self.temporal_position_v.get(1).unwrap();

    //     debug!(
    //         "newer_include_older: chunk_1_temporal_position: {}",
    //         chunk_1_temporal_position
    //     );

    //     let interval_0_duration = self.interval_c.get(0).unwrap().get_duration();
    //     let interval_1_duration = self.interval_c.get(1).unwrap().get_duration();

    //     let interval_0_included_in_interval_1_ok = if self.byte_relation.is_inclusion_01() {
    //         debug!("newer_include_older: self: {:#?}", self);
    //         assert!(interval_0_duration <= interval_1_duration);
    //         chunk_0_temporal_position < chunk_1_temporal_position
    //     } else {
    //         true
    //     };

    //     debug!("newer_include_older: end");
    //     interval_0_included_in_interval_1_ok
    // }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByteTimeSequenceC<Rc: Serialize> {
    #[serde(serialize_with = "ordered_map")]
    hm: HashMap<TestIndex, ByteTimeSequenceD<Rc>>,
}

fn ordered_map<Rc: Serialize, S>(
    value: &HashMap<TestIndex, ByteTimeSequenceD<Rc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

impl<Rc: Serialize> FromIterator<(TestIndex, ByteTimeSequenceD<Rc>)> for ByteTimeSequenceC<Rc> {
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = (TestIndex, ByteTimeSequenceD<Rc>)>,
    {
        Self {
            hm: HashMap::from_iter(iter),
        }
    }
}

impl<Rc: Serialize> ByteTimeSequenceC<Rc> {
    pub fn len(&self) -> usize {
        self.hm.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hm.is_empty()
    }

    pub fn get(&self, index: &TestIndex) -> Option<&ByteTimeSequenceD<Rc>> {
        self.hm.get(index)
    }

    pub fn keys(&self) -> Keys<TestIndex, ByteTimeSequenceD<Rc>> {
        self.hm.keys()
    }

    pub fn values(&self) -> Values<TestIndex, ByteTimeSequenceD<Rc>> {
        self.hm.values()
    }

    pub fn of_data_pair(
        export_mode: &ExportMode,
        simple_chunk_pattern_sl: &PatternD,
        internet_checksum_chunk_pattern_sl: &PatternD,
        ipv4_invariant_checksum_chunk_pattern_c: &ChunkBasedPatternC,
        ipv6_invariant_checksum_chunk_pattern_c: &ChunkBasedPatternC,
        temporal_shuffling: bool,
        // byte_pair_sequence_c: &BytePairSequenceC,
        byte_sequence_c_pair: &ByteSequenceC<AllenIntervalAlgebraRelation>,
    ) -> ByteTimeSequenceC<AllenIntervalAlgebraRelation> {
        debug!("of_data: start");

        debug!("of_data: temporal_shuffling: {}", temporal_shuffling);

        // Build possible permutation for 2 chunks.
        debug!("of_data: building position permutation");
        let temporal_position_v_v = if temporal_shuffling {
            std::ops::Range { start: 0, end: 2 }
                .permutations(2)
                .collect::<Vec<Vec<u16>>>()
        } else {
            vec![vec![0, 1]]
        };
        debug!(
            "of_data: temporal_position_v_v: {:?}",
            temporal_position_v_v
        );

        let mut tuple_v: Vec<(&TestIndex, &ByteSequenceD<AllenIntervalAlgebraRelation>)> =
            byte_sequence_c_pair.iter().collect();
        tuple_v.sort_by_key(|(k, _v)| *k);

        debug!("of_data: building byte_time_pair_sequence_v_v");
        let byte_time_pair_sequence_v_v = tuple_v
            .iter()
            .map(
                |(byte_sequence_index, byte_sequence_d): &(
                    &TestIndex,
                    &ByteSequenceD<AllenIntervalAlgebraRelation>,
                )| {
                    debug!("of_data: byte_sequence_index: {:?}", byte_sequence_index);

                    // For each pair sequence, we generate chunks for all possible temporal
                    // positions.
                    let byte_time_pair_sequence_map = temporal_position_v_v
                        .iter()
                        .map(|temporal_position_v| {
                            ByteTimeSequenceD::<AllenIntervalAlgebraRelation>::of_data(
                                **byte_sequence_index,
                                export_mode,
                                simple_chunk_pattern_sl,
                                internet_checksum_chunk_pattern_sl,
                                &ipv4_invariant_checksum_chunk_pattern_c,
                                &ipv6_invariant_checksum_chunk_pattern_c,
                                byte_sequence_d,
                                temporal_position_v,
                            )
                        })
                        .collect();

                    // TODO: remove this code
                    // // Discard PairChunkD when an older chunk is included
                    // // (ie during/starts/finishes/equals) in a new one because an OS will probably not
                    // // consider the new one at all.
                    // // New and old are here related to temporal position.
                    // // Example: D relation with temporal position 10 is invalid because chunk 0
                    // // will be during chunk 1, and thus, chunk 0 will (probably) never be considered
                    // // during OS reassembly because its temporal position is after chunk 1.
                    // byte_time_pair_sequence_map
                    //     .into_iter()
                    //     .filter(|byte_time_pair_sequence| byte_time_pair_sequence.newer_include_older())
                    //     .collect()

                    byte_time_pair_sequence_map
                },
            )
            .collect::<Vec<Vec<ByteTimeSequenceD<AllenIntervalAlgebraRelation>>>>();

        debug!("of_data: building byte_time_pair_sequence_v");
        let byte_time_pair_sequence_v = byte_time_pair_sequence_v_v.iter().flatten();

        debug!("of_data: building hm");
        let hm = byte_time_pair_sequence_v
            .enumerate()
            .map(|(i, d)| {
                let u = i as u16;
                let test_index = TestIndex(u);
                (test_index, (*d).clone())
            })
            .collect::<HashMap<_, _>>();

        debug!(
            "of_data: length reduced from {} to {} using time/inclusion criteria",
            tuple_v.len() * temporal_position_v_v.len(),
            hm.len()
        );

        debug!("of_data: end");
        ByteTimeSequenceC { hm }
    }

    pub fn of_data_triplet(
        chunk_index_offset: u16,
        export_mode: &ExportMode,
        simple_chunk_pattern_sl: &PatternD,
        internet_checksum_chunk_pattern_sl: &PatternD,
        ipv4_invariant_checksum_chunk_pattern_c: &ChunkBasedPatternC,
        ipv6_invariant_checksum_chunk_pattern_c: &ChunkBasedPatternC,
        temporal_shuffling: bool,
        byte_sequence_c_triplet: &ByteSequenceC<RelationTripletD>,
    ) -> ByteTimeSequenceC<RelationTripletD> {
        debug!("of_data: start");

        // Build possible permutation for 3 chunks.
        debug!("of_data: building position permutation");
        let temporal_position_v_v = if temporal_shuffling {
            std::ops::Range { start: 0, end: 3 }
                .permutations(3)
                .collect::<Vec<Vec<u16>>>()
        } else {
            vec![vec![0, 1, 2]]
        };
        debug!(
            "of_data: temporal_position_v_v: {:?}",
            temporal_position_v_v
        );

        let mut tuple_v: Vec<(&TestIndex, &ByteSequenceD<RelationTripletD>)> =
            byte_sequence_c_triplet.iter().collect();
        tuple_v.sort_by_key(|(k, _v)| *k);

        debug!("of_data: building byte_time_triplet_sequence_v_v");
        let byte_time_triplet_sequence_v_v = tuple_v
            .iter()
            .map(|(sequence_index, triplet_sequence_d)| {
                debug!("of_data: sequence_index: {:?}", sequence_index);

                // For each triplet sequence, we generate chunks for all possible temporal
                // positions.
                let byte_time_triplet_sequence_d_map = temporal_position_v_v
                    .iter()
                    .map(|temporal_position_v| {
                        ByteTimeSequenceD::<RelationTripletD>::of_data(
                            **sequence_index,
                            export_mode,
                            simple_chunk_pattern_sl,
                            internet_checksum_chunk_pattern_sl,
                            &ipv4_invariant_checksum_chunk_pattern_c,
                            &ipv6_invariant_checksum_chunk_pattern_c,
                            triplet_sequence_d,
                            temporal_position_v,
                        )
                    })
                    .collect::<Vec<ByteTimeSequenceD<RelationTripletD>>>();

                // TODO: remove this code
                // Discard TripletChunkD when an newer chunk is included
                // (ie during/starts/finishes/equals) in an old one because an OS will probably not
                // consider the new one at all.
                // New and old are here related to temporal position.
                // Example: (B) D D triplet with temporal position 021 is invalid because chunk 1
                // will be during chunk 2, and thus, chunk 1 will (probably) never be considered
                // during OS reassembly because its temporal position is after chunk 2.
                // byte_time_triplet_sequence_d_map
                //     .into_iter()
                //     .filter(|byte_time_triplet_sequence_d| {
                //         byte_time_triplet_sequence_d.newer_include_older()
                //     })
                //     .collect()

                byte_time_triplet_sequence_d_map
            })
            .collect::<Vec<Vec<ByteTimeSequenceD<RelationTripletD>>>>();

        debug!("of_data: building byte_time_triplet_sequence_v");
        let byte_time_triplet_sequence_v = byte_time_triplet_sequence_v_v.iter().flatten();

        debug!("of_data: building hm");
        let hm = byte_time_triplet_sequence_v
            .enumerate()
            .map(|(i, d)| (TestIndex(chunk_index_offset + i as u16), (*d).clone()))
            .collect::<HashMap<_, _>>();

        debug!(
            "of_data: length reduced from {} to {} using time/inclusion criteria",
            tuple_v.len() * temporal_position_v_v.len(),
            hm.len()
        );

        debug!("of_data: end");
        ByteTimeSequenceC { hm }
    }

    pub fn iter(&self) -> Iter<TestIndex, ByteTimeSequenceD<Rc>> {
        self.hm.iter()
    }
}
