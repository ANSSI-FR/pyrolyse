use std::collections::hash_map::Iter;
use std::collections::{BTreeMap, HashMap};
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::Iterator;

use serde::{Deserialize, Serialize, Serializer};

use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::relation::relation_container::RelationContainer;

// TODO: change chunk index: 1 <-> 2
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct RelationTripletD {
    /// Relation between chunk 0 and 1
    relation_01: AllenIntervalAlgebraRelation,
    /// Relation between chunk 0 and 2
    relation_02: AllenIntervalAlgebraRelation,
    /// Relation between chunk 1 and 2
    relation_12: AllenIntervalAlgebraRelation,
}

impl RelationContainer for RelationTripletD {
    fn to_sparq_string(&self) -> String {
        // match self {
        //     AllenIntervalAlgebraRelation::Eq => "eq".to_string(),
        //     AllenIntervalAlgebraRelation::B => "b".to_string(),
        //     AllenIntervalAlgebraRelation::Bi => "bi".to_string(),
        //     AllenIntervalAlgebraRelation::O => "o".to_string(),
        //     AllenIntervalAlgebraRelation::Oi => "oi".to_string(),
        //     AllenIntervalAlgebraRelation::S => "s".to_string(),
        //     AllenIntervalAlgebraRelation::Si => "si".to_string(),
        //     AllenIntervalAlgebraRelation::D => "d".to_string(),
        //     AllenIntervalAlgebraRelation::Di => "di".to_string(),
        //     AllenIntervalAlgebraRelation::F => "f".to_string(),
        //     AllenIntervalAlgebraRelation::Fi => "fi".to_string(),
        // }
        format!(
            "{:?} {:?} {:?}",
            self.relation_01, self.relation_02, self.relation_12
        )
    }

    // fn build_sparq_constraint_string(&self, i_init: u32) -> String {
    //     format!(
    //         "(i{} {} i{}) (i{} {} i{}) (i{} {} i{})",
    //         i_init,
    //         self.relation_01.to_sparq_string(),
    //         i_init + 1,
    //         i_init,
    //         self.relation_02.to_sparq_string(),
    //         i_init + 2,
    //         i_init + 1,
    //         self.relation_12.to_sparq_string(),
    //         i_init + 2
    //     )
    // }
    fn build_sparq_constraint_string(&self, i_init: u32) -> String {
        format!(
            "((i{} {} i{}) (i{} {} i{}) (i{} {} i{}))",
            i_init,
            self.relation_01.to_sparq_string(),
            i_init + 1,
            i_init,
            self.relation_02.to_sparq_string(),
            i_init + 2,
            i_init + 1,
            self.relation_12.to_sparq_string(),
            i_init + 2
        )
    }

    fn to_v(&self) -> Vec<AllenIntervalAlgebraRelation> {
        vec![
            (self.relation_01).clone(),
            (self.relation_02).clone(),
            (self.relation_12).clone(),
        ]
    }
}

impl RelationTripletD {
    pub fn new(
        relation_01: AllenIntervalAlgebraRelation,
        relation_02: AllenIntervalAlgebraRelation,
        relation_12: AllenIntervalAlgebraRelation,
    ) -> RelationTripletD {
        RelationTripletD {
            relation_01,
            relation_02,
            relation_12,
        }
    }

    pub fn get_relation_01(&self) -> &AllenIntervalAlgebraRelation {
        &self.relation_01
    }

    pub fn get_relation_02(&self) -> &AllenIntervalAlgebraRelation {
        &self.relation_02
    }

    pub fn get_relation_12(&self) -> &AllenIntervalAlgebraRelation {
        &self.relation_12
    }

    pub fn is_peo_like(&self) -> bool {
        let relation_02 = self.get_relation_02();
        let relation_12 = self.get_relation_12();

        let _2_is_next_to_0_or_1 = *relation_02 == AllenIntervalAlgebraRelation::Mi
            || *relation_12 == AllenIntervalAlgebraRelation::Mi;

        let _0_and_1_are_after_2 = (*relation_02 == AllenIntervalAlgebraRelation::Mi
            || *relation_02 == AllenIntervalAlgebraRelation::Bi)
            && (*relation_12 == AllenIntervalAlgebraRelation::Mi
                || *relation_12 == AllenIntervalAlgebraRelation::Bi);

        // let _2_does_not_overlap_with_0_or_1 =
        //     !relation_02.is_overlap() && !relation_12.is_overlap();

        _2_is_next_to_0_or_1 && _0_and_1_are_after_2
        //  && _2_does_not_overlap_with_0_or_1
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationTripletC {
    #[serde(serialize_with = "ordered_map")]
    hm: HashMap<u16, RelationTripletD>,
}

fn ordered_map<S>(value: &HashMap<u16, RelationTripletD>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

impl FromIterator<(u16, RelationTripletD)> for RelationTripletC {
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = (u16, RelationTripletD)>,
    {
        Self {
            hm: HashMap::from_iter(iter),
        }
    }
}

impl RelationTripletC {
    pub fn new(hm: HashMap<u16, RelationTripletD>) -> RelationTripletC {
        RelationTripletC { hm }
    }

    pub fn get(&self, index: &u16) -> Option<&RelationTripletD> {
        self.hm.get(index)
    }

    pub fn len(&self) -> usize {
        self.hm.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hm.is_empty()
    }

    /// Returns a relation triplet (3 relations) container from a vector of vector of 2 relations.
    /// A relation triplet completely defines the position of 3 intervals/segments/chunks.
    /// We always add a Before relation between chunks 0 and 2.
    ///
    /// # Arguments
    ///
    /// * `v` - A vector or vector of relation.
    pub fn of_relation_pair(v: Vec<Vec<&AllenIntervalAlgebraRelation>>) -> RelationTripletC {
        let relation_triplet_v = v
            .iter()
            .enumerate()
            .map(|(index, relation_v)| {
                assert_eq!(relation_v.len(), 2);
                let second = relation_v[0].clone();
                let third = relation_v[1].clone();
                (
                    index as u16,
                    RelationTripletD::new(AllenIntervalAlgebraRelation::B, second, third),
                )
            })
            .collect();
        RelationTripletC {
            hm: relation_triplet_v,
        }
    }

    /// Returns a relation triplet (3 relations) container from a vector of vector of 3 relations.
    /// A relation triplet completely defines the position of 3 intervals/segments/chunks.
    ///
    /// # Arguments
    ///
    /// * `v` - A vector or vector of relation.
    pub fn of_relation_triplet(v: Vec<Vec<&AllenIntervalAlgebraRelation>>) -> RelationTripletC {
        let relation_triplet_v = v
            .iter()
            .enumerate()
            .map(|(index, relation_v)| {
                assert_eq!(relation_v.len(), 3);
                let first = relation_v[0].clone();
                let second = relation_v[1].clone();
                let third = relation_v[2].clone();
                (index as u16, RelationTripletD::new(first, second, third))
            })
            .collect();
        RelationTripletC {
            hm: relation_triplet_v,
        }
    }

    pub fn reindex(&self) -> RelationTripletC {
        let mut v: Vec<RelationTripletD> = self.hm.values().cloned().collect();
        v.sort();
        let hm = v
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, relation_triplet_d)| (index as u16, relation_triplet_d))
            .collect();
        RelationTripletC::new(hm)
    }

    pub fn iter(&self) -> Iter<u16, RelationTripletD> {
        self.hm.iter()
    }
}
