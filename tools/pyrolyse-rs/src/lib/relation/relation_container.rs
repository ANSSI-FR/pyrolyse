
// use serde::{Deserialize, Serialize, Serializer};

use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;

pub trait RelationContainer {
    fn to_sparq_string(&self) -> String;
    fn build_sparq_constraint_string(&self, i_init: u32) -> String;
    fn to_v(&self) -> Vec<AllenIntervalAlgebraRelation>;
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct RelationTriplet {
//     // Relation between 0 and 1: always B (before)
//     relation_01: AllenIntervalAlgebraRelation,
//     // Relation between 0 and 2
//     relation_02: AllenIntervalAlgebraRelation,
//     // Relation between 1 and 2
//     relation_12: AllenIntervalAlgebraRelation,
// }

// impl RelationContainer for RelationTriplet {
//     fn to_sparq_string(&self) -> String {
//         // match self {
//         //     AllenIntervalAlgebraRelation::Eq => "eq".to_string(),
//         //     AllenIntervalAlgebraRelation::B => "b".to_string(),
//         //     AllenIntervalAlgebraRelation::Bi => "bi".to_string(),
//         //     AllenIntervalAlgebraRelation::O => "o".to_string(),
//         //     AllenIntervalAlgebraRelation::Oi => "oi".to_string(),
//         //     AllenIntervalAlgebraRelation::S => "s".to_string(),
//         //     AllenIntervalAlgebraRelation::Si => "si".to_string(),
//         //     AllenIntervalAlgebraRelation::D => "d".to_string(),
//         //     AllenIntervalAlgebraRelation::Di => "di".to_string(),
//         //     AllenIntervalAlgebraRelation::F => "f".to_string(),
//         //     AllenIntervalAlgebraRelation::Fi => "fi".to_string(),
//         // }
//         format!(
//             "{:?} {:?} {:?}",
//             self.relation_01, self.relation_02, self.relation_12
//         )
//     }

//     fn build_sparq_constraint_string(&self, i_init: u32) -> String {
//         format!(
//             "(i{} {} i{}) (i{} {} i{}) (i{} {} i{})",
//             i_init,
//             self.relation_01.to_sparq_string(),
//             i_init + 1,
//             i_init,
//             self.relation_02.to_sparq_string(),
//             i_init + 2,
//             i_init + 1,
//             self.relation_12.to_sparq_string(),
//             i_init + 2
//         )
//     }

//     fn to_v(&self) -> Vec<AllenIntervalAlgebraRelation> {
//         vec![
//             (self.relation_01).clone(),
//             (self.relation_02).clone(),
//             (self.relation_12).clone(),
//         ]
//     }
// }
