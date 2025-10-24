use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PolicyEvaluation {
    // Progressive
    Progressive,
    // Once: we include a single short chunk at the byte-wise begining and send it at the end
    OnceStart,
    // Once: we include a single short chunk at the byte-wise end and send it at the end
    OnceEnd,
    // Once: we include a single short chunk at the byte-wise end, and send it at the end
    OnceStartEnd,
    // Once: we include a single short chunk at the byte-wise begining and at the end,
    // and send them at the end (end one first, start one second)
    OnceEndStart,
}

impl PolicyEvaluation {
    pub fn of_string(s: &str) -> PolicyEvaluation {
        match s {
            "p" => PolicyEvaluation::Progressive,
            "os" => PolicyEvaluation::OnceStart,
            "oe" => PolicyEvaluation::OnceEnd,
            "ose" => PolicyEvaluation::OnceStartEnd,
            "oes" => PolicyEvaluation::OnceEndStart,
            _ => panic!(
                "Invalid string specified for policy evaluation (not p or o): {}",
                s
            ),
        }
    }
}
