use serde::{Deserialize, Serialize};

use crate::relation::relation_container::RelationContainer;
use crate::misc::interval::IntervalD;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AllenIntervalAlgebraRelation {
    Eq,
    M,
    Mi,
    B,
    Bi,
    O,
    Oi,
    S,
    Si,
    D,
    Di,
    F,
    Fi,
}

impl RelationContainer for AllenIntervalAlgebraRelation {
    fn build_sparq_constraint_string(&self, i_init: u32) -> String {
        format!("((i{} {} i{}))", i_init, self.to_sparq_string(), i_init + 1)
    }

    fn to_sparq_string(&self) -> String {
        match self {
            AllenIntervalAlgebraRelation::Eq => "eq".to_string(),
            AllenIntervalAlgebraRelation::M => "m".to_string(),
            AllenIntervalAlgebraRelation::Mi => "mi".to_string(),
            AllenIntervalAlgebraRelation::B => "b".to_string(),
            AllenIntervalAlgebraRelation::Bi => "bi".to_string(),
            AllenIntervalAlgebraRelation::O => "o".to_string(),
            AllenIntervalAlgebraRelation::Oi => "oi".to_string(),
            AllenIntervalAlgebraRelation::S => "s".to_string(),
            AllenIntervalAlgebraRelation::Si => "si".to_string(),
            AllenIntervalAlgebraRelation::D => "d".to_string(),
            AllenIntervalAlgebraRelation::Di => "di".to_string(),
            AllenIntervalAlgebraRelation::F => "f".to_string(),
            AllenIntervalAlgebraRelation::Fi => "fi".to_string(),
        }
    }

    fn to_v(&self) -> Vec<AllenIntervalAlgebraRelation> {
        vec![(*self).clone()]
    }
}

impl AllenIntervalAlgebraRelation {
    pub fn all() -> Vec<AllenIntervalAlgebraRelation> {
        vec![
            AllenIntervalAlgebraRelation::Eq,
            AllenIntervalAlgebraRelation::M,
            AllenIntervalAlgebraRelation::Mi,
            AllenIntervalAlgebraRelation::B,
            AllenIntervalAlgebraRelation::Bi,
            AllenIntervalAlgebraRelation::O,
            AllenIntervalAlgebraRelation::Oi,
            AllenIntervalAlgebraRelation::S,
            AllenIntervalAlgebraRelation::Si,
            AllenIntervalAlgebraRelation::D,
            AllenIntervalAlgebraRelation::Di,
            AllenIntervalAlgebraRelation::F,
            AllenIntervalAlgebraRelation::Fi,
        ]
    }

    pub fn all_except_before_like() -> Vec<AllenIntervalAlgebraRelation> {
        vec![
            AllenIntervalAlgebraRelation::Eq,
            AllenIntervalAlgebraRelation::M,
            AllenIntervalAlgebraRelation::Mi,
            AllenIntervalAlgebraRelation::O,
            AllenIntervalAlgebraRelation::Oi,
            AllenIntervalAlgebraRelation::S,
            AllenIntervalAlgebraRelation::Si,
            AllenIntervalAlgebraRelation::D,
            AllenIntervalAlgebraRelation::Di,
            AllenIntervalAlgebraRelation::F,
            AllenIntervalAlgebraRelation::Fi,
        ]
    }

    pub fn no_inverse() -> Vec<AllenIntervalAlgebraRelation> {
        vec![
            AllenIntervalAlgebraRelation::Eq,
            AllenIntervalAlgebraRelation::M,
            AllenIntervalAlgebraRelation::B,
            AllenIntervalAlgebraRelation::O,
            AllenIntervalAlgebraRelation::S,
            AllenIntervalAlgebraRelation::D,
            AllenIntervalAlgebraRelation::F,
        ]
    }

    pub fn with_overlap() -> Vec<AllenIntervalAlgebraRelation> {
        vec![
            AllenIntervalAlgebraRelation::Eq,
            AllenIntervalAlgebraRelation::O,
            AllenIntervalAlgebraRelation::Oi,
            AllenIntervalAlgebraRelation::S,
            AllenIntervalAlgebraRelation::Si,
            AllenIntervalAlgebraRelation::D,
            AllenIntervalAlgebraRelation::Di,
            AllenIntervalAlgebraRelation::F,
            AllenIntervalAlgebraRelation::Fi,
        ]
    }

    // pub fn to_sparq_string(&self) -> String {
    //     match self {
    //         AllenIntervalAlgebraRelation::Eq => "eq".to_string(),
    //         AllenIntervalAlgebraRelation::B => "b".to_string(),
    //         AllenIntervalAlgebraRelation::Bi => "bi".to_string(),
    //         AllenIntervalAlgebraRelation::O => "o".to_string(),
    //         AllenIntervalAlgebraRelation::Oi => "oi".to_string(),
    //         AllenIntervalAlgebraRelation::S => "s".to_string(),
    //         AllenIntervalAlgebraRelation::Si => "si".to_string(),
    //         AllenIntervalAlgebraRelation::D => "d".to_string(),
    //         AllenIntervalAlgebraRelation::Di => "di".to_string(),
    //         AllenIntervalAlgebraRelation::F => "f".to_string(),
    //         AllenIntervalAlgebraRelation::Fi => "fi".to_string(),
    //     }
    // }

    pub fn is_inverse(&self) -> bool {
        match self {
            AllenIntervalAlgebraRelation::Eq => false,
            AllenIntervalAlgebraRelation::M => false,
            AllenIntervalAlgebraRelation::Mi => true,
            AllenIntervalAlgebraRelation::B => false,
            AllenIntervalAlgebraRelation::Bi => true,
            AllenIntervalAlgebraRelation::O => false,
            AllenIntervalAlgebraRelation::Oi => true,
            AllenIntervalAlgebraRelation::S => false,
            AllenIntervalAlgebraRelation::Si => true,
            AllenIntervalAlgebraRelation::D => false,
            AllenIntervalAlgebraRelation::Di => true,
            AllenIntervalAlgebraRelation::F => false,
            AllenIntervalAlgebraRelation::Fi => true,
        }
    }

    /// Return if there is inclusion of 0 in 1 for relation 0 R 1
    pub fn is_inclusion(&self) -> bool {
        match self {
            AllenIntervalAlgebraRelation::Eq => true,
            AllenIntervalAlgebraRelation::M => false,
            AllenIntervalAlgebraRelation::Mi => false,
            AllenIntervalAlgebraRelation::B => false,
            AllenIntervalAlgebraRelation::Bi => false,
            AllenIntervalAlgebraRelation::O => false,
            AllenIntervalAlgebraRelation::Oi => false,
            AllenIntervalAlgebraRelation::S => true,
            AllenIntervalAlgebraRelation::Si => false,
            AllenIntervalAlgebraRelation::D => true,
            AllenIntervalAlgebraRelation::Di => false,
            AllenIntervalAlgebraRelation::F => true,
            AllenIntervalAlgebraRelation::Fi => false,
        }
    }

    /// Return if there is overlap between 0 and 1 for relation 0 R 1
    pub fn is_overlap(&self) -> bool {
        match self {
            AllenIntervalAlgebraRelation::Eq => true,
            AllenIntervalAlgebraRelation::M => false,
            AllenIntervalAlgebraRelation::Mi => false,
            AllenIntervalAlgebraRelation::B => false,
            AllenIntervalAlgebraRelation::Bi => false,
            AllenIntervalAlgebraRelation::O => true,
            AllenIntervalAlgebraRelation::Oi => true,
            AllenIntervalAlgebraRelation::S => true,
            AllenIntervalAlgebraRelation::Si => true,
            AllenIntervalAlgebraRelation::D => true,
            AllenIntervalAlgebraRelation::Di => true,
            AllenIntervalAlgebraRelation::F => true,
            AllenIntervalAlgebraRelation::Fi => true,
        }
    }

    /// Return if 0 starts before 1 for relation 0 R 1
    pub fn start_before(&self) -> bool {
        match self {
            AllenIntervalAlgebraRelation::Eq => false,
            AllenIntervalAlgebraRelation::M => true,
            AllenIntervalAlgebraRelation::Mi => false,
            AllenIntervalAlgebraRelation::B => true,
            AllenIntervalAlgebraRelation::Bi => false,
            AllenIntervalAlgebraRelation::O => true,
            AllenIntervalAlgebraRelation::Oi => false,
            AllenIntervalAlgebraRelation::S => false,
            AllenIntervalAlgebraRelation::Si => false,
            AllenIntervalAlgebraRelation::D => false,
            AllenIntervalAlgebraRelation::Di => true,
            AllenIntervalAlgebraRelation::F => false,
            AllenIntervalAlgebraRelation::Fi => true,
        }
    }

    /// Return if 0 starts after 1 for relation 0 R 1
    pub fn start_after(&self) -> bool {
        match self {
            AllenIntervalAlgebraRelation::Eq => false,
            AllenIntervalAlgebraRelation::M => false,
            AllenIntervalAlgebraRelation::Mi => true,
            AllenIntervalAlgebraRelation::B => false,
            AllenIntervalAlgebraRelation::Bi => true,
            AllenIntervalAlgebraRelation::O => false,
            AllenIntervalAlgebraRelation::Oi => true,
            AllenIntervalAlgebraRelation::S => false,
            AllenIntervalAlgebraRelation::Si => false,
            AllenIntervalAlgebraRelation::D => true,
            AllenIntervalAlgebraRelation::Di => false,
            AllenIntervalAlgebraRelation::F => true,
            AllenIntervalAlgebraRelation::Fi => false,
        }
    }

    /// Return if 0 ends before 1 for relation 0 R 1
    pub fn end_before(&self) -> bool {
        match self {
            AllenIntervalAlgebraRelation::Eq => false,
            AllenIntervalAlgebraRelation::M => true,
            AllenIntervalAlgebraRelation::Mi => false,
            AllenIntervalAlgebraRelation::B => true,
            AllenIntervalAlgebraRelation::Bi => false,
            AllenIntervalAlgebraRelation::O => true,
            AllenIntervalAlgebraRelation::Oi => false,
            AllenIntervalAlgebraRelation::S => true,
            AllenIntervalAlgebraRelation::Si => false,
            AllenIntervalAlgebraRelation::D => true,
            AllenIntervalAlgebraRelation::Di => false,
            AllenIntervalAlgebraRelation::F => false,
            AllenIntervalAlgebraRelation::Fi => false,
        }
    }

    /// Return if 0 ends after 1 for relation 0 R 1
    pub fn end_after(&self) -> bool {
        match self {
            AllenIntervalAlgebraRelation::Eq => false,
            AllenIntervalAlgebraRelation::M => false,
            AllenIntervalAlgebraRelation::Mi => true,
            AllenIntervalAlgebraRelation::B => false,
            AllenIntervalAlgebraRelation::Bi => true,
            AllenIntervalAlgebraRelation::O => false,
            AllenIntervalAlgebraRelation::Oi => true,
            AllenIntervalAlgebraRelation::S => false,
            AllenIntervalAlgebraRelation::Si => true,
            AllenIntervalAlgebraRelation::D => false,
            AllenIntervalAlgebraRelation::Di => true,
            AllenIntervalAlgebraRelation::F => false,
            AllenIntervalAlgebraRelation::Fi => false,
        }
    }

    pub fn is_before_like(&self) -> bool {
        match self {
            AllenIntervalAlgebraRelation::Eq => false,
            AllenIntervalAlgebraRelation::M => false,
            AllenIntervalAlgebraRelation::Mi => false,
            AllenIntervalAlgebraRelation::B => true,
            AllenIntervalAlgebraRelation::Bi => true,
            AllenIntervalAlgebraRelation::O => false,
            AllenIntervalAlgebraRelation::Oi => false,
            AllenIntervalAlgebraRelation::S => false,
            AllenIntervalAlgebraRelation::Si => false,
            AllenIntervalAlgebraRelation::D => false,
            AllenIntervalAlgebraRelation::Di => false,
            AllenIntervalAlgebraRelation::F => false,
            AllenIntervalAlgebraRelation::Fi => false,
        }
    }

    pub fn of_intervals(
        interval_0: &IntervalD,        
        interval_1: &IntervalD,        
    ) -> AllenIntervalAlgebraRelation {
        let interval_0_start = interval_0.get_start();
        let interval_0_end = interval_0.get_end() + 1;
        let interval_1_start = interval_1.get_start();
        let interval_1_end = interval_1.get_end() + 1;

        match interval_0_start.cmp(&interval_1_start) {
            Ordering::Greater => {
                // Oi, F, D, Bi, Mi
                match interval_0_start.cmp(&interval_1_end) {
                    Ordering::Greater => {
                        // Bi
                        AllenIntervalAlgebraRelation::Bi
                    }
                    Ordering::Equal => {
                        // Mi
                        AllenIntervalAlgebraRelation::Mi
                    } 
                    Ordering::Less => {
                        // Oi, F, D
                        match interval_0_end.cmp(&interval_1_end) {
                            Ordering::Greater => {
                                // Oi
                                AllenIntervalAlgebraRelation::Oi
                            }
                            Ordering::Equal => {
                                // F
                                AllenIntervalAlgebraRelation::F
                            } 
                            Ordering::Less => {
                                // D
                                AllenIntervalAlgebraRelation::D
                            } 
                        }
                    } 
                }
            },
            Ordering::Equal => {
                // Eq, S, Si
                match interval_0_end.cmp(&interval_1_end) {
                    Ordering::Greater => {
                        // Si
                        AllenIntervalAlgebraRelation::Si
                    }
                    Ordering::Equal => {
                        // Eq
                        AllenIntervalAlgebraRelation::Eq
                    } 
                    Ordering::Less => {
                        // S
                        AllenIntervalAlgebraRelation::S
                    } 
                }
            } 
            Ordering::Less => {
                // Fi, O, Di, B, M
                match interval_0_end.cmp(&interval_1_start) {
                    Ordering::Greater => {
                        // Fi, O, Di
                        match interval_0_end.cmp(&interval_1_end) {
                            Ordering::Greater => {
                                // Di
                                AllenIntervalAlgebraRelation::Di
                            }
                            Ordering::Equal => {
                                // Fi
                                AllenIntervalAlgebraRelation::Fi
                            } 
                            Ordering::Less => {
                                // O
                                AllenIntervalAlgebraRelation::O
                            } 
                        }
                    }
                    Ordering::Equal => {
                        // M
                        AllenIntervalAlgebraRelation::M
                    } 
                    Ordering::Less => {
                        // B
                        AllenIntervalAlgebraRelation::B
                    } 
                }
            } 
        }
    }
}
