use std::net::IpAddr;

use crate::misc::icmp_echo_request_generation;
use crate::misc::test_target::TestTarget;

use pnet::util::MacAddr;

// TODO: rename to IcmpPacket
#[derive(Debug, Clone)]
pub struct IcmpPacket {
    // TODO: add time: t_sec, t_usec
    // ts_sec: u32,
    mac_src_addr: MacAddr,
    mac_dst_addr: MacAddr,
    ip_src: IpAddr,
    ip_dst: IpAddr,
    ip_id: u16,
    ip_more_fragment_flag: bool,
    ip_offset: u16,
    icmp_id: u16,
    icmp_sn: u16,
    data_v: Vec<u8>,
}

impl IcmpPacket {
    pub fn new(
        // ts_sec: u32,
        mac_src_addr: MacAddr,
        mac_dst_addr: MacAddr,
        ip_src: IpAddr,
        ip_dst: IpAddr,
        ip_id: u16,
        ip_more_fragment_flag: bool,
        ip_offset: u16,
        icmp_id: u16,
        icmp_sn: u16,
        data_v: Vec<u8>,
    ) -> IcmpPacket {
        IcmpPacket {
            // ts_sec,
            mac_src_addr,
            mac_dst_addr,
            ip_src,
            ip_dst,
            ip_id,
            ip_more_fragment_flag,
            ip_offset,
            icmp_id,
            icmp_sn,
            data_v,
        }
    }

    pub fn build_ethernet(&self) -> Vec<u8> {
        match self.ip_src {
            IpAddr::V4(_ip4) => {
                let test_target = TestTarget::new(
                    self.mac_src_addr,
                    self.mac_dst_addr,
                    self.ip_src,
                    self.ip_dst,
                );
                icmp_echo_request_generation::build_ethernet_ipv4(
                    &test_target,

                    self.ip_id,
                    // Next line does the same as: if self.ip_more_fragment_flag { 1 } else { 0 },
                    u8::from(self.ip_more_fragment_flag),
                    self.ip_offset,
                    &self.data_v,
                )
            }
            IpAddr::V6(_ip6) => {
                let test_target = TestTarget::new(
                    self.mac_src_addr,
                    self.mac_dst_addr,
                    self.ip_src,
                    self.ip_dst,
                );
                icmp_echo_request_generation::build_ethernet_ipv6(
                    &test_target,
                    self.ip_id,
                    u8::from(self.ip_more_fragment_flag),
                    self.ip_offset,
                    &self.data_v,
                )
               }
        }
    }
}
