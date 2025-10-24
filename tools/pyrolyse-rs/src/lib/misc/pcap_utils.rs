// use std::fs::File;
use std::io;
use std::io::Write;
// use std::path::Path;

// use pnet::packet::ethernet;
// use pnet::packet::ethernet::EtherType;
// use pnet::packet::icmp;
// use pnet::packet::icmp::{echo_request, IcmpCode, IcmpPacket, IcmpTypes};
// use pnet::packet::icmpv6;
// use pnet::packet::ip::IpNextHeaderProtocol;
// use pnet::packet::ip::IpNextHeaderProtocols;
// use pnet::packet::ipv4;
// use pnet::packet::ipv6;
// use pnet::packet::Packet;

use pcap_parser::ToVec;
use pcap_parser::{LegacyPcapBlock, Linktype};

// use crate::misc::test_target_generic::TestTarget;

// use std::net::IpAddr;
// use std::net::Ipv4Addr;
// use std::net::Ipv6Addr;

pub fn pcap_write_header<W: Write>(to: &mut W, snaplen: usize) -> Result<usize, io::Error> {
    let mut hdr = pcap_parser::PcapHeader::new();
    hdr.snaplen = snaplen as u32;
    hdr.network = Linktype::ETHERNET;
    let s = hdr.to_vec().unwrap();
    to.write_all(&s)?;
    Ok(s.len())
}

pub fn pcap_write_packet<W: Write>(
    to: &mut W,
    ts_sec: u32,
    ts_usec: u32,
    data: &[u8],
) -> Result<usize, io::Error> {
    let mut record = LegacyPcapBlock {
        ts_sec,
        ts_usec,
        caplen: data.len() as u32,  // packet.header.caplen,
        origlen: data.len() as u32, // packet.header.len,
        data,
    };
    let s = record.to_vec().unwrap();
    let sz = to.write(&s)?;

    Ok(sz)
}
