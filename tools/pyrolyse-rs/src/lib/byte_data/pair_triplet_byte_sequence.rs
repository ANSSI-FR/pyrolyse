use std::collections::HashMap;
use std::iter::Iterator;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::byte_data::byte_sequence::ByteSequenceC;
use crate::misc::file_utils;
use crate::misc::interval::IntervalD;
use crate::misc::test_index::TestIndex;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::relation::consistent_relation_triplet_pair::ConsistentRelationTripletPair;
use crate::relation::relation_triplet::RelationTripletD;

/// Contains sequence for pair and triplet.
/// This only represents spatial/data location/relation.
/// This does NOT represents temporal location/relation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairTripletByteSequence {
    byte_pair_sequence_c: ByteSequenceC<AllenIntervalAlgebraRelation>,
    byte_triplet_sequence_c: ByteSequenceC<RelationTripletD>,
}

impl PairTripletByteSequence {
    pub fn new(
        byte_pair_sequence_c: ByteSequenceC<AllenIntervalAlgebraRelation>,
        byte_triplet_sequence_c: ByteSequenceC<RelationTripletD>,
    ) -> PairTripletByteSequence {
        PairTripletByteSequence {
            byte_pair_sequence_c,
            byte_triplet_sequence_c,
        }
    }

    pub fn get_byte_pair_sequence_c(&self) -> &ByteSequenceC<AllenIntervalAlgebraRelation> {
        &self.byte_pair_sequence_c
    }

    pub fn get_byte_triplet_sequence_c(&self) -> &ByteSequenceC<RelationTripletD> {
        &self.byte_triplet_sequence_c
    }

    /// Returns a relation triplet (3 relations) container from a vector of vector of 2 relations.
    /// A relation triplet completely defines the position of 3 intervals/segments/chunks.
    ///
    /// # Arguments
    ///
    /// * `v` - A vector or vector of relation.
    pub fn of_consistent_relation_triplet_pair(
        sparq_path: &Path,
        consistent_relation_triplet_pair: ConsistentRelationTripletPair,
    ) -> PairTripletByteSequence {
        debug!("of_consistent_relation_triplet_pair: start");

        let sequence_index_offset = 0;
        let interval_index_offset = 0;
        let interval_data_offset = 0;
        let relation_pair_v = consistent_relation_triplet_pair.get_relation_pair_v();
        let pair_sequence_c: ByteSequenceC<AllenIntervalAlgebraRelation> =
            ByteSequenceC::<AllenIntervalAlgebraRelation>::of_relation_pair_v(
                sparq_path,
                sequence_index_offset,
                interval_index_offset,
                interval_data_offset,
                relation_pair_v,
            );

        // Build index offset for triplet interval generation.
        // let sequence_index_offset = pair_sequence_c.len() as u16;
        let sequence_index_offset = 100_u16;
        let interval_index_offset = pair_sequence_c.get_interval_index_total_length();
        debug!(
            "of_consistent_relation_triplet_pair: pair_sequence_c interval_index_offset: {}",
            interval_index_offset
        );
        let interval_data_offset = pair_sequence_c.get_interval_data_total_length();
        debug!(
            "of_consistent_relation_triplet_pair: pair_sequence_c interval_data_offset: {}",
            interval_data_offset
        );

        let relation_triplet_c = consistent_relation_triplet_pair.get_relation_triplet_c();
        let triplet_sequence_c = ByteSequenceC::<RelationTripletD>::of_relation_triplet_c(
            sparq_path,
            sequence_index_offset,
            interval_index_offset,
            interval_data_offset,
            relation_triplet_c,
        );

        debug!("of_consistent_relation_triplet_pair: end");
        PairTripletByteSequence::new(pair_sequence_c, triplet_sequence_c)
    }

    pub fn remove_duplicate(&self) -> PairTripletByteSequence {
        let pair_sequence_c_wo_duplicate =
            ByteSequenceC::remove_duplicate(&self.byte_pair_sequence_c);
        let triplet_sequence_c_wo_duplicate =
            ByteSequenceC::remove_duplicate(&self.byte_triplet_sequence_c);

        PairTripletByteSequence::new(
            pair_sequence_c_wo_duplicate,
            triplet_sequence_c_wo_duplicate,
        )
    }

    pub fn export_csv(&self, file_path: &Path) {
        let triplet_interval_v_hm: HashMap<TestIndex, Vec<(usize, (&u16, &IntervalD))>> = self
            .byte_triplet_sequence_c
            .iter()
            .map(|(index, triplet_sequence_d)| {
                (
                    *index,
                    triplet_sequence_d
                        .get_global_interval_c()
                        .iter()
                        .map(|(index, interval_d)| (index, interval_d))
                        .enumerate()
                        .collect(),
                )
            })
            .collect();
        let triplet_interval_v: Vec<(TestIndex, usize, u16, IntervalD)> = triplet_interval_v_hm
            .iter()
            .flat_map(|(meta_index, v)| {
                let v: Vec<(TestIndex, usize, u16, IntervalD)> = v
                    .iter()
                    .map(|(interval_index, (index, interval))| {
                        (*meta_index, *interval_index, **index, (**interval).clone())
                    })
                    .collect();
                v
            })
            .collect();

        let pair_interval_v_hm: HashMap<TestIndex, Vec<(usize, (&u16, &IntervalD))>> = self
            .byte_pair_sequence_c
            .iter()
            .map(|(index, pair_sequence_d)| {
                (
                    *index,
                    pair_sequence_d
                        .get_global_interval_c()
                        .iter()
                        .map(|(index, interval_d)| (index, interval_d))
                        .enumerate()
                        .collect(),
                )
            })
            .collect();
        let pair_interval_v: Vec<(TestIndex, usize, u16, IntervalD)> = pair_interval_v_hm
            .iter()
            .flat_map(|(meta_index, v)| {
                let v: Vec<(TestIndex, usize, u16, IntervalD)> = v
                    .iter()
                    .map(|(interval_index, (index, interval))| {
                        (*meta_index, *interval_index, **index, (**interval).clone())
                    })
                    .collect();
                v
            })
            .collect();

        // Build an interval vector with all intervals (from pairs and triplets).
        let mut interval_v: Vec<(TestIndex, usize, u16, IntervalD)> = vec![];
        interval_v.extend(triplet_interval_v);
        interval_v.extend(pair_interval_v);
        interval_v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        debug!("\n\n\nexport_csv: interval_v: {:?}", interval_v);

        let mut wtr = csv::Writer::from_writer(vec![]);
        for (meta_index, interval_index, index, interval) in interval_v.iter() {
            let mut v = interval.get_csv_data_s_v(meta_index.0, *index);
            v.push(format!("{:?}", interval_index));

            wtr.write_record(v)
                .expect("could not write vector to writer");
        }

        let v = wtr.into_inner().expect("could not get v out of Writer");
        let s = String::from_utf8(v).expect("could not get string out of vector");

        // TODO: add option for index
        let mut csv_column_s_v = IntervalD::get_csv_column_s_v(0, "");
        csv_column_s_v.push("y_pos".to_string());

        let s = format!("{}\n{}", csv_column_s_v.join(","), s);

        file_utils::write_in_file_path(file_path, &s);
    }
}
