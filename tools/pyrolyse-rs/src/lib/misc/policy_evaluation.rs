use serde::{Deserialize, Serialize};

use crate::misc::extra_chunk_time::ExtraChunkTime;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum PolicyEvaluation {
     
    // Progressive 
    // /!\ new progressive is different from old progressive (there is no extra chunk that precedes 
    //    the chunks of the sequence under test for new progressive)
    ProgressiveAllFinishing,
    ProgressiveOldestFinishing,
    ProgressiveMidFinishing,
    ProgressiveNewestFinishing,
    ProgressiveOldestNewestFinishing,
    ProgressiveOldestMidFinishing,
    ProgressiveMidNewestFinishing,
    ProgressiveAllStarting,
    ProgressiveOldestStarting,
    ProgressiveMidStarting,
    ProgressiveNewestStarting,
    ProgressiveOldestNewestStarting,
    ProgressiveOldestMidStarting,
    ProgressiveMidNewestStarting,
    
    // Once: we include a single short chunk at the byte-wise begining and send it before test case's chunks
    // SHOULD BE THE SAME THAT OLD Progressive
    OnceStartPrecedesAllFinishing,
    OnceStartPrecedesOldestFinishing,
    OnceStartPrecedesMidFinishing,
    OnceStartPrecedesNewestFinishing,
    OnceStartPrecedesOldestNewestFinishing,
    OnceStartPrecedesOldestMidFinishing,
    OnceStartPrecedesMidNewestFinishing,
    OnceStartPrecedesAllStarting,
    OnceStartPrecedesMidStarting,
    OnceStartPrecedesOldestStarting,
    OnceStartPrecedesNewestStarting,
    OnceStartPrecedesOldestNewestStarting,
    OnceStartPrecedesOldestMidStarting,
    OnceStartPrecedesMidNewestStarting,
    
    // Once: we include a single short chunk at the byte-wise begining and send it after test case's chunks
    // SHOULD BE THE SAME THAT OLD OnceStart
    OnceStartFollowsAllFinishing,
    OnceStartFollowsOldestFinishing,
    OnceStartFollowsMidFinishing,
    OnceStartFollowsNewestFinishing,
    OnceStartFollowsOldestNewestFinishing,
    OnceStartFollowsOldestMidFinishing,
    OnceStartFollowsMidNewestFinishing,
    OnceStartFollowsAllStarting,
    OnceStartFollowsMidStarting,
    OnceStartFollowsOldestStarting,
    OnceStartFollowsNewestStarting,
    OnceStartFollowsOldestNewestStarting,
    OnceStartFollowsOldestMidStarting,
    OnceStartFollowsMidNewestStarting,
    
    // Once: we include a single short chunk at the byte-wise end and send it after test case's chunks
    OnceEndFollows,
    // Once: we include a single short chunk at the byte-wise end and send it before test case's chunks
    OnceEndPrecedes,
    // Once: we include a short chunk at the byte-wise begining and send it before test case's chunks
    // and we include a short chunk at the byte-wise end and send it after test case's chunks.
    // We send Start before End.
    // SHOULD BE THE SAME THAT OLD OnceStartEnd
    OnceStartFollowsEndFollows,
    // Once: we include a short chunk at the byte-wise begining and send it after test case's chunks
    // and we include a short chunk at the byte-wise end and send it after test case's chunks.
    // We send End before Start.
    // SHOULD BE THE SAME THAT OLD OnceEndStart
    OnceEndFollowsStartFollows,
    // Once: we include a short chunk at the byte-wise begining and send it before test case's chunks
    // and we include a short chunk at the byte-wise end and send it after test case's chunks.
    // We send Start before End.
    // SHOULD BE THE SAME THAT OLD OnceEnd
    OnceStartPrecedesEndFollows,
    // Once: we include a short chunk at the byte-wise begining and send it after test case's chunks
    // and we include a short chunk at the byte-wise end and send it before test case's chunks.
    // We send End before Start.
    OnceEndPrecedesStartFollows,
    // Once: we include a short chunk at the byte-wise begining and send it before test case's chunks
    // and we include a short chunk at the byte-wise end and send it before test case's chunks.
    // We send Start before End.
    OnceStartPrecedesEndPrecedes,
    // Once: we include a short chunk at the byte-wise begining and send it before test case's chunks
    // and we include a short chunk at the byte-wise end and send it before test case's chunks.
    // We send End before Start.
    OnceEndPrecedesStartPrecedes,

}

impl PolicyEvaluation {
    pub fn of_string(s: &str) -> PolicyEvaluation {

        match s {
            "p-af" => PolicyEvaluation::ProgressiveAllFinishing,
            "p-of" => PolicyEvaluation::ProgressiveOldestFinishing,
            "p-mf" => PolicyEvaluation::ProgressiveMidFinishing,
            "p-nf" => PolicyEvaluation::ProgressiveNewestFinishing,
            "p-onf" => PolicyEvaluation::ProgressiveOldestNewestFinishing,
            "p-omf" => PolicyEvaluation::ProgressiveOldestMidFinishing,
            "p-mnf" => PolicyEvaluation::ProgressiveMidNewestFinishing,
            "p-as" => PolicyEvaluation::ProgressiveAllStarting,
            "p-os" => PolicyEvaluation::ProgressiveOldestStarting,
            "p-ms" => PolicyEvaluation::ProgressiveMidStarting,
            "p-ns" => PolicyEvaluation::ProgressiveNewestStarting,
            "p-ons" => PolicyEvaluation::ProgressiveOldestNewestStarting,
            "p-oms" => PolicyEvaluation::ProgressiveOldestMidStarting,
            "p-mns" => PolicyEvaluation::ProgressiveMidNewestStarting,

            "osp-af" => PolicyEvaluation::OnceStartPrecedesAllFinishing,
            "osp-of" => PolicyEvaluation::OnceStartPrecedesOldestFinishing,
            "osp-mf" => PolicyEvaluation::OnceStartPrecedesMidFinishing,
            "osp-nf" => PolicyEvaluation::OnceStartPrecedesNewestFinishing,
            "osp-onf" => PolicyEvaluation::OnceStartPrecedesOldestNewestFinishing,
            "osp-omf" => PolicyEvaluation::OnceStartPrecedesOldestMidFinishing,
            "osp-mnf" => PolicyEvaluation::OnceStartPrecedesMidNewestFinishing,
            "osp-as" => PolicyEvaluation::OnceStartPrecedesAllStarting,
            "osp-os" => PolicyEvaluation::OnceStartPrecedesOldestStarting,
            "osp-ms" => PolicyEvaluation::OnceStartPrecedesMidStarting,
            "osp-ns" => PolicyEvaluation::OnceStartPrecedesNewestStarting,
            "osp-ons" => PolicyEvaluation::OnceStartPrecedesOldestNewestStarting,
            "osp-oms" => PolicyEvaluation::OnceStartPrecedesOldestMidStarting,
            "osp-mns" => PolicyEvaluation::OnceStartPrecedesMidNewestStarting,


            "osf-af" => PolicyEvaluation::OnceStartFollowsAllFinishing,
            "osf-of" => PolicyEvaluation::OnceStartFollowsOldestFinishing,
            "osf-mf" => PolicyEvaluation::OnceStartFollowsMidFinishing,
            "osf-nf" => PolicyEvaluation::OnceStartFollowsNewestFinishing,
            "osf-onf" => PolicyEvaluation::OnceStartFollowsOldestNewestFinishing,
            "osf-omf" => PolicyEvaluation::OnceStartFollowsOldestMidFinishing,
            "osf-mnf" => PolicyEvaluation::OnceStartFollowsMidNewestFinishing,
            "osf-as" => PolicyEvaluation::OnceStartFollowsAllStarting,
            "osf-os" => PolicyEvaluation::OnceStartFollowsOldestStarting,
            "osf-ms" => PolicyEvaluation::OnceStartFollowsMidStarting,
            "osf-ns" => PolicyEvaluation::OnceStartFollowsNewestStarting,
            "osf-ons" => PolicyEvaluation::OnceStartFollowsOldestNewestStarting,
            "osf-oms" => PolicyEvaluation::OnceStartFollowsOldestMidStarting,
            "osf-mns" => PolicyEvaluation::OnceStartFollowsMidNewestStarting,


            "oef" => PolicyEvaluation::OnceEndFollows,
            "oep" => PolicyEvaluation::OnceEndPrecedes,
            "osfef" => PolicyEvaluation::OnceStartFollowsEndFollows,
            "oefsf" => PolicyEvaluation::OnceEndFollowsStartFollows,
            "ospef" => PolicyEvaluation::OnceStartPrecedesEndFollows,
            "oepsf" => PolicyEvaluation::OnceEndPrecedesStartFollows,
            "ospep" => PolicyEvaluation::OnceStartPrecedesEndPrecedes,
            "oepsp" => PolicyEvaluation::OnceEndPrecedesStartPrecedes,
            _ => panic!(
                "Invalid string specified for policy evaluation: {}",
                s
            ),
        }
    }

    pub fn of_complete_string(s: &str) -> PolicyEvaluation {

        match s {
            "pep-af" => PolicyEvaluation::ProgressiveAllFinishing,
            "pep-of" => PolicyEvaluation::ProgressiveOldestFinishing,
            "pep-mf" => PolicyEvaluation::ProgressiveMidFinishing,
            "pep-nf" => PolicyEvaluation::ProgressiveNewestFinishing,
            "pep-onf" => PolicyEvaluation::ProgressiveOldestNewestFinishing,
            "pep-omf" => PolicyEvaluation::ProgressiveOldestMidFinishing,
            "pep-mnf" => PolicyEvaluation::ProgressiveMidNewestFinishing,
            "pep-as" => PolicyEvaluation::ProgressiveAllStarting,
            "pep-os" => PolicyEvaluation::ProgressiveOldestStarting,
            "pep-ms" => PolicyEvaluation::ProgressiveMidStarting,
            "pep-ns" => PolicyEvaluation::ProgressiveNewestStarting,
            "pep-ons" => PolicyEvaluation::ProgressiveOldestNewestStarting,
            "pep-oms" => PolicyEvaluation::ProgressiveOldestMidStarting,
            "pep-mns" => PolicyEvaluation::ProgressiveMidNewestStarting,

            "peosp-af" => PolicyEvaluation::OnceStartPrecedesAllFinishing,
            "peosp-of" => PolicyEvaluation::OnceStartPrecedesOldestFinishing,
            "peosp-mf" => PolicyEvaluation::OnceStartPrecedesMidFinishing,
            "peosp-nf" => PolicyEvaluation::OnceStartPrecedesNewestFinishing,
            "peosp-onf" => PolicyEvaluation::OnceStartPrecedesOldestNewestFinishing,
            "peosp-omf" => PolicyEvaluation::OnceStartPrecedesOldestMidFinishing,
            "peosp-mnf" => PolicyEvaluation::OnceStartPrecedesMidNewestFinishing,
            "peosp-as" => PolicyEvaluation::OnceStartPrecedesAllStarting,
            "peosp-os" => PolicyEvaluation::OnceStartPrecedesOldestStarting,
            "peosp-ms" => PolicyEvaluation::OnceStartPrecedesMidStarting,
            "peosp-ns" => PolicyEvaluation::OnceStartPrecedesNewestStarting,
            "peosp-ons" => PolicyEvaluation::OnceStartPrecedesOldestNewestStarting,
            "peosp-oms" => PolicyEvaluation::OnceStartPrecedesOldestMidStarting,
            "peosp-mns" => PolicyEvaluation::OnceStartPrecedesMidNewestStarting,

            "peosf-af" => PolicyEvaluation::OnceStartFollowsAllFinishing,
            "peosf-of" => PolicyEvaluation::OnceStartFollowsOldestFinishing,
            "peosf-mf" => PolicyEvaluation::OnceStartFollowsMidFinishing,
            "peosf-nf" => PolicyEvaluation::OnceStartFollowsNewestFinishing,
            "peosf-onf" => PolicyEvaluation::OnceStartFollowsOldestNewestFinishing,
            "peosf-omf" => PolicyEvaluation::OnceStartFollowsOldestMidFinishing,
            "peosf-mnf" => PolicyEvaluation::OnceStartFollowsMidNewestFinishing,
            "peosf-as" => PolicyEvaluation::OnceStartFollowsAllStarting,
            "peosf-os" => PolicyEvaluation::OnceStartFollowsOldestStarting,
            "peosf-ms" => PolicyEvaluation::OnceStartFollowsMidStarting,
            "peosf-ns" => PolicyEvaluation::OnceStartFollowsNewestStarting,
            "peosf-ons" => PolicyEvaluation::OnceStartFollowsOldestNewestStarting,
            "peosf-oms" => PolicyEvaluation::OnceStartFollowsOldestMidStarting,
            "peosf-mns" => PolicyEvaluation::OnceStartFollowsMidNewestStarting,

            "peoef" => PolicyEvaluation::OnceEndFollows,
            "peoep" => PolicyEvaluation::OnceEndPrecedes,
            "peosfef" => PolicyEvaluation::OnceStartFollowsEndFollows,
            "peoefsf" => PolicyEvaluation::OnceEndFollowsStartFollows,
            "peospef" => PolicyEvaluation::OnceStartPrecedesEndFollows,
            "peoepsf" => PolicyEvaluation::OnceEndPrecedesStartFollows,
            "peospep" => PolicyEvaluation::OnceStartPrecedesEndPrecedes,
            "peoepsp" => PolicyEvaluation::OnceEndPrecedesStartPrecedes,
            _ => panic!(
                "Invalid string specified for policy evaluation: {}",
                s
            ),
        }
    }

    pub fn get_extra_chunk_before_test_case(&self) -> ExtraChunkTime {
        match self {

            PolicyEvaluation::ProgressiveAllFinishing
            | PolicyEvaluation::ProgressiveOldestFinishing
            | PolicyEvaluation::ProgressiveMidFinishing
            | PolicyEvaluation::ProgressiveNewestFinishing
            | PolicyEvaluation::ProgressiveOldestNewestFinishing
            | PolicyEvaluation::ProgressiveOldestMidFinishing
            | PolicyEvaluation::ProgressiveMidNewestFinishing
            | PolicyEvaluation::ProgressiveAllStarting
            | PolicyEvaluation::ProgressiveOldestStarting
            | PolicyEvaluation::ProgressiveMidStarting
            | PolicyEvaluation::ProgressiveNewestStarting
            | PolicyEvaluation::ProgressiveOldestNewestStarting
            | PolicyEvaluation::ProgressiveOldestMidStarting
            | PolicyEvaluation::ProgressiveMidNewestStarting
            => ExtraChunkTime::None,

            PolicyEvaluation::OnceStartPrecedesAllFinishing
            | PolicyEvaluation::OnceStartPrecedesOldestFinishing
            | PolicyEvaluation::OnceStartPrecedesMidFinishing
            | PolicyEvaluation::OnceStartPrecedesNewestFinishing
            | PolicyEvaluation::OnceStartPrecedesOldestNewestFinishing
            | PolicyEvaluation::OnceStartPrecedesOldestMidFinishing
            | PolicyEvaluation::OnceStartPrecedesMidNewestFinishing
            | PolicyEvaluation::OnceStartPrecedesAllStarting
            | PolicyEvaluation::OnceStartPrecedesOldestStarting
            | PolicyEvaluation::OnceStartPrecedesMidStarting
            | PolicyEvaluation::OnceStartPrecedesNewestStarting
            | PolicyEvaluation::OnceStartPrecedesOldestNewestStarting
            | PolicyEvaluation::OnceStartPrecedesOldestMidStarting
            | PolicyEvaluation::OnceStartPrecedesMidNewestStarting
            => ExtraChunkTime::Precedes,
            
            PolicyEvaluation::OnceStartFollowsAllFinishing
            | PolicyEvaluation::OnceStartFollowsOldestFinishing
            | PolicyEvaluation::OnceStartFollowsMidFinishing
            | PolicyEvaluation::OnceStartFollowsNewestFinishing
            | PolicyEvaluation::OnceStartFollowsOldestNewestFinishing
            | PolicyEvaluation::OnceStartFollowsOldestMidFinishing
            | PolicyEvaluation::OnceStartFollowsMidNewestFinishing
            | PolicyEvaluation::OnceStartFollowsAllStarting
            | PolicyEvaluation::OnceStartFollowsOldestStarting
            | PolicyEvaluation::OnceStartFollowsMidStarting
            | PolicyEvaluation::OnceStartFollowsNewestStarting
            | PolicyEvaluation::OnceStartFollowsOldestNewestStarting
            | PolicyEvaluation::OnceStartFollowsOldestMidStarting
            | PolicyEvaluation::OnceStartFollowsMidNewestStarting
            => ExtraChunkTime::Follows,

            PolicyEvaluation::OnceEndFollows => ExtraChunkTime::None,
            PolicyEvaluation::OnceEndPrecedes => ExtraChunkTime::None,
            PolicyEvaluation::OnceStartFollowsEndFollows => ExtraChunkTime::Follows,
            PolicyEvaluation::OnceEndFollowsStartFollows => ExtraChunkTime::Follows,
            PolicyEvaluation::OnceStartPrecedesEndFollows => ExtraChunkTime::Precedes,
            PolicyEvaluation::OnceEndPrecedesStartFollows => ExtraChunkTime::Follows,
            PolicyEvaluation::OnceStartPrecedesEndPrecedes => ExtraChunkTime::Precedes,
            PolicyEvaluation::OnceEndPrecedesStartPrecedes => ExtraChunkTime::Precedes,
        }
    }

    pub fn get_extra_chunk_after_test_case(&self) -> ExtraChunkTime {
        
        match self {
            PolicyEvaluation::ProgressiveAllFinishing
            | PolicyEvaluation::ProgressiveOldestFinishing
            | PolicyEvaluation::ProgressiveMidFinishing
            | PolicyEvaluation::ProgressiveNewestFinishing
            | PolicyEvaluation::ProgressiveOldestNewestFinishing
            | PolicyEvaluation::ProgressiveOldestMidFinishing
            | PolicyEvaluation::ProgressiveMidNewestFinishing
            | PolicyEvaluation::ProgressiveAllStarting
            | PolicyEvaluation::ProgressiveOldestStarting
            | PolicyEvaluation::ProgressiveMidStarting
            | PolicyEvaluation::ProgressiveNewestStarting
            | PolicyEvaluation::ProgressiveOldestNewestStarting
            | PolicyEvaluation::ProgressiveOldestMidStarting
            | PolicyEvaluation::ProgressiveMidNewestStarting
            => ExtraChunkTime::None,

            PolicyEvaluation::OnceStartPrecedesAllFinishing
            | PolicyEvaluation::OnceStartPrecedesOldestFinishing
            | PolicyEvaluation::OnceStartPrecedesMidFinishing
            | PolicyEvaluation::OnceStartPrecedesNewestFinishing
            | PolicyEvaluation::OnceStartPrecedesOldestNewestFinishing
            | PolicyEvaluation::OnceStartPrecedesOldestMidFinishing
            | PolicyEvaluation::OnceStartPrecedesMidNewestFinishing
            | PolicyEvaluation::OnceStartPrecedesAllStarting
            | PolicyEvaluation::OnceStartPrecedesOldestStarting
            | PolicyEvaluation::OnceStartPrecedesMidStarting
            | PolicyEvaluation::OnceStartPrecedesNewestStarting
            | PolicyEvaluation::OnceStartPrecedesOldestNewestStarting
            | PolicyEvaluation::OnceStartPrecedesOldestMidStarting
            | PolicyEvaluation::OnceStartPrecedesMidNewestStarting
            => ExtraChunkTime::None,
            
            PolicyEvaluation::OnceStartFollowsAllFinishing
            | PolicyEvaluation::OnceStartFollowsOldestFinishing
            | PolicyEvaluation::OnceStartFollowsMidFinishing
            | PolicyEvaluation::OnceStartFollowsNewestFinishing
            | PolicyEvaluation::OnceStartFollowsOldestNewestFinishing
            | PolicyEvaluation::OnceStartFollowsOldestMidFinishing
            | PolicyEvaluation::OnceStartFollowsMidNewestFinishing
            | PolicyEvaluation::OnceStartFollowsAllStarting
            | PolicyEvaluation::OnceStartFollowsOldestStarting
            | PolicyEvaluation::OnceStartFollowsMidStarting
            | PolicyEvaluation::OnceStartFollowsNewestStarting
            | PolicyEvaluation::OnceStartFollowsOldestNewestStarting
            | PolicyEvaluation::OnceStartFollowsOldestMidStarting
            | PolicyEvaluation::OnceStartFollowsMidNewestStarting
            => ExtraChunkTime::None,

            PolicyEvaluation::OnceEndFollows => ExtraChunkTime::Follows,
            PolicyEvaluation::OnceEndPrecedes => ExtraChunkTime::Precedes,
            PolicyEvaluation::OnceStartFollowsEndFollows => ExtraChunkTime::Follows,
            PolicyEvaluation::OnceEndFollowsStartFollows => ExtraChunkTime::Follows,
            PolicyEvaluation::OnceStartPrecedesEndFollows => ExtraChunkTime::Follows,
            PolicyEvaluation::OnceEndPrecedesStartFollows => ExtraChunkTime::Precedes,
            PolicyEvaluation::OnceStartPrecedesEndPrecedes => ExtraChunkTime::Precedes,
            PolicyEvaluation::OnceEndPrecedesStartPrecedes => ExtraChunkTime::Precedes,
        }
    }

    pub fn generate_starting_policy_v() -> Vec<PolicyEvaluation> {
        return vec![
            PolicyEvaluation::ProgressiveAllStarting, 
            PolicyEvaluation::ProgressiveOldestStarting, 
            PolicyEvaluation::ProgressiveMidStarting, 
            PolicyEvaluation::ProgressiveNewestStarting, 
            PolicyEvaluation::ProgressiveOldestNewestStarting,
            PolicyEvaluation::ProgressiveOldestMidStarting,
            PolicyEvaluation::ProgressiveMidNewestStarting,
            PolicyEvaluation::OnceStartPrecedesAllStarting,
            PolicyEvaluation::OnceStartPrecedesOldestStarting,
            PolicyEvaluation::OnceStartPrecedesMidStarting,
            PolicyEvaluation::OnceStartPrecedesNewestStarting,
            PolicyEvaluation::OnceStartPrecedesOldestNewestStarting,
            PolicyEvaluation::OnceStartPrecedesOldestMidStarting,
            PolicyEvaluation::OnceStartPrecedesMidNewestStarting,
            PolicyEvaluation::OnceStartFollowsAllStarting,
            PolicyEvaluation::OnceStartFollowsOldestStarting,
            PolicyEvaluation::OnceStartFollowsMidStarting,
            PolicyEvaluation::OnceStartFollowsNewestStarting,
            PolicyEvaluation::OnceStartFollowsOldestNewestStarting,
            PolicyEvaluation::OnceStartFollowsOldestMidStarting,
            PolicyEvaluation::OnceStartFollowsMidNewestStarting,
        ]
    } 

    pub fn generate_finishing_policy_v() -> Vec<PolicyEvaluation> {
        return vec![
            PolicyEvaluation::ProgressiveAllFinishing, 
            PolicyEvaluation::ProgressiveOldestFinishing, 
            PolicyEvaluation::ProgressiveMidFinishing, 
            PolicyEvaluation::ProgressiveNewestFinishing, 
            PolicyEvaluation::ProgressiveOldestNewestFinishing,
            PolicyEvaluation::ProgressiveOldestMidFinishing,
            PolicyEvaluation::ProgressiveMidNewestFinishing,
            PolicyEvaluation::OnceStartPrecedesAllFinishing,
            PolicyEvaluation::OnceStartPrecedesOldestFinishing,
            PolicyEvaluation::OnceStartPrecedesMidFinishing,
            PolicyEvaluation::OnceStartPrecedesNewestFinishing,
            PolicyEvaluation::OnceStartPrecedesOldestNewestFinishing,
            PolicyEvaluation::OnceStartPrecedesOldestMidFinishing,
            PolicyEvaluation::OnceStartPrecedesMidNewestFinishing,
            PolicyEvaluation::OnceStartFollowsAllFinishing,
            PolicyEvaluation::OnceStartFollowsOldestFinishing,
            PolicyEvaluation::OnceStartFollowsMidFinishing,
            PolicyEvaluation::OnceStartFollowsNewestFinishing,
            PolicyEvaluation::OnceStartFollowsOldestNewestFinishing,
            PolicyEvaluation::OnceStartFollowsOldestMidFinishing,
            PolicyEvaluation::OnceStartFollowsMidNewestFinishing,
        ]
    } 
}
