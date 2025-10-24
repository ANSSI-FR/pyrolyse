use std::fmt::Debug;
use std::net::IpAddr;
use std::path::Path;

use pnet::packet::ip::IpNextHeaderProtocol;

use crate::byte_time_data::pair_triplet_byte_time_sequence::PairTripletByteTimeSequence;
use crate::misc::ip_addr_fragmentation_trait_impl::IpAddrForFragmentationTesting;
use crate::misc::policy_evaluation::PolicyEvaluation;
use crate::misc::test_index::TestIndex;
use crate::misc::test_target::TestTarget;
use crate::pcap::testing_trace::TestingTraceC;
use crate::position::payload_mode::PayloadMode;

/// Contains TestingPacket for relation/chunk pairs and triplets.
// #[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct AllTestingTrace<I: Into<IpAddr>> {
    // chunk_pattern_v: Vec<String>,
    pair_testing_trace_c: TestingTraceC<I>,
    triplet_testing_trace_c: TestingTraceC<I>,
}

impl<I: Debug + Copy + Into<IpAddr> + IpAddrForFragmentationTesting> AllTestingTrace<I> {
    pub fn new(
        pair_testing_trace_c: TestingTraceC<I>,
        triplet_testing_trace_c: TestingTraceC<I>,
    ) -> AllTestingTrace<I> {
        AllTestingTrace {
            pair_testing_trace_c,
            triplet_testing_trace_c,
        }
    }

    pub fn of_data(
        internet_checksum_chunk_pattern_sl: &[u8],
        test_target: &TestTarget<I>,
        policy_evaluation: &PolicyEvaluation,
        test_index_offset: TestIndex,
        pair_triplet_byte_time_sequence: &PairTripletByteTimeSequence,
        payload_mode: &PayloadMode,
    ) -> AllTestingTrace<I> {
        debug!("of_data: start");

        // let internet_checksum_chunk_pattern_v =
        //     pair_triplet_byte_time_sequence.get_internet_checksum_chunk_pattern_v();

        let pair_icmp_trace_c = TestingTraceC::of_data_byte_time_pair_sequence_c(
            // internet_checksum_chunk_pattern_v,
            test_target,
            policy_evaluation,
            internet_checksum_chunk_pattern_sl,
            test_index_offset,
            pair_triplet_byte_time_sequence.get_byte_time_sequence_c_pair(),
            payload_mode
        );
        let triplet_icmp_trace_c = TestingTraceC::of_data_byte_time_triplet_sequence_c(
            // internet_checksum_chunk_pattern_v,
            test_target,
            policy_evaluation,
            internet_checksum_chunk_pattern_sl,
            test_index_offset,
            pair_triplet_byte_time_sequence.get_byte_time_sequence_c_triplet(),
            payload_mode
        );

        debug!("of_data: end");
        AllTestingTrace::new(pair_icmp_trace_c, triplet_icmp_trace_c)
    }

    pub fn export(
        &self,
        payload_protocol: IpNextHeaderProtocol,
        pcap_directory_path: &Path,
        test_index_offset: TestIndex,
    ) {
        debug!("export: start");
        debug!("export: pair");
        self.pair_testing_trace_c
            .export(payload_protocol, pcap_directory_path, test_index_offset);
        debug!("export: triplet");
        self.triplet_testing_trace_c.export(
            payload_protocol,
            pcap_directory_path,
            test_index_offset,
        );
        debug!("export: end");
    }
}
