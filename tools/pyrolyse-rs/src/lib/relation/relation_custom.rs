use std::iter::Iterator;

use serde::{Deserialize, Serialize};

use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::relation::relation_container::RelationContainer;

// TODO: change chunk index: 1 <-> 2
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RelationCustomD {
    v: Vec<(u32, u32, AllenIntervalAlgebraRelation)>,
}

impl RelationContainer for RelationCustomD {
    fn to_sparq_string(&self) -> String {
        self.v
            .iter()
            .map(|(_, _, r)| r.to_sparq_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn build_sparq_constraint_string(&self, i_init: u32) -> String {
        format!(
            "({})",
            self.v
                .iter()
                .map(|(i0, i1, r)| {
                    format!(
                        "(i{} {} i{})",
                        i_init + i0,
                        r.to_sparq_string(),
                        i_init + i1,
                    )
                })
                .collect::<Vec<String>>()
                .join(" ")
        )
    }

    fn to_v(&self) -> Vec<AllenIntervalAlgebraRelation> {
        self.v.iter().map(|(_, _, r)| r.clone()).collect()
    }
}
