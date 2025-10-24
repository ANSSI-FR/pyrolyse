// use crate::misc::interval::IntervalC;
// use itertools::Itertools;
// use std::fs::File;
// use std::io;
// use std::iter::Iterator;
use std::path::Path;

// use serde::{Deserialize, Serialize};

use crate::byte_time_data::pair_triplet_byte_time_sequence::PairTripletByteTimeSequence;
use crate::icmp_pcap::icmp_trace_deprecated::IcmpTraceC;
use crate::misc::policy_evaluation::PolicyEvaluation;
use crate::misc::test_target_deprecated::TestTarget;

/// Contains IcmpPacket for relation/chunk pairs and triplets.
// #[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct AllIcmpTrace {
    // chunk_pattern_v: Vec<String>,
    pair_icmp_trace_c: IcmpTraceC,
    triplet_icmp_trace_c: IcmpTraceC,
}

impl AllIcmpTrace {
    pub fn new(pair_icmp_trace_c: IcmpTraceC, triplet_icmp_trace_c: IcmpTraceC) -> AllIcmpTrace {
        AllIcmpTrace {
            pair_icmp_trace_c,
            triplet_icmp_trace_c,
        }
    }

    pub fn of_data(
        internet_checksum_chunk_pattern_v: Vec<String>,
        test_target: &TestTarget,
        policy_evaluation: &PolicyEvaluation,
        byte_time_sequence: &PairTripletByteTimeSequence,
    ) -> AllIcmpTrace {
        debug!("of_data: start");

        // let internet_checksum_chunk_pattern_v =
        //     byte_time_sequence.get_internet_checksum_chunk_pattern_v();

        let pair_icmp_trace_c = IcmpTraceC::of_data_byte_time_pair_sequence_c(
            test_target,
            policy_evaluation,
            &internet_checksum_chunk_pattern_v,
            byte_time_sequence.get_byte_time_sequence_c_pair(),
        );
        let triplet_icmp_trace_c = IcmpTraceC::of_data_byte_time_triplet_sequence_c(
            test_target,
            policy_evaluation,
            &internet_checksum_chunk_pattern_v,
            byte_time_sequence.get_byte_time_sequence_c_triplet(),
        );

        debug!("of_data: end");
        AllIcmpTrace::new(pair_icmp_trace_c, triplet_icmp_trace_c)
    }

    pub fn export(&self, pcap_directory_path: &Path) {
        debug!("export: start");
        self.pair_icmp_trace_c.export(pcap_directory_path);
        self.triplet_icmp_trace_c.export(pcap_directory_path);

        debug!("export: end");
    }
}
