use std::net::IpAddr;

use crate::misc::ip_addr_fragmentation_trait_impl::IpAddrForFragmentationTesting;
use crate::misc::test_index::TestIndex;
use crate::misc::test_target::TestTarget;

#[derive(Debug, Clone)]
pub struct TestingPacket<I: Into<IpAddr>> {
    // TODO: add time: t_sec, t_usec
    // ts_sec: u32,
    test_target: TestTarget<I>,
    test_index: TestIndex,
    ip_more_fragment_flag: bool,
    ip_offset: u16,
    payload_data_v: Vec<u8>,
}

impl<I: Copy + Into<IpAddr> + IpAddrForFragmentationTesting> TestingPacket<I> {
    pub fn new(
        // ts_sec: u32,
        test_target: TestTarget<I>,
        test_index: TestIndex,
        ip_more_fragment_flag: bool,
        ip_offset: u16,
        payload_data_v: Vec<u8>,
    ) -> TestingPacket<I> {
        TestingPacket {
            test_target,
            test_index,
            ip_more_fragment_flag,
            ip_offset,
            payload_data_v,
        }
    }

    pub fn get_test_index(&self) -> TestIndex {
        self.test_index
    }

    pub fn build_ethernet_pdu_data_v_for_icmp(&self, ip_id: u16) -> Vec<u8> {
        // let ip_id = test_index_offset + self.test_index;
        I::build_ethernet_pdu_data_v_for_icmp(
            &self.test_target,
            ip_id,
            // Next line does the same as: if self.ip_more_fragment_flag { 1 } else { 0 },
            u8::from(self.ip_more_fragment_flag),
            self.ip_offset,
            &self.payload_data_v,
        )
    }

    pub fn build_ethernet_pdu_data_v_for_udp(&self, ip_id: u16) -> Vec<u8> {
        // let ip_id = test_index_offset + self.test_index;
        I::build_ethernet_pdu_data_v_for_udp(
            &self.test_target,
            ip_id,
            // Next line does the same as: if self.ip_more_fragment_flag { 1 } else { 0 },
            u8::from(self.ip_more_fragment_flag),
            self.ip_offset,
            &self.payload_data_v,
        )
    }
}
