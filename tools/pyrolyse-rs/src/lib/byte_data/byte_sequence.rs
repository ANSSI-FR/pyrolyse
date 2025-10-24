use itertools::izip;
use std::collections::hash_map::Iter;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::Iterator;
use std::path::Path;

use serde::{Deserialize, Serialize, Serializer};

use crate::misc::collections_utils;
use crate::misc::interval::IntervalC;
use crate::misc::test_index::TestIndex;
use crate::misc::sparq_wrapper;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::relation::relation_container::RelationContainer;
use crate::relation::relation_triplet::RelationTripletC;
use crate::relation::relation_triplet::RelationTripletD;

// TODO: add hole managment
/// Contains a sequence for two chunks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByteSequenceD<Rc> {
    /// Relation between the two chunks.
    // relation: AllenIntervalAlgebraRelation,
    rc: Rc,
    /// Intervals with byte index starting at 0.
    /// Used for data generation.
    base_interval_c: IntervalC,
    /// Intervals with incremental byte index, ie starting after the previous sequence.
    /// Used for data export and display.
    global_interval_c: IntervalC,
}

impl<Rc: Clone + Serialize + RelationContainer> ByteSequenceD<Rc> {
    pub fn new(
        // relation: AllenIntervalAlgebraRelation,
        rc: Rc,
        base_interval_c: IntervalC,
        global_interval_c: IntervalC,
    ) -> ByteSequenceD<Rc> {
        ByteSequenceD {
            rc,
            base_interval_c,
            global_interval_c,
        }
    }

    // pub fn get_relation(&self) -> &AllenIntervalAlgebraRelation {
    //     &self.relation
    // }

    pub fn get_rc(&self) -> &Rc {
        &self.rc
    }

    pub fn get_base_interval_c(&self) -> &IntervalC {
        &self.base_interval_c
    }

    pub fn get_global_interval_c(&self) -> &IntervalC {
        &self.global_interval_c
    }

    pub fn of_data(
        // relation: &AllenIntervalAlgebraRelation,
        rc: &Rc,
        interval_c: &IntervalC,
        index_offset: u16,
        data_offset: u16,
    ) -> ByteSequenceD<Rc> {
        ByteSequenceD::new(
            rc.clone(),
            (*interval_c).clone(),
            interval_c.apply_offset(index_offset, data_offset),
        )
    }

    pub fn build_sparq_constraint_string(&self, i_init: u32) -> String {
        self.rc.build_sparq_constraint_string(i_init)
    }

    pub fn get_relation_s(&self) -> String {
        self.rc.to_sparq_string()
    }

    pub fn get_csv_column_s_v(len: u32) -> Vec<String> {
        let relation_s_v = vec!["relation".to_string()];
        let base_interval_s_v = IntervalC::get_csv_column_s_v(len, "b");
        let global_interval_s_v = IntervalC::get_csv_column_s_v(len, "g");
        let v = vec![relation_s_v, base_interval_s_v, global_interval_s_v];
        v.into_iter().flatten().collect()
    }

    pub fn get_csv_data_s_v(&self, meta_index: u16) -> Vec<String> {
        let relation_s_v = vec![self.get_relation_s()];
        let base_interval_s_v = self.base_interval_c.get_csv_data_s_v(meta_index);
        let global_interval_s_v = self.global_interval_c.get_csv_data_s_v(meta_index);
        let v = vec![relation_s_v, base_interval_s_v, global_interval_s_v];
        v.into_iter().flatten().collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByteSequenceC<Rc: Serialize> {
    #[serde(serialize_with = "ordered_map")]
    hm: HashMap<TestIndex, ByteSequenceD<Rc>>,
}

fn ordered_map<Rc: Serialize, S>(
    value: &HashMap<TestIndex, ByteSequenceD<Rc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

impl<Rc: Serialize> FromIterator<(TestIndex, ByteSequenceD<Rc>)> for ByteSequenceC<Rc> {
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = (TestIndex, ByteSequenceD<Rc>)>,
    {
        Self {
            hm: HashMap::from_iter(iter),
        }
    }
}

impl<Rc: Debug + Clone + Serialize + RelationContainer> ByteSequenceC<Rc> {
    pub fn len(&self) -> usize {
        self.hm.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hm.is_empty()
    }

    pub fn get_interval_index_total_length(&self) -> u16 {
        (self.hm.len() as u16) * 2
    }

    pub fn get_interval_data_total_length(&self) -> u16 {
        self.hm
            .values()
            .map(|pair_sequence_d| pair_sequence_d.global_interval_c.get_total_length())
            .sum()
    }

    pub fn of_relation_pair_v(
        sparq_path: &Path,
        sequence_index_offset: u16,
        interval_index_offset: u16,
        interval_data_offset: u16,
        relation_v: &[AllenIntervalAlgebraRelation],
    ) -> ByteSequenceC<AllenIntervalAlgebraRelation> {
        debug!("PairSequenceC: of_relation_pair_v: start");

        // Quantify triplet/pair using SparQ
        let interval_c_v: Vec<IntervalC> = relation_v
            .iter()
            .map(|allen_interval_algebra_relation| {
                let constraint_s = allen_interval_algebra_relation.build_sparq_constraint_string(0);
                debug!("constraint_s: {}", constraint_s);
                let sparq_output_s =
                    sparq_wrapper::get_sparq_quantification(sparq_path, constraint_s);
                IntervalC::of_sparq_string(sparq_output_s)
            })
            .collect();
        debug!(
            "PairSequenceC: of_relation_pair_v: interval_c_v ({}): {:?}",
            interval_c_v.len(),
            interval_c_v
        );

        // Build interval total length
        let length_v: Vec<u16> = interval_c_v
            .iter()
            .map(|interval_c| interval_c.get_total_length())
            .collect();
        debug!(
            "PairSequenceC: of_relation_pair_v: length_v ({}): {:?}",
            length_v.len(),
            length_v
        );

        // Build cumulative total length
        let mut data_offset_v_tmp = length_v.iter().fold(vec![], |mut acc, length| {
            let offset: u16 = if acc.is_empty() {
                *length
            } else {
                acc[acc.len() - 1] + length
            };
            acc.append(&mut vec![offset]);
            acc
        });
        debug!(
            "PairSequenceC: of_relation_pair_v: data_offset_v_tmp ({}): {:?}",
            data_offset_v_tmp.len(),
            data_offset_v_tmp
        );
        // Adding 0 for initial null offset
        let mut data_offset_v = vec![0];
        data_offset_v.append(&mut data_offset_v_tmp);
        // Removing last offset
        data_offset_v.truncate(length_v.len());
        debug!(
            "PairSequenceC: of_relation_pair_v: data_offset_v ({}): {:?}",
            data_offset_v.len(),
            data_offset_v
        );

        let len = interval_c_v.len();
        debug!(
            "PairSequenceC: of_relation_pair_v: len-based range ({}): {:?}",
            (0..len - 1).len(),
            (0..len - 1)
        );

        // Build PairSequenceD using previously built offsets.
        let hm = izip!(relation_v.iter(), interval_c_v, 0..len, data_offset_v)
            .map(
                |(allen_interval_algebra_relation, interval_c, pair_index, data_offset)| {
                    let index_offset = pair_index * 2;
                    (
                        TestIndex(sequence_index_offset + pair_index as u16),
                        ByteSequenceD::of_data(
                            allen_interval_algebra_relation,
                            &interval_c,
                            interval_index_offset + index_offset as u16,
                            interval_data_offset + data_offset,
                        ),
                    )
                },
            )
            .collect();

        debug!("PairSequenceC: of_relation_pair_v: end");

        ByteSequenceC { hm }
    }

    pub fn of_relation_triplet_c(
        sparq_path: &Path,
        sequence_index_offset: u16,
        interval_index_offset: u16,
        interval_data_offset: u16,
        relation_triplet_c: &RelationTripletC,
    ) -> ByteSequenceC<RelationTripletD> {
        debug!("of_consistent_relation_triplet_pair: start");

        debug!(
            "of_consistent_relation_triplet_pair: relation_triplet_c length: {:?}",
            relation_triplet_c.len(),
        );

        // Quantify triplet/pair using SparQ
        let mut interval_c_v: Vec<(u16, IntervalC)> = relation_triplet_c
            .iter()
            .map(|(index, relation_triplet_d)| {
                let constraint_s = relation_triplet_d.build_sparq_constraint_string(0);
                debug!("constraint_s: {}", constraint_s);
                let sparq_output_s =
                    sparq_wrapper::get_sparq_quantification(sparq_path, constraint_s);
                (*index, IntervalC::of_sparq_string(sparq_output_s))
            })
            .collect();
        interval_c_v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        debug!(
            "of_consistent_relation_triplet_pair: interval_c_v ({}): {:?}",
            interval_c_v.len(),
            interval_c_v
        );

        // Build interval total length
        let length_v: Vec<u16> = interval_c_v
            .iter()
            .map(|(_index, interval_c)| interval_c.get_total_length())
            .collect();
        debug!(
            "of_consistent_relation_triplet_pair: length_v ({}): {:?}",
            length_v.len(),
            length_v
        );
        // let total_length = length_v.iter().sum();

        // Build offset
        let mut data_offset_v_tmp = length_v.iter().fold(vec![], |mut acc, length| {
            let offset: u16 = if acc.is_empty() {
                *length
            } else {
                acc[acc.len() - 1] + length
            };
            acc.append(&mut vec![offset]);
            acc
        });
        debug!(
            "of_consistent_relation_triplet_pair: data_offset_v_tmp ({}): {:?}",
            data_offset_v_tmp.len(),
            data_offset_v_tmp
        );
        // Adding 0 for initial null offset
        let mut data_offset_v = vec![0];
        data_offset_v.append(&mut data_offset_v_tmp);
        // Removing last offset
        debug!(
            "of_consistent_relation_triplet_pair: data_offset_v ({}): {:?}",
            data_offset_v.len(),
            data_offset_v
        );

        let len = interval_c_v.len();
        debug!(
            "of_consistent_relation_triplet_pair: len-based range ({}): {:?}",
            (0..len - 1).len(),
            (0..len - 1)
        );

        // Final
        let hm = izip!(interval_c_v, 0..len, data_offset_v)
            .map(|((index, interval_c), triplet_index, data_offset)| {
                let relation_triplet_d = relation_triplet_c.get(&index).unwrap();
                let index_offset = triplet_index * 3;
                (
                    TestIndex(sequence_index_offset + index),
                    ByteSequenceD::of_data(
                        relation_triplet_d,
                        &interval_c,
                        interval_index_offset + index_offset as u16,
                        interval_data_offset + data_offset,
                    ),
                )
            })
            .collect();

        debug!("of_consistent_relation_triplet_pair: end");

        ByteSequenceC { hm }
    }

    pub fn remove_duplicate(&self) -> ByteSequenceC<Rc> {
        debug!("remove_duplicate: start");

        let v: Vec<_> = self
            .hm
            .iter()
            .map(|(i, byte_pair_sequence_d)| {
                // Extract sorted tuple v for each interval_c
                debug!(
                    "remove_duplicate: i: {:?} - relation: {:?}",
                    i, byte_pair_sequence_d.rc
                );

                let mut start_end_tuple_v: Vec<_> = byte_pair_sequence_d
                    .base_interval_c
                    .iter()
                    .map(|(_index, interval_d)| (interval_d.get_start(), interval_d.get_end()))
                    .collect();
                debug!(
                    "remove_duplicate: start_end_tuple_v before sorting:\n{:?}",
                    start_end_tuple_v
                );
                start_end_tuple_v.sort_unstable();
                debug!(
                    "remove_duplicate: start_end_tuple_v after sorting:\n{:?}",
                    start_end_tuple_v
                );

                (*i, start_end_tuple_v)
            })
            .collect();

        // Group BytePairSequenceD index by sorted tuple
        let sorted_tuple_i_v_hm =
            collections_utils::group_iter_to_hm(v.iter(), &|(_i, v)| (*v).clone(), &|(i, _v)| *i);

        // Keep only one BytePairSequenceC (least inverse relation?)
        let sorted_tuple_i_hm: HashMap<_, TestIndex> = sorted_tuple_i_v_hm
            .into_iter()
            .map(|(tuple_v, i_v)| {
                debug!("remove_duplicate: {:?}: {:?}", tuple_v, i_v);
                // (tuple_v, i_v[0])

                let mut data_v: Vec<_> = i_v
                    .into_iter()
                    .map(|i| {
                        let rc = self.hm.get(&i).unwrap().rc.clone();
                        let relation_v = rc.to_v();
                        let relation_not_inverse_iter =
                            relation_v.iter().filter(|r| r.is_inverse());
                        let nb_not_inverse = relation_not_inverse_iter.count();
                        (i, nb_not_inverse)
                    })
                    .collect();
                data_v.sort_by_key(|(_, i)| *i);
                debug!("remove_duplicate: data_v: {:?}", data_v);

                (tuple_v, data_v[0].0)
            })
            .collect();

        let hm = sorted_tuple_i_hm
            .values()
            .map(|i| (*i, self.hm.get(i).unwrap().clone()))
            .collect();

        // TODO: reprocess global interval value

        debug!("remove_duplicate: end");

        ByteSequenceC { hm }
    }

    pub fn iter(&self) -> Iter<TestIndex, ByteSequenceD<Rc>> {
        self.hm.iter()
    }
}
