use serde::{Deserialize, Serialize};

use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::relation::relation_triplet::RelationTripletC;

/// Consistent (according to SparQ) relation triplet/pair
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsistentRelationTripletPair {
    /// Relation for 2 chunks
    relation_pair_v: Vec<AllenIntervalAlgebraRelation>,
    /// Relation for 3 chunks
    relation_triplet_c: RelationTripletC,
}

impl ConsistentRelationTripletPair {
    pub fn new(
        relation_pair_v: Vec<AllenIntervalAlgebraRelation>,
        relation_triplet_c: RelationTripletC,
    ) -> ConsistentRelationTripletPair {
        ConsistentRelationTripletPair {
            relation_pair_v,
            relation_triplet_c,
        }
    }

    pub fn get_relation_pair_v(&self) -> &Vec<AllenIntervalAlgebraRelation> {
        &self.relation_pair_v
    }

    pub fn get_relation_triplet_c(&self) -> &RelationTripletC {
        &self.relation_triplet_c
    }
}
