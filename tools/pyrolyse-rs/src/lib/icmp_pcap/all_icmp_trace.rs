use std::fmt::Debug;
use std::path::Path;

use crate::byte_time_data::pair_triplet_byte_time_sequence::PairTripletByteTimeSequence;
use crate::icmp_pcap::icmp_trace::IcmpTraceC;
use crate::icmp_pcap::policy_evaluation::PolicyEvaluation;
use crate::misc::ip_addr_container_generic::IpAddrGeneric;
use crate::misc::test_target::TestTarget;

/// Contains IcmpPacket for relation/chunk pairs and triplets.
// #[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct AllIcmpTrace<I> {
    // chunk_pattern_v: Vec<String>,
    pair_icmp_trace_c: IcmpTraceC<I>,
    triplet_icmp_trace_c: IcmpTraceC<I>,
}

impl<I: Debug + Copy + IpAddrGeneric> AllIcmpTrace<I> {
    pub fn new(
        pair_icmp_trace_c: IcmpTraceC<I>,
        triplet_icmp_trace_c: IcmpTraceC<I>,
    ) -> AllIcmpTrace<I> {
        AllIcmpTrace {
            pair_icmp_trace_c,
            triplet_icmp_trace_c,
        }
    }

    pub fn of_data(
        test_target: &TestTarget<I>,
        policy_evaluation: &PolicyEvaluation,
        pair_triplet_byte_time_sequence: &PairTripletByteTimeSequence,
    ) -> AllIcmpTrace<I> {
        debug!("of_data: start");

        let internet_checksum_chunk_pattern_v =
            pair_triplet_byte_time_sequence.get_internet_checksum_chunk_pattern_v();

        let pair_icmp_trace_c = IcmpTraceC::of_data_byte_time_pair_sequence_c(
            test_target,
            policy_evaluation,
            internet_checksum_chunk_pattern_v,
            pair_triplet_byte_time_sequence.get_byte_time_sequence_c_pair(),
        );
        let triplet_icmp_trace_c = IcmpTraceC::of_data_byte_time_triplet_sequence_c(
            test_target,
            policy_evaluation,
            internet_checksum_chunk_pattern_v,
            pair_triplet_byte_time_sequence.get_byte_time_sequence_c_triplet(),
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
