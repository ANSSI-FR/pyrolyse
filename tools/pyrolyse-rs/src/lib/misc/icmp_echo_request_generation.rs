use pnet::packet::icmp;
use pnet::packet::icmp::{echo_request, IcmpCode, IcmpPacket, IcmpTypes};
use pnet::packet::icmpv6;
use pnet::packet::Packet;

use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

pub fn build_icmpv4_pdu_data_v(
    // test_index_offset: u16,
    // test_index: u16,
    icmp_identifier: u16,
    icmp_sequence_number: u16,
    icmp_payload: &[u8],
    _ip_src: &Ipv4Addr,
    _ip_dst: &Ipv4Addr,
) -> Vec<u8> {
    debug!("build_icmpv4_pdu_data_v: start");

    // Old
    //let icmp_payload_len = icmp_payload.len();
    //let icmp_echo_request_header_len = 8;
    //let icmp_packet_len = icmp_echo_request_header_len + icmp_payload_len;

    let icmp_payload_len = icmp_payload.len();
    let icmp_echo_request_header_len = 8;
    let icmp_packet_len = icmp_echo_request_header_len + icmp_payload_len;

    let mut icmp_data_like_v: Vec<u8> = vec![0; icmp_packet_len];
    //let payload_v: Vec<u8> = vec![];

    // let mut icmp_data = Vec::<u8>::new();
    let echo_request = echo_request::EchoRequest {
        icmp_type: IcmpTypes::EchoRequest,
        icmp_code: IcmpCode::new(0),
        checksum: 0,
        // identifier: test_index_offset + test_index,
        // sequence_number: test_index_offset + test_index,
        identifier: icmp_identifier,
        sequence_number: icmp_sequence_number,
        payload: icmp_payload.to_vec(),
    };
    let mut mutable_echo_request_packet =
        echo_request::MutableEchoRequestPacket::new(&mut icmp_data_like_v[..]).unwrap();
    mutable_echo_request_packet.populate(&echo_request);
    let mutable_echo_request_packet_u8_s = mutable_echo_request_packet.packet();
    let icmp_packet = IcmpPacket::new(mutable_echo_request_packet_u8_s).unwrap();
    let checksum = icmp::checksum(&icmp_packet);
    mutable_echo_request_packet.set_checksum(checksum);
    debug!("build_icmpv4_pdu_data_v: end");
    mutable_echo_request_packet.packet().to_vec()

    // First pattern payload version
    //let icmp_payload_len = 8;
    //let icmp_echo_request_header_len = 8;
    //let icmp_packet_len = icmp_echo_request_header_len + icmp_payload_len;
    //
    //let mut icmp_data_like_v: Vec<u8> = vec![0; icmp_packet_len];
    //let first_pattern = &icmp_payload[..icmp_payload_len];
    //
    //// let mut icmp_data = Vec::<u8>::new();
    //
    //let echo_request = echo_request::EchoRequest {
    //    icmp_type: IcmpTypes::EchoRequest,
    //    icmp_code: IcmpCode::new(0),
    //    checksum: 0,
    //    // identifier: test_index_offset + test_index,
    //    // sequence_number: test_index_offset + test_index,
    //    identifier: icmp_identifier,
    //    sequence_number: icmp_sequence_number,
    //    payload: first_pattern.to_vec(),
    //};
    //let mut mutable_echo_request_packet =
    //    echo_request::MutableEchoRequestPacket::new(&mut icmp_data_like_v[..]).unwrap();
    //mutable_echo_request_packet.populate(&echo_request);
    //let mutable_echo_request_packet_u8_s = mutable_echo_request_packet.packet();
    //let icmp_packet = IcmpPacket::new(mutable_echo_request_packet_u8_s).unwrap();
    //let checksum = icmp::checksum(&icmp_packet);
    //mutable_echo_request_packet.set_checksum(checksum);
    //mutable_echo_request_packet.packet().to_vec()
}

pub fn build_icmpv6_pdu_data_v(
    // test_index_offset: u16,
    // test_index: u16,
    icmp_identifier: u16,
    icmp_sequence_number: u16,
    icmp_payload: &[u8],
    ip_src: &Ipv6Addr,
    ip_dst: &Ipv6Addr,
) -> Vec<u8> {
    debug!("build_icmpv6_pdu_data_v: start");
    let icmp_payload_len = icmp_payload.len();
    //let icmp_payload_len = 8;
    let icmp_echo_request_header_len = 8;
    let icmp_packet_len = icmp_echo_request_header_len + icmp_payload_len;

    let mut icmp_data_like_v: Vec<u8> = vec![0; icmp_packet_len];
    //let payload_v: Vec<u8> = vec![];

    // let mut icmp_data = Vec::<u8>::new();

    let echo_request = icmpv6::echo_request::EchoRequest {
        icmpv6_type: icmpv6::Icmpv6Types::EchoRequest,
        icmpv6_code: icmpv6::Icmpv6Code::new(0),
        checksum: 0,
        // identifier: test_index_offset + test_index,
        // sequence_number: test_index_offset + test_index,
        identifier: icmp_identifier,
        sequence_number: icmp_sequence_number,
        payload: icmp_payload.to_vec(),
        //payload: payload_v,
    };
    let mut mutable_echo_request_packet =
        icmpv6::echo_request::MutableEchoRequestPacket::new(&mut icmp_data_like_v[..]).unwrap();
    mutable_echo_request_packet.populate(&echo_request);
    let mutable_echo_request_packet_u8_s = mutable_echo_request_packet.packet();
    let icmp_packet = icmpv6::Icmpv6Packet::new(mutable_echo_request_packet_u8_s).unwrap();
    // XXX checksum issue here ?
    let checksum = icmpv6::checksum(&icmp_packet, ip_src, ip_dst);
    mutable_echo_request_packet.set_checksum(checksum);
    debug!("build_icmpv6_pdu_data_v: end");
    mutable_echo_request_packet.packet().to_vec()
}
