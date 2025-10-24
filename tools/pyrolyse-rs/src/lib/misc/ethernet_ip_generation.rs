// use std::fs::File;
// use std::io;
// use std::io::Write;
// use std::path::Path;
// use std::net::IpAddr;
use std::fmt::Debug;
use std::fmt::Display;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

use pnet::packet::ethernet;
// use pnet::packet::ethernet::EtherType;
use pnet::packet::ip::IpNextHeaderProtocol;
// use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4;
use pnet::packet::ipv6;
// use pnet::packet::udp;
// use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
// use pnet::packet::icmp;
// use pnet::packet::icmp::{echo_request, IcmpCode, IcmpPacket, IcmpTypes};
// use pnet::packet::icmpv6;

// use pcap_parser::ToVec;
// use pcap_parser::{LegacyPcapBlock, Linktype};

use crate::misc::ip_addr_fragmentation_trait_impl::IpAddrForFragmentationTesting;
use crate::misc::test_target::TestTarget;
// use crate::misc::pcap_utils;

pub fn build_ethernet_pdu_payload<
    I: Display + Debug + PartialEq + Clone + IpAddrForFragmentationTesting,
>(
    test_target: &TestTarget<I>,
    // id: u16,
    // fragment_flag: u8,
    // fragment_offset: u16,
    ethernet_payload: &[u8],
) -> Vec<u8> {
    let ethernet_payload_len = ethernet_payload.len();

    // EtherType::new(0x0800)
    let ether_type = test_target.get_ether_type();

    // let ip_header_len = 20;
    // let ip_pdu_len = ip_header_len + payload_len;

    let ethernet_header_len = 14;
    let ethernet_packet_len = ethernet_header_len + ethernet_payload_len;

    // let mut ethernet_payload_data_like_v: Vec<u8> = vec![0; ethernet_payload_len];

    let mut ethernet_data_like_v: Vec<u8> = vec![0; ethernet_packet_len];

    let mut ethernet_packet =
        ethernet::MutableEthernetPacket::new(&mut ethernet_data_like_v[..]).unwrap();
    ethernet_packet.set_source(test_target.mac_addr_src);
    ethernet_packet.set_destination(test_target.mac_addr_dst);
    ethernet_packet.set_ethertype(ether_type);
    ethernet_packet.set_payload(ethernet_payload);
    let ethernet_pdu_v = ethernet_packet.packet().to_vec();
    // debug!("ethernet_packet: {:?}", ethernet_packet);
    // debug!("ethernet_pdu_v: {:?}", ethernet_pdu_v);

    ethernet_pdu_v
}

pub fn build_ipv4_pdu_payload(
    test_target: &TestTarget<Ipv4Addr>,
    // test_index_offset: u16,
    // test_index: u16,
    ip_id: u16,
    fragment_flag: u8,
    fragment_offset: u16,
    ip_protocol: IpNextHeaderProtocol,
    ip_payload: &[u8],
) -> Vec<u8> {
    let ip_payload_len = ip_payload.len();

    let ip_header_len = 20;
    let ip_packet_len = ip_header_len + ip_payload_len;

    let mut ipv4_data_like_v: Vec<u8> = vec![0; ip_packet_len];

    // let mut ipv4_ethernet = Vec::<u8>::new();

    let ipv4 = ipv4::Ipv4 {
        version: 4,
        header_length: (ip_header_len / 4) as u8,
        dscp: 0,
        ecn: 0,
        total_length: ip_packet_len as u16,
        identification: ip_id,
        flags: fragment_flag,
        // flags: fragment_flag + ipv4::Ipv4Flags::DontFragment,
        fragment_offset,
        ttl: 64,
        next_level_protocol: ip_protocol,
        checksum: 0,
        //source: test_target.ip_src,
        //destination: test_target.ip_src,
        source: test_target.ip_addr_src,
        destination: test_target.ip_addr_dst,
        options: vec![],
        payload: ip_payload.to_vec(),
    };
    let mut mutable_ipv4_packet = ipv4::MutableIpv4Packet::new(&mut ipv4_data_like_v[..]).unwrap();
    mutable_ipv4_packet.populate(&ipv4);
    let checksum = ipv4::checksum(&mutable_ipv4_packet.to_immutable());
    mutable_ipv4_packet.set_checksum(checksum);
    let ipv4_pdu_data_v = mutable_ipv4_packet.packet().to_vec();
    // debug!("mutable_ipv4_packet: {:?}", mutable_ipv4_packet);
    // debug!("ipv4_pdu_data_v ({}): {:?}", ipv4_pdu_data_v.len(), ipv4_pdu_data_v);

    ipv4_pdu_data_v
}

pub fn build_ipv6_pdu_payload(
    test_target: &TestTarget<Ipv6Addr>,
    id: u16,
    fragment_flag: u8,
    fragment_offset: u16,
    ip_protocol: IpNextHeaderProtocol,
    ip_payload: &[u8],
) -> Vec<u8> {
    debug!("build_ethernet_ipv6: start");

    //let mut ipv6_add_src = match test_target_ipv6.ip_src {
    //    IpAddr::Ipv6Addr => {
    let ip_payload_len = ip_payload.len();

    let ipv6fragment_extension_header_header_len = 8;
    let ipv6_fragment_extension_header_packet_len =
        ipv6fragment_extension_header_header_len + ip_payload_len;
    debug!(
        "build_ethernet_ipv6: ipv6_fragment_extension_header_packet_len: {}",
        ipv6_fragment_extension_header_packet_len
    );

    let ipv6_header_len = 40;
    let ipv6_packet_len = ipv6_header_len + ipv6_fragment_extension_header_packet_len;
    debug!("build_ethernet_ipv6: ipv6_packet_len: {}", ipv6_packet_len);

    let ethernet_header_len = 14;
    let ethernet_packet_len = ethernet_header_len + ipv6_packet_len;
    debug!(
        "build_ethernet_ipv6: ethernet_packet_len: {}",
        ethernet_packet_len
    );

    // We shift fragment_offset 3 bits to the left and add the MF bit from fragment_flag (last bit).
    let fragment_offset_with_flags = fragment_offset * 8 + fragment_flag as u16 % 2;

    let mut fragment_data_like_v: Vec<u8> = vec![0; ipv6_fragment_extension_header_packet_len];
    let fragment = ipv6::Fragment {
        //next_header: IpNextHeaderProtocol::new(1),
        // IpNextHeaderProtocol::new(58) = Ipv6Icmp
        next_header: ip_protocol,
        reserved: 0,
        fragment_offset_with_flags,
        id: id as u32,
        payload: ip_payload.to_vec(),
    };
    let mut mutable_fragment_packet =
        ipv6::MutableFragmentPacket::new(&mut fragment_data_like_v[..]).unwrap();
    mutable_fragment_packet.populate(&fragment);

    debug!(
        "build_ethernet_ipv6: mutable_fragment_packet.packet().to_vec(): {:?}",
        mutable_fragment_packet.packet().to_vec()
    );

    let mut ipv6_data_like_v: Vec<u8> = vec![0; ipv6_packet_len];

    // let mut ipv6_ethernet = Vec::<u8>::new();

    let ipv6 = ipv6::Ipv6 {
        version: 6,
        // TODO: fix this value
        traffic_class: 0,
        flow_label: id as u32,
        // TODO: add conversion with error
        payload_length: ipv6_fragment_extension_header_packet_len as u16,
        next_header: IpNextHeaderProtocol::new(44),
        hop_limit: 64,
        source: test_target.ip_addr_src,
        destination: test_target.ip_addr_dst,

        payload: mutable_fragment_packet.packet().to_vec(),
    };
    let mut mutable_ipv6_packet = ipv6::MutableIpv6Packet::new(&mut ipv6_data_like_v[..]).unwrap();
    mutable_ipv6_packet.populate(&ipv6);
    // let checksum = ipv4::checksum(&mutable_ipv4_packet.to_immutable());
    // mutable_ipv4_packet.set_checksum(checksum);
    let ipv6_pdu_data_v = mutable_ipv6_packet.packet().to_vec();
    // debug!("mutable_ipv4_packet: {:?}", mutable_ipv4_packet);
    // debug!("ipv6_pdu_data_v ({}): {:?}", ipv4_data_v.len(), ipv6_pdu_data_v);

    debug!("build_ethernet_ipv6: end");

    ipv6_pdu_data_v
}

pub fn build_ipv6_pdu_payload_tcp_testing(
    test_target: &TestTarget<Ipv6Addr>,
    id: u16,
    ip_payload: &[u8],
) -> Vec<u8> {
    debug!("build_ethernet_ipv6: start");

    //let mut ipv6_add_src = match test_target_ipv6.ip_src {
    //    IpAddr::Ipv6Addr => {
    let ip_payload_len = ip_payload.len();


    let ipv6_header_len = 40;
    let ipv6_packet_len = ipv6_header_len + ip_payload_len;
    debug!("build_ethernet_ipv6: ipv6_packet_len: {}", ipv6_packet_len);

    let ethernet_header_len = 14;
    let ethernet_packet_len = ethernet_header_len + ipv6_packet_len;
    debug!(
        "build_ethernet_ipv6: ethernet_packet_len: {}",
        ethernet_packet_len
    );

    let mut ipv6_data_like_v: Vec<u8> = vec![0; ipv6_packet_len];

    let ipv6 = ipv6::Ipv6 {
        version: 6,
        // TODO: fix this value
        traffic_class: 0,
        flow_label: id as u32,
        // TODO: add conversion with error
        payload_length: ip_payload_len as u16,
        next_header: IpNextHeaderProtocol::new(6),
        hop_limit: 64,
        source: test_target.ip_addr_src,
        destination: test_target.ip_addr_dst,

        payload: ip_payload.to_vec(),
    };
    let mut mutable_ipv6_packet = ipv6::MutableIpv6Packet::new(&mut ipv6_data_like_v[..]).unwrap();
    mutable_ipv6_packet.populate(&ipv6);
    // let checksum = ipv4::checksum(&mutable_ipv4_packet.to_immutable());
    // mutable_ipv4_packet.set_checksum(checksum);
    let ipv6_pdu_data_v = mutable_ipv6_packet.packet().to_vec();
    // debug!("mutable_ipv4_packet: {:?}", mutable_ipv4_packet);
    // debug!("ipv6_pdu_data_v ({}): {:?}", ipv4_data_v.len(), ipv6_pdu_data_v);

    debug!("build_ethernet_ipv6: end");

    ipv6_pdu_data_v
}