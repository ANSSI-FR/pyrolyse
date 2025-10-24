use std::str::FromStr;
use serde::{Deserialize, Serialize};

use crate::misc::extra_chunk_time::ExtraChunkTime;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum TcpScenario {
    // Old scenarii
    // Send ack after sending all data.
    //Pep1,
    // Send ack after every answer.
    //Pep2,
    //Peos,
    
    //Penp2,
    //Peosp2,
    //Peosf,
    //Peoef2,
    //Peoep2,
    //Peosfef,
    //Peoefsf,
    //Peospef2,
    //Peoepsf,
    //Peospep2,
    //Peoepsp2,

    // New scenarii - change name to fit IP scenarii

    // should be equivalent to pep2 or penp2 
    ProgressiveAckProgressive,
    // should be equivalent to pep1 
    ProgressiveAckOnce,
    // should be equivalent to peosp2 
    OnceStartPrecedesAckProgressive,
    OnceStartPrecedesAckOnce,
    // should be equivalent to peoef2 
    OnceEndFollowsAckProgressive,
    OnceEndFollowsAckOnce,
    // should be equivalent to peoep2 
    OnceEndPrecedesAckProgressive,
    OnceEndPrecedesAckOnce,
    // should be equivalent to peospef2 
    OnceStartPrecedesEndFollowsAckProgressive,
    OnceStartPrecedesEndFollowsAckOnce,
    // should be equivalent to peospep2 
    OnceStartPrecedesEndPrecedesAckProgressive,
    OnceStartPrecedesEndPrecedesAckOnce,
    // should be equivalent to peoepsp2 
    OnceEndPrecedesStartPrecedesAckProgressive,
    OnceEndPrecedesStartPrecedesAckOnce,

    // should be equivalent to peos and peosf 
    OnceStartFollows,
    // should be equivalent to peosfef 
    OnceStartFollowsEndFollows,
    // should be equivalent to peoefsf 
    OnceEndFollowsStartFollows,
    // should be equivalent to peoepsf 
    OnceEndPrecedesStartFollows,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTcpScenarioError(String);

impl FromStr for TcpScenario {
    type Err = ParseTcpScenarioError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pep-ap" => Ok(TcpScenario::ProgressiveAckProgressive),
            "pep-ao" => Ok(TcpScenario::ProgressiveAckOnce),

            "peosp-ap" => Ok(TcpScenario::OnceStartPrecedesAckProgressive),
            "peosp-ao" => Ok(TcpScenario::OnceStartPrecedesAckOnce),

            "peoef-ap" => Ok(TcpScenario::OnceEndFollowsAckProgressive),
            "peoef-ao" => Ok(TcpScenario::OnceEndFollowsAckOnce),

            "peoep-ap" => Ok(TcpScenario::OnceEndPrecedesAckProgressive),
            "peoep-ao" => Ok(TcpScenario::OnceEndPrecedesAckOnce),

            "peospef-ap" => Ok(TcpScenario::OnceStartPrecedesEndFollowsAckProgressive),
            "peospef-ao" => Ok(TcpScenario::OnceStartPrecedesEndFollowsAckOnce),

            "peospep-ap" => Ok(TcpScenario::OnceStartPrecedesEndPrecedesAckProgressive),
            "peospep-ao" => Ok(TcpScenario::OnceStartPrecedesEndPrecedesAckOnce),

            "peoepsp-ap" => Ok(TcpScenario::OnceEndPrecedesStartPrecedesAckProgressive),
            "peoepsp-ao" => Ok(TcpScenario::OnceEndPrecedesStartPrecedesAckOnce),

            "peosf" => Ok(TcpScenario::OnceStartFollows),
            "peosfef" => Ok(TcpScenario::OnceStartFollowsEndFollows),
            "peoefsf" => Ok(TcpScenario::OnceEndFollowsStartFollows),
            "peoepsf" => Ok(TcpScenario::OnceEndPrecedesStartFollows),
            
            _ => Err(ParseTcpScenarioError(s.to_string())),
        }
    }
}

impl TcpScenario {

    pub fn get_extra_chunk_before_test_case(&self) -> ExtraChunkTime {
        match self {
            TcpScenario::ProgressiveAckProgressive 
            | TcpScenario::ProgressiveAckOnce 
            | TcpScenario::OnceEndFollowsAckProgressive 
            | TcpScenario::OnceEndFollowsAckOnce 
            | TcpScenario::OnceEndPrecedesAckProgressive 
            | TcpScenario::OnceEndPrecedesAckOnce 
            => ExtraChunkTime::None,
            
            TcpScenario::OnceStartFollows
            | TcpScenario::OnceStartFollowsEndFollows
            | TcpScenario::OnceEndFollowsStartFollows
            | TcpScenario::OnceEndPrecedesStartFollows
            => ExtraChunkTime::Follows, 
            
            TcpScenario::OnceStartPrecedesAckProgressive
            | TcpScenario::OnceStartPrecedesAckOnce
            | TcpScenario::OnceStartPrecedesEndFollowsAckProgressive
            | TcpScenario::OnceStartPrecedesEndFollowsAckOnce
            | TcpScenario::OnceStartPrecedesEndPrecedesAckProgressive
            | TcpScenario::OnceStartPrecedesEndPrecedesAckOnce
            | TcpScenario::OnceEndPrecedesStartPrecedesAckProgressive
            | TcpScenario::OnceEndPrecedesStartPrecedesAckOnce
            => ExtraChunkTime::Precedes,
        }
    }

    pub fn get_extra_chunk_after_test_case(&self) -> ExtraChunkTime {
        match self {
            TcpScenario::ProgressiveAckProgressive 
            | TcpScenario::ProgressiveAckOnce 
            | TcpScenario::OnceStartFollows
            | TcpScenario::OnceStartPrecedesAckProgressive
            | TcpScenario::OnceStartPrecedesAckOnce
            => ExtraChunkTime::None,
            
            TcpScenario::OnceEndFollowsAckProgressive 
            | TcpScenario::OnceEndFollowsAckOnce 
            | TcpScenario::OnceStartFollowsEndFollows
            | TcpScenario::OnceEndFollowsStartFollows
            | TcpScenario::OnceStartPrecedesEndFollowsAckProgressive
            | TcpScenario::OnceStartPrecedesEndFollowsAckOnce
            => ExtraChunkTime::Follows, 
            
            TcpScenario::OnceEndPrecedesAckProgressive 
            | TcpScenario::OnceEndPrecedesAckOnce 
            | TcpScenario::OnceEndPrecedesStartFollows
            | TcpScenario::OnceStartPrecedesEndPrecedesAckProgressive
            | TcpScenario::OnceStartPrecedesEndPrecedesAckOnce
            | TcpScenario::OnceEndPrecedesStartPrecedesAckProgressive
            | TcpScenario::OnceEndPrecedesStartPrecedesAckOnce
            => ExtraChunkTime::Precedes,
        }
    }

}