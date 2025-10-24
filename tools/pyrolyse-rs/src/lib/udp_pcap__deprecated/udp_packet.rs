use crate::misc::ip_addr_container_generic::IpAddrGeneric;
use crate::misc::test_target::TestTarget;

// use pnet::util::MacAddr;

#[derive(Debug, Clone)]
pub struct UdpPacket<I> {
    // TODO: add time: t_sec, t_usec
    // ts_sec: u32,
    // mac_src_addr: MacAddr,
    // mac_dst_addr: MacAddr,
    // ip_src: I,
    // ip_dst: I,
    test_target: TestTarget<I>,
    ip_id: u16,
    ip_more_fragment_flag: bool,
    ip_offset: u16,
    payload_data_v: Vec<u8>,
}

impl<I: Copy + IpAddrGeneric> UdpPacket<I> {
    pub fn new(
        // ts_sec: u32,
        // mac_src_addr: MacAddr,
        // mac_dst_addr: MacAddr,
        // ip_src: I,
        // ip_dst: I,
        test_target: TestTarget<I>,
        ip_id: u16,
        ip_more_fragment_flag: bool,
        ip_offset: u16,
        payload_data_v: Vec<u8>,
    ) -> UdpPacket<I> {
        UdpPacket {
            // ts_sec,
            // mac_src_addr,
            // mac_dst_addr,
            // ip_src,
            // ip_dst,
            test_target,
            ip_id,
            ip_more_fragment_flag,
            ip_offset,
            payload_data_v,
        }
    }

    pub fn build_ethernet_pdu_data_v(&self) -> Vec<u8> {
        // let test_target = TestTarget::new(
        //     self.mac_src_addr,
        //     self.mac_dst_addr,
        //     self.ip_src,
        //     self.ip_dst,
        // );
        I::build_ethernet_pdu_data_v_for_udp(
            &self.test_target,
            self.ip_id,
            // Next line does the same as: if self.ip_more_fragment_flag { 1 } else { 0 },
            u8::from(self.ip_more_fragment_flag),
            self.ip_offset,
            &self.payload_data_v,
        )
    }
}
