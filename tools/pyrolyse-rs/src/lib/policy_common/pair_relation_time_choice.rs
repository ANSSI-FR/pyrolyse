use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::Iterator;
use std::str;

use serde::{Deserialize, Serialize};

use crate::misc::pair_time_position::PairTimePosition;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::policy_common::pair_choice::PairChoice;
use crate::tcp_full_policy;
use crate::ip_full_policy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairRelationTimeChoice {
    hm: HashMap<(AllenIntervalAlgebraRelation, PairTimePosition), PairChoice>,
}

impl FromIterator<((AllenIntervalAlgebraRelation, PairTimePosition), PairChoice)>
    for PairRelationTimeChoice
{
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = ((AllenIntervalAlgebraRelation, PairTimePosition), PairChoice)>,
    {
        Self {
            hm: HashMap::from_iter(iter),
        }
    }
}

impl PairRelationTimeChoice {
    pub fn len(&self) -> usize {
        self.hm.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hm.is_empty()
    }

    pub fn get(
        &self,
        tuple: &(AllenIntervalAlgebraRelation, PairTimePosition),
    ) -> Option<&PairChoice> {
        self.hm.get(tuple)
    }

    pub fn get_index_total_length(&self) -> u32 {
        (self.hm.len() as u32) * 3
    }

    pub fn of_tcp_full_pair_position_policy_data_c(
        pair_position_policy_data_c: &tcp_full_policy::pair_position_policy_data::PairPositionPolicyDataC,
    ) -> PairRelationTimeChoice {
        debug!("of_pair_position_policy_c: start");

        let hm = pair_position_policy_data_c
            .iter()
            .map(|(_index, pair_position_policy_d)| {
                (
                    (
                        (*pair_position_policy_d.get_relation()).clone(),
                        (*pair_position_policy_d.get_pair_time_position()).clone(),
                    ),
                    (*pair_position_policy_d.get_pair_choice()).clone(),
                )
            })
            .collect::<HashMap<(AllenIntervalAlgebraRelation, PairTimePosition), PairChoice>>();

        if hm.len() != pair_position_policy_data_c.len() {
            debug!("of_pair_position_policy_c: couple relation/policy are distinct from pair_overlap_policy_c => double test");
        };

        debug!("of_pair_position_policy_c: end");
        PairRelationTimeChoice { hm }
    }

    pub fn of_ip_full_pair_position_policy_data_c(
        pair_position_policy_data_c: &ip_full_policy::pair_position_policy_data::PairPositionPolicyDataC,
    ) -> PairRelationTimeChoice {
        debug!("of_pair_position_policy_c: start");

        let hm = pair_position_policy_data_c
            .iter()
            .map(|(_index, pair_position_policy_d)| {
                (
                    (
                        (*pair_position_policy_d.get_relation()).clone(),
                        (*pair_position_policy_d.get_pair_time_position()).clone(),
                    ),
                    (*pair_position_policy_d.get_pair_choice()).clone(),
                )
            })
            .collect::<HashMap<(AllenIntervalAlgebraRelation, PairTimePosition), PairChoice>>();

        if hm.len() != pair_position_policy_data_c.len() {
            debug!("of_pair_position_policy_c: couple relation/policy are distinct from pair_overlap_policy_c => double test");
        };

        debug!("of_pair_position_policy_c: end");
        PairRelationTimeChoice { hm }
    }

    pub fn iter(&self) -> Iter<(AllenIntervalAlgebraRelation, PairTimePosition), PairChoice> {
        self.hm.iter()
    }
}
