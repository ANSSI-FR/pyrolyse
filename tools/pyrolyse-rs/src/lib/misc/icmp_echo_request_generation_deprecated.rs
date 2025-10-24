use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use pnet::packet::ethernet;
use pnet::packet::ethernet::EtherType;
use pnet::packet::icmp;
use pnet::packet::icmp::{echo_request, IcmpCode, IcmpPacket, IcmpTypes};
use pnet::packet::icmpv6;
use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::ipv4;
use pnet::packet::ipv6;
use pnet::packet::Packet;

// use pcap_parser::ToVec;
// use pcap_parser::{LegacyPcapBlock, Linktype};

use crate::misc::test_target_deprecated::TestTarget;
use crate::misc::pcap_utils;

use std::net::IpAddr;

// pub fn pcap_write_header<W: Write>(to: &mut W, snaplen: usize) -> Result<usize, io::Error> {
//     let mut hdr = pcap_parser::PcapHeader::new();
//     hdr.snaplen = snaplen as u32;
//     hdr.network = Linktype::ETHERNET;
//     let s = hdr.to_vec().unwrap();
//     to.write_all(&s)?;
//     Ok(s.len())
// }

// pub fn pcap_write_packet<W: Write>(
//     to: &mut W,
//     ts_sec: u32,
//     ts_usec: u32,
//     data: &[u8],
// ) -> Result<usize, io::Error> {
//     let mut record = LegacyPcapBlock {
//         ts_sec,
//         ts_usec,
//         caplen: data.len() as u32,  // packet.header.caplen,
//         origlen: data.len() as u32, // packet.header.len,
//         data,
//     };
//     let s = record.to_vec().unwrap();
//     let sz = to.write(&s)?;

//     Ok(sz)
// }

pub fn build_icmp_data(
    icmp_identifier: u16,
    icmp_sequence_number: u16,
    payload: &[u8],
    ip_src: &IpAddr,
    ip_dst: &IpAddr,
) -> Vec<u8> {
    let payload_len = payload.len();
    let icmp_echo_request_header_len = 8;
    let icmp_packet_len = icmp_echo_request_header_len + payload_len;

    let mut icmp_data_like_v: Vec<u8> = vec![0; icmp_packet_len];

    let mut icmp_data = Vec::<u8>::new();

    match ip_src {
        IpAddr::V4(_ipaddr_src) => match ip_dst {
            IpAddr::V4(_ipaddr_dst) => {
                let echo_request = echo_request::EchoRequest {
                    icmp_type: IcmpTypes::EchoRequest,
                    icmp_code: IcmpCode::new(0),
                    checksum: 0,
                    identifier: icmp_identifier,
                    sequence_number: icmp_sequence_number,
                    payload: payload.to_vec(),
                };
                let mut mutable_echo_request_packet =
                    echo_request::MutableEchoRequestPacket::new(&mut icmp_data_like_v[..]).unwrap();
                mutable_echo_request_packet.populate(&echo_request);
                let mutable_echo_request_packet_u8_s = mutable_echo_request_packet.packet();
                let icmp_packet = IcmpPacket::new(mutable_echo_request_packet_u8_s).unwrap();
                let checksum = icmp::checksum(&icmp_packet);
                mutable_echo_request_packet.set_checksum(checksum);
                let icmp_data_v = mutable_echo_request_packet.packet().to_vec();
                icmp_data = icmp_data_v
            }
            IpAddr::V6(_ipaddr_dst) => println!("Mismatch between ip versions"),
        },
        IpAddr::V6(ipaddr_src) => match ip_dst {
            IpAddr::V6(ipaddr_dst) => {
                let echo_request = icmpv6::echo_request::EchoRequest {
                    icmpv6_type: icmpv6::Icmpv6Types::EchoRequest,
                    icmpv6_code: icmpv6::Icmpv6Code::new(0),
                    checksum: 0,
                    identifier: icmp_identifier,
                    sequence_number: icmp_sequence_number,
                    payload: payload.to_vec(),
                };
                let mut mutable_echo_request_packet =
                    icmpv6::echo_request::MutableEchoRequestPacket::new(&mut icmp_data_like_v[..])
                        .unwrap();
                mutable_echo_request_packet.populate(&echo_request);
                let mutable_echo_request_packet_u8_s = mutable_echo_request_packet.packet();
                let icmp_packet =
                    icmpv6::Icmpv6Packet::new(mutable_echo_request_packet_u8_s).unwrap();
                let checksum = icmpv6::checksum(&icmp_packet, ipaddr_src, ipaddr_dst);
                mutable_echo_request_packet.set_checksum(checksum);
                let icmp_data_v = mutable_echo_request_packet.packet().to_vec();
                icmp_data = icmp_data_v
            }
            IpAddr::V4(_ipaddr_dst) => println!("Mismatch between ip versions"),
        },
    }
    icmp_data
}

pub fn build_ethernet_ipv6(
    test_target_ipv6: &TestTarget,
    id: u16,
    fragment_flag: u8,
    fragment_offset: u16,
    payload: &[u8],
) -> Vec<u8> {
    debug!("build_ethernet_ipv6: start");

    //let mut ipv6_add_src = match test_target_ipv6.ip_src {
    //    IpAddr::Ipv6Addr => {
    let payload_len = payload.len();

    let ipv6fragment_extension_header_header_len = 8;
    let ipv6_fragment_extension_header_packet_len =
        ipv6fragment_extension_header_header_len + payload_len;
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
        next_header: IpNextHeaderProtocol::new(58),
        reserved: 0,
        fragment_offset_with_flags,
        id: id as u32,
        payload: payload.to_vec(),
    };
    let mut mutable_fragment_packet =
        ipv6::MutableFragmentPacket::new(&mut fragment_data_like_v[..]).unwrap();
    mutable_fragment_packet.populate(&fragment);

    debug!(
        "build_ethernet_ipv6: mutable_fragment_packet.packet().to_vec(): {:?}",
        mutable_fragment_packet.packet().to_vec()
    );

    let mut ipv6_data_like_v: Vec<u8> = vec![0; ipv6_packet_len];

    let mut ipv6_ethernet = Vec::<u8>::new();

    match test_target_ipv6.ip_src {
        IpAddr::V6(ipaddr_src) => {
            match test_target_ipv6.ip_dst {
                IpAddr::V6(ipaddr_dst) => {
                    let ipv6 = ipv6::Ipv6 {
                        version: 6,
                        // TODO: fix this value
                        traffic_class: 0,
                        flow_label: id as u32,
                        // TODO: add conversion with error
                        payload_length: ipv6_fragment_extension_header_packet_len as u16,
                        next_header: IpNextHeaderProtocol::new(44),
                        hop_limit: 64,
                        //source: test_target_ipv6.ip_src,
                        //destination: test_target_ipv6.ip_dst,
                        source: ipaddr_src,
                        destination: ipaddr_dst,

                        payload: mutable_fragment_packet.packet().to_vec(),
                    };
                    let mut mutable_ipv6_packet =
                        ipv6::MutableIpv6Packet::new(&mut ipv6_data_like_v[..]).unwrap();
                    mutable_ipv6_packet.populate(&ipv6);
                    // let checksum = ipv4::checksum(&mutable_ipv4_packet.to_immutable());
                    // mutable_ipv4_packet.set_checksum(checksum);
                    let ipv6_data_v = mutable_ipv6_packet.packet().to_vec();
                    // debug!("mutable_ipv4_packet: {:?}", mutable_ipv4_packet);
                    // debug!("ipv4_data_v ({}): {:?}", ipv4_data_v.len(), ipv4_data_v);

                    let mut ethernet_data_like_v: Vec<u8> = vec![0; ethernet_packet_len];

                    let mut ethernet_packet =
                        ethernet::MutableEthernetPacket::new(&mut ethernet_data_like_v[..])
                            .unwrap();
                    ethernet_packet.set_source(test_target_ipv6.macaddr_src);
                    ethernet_packet.set_destination(test_target_ipv6.macaddr_dst);
                    //ethernet_packet.set_ethertype(EtherType::new(0x0800));
                    ethernet_packet.set_ethertype(EtherType::new(0x86DD));
                    ethernet_packet.set_payload(&ipv6_data_v);
                    let ethernet_data = ethernet_packet.packet();
                    // debug!("ethernet_packet: {:?}", ethernet_packet);
                    // debug!("ethernet_data: {:?}", ethernet_data);

                    debug!("build_ethernet_ipv6: end");

                    ipv6_ethernet = ethernet_data.to_vec();
                }
                IpAddr::V4(_ipaddr_dst) => println!("Ipv version error in destination"),
            }
        }
        IpAddr::V4(_ipaddr_src) => println!("Ipv version error in source"),
    }
    ipv6_ethernet
}

pub fn build_ethernet_ipv4(
    test_target: &TestTarget,
    id: u16,
    fragment_flag: u8,
    fragment_offset: u16,
    payload: &[u8],
) -> Vec<u8> {
    let payload_len = payload.len();

    let ip_header_len = 20;
    let ip_packet_len = ip_header_len + payload_len;

    let ethernet_header_len = 14;
    let ethernet_packet_len = ethernet_header_len + ip_packet_len;

    let mut ipv4_data_like_v: Vec<u8> = vec![0; ip_packet_len];

    let mut ipv4_ethernet = Vec::<u8>::new();

    match test_target.ip_src {
        IpAddr::V4(ipaddr_src) => {
            match test_target.ip_dst {
                IpAddr::V4(ipaddr_dst) => {
                    let ipv4 = ipv4::Ipv4 {
                        version: 4,
                        header_length: (ip_header_len / 4) as u8,
                        dscp: 0,
                        ecn: 0,
                        total_length: ip_packet_len as u16,
                        identification: id,
                        flags: fragment_flag,
                        fragment_offset,
                        ttl: 64,
                        next_level_protocol: IpNextHeaderProtocol::new(1),
                        checksum: 0,
                        //source: test_target.ip_src,
                        //destination: test_target.ip_src,
                        source: ipaddr_src,
                        destination: ipaddr_dst,
                        options: vec![],
                        payload: payload.to_vec(),
                    };
                    let mut mutable_ipv4_packet =
                        ipv4::MutableIpv4Packet::new(&mut ipv4_data_like_v[..]).unwrap();
                    mutable_ipv4_packet.populate(&ipv4);
                    let checksum = ipv4::checksum(&mutable_ipv4_packet.to_immutable());
                    mutable_ipv4_packet.set_checksum(checksum);
                    let ipv4_data_v = mutable_ipv4_packet.packet().to_vec();
                    // debug!("mutable_ipv4_packet: {:?}", mutable_ipv4_packet);
                    // debug!("ipv4_data_v ({}): {:?}", ipv4_data_v.len(), ipv4_data_v);

                    let mut ethernet_data_like_v: Vec<u8> = vec![0; ethernet_packet_len];

                    let mut ethernet_packet =
                        ethernet::MutableEthernetPacket::new(&mut ethernet_data_like_v[..])
                            .unwrap();
                    ethernet_packet.set_source(test_target.macaddr_src);
                    ethernet_packet.set_destination(test_target.macaddr_dst);
                    ethernet_packet.set_ethertype(EtherType::new(0x0800));
                    ethernet_packet.set_payload(&ipv4_data_v);
                    let ethernet_data = ethernet_packet.packet();
                    // debug!("ethernet_packet: {:?}", ethernet_packet);
                    // debug!("ethernet_data: {:?}", ethernet_data);

                    ipv4_ethernet = ethernet_data.to_vec()
                }
                IpAddr::V6(_ipaddr_dst) => println!("Ipv version error in source"),
            }
        }
        IpAddr::V6(_ipaddr_src) => println!("Ipv version error in source"),
    }

    ipv4_ethernet
}

//pub fn build_ethernet_ipv6_icmp(
//    test_target: &TestTarget,
//    icmp_identifier: u16,
//    icmp_sequence_number: u16,
//    fragment_flag: u8,
//    fragment_offset: u16,
//    payload: &[u8],
//) -> Vec<u8> {
//    let payload_len = payload.len();
//    let icmp_echo_request_header_len = 8;
//    let icmp_packet_len = icmp_echo_request_header_len + payload_len;
//
//    let ip_header_len = 20;
//    let ip_packet_len = ip_header_len + icmp_packet_len;
//
//    let ethernet_header_len = 14;
//    let ethernet_packet_len = ethernet_header_len + ip_packet_len;
//
//    let mut icmp_data_like_v: Vec<u8> = vec![0; icmp_packet_len];
//
//    let echo_request = echo_request::EchoRequest {
//        icmp_type: IcmpTypes::EchoRequest,
//        icmp_code: IcmpCode::new(0),
//        checksum: 0,
//        identifier: icmp_identifier,
//        sequence_number: icmp_sequence_number,
//        payload: payload.to_vec(),
//    };
//    let mut mutable_echo_request_packet =
//        echo_request::MutableEchoRequestPacket::new(&mut icmp_data_like_v[..]).unwrap();
//    mutable_echo_request_packet.populate(&echo_request);
//    let mutable_echo_request_packet_u8s = mutable_echo_request_packet.packet();
//    let icmp_packet = IcmpPacket::new(mutable_echo_request_packet_u8s).unwrap();
//    let checksum = icmp::checksum(&icmp_packet);
//    mutable_echo_request_packet.set_checksum(checksum);
//    let icmp_data_v = mutable_echo_request_packet.packet().to_vec();
//    // debug!(
//    //     "mutable_echo_request_packet: {:?}",
//    //     mutable_echo_request_packet
//    // );
//    // debug!("icmp_data_v ({}): {:?}", icmp_data_v.len(), icmp_data_v);
//
//    let mut ipv4_data_like_v: Vec<u8> = vec![0; ip_packet_len];
//    let ipv4 = ipv4::Ipv4 {
//        version: 4,
//        header_length: (ip_header_len / 4) as u8,
//        dscp: 0,
//        ecn: 0,
//        total_length: ip_packet_len as u16,
//        identification: 0,
//        flags: fragment_flag,
//        fragment_offset,
//        ttl: 64,
//        next_level_protocol: IpNextHeaderProtocol::new(1),
//        checksum: 0,
//        source: test_target.ip_src,
//        destination: test_target.ip_src,
//        options: vec![],
//        payload: icmp_data_v,
//    };
//    let mut mutable_ipv4_packet = ipv4::MutableIpv4Packet::new(&mut ipv4_data_like_v[..]).unwrap();
//    mutable_ipv4_packet.populate(&ipv4);
//    let checksum = ipv4::checksum(&mutable_ipv4_packet.to_immutable());
//    mutable_ipv4_packet.set_checksum(checksum);
//    let ipv4_data_v = mutable_ipv4_packet.packet().to_vec();
//    // debug!("mutable_ipv4_packet: {:?}", mutable_ipv4_packet);
//    // debug!("ipv4_data_v ({}): {:?}", ipv4_data_v.len(), ipv4_data_v);
//
//    let mut ethernet_data_like_v: Vec<u8> = vec![0; ethernet_packet_len];
//
//    let mut ethernet_packet =
//        ethernet::MutableEthernetPacket::new(&mut ethernet_data_like_v[..]).unwrap();
//    ethernet_packet.set_source(test_target.macaddr_src);
//    ethernet_packet.set_destination(test_target.macaddr_dst);
//    ethernet_packet.set_ethertype(EtherType::new(0x0800));
//    ethernet_packet.set_payload(&ipv4_data_v);
//    let ethernet_data = ethernet_packet.packet();
//    // debug!("ethernet_packet: {:?}", ethernet_packet);
//    // debug!("ethernet_data: {:?}", ethernet_data);
//
//    ethernet_data.to_vec()
//}

pub fn simple_fragmentation_test(
    test_target_ipv6: &TestTarget,
    icmp_identifier: u16,
    icmp_sequence_number: u16,
) -> io::Result<()> {
    let data_s = "AABBCCDD".to_string();
    let data = data_s.as_bytes();
    let icmp_packet_data = build_icmp_data(
        // test_target.icmp_identifier,
        // test_target.icmp_sequence_number,
        icmp_identifier,
        icmp_sequence_number,
        data,
        &test_target_ipv6.ip_src,
        &test_target_ipv6.ip_dst,
    );
    let icmp_header = &icmp_packet_data[0..8].to_vec();

    let packey_0 = build_ethernet_ipv6(
        test_target_ipv6,
        // test_target_ipv6.macaddr_src,
        // test_target_ipv6.macaddr_dst,
        // test_target_ipv6.ipv4addr_src,
        // test_target_ipv6.ipv4addr_dst,
        111,
        1,
        0,
        // test_target,
        icmp_header,
    );

    let packey_1 = build_ethernet_ipv6(
        test_target_ipv6,
        // test_target_ipv6.macaddr_src,
        // test_target_ipv6.macaddr_dst,
        // test_target_ipv6.ipv4addr_src,
        // test_target_ipv6.ipv4addr_dst,
        111,
        0,
        1,
        // test_target,
        data,
    );

    let output_path_s = "simple_ICMP_fragmentation_test.pcap";
    let output_path = Path::new(output_path_s);
    println!("Value for w: {:?}", output_path);

    let mut output_file = if output_path
        .to_str()
        .expect("Could not convert output_path to string")
        == "-"
    {
        Box::new(io::stdout())
    } else {
        let file = File::create(output_path)?;
        Box::new(file) as Box<dyn Write>
    };
    pcap_utils::pcap_write_header(&mut output_file, 1500)?;

    let written = pcap_utils::pcap_write_packet(&mut output_file, 0, 0, &packey_0)?;
    println!("written: {:?}", written);
    let written = pcap_utils::pcap_write_packet(&mut output_file, 3, 0, &packey_1)?;
    println!("written: {:?}", written);

    Ok(())
}

pub fn simple_fragmentation_overlap_test(
    test_target_ipv6: &TestTarget,
    icmp_identifier: u16,
    icmp_sequence_number: u16,
) -> io::Result<()> {
    let s_0 = "AABBCCDD".to_string();
    let s_1 = "BBCCDDAA".to_string();
    let s_2 = "CCDDAABB".to_string();
    let s_3 = "DDAABBCC".to_string();
    let data_s = [s_0.clone(), s_1.clone()].join("");
    let data = data_s.as_bytes();

    let icmp_packet_data = build_icmp_data(
        // test_target.icmp_identifier,
        // test_target.icmp_sequence_number,
        icmp_identifier,
        icmp_sequence_number,
        data,
        &test_target_ipv6.ip_src,
        &test_target_ipv6.ip_dst,
    );
    let icmp_header: &Vec<u8> = &icmp_packet_data[0..8].to_vec();

    let mut payload_0 = icmp_header.clone();
    let mut s0_u8_v: Vec<u8> = s_0.as_bytes().to_vec();
    let mut s1_u8_v: Vec<u8> = s_1.as_bytes().to_vec();
    payload_0.append(&mut s0_u8_v);
    payload_0.append(&mut s1_u8_v);
    let payload_1_s = [s_2, s_3].join("");
    let payload_1 = payload_1_s.as_bytes();

    let packet_0 = build_ethernet_ipv6(
        test_target_ipv6,
        // test_target_ipv6.macaddr_src,
        // test_target_ipv6.macaddr_dst,
        // test_target_ipv6.ipv4addr_src,
        // test_target_ipv6.ipv4addr_dst,
        111,
        1,
        0,
        // test_target,
        &payload_0,
    );

    let packet_1 = build_ethernet_ipv6(
        test_target_ipv6,
        // test_target_ipv6.macaddr_src,
        // test_target_ipv6.macaddr_dst,
        // test_target_ipv6.ipv4addr_src,
        // test_target_ipv6.ipv4addr_dst,
        111,
        0,
        2,
        // test_target,
        payload_1,
    );

    let output_path_s = "simple_ICMP_fragmentation_overlap_test.pcap";
    let output_path = Path::new(output_path_s);
    println!("Value for w: {:?}", output_path);

    let mut output_file = if output_path
        .to_str()
        .expect("Could not convert output_path to string")
        == "-"
    {
        Box::new(io::stdout())
    } else {
        let file = File::create(output_path)?;
        Box::new(file) as Box<dyn Write>
    };
    pcap_utils::pcap_write_header(&mut output_file, 1500)?;

    let written = pcap_utils::pcap_write_packet(&mut output_file, 0, 0, &packet_0)?;
    println!("written: {:?}", written);
    let written = pcap_utils::pcap_write_packet(&mut output_file, 3, 0, &packet_1)?;
    println!("written: {:?}", written);

    Ok(())
}

//pub fn simple_test() -> io::Result<()> {
//    let macaddr_src_s = "dc:4a:3e:65:92:69";
//    let macaddr_dst_s = "ac:16:2d:9c:3c:ad";
//    let ipv4_src_s = "192.168.39.30";
//    let ipv4_dst_s = "192.168.233.64";
//
//    let macaddr_src = macaddr_src_s.parse().unwrap();
//    let macaddr_dst = macaddr_dst_s.parse().unwrap();
//
//    let ipv4_src = ipv4_src_s.parse().unwrap();
//    let ipv4_dst = ipv4_dst_s.parse().unwrap();
//
//    let icmp_identifier = 10;
//    let icmp_sequence_number = 1;
//
//    let test_target = TestTarget::new(
//        macaddr_src,
//        macaddr_dst,
//        ipv4_src,
//        ipv4_dst,
//        // icmp_identifier,
//        // icmp_sequence_number,
//    );
//
//    let output_path_s = "toto_rust.pcap";
//    let output_path = Path::new(output_path_s);
//    // if !output_path.exists() {
//    //     let e = io::Error::new(
//    //         io::ErrorKind::Other,
//    //         format!("output_path ({:?}) does not exists", output_path),
//    //     );
//    //     return Err(e);
//    // }
//    println!("Value for w: {:?}", output_path);
//
//    let mut output_file = if output_path
//        .to_str()
//        .expect("Could not convert output_path to string")
//        == "-"
//    {
//        Box::new(io::stdout())
//    } else {
//        let file = File::create(output_path)?;
//        Box::new(file) as Box<dyn Write>
//    };
//    pcap_write_header(&mut output_file, 1500)?;
//
//    println!("pcap_write_header");
//
//    let payload_0 = b"00AA";
//    let ethernet_data_v = build_ethernet_ipv6_icmp(
//        &test_target,
//        icmp_identifier,
//        icmp_sequence_number,
//        0,
//        0,
//        payload_0,
//    );
//    let written = pcap_write_packet(&mut output_file, 0, 0, &ethernet_data_v)?;
//    println!("written: {:?}", written);
//
//    let payload_0 = b"AA00";
//    let ethernet_data_v = build_ethernet_ipv6_icmp(
//        &test_target,
//        icmp_identifier,
//        icmp_sequence_number,
//        0,
//        0,
//        payload_0,
//    );
//    let written = pcap_write_packet(&mut output_file, 0, 0, &ethernet_data_v)?;
//    println!("written: {:?}", written);
//
//    Ok(())
//}
