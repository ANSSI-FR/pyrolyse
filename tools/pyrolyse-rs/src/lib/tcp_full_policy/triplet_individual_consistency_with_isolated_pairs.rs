use serde::{Deserialize, Serialize};

use crate::byte_time_data::chunk::ChunkC;
//use crate::tcp_complicated_policy::triplet_position_policy::TripletPositionPolicy;
use crate::relation::relation_triplet::RelationTripletD;
use crate::policy_common::policy_consistency::PolicyConsistency;


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum TripletIndividualConsistencyWithIsolatedPairs {
    Consistent,
    NotConsistent
}

impl TripletIndividualConsistencyWithIsolatedPairs {
  pub fn of_data(
      chunk_c: &ChunkC,
      //triplet_position_policy: &TripletPositionPolicy, 
      relation_triplet_d: &RelationTripletD,
      consistent_pair_position_policy_01_first_second: &PolicyConsistency,
      consistent_pair_position_policy_02_first_second: &PolicyConsistency,
      consistent_pair_position_policy_12_first_second: &PolicyConsistency
  ) -> TripletIndividualConsistencyWithIsolatedPairs {
      let policy_01_first_second = match consistent_pair_position_policy_01_first_second {
          PolicyConsistency::NotConsistent | PolicyConsistency::NotConsistentPeoLike => false,
          PolicyConsistency::Na | PolicyConsistency::Consistent => true
      };
      let policy_02_first_second = match consistent_pair_position_policy_02_first_second {
          PolicyConsistency::NotConsistent | PolicyConsistency::NotConsistentPeoLike => false,
          PolicyConsistency::Na | PolicyConsistency::Consistent => true
      };
      let policy_12_first_second = match consistent_pair_position_policy_12_first_second {
          PolicyConsistency::NotConsistent | PolicyConsistency::NotConsistentPeoLike => false,
          PolicyConsistency::Na | PolicyConsistency::Consistent => true
      };

      let merged_policy_first_second = policy_01_first_second & policy_02_first_second & policy_12_first_second;

      if merged_policy_first_second {
          return TripletIndividualConsistencyWithIsolatedPairs::Consistent
      }

      //let relation_triplet = triplet_position_policy.get_relation_triplet();
      let relation_v = vec![
          relation_triplet_d.get_relation_01(),
          relation_triplet_d.get_relation_02(),
          relation_triplet_d.get_relation_12()
      ];

      let before_relation_like_number = relation_v.iter().fold(0, |acc, &relation| {
          if relation.is_before_like() { acc + 1 } 
          else { acc }
      });

      if before_relation_like_number < 2 { 
          return TripletIndividualConsistencyWithIsolatedPairs::NotConsistent

      } else if before_relation_like_number == 3 {
          return TripletIndividualConsistencyWithIsolatedPairs::Consistent
      }

      let chunk_0 = chunk_c.get(&0).unwrap(); 
      let chunk_1 = chunk_c.get(&1).unwrap();
      let chunk_2 = chunk_c.get(&2).unwrap();
      //let consistency_b = if !relation_triplet.get_relation_01().is_before_like() {
      //    chunk_0.get_start() | chunk_1.get_start() 
      //} else if !relation_triplet.get_relation_02().is_before_like() {
      //    chunk_0.get_start() | chunk_2.get_start() 
      //} else {
      //    assert!(!relation_triplet.get_relation_12().is_before_like());
      //    chunk_1.get_start() | chunk_2.get_start() 
      //};
      let consistency_b = if !relation_triplet_d.get_relation_01().is_before_like() {
          (chunk_0.get_start() | chunk_1.get_start()) & policy_01_first_second
      } else if !relation_triplet_d.get_relation_02().is_before_like() {
          (chunk_0.get_start() | chunk_2.get_start()) & policy_02_first_second 
      } else {
          assert!(!relation_triplet_d.get_relation_12().is_before_like());
          (chunk_1.get_start() | chunk_2.get_start()) & policy_12_first_second 
      };

      match consistency_b {
          false => TripletIndividualConsistencyWithIsolatedPairs::Consistent,
          true => TripletIndividualConsistencyWithIsolatedPairs::NotConsistent
      }

  }
}