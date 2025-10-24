use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

use cursock::Adapter;
use pnet::packet::ethernet::EtherType;
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::udp;

use crate::misc::icmp_echo_request_generation;
use crate::misc::test_target::TestTarget;
use crate::misc::udp_generation;

use super::ethernet_ip_generation;

use pnet::packet::{tcp::MutableTcpPacket, tcp::Tcp, tcp::TcpPacket, Packet};
use crate::tcp_chunk::tcp_tcb::TcpTcb;

use std::fmt::Debug;
use std::fmt::Display;


pub trait IpAddrForFragmentationTesting: Into<IpAddr> {
    fn get_ether_type(ip: &Self) -> EtherType;

    fn get_from_adapter(adapter: &Adapter) -> &Option<Self>;

    fn build_icmp_pdu_data_v(
        icmp_identifier: u16,
        icmp_sequence_number: u16,
        payload: &[u8],
        ip_src: &Self,
        ip_dst: &Self,
    ) -> Vec<u8>;

    fn build_ethernet_pdu_data_v_for_icmp(
        test_target: &TestTarget<Self>,
        ip_id: u16,
        fragment_flag: u8,
        fragment_offset: u16,
        payload: &[u8],
    ) -> Vec<u8>
    where
        Self: Sized;

    fn build_udp_pdu_data_v(
        udp_src_port: u16,
        payload: &[u8],
        ip_src: &Self,
        ip_dst: &Self,
    ) -> Vec<u8>;

    fn build_ethernet_pdu_data_v_for_udp(
        test_target: &TestTarget<Self>,
        ip_id: u16,
        fragment_flag: u8,
        fragment_offset: u16,
        payload: &[u8],
    ) -> Vec<u8>
    where
        Self: Sized;

    fn get_bytes_for_checksum_correction(
        //test_target: &TestTarget<Self>,
        local_checksum: usize,
        reference_checksum: usize,
        max_ending_byte_offset: usize,
        local_ending_offset: usize
    ) -> usize;

    fn build_ethernet_ip_tcp_data(
        test_target: &TestTarget<Self>,
        source_port: u16,
        destination_port: u16,
        sequence_number: u32,
        acknowledgment_number: u32,
        flags: u16,
        //flags: u8,
        payload: &[u8],
    ) -> Vec<u8>;

    fn display_tcp_packet(
        test_target: &TestTarget<Self>,
        port_destination: u16,
        port_source: u16,
        tcp_tcb: &TcpTcb,
        ip_source: &IpAddr,
        ip_destination: &IpAddr,
        tcp_packet: &TcpPacket,
    );
}

impl IpAddrForFragmentationTesting for Ipv4Addr {
    fn get_ether_type(_: &Self) -> EtherType {
        EtherTypes::Ipv4
    }

    fn get_from_adapter(adapter: &Adapter) -> &Option<Ipv4Addr> {
        adapter.get_ipv4()
    }

    fn build_icmp_pdu_data_v(
        icmp_identifier: u16,
        icmp_sequence_number: u16,
        payload: &[u8],
        ip_src: &Self,
        ip_dst: &Self,
    ) -> Vec<u8> {
        icmp_echo_request_generation::build_icmpv4_pdu_data_v(
            icmp_identifier,
            icmp_sequence_number,
            payload,
            ip_src,
            ip_dst,
        )
    }

    fn build_ethernet_pdu_data_v_for_icmp(
        test_target: &TestTarget<Self>,
        ip_id: u16,
        fragment_flag: u8,
        fragment_offset: u16,
        icmp_pdu_data: &[u8],
    ) -> Vec<u8> {
        let ipv4_pdu_payload = ethernet_ip_generation::build_ipv4_pdu_payload(
            test_target,
            ip_id,
            fragment_flag,
            fragment_offset,
            IpNextHeaderProtocols::Icmp,
            icmp_pdu_data,
        );
        ethernet_ip_generation::build_ethernet_pdu_payload(test_target, &ipv4_pdu_payload)
    }

    fn build_udp_pdu_data_v(
        udp_src_port: u16,
        payload: &[u8],
        ip_src: &Self,
        ip_dst: &Self,
    ) -> Vec<u8> {
        udp_generation::build_udp_pdu_data_v(
            udp_src_port,
            payload,
            ip_src,
            ip_dst,
            &udp::ipv4_checksum,
        )
    }

    fn build_ethernet_pdu_data_v_for_udp(
        test_target: &TestTarget<Self>,
        ip_id: u16,
        fragment_flag: u8,
        fragment_offset: u16,
        udp_pdu_data: &[u8],
    ) -> Vec<u8> {
        let ipv4_pdu_payload = ethernet_ip_generation::build_ipv4_pdu_payload(
            test_target,
            ip_id,
            fragment_flag,
            fragment_offset,
            IpNextHeaderProtocols::Udp,
            udp_pdu_data,
        );
        ethernet_ip_generation::build_ethernet_pdu_payload(test_target, &ipv4_pdu_payload)
    }

    fn get_bytes_for_checksum_correction(
        //test_target: &TestTarget<Self>,
        local_checksum: usize,
        reference_checksum: usize,
        _max_ending_byte_offset: usize,
        _local_ending_offset: usize
    ) -> usize {
        local_checksum - reference_checksum
    }

    fn build_ethernet_ip_tcp_data(
        test_target: &TestTarget<Self>,
        source_port: u16,
        destination_port: u16,
        sequence_number: u32,
        acknowledgment_number: u32,
        flags: u16,
        //flags: u8,
        payload: &[u8],
    ) -> Vec<u8> {
        let payload_len = payload.len();
        let tcp_header_len = 20;
        let tcp_packet_len = tcp_header_len + payload_len;

        let mut tcp_data_like_v: Vec<u8> = vec![0; tcp_packet_len];

        let v = payload.to_vec();
        let tcp = Tcp {
            source: source_port,
            destination: destination_port,
            sequence: sequence_number,
            acknowledgement: acknowledgment_number,
            data_offset: 5,
            reserved: 0,
            flags,
            window: 8192,
            checksum: 0,
            urgent_ptr: 0,
            options: vec![],
            payload: v,
        };
        let mut mutable_tcp_packet = MutableTcpPacket::new(&mut tcp_data_like_v[..]).unwrap();
        mutable_tcp_packet.populate(&tcp);
        let mutable_echo_request_packet_u8_s = mutable_tcp_packet.packet();
        let tcp_packet = TcpPacket::new(mutable_echo_request_packet_u8_s).unwrap();
        let checksum = pnet::packet::tcp::ipv4_checksum(
            &tcp_packet,
            &test_target.ip_addr_src,
            &test_target.ip_addr_dst,
        );
        mutable_tcp_packet.set_checksum(checksum);
        let tcp_pdu_data = mutable_tcp_packet.packet().to_vec();

        let ip_id = 0;
        let fragment_flag = 0;
        let fragment_offset = 0;

        let ipv4_pdu_payload = ethernet_ip_generation::build_ipv4_pdu_payload(
            test_target,
            ip_id,
            fragment_flag,
            fragment_offset,
            IpNextHeaderProtocols::Tcp,
            &tcp_pdu_data,
        );

        ethernet_ip_generation::build_ethernet_pdu_payload(test_target, &ipv4_pdu_payload)
    }

        fn display_tcp_packet(
        test_target: &TestTarget<Self>,
        port_destination: u16,
        port_source: u16,
        tcp_tcb: &TcpTcb,
        ip_source: &IpAddr,
        ip_destination: &IpAddr,
        tcp_packet: &TcpPacket,
    ) where Self: PartialEq, Self: Debug, Self: Display {
        let tcp_flags = tcp_packet.get_flags();

        let tcp_payload_sl_length = tcp_packet.payload().len();

        //if ip_packet.get_source() == test_target.ip_addr_dst
        //    && ip_packet.get_destination() == test_target.ip_addr_src
        if ip_source == &test_target.ip_addr_dst
            && ip_destination == &test_target.ip_addr_src
            && tcp_packet.get_source() == port_destination
            && tcp_packet.get_destination() == port_source
        {
            debug!(
                "display_packet: {:?} -> {:?} : {:?} - S={:?} A={:?} - Data: {:?}",
                ip_source,
                ip_destination,
                tcp_flags,
                tcp_packet.get_sequence() - tcp_tcb.iss,
                tcp_packet.get_acknowledgement() - tcp_tcb.irs,
                tcp_payload_sl_length
            );
        } else {
            debug!(
                "display_packet: {}:{} -> {}:{} : {} - S={:?} A={:?} - Data: {:?}",
                ip_source,
                tcp_packet.get_source(),
                ip_destination,
                tcp_packet.get_destination(),
                tcp_flags,
                tcp_packet.get_sequence(),
                tcp_packet.get_acknowledgement(),
                tcp_payload_sl_length
            );
        }
    }
}

impl IpAddrForFragmentationTesting for Ipv6Addr {
    fn get_ether_type(_: &Self) -> EtherType {
        EtherTypes::Ipv6
    }

    fn get_from_adapter(adapter: &Adapter) -> &Option<Ipv6Addr> {
        adapter.get_ipv6()
    }

    fn build_icmp_pdu_data_v(
        icmp_identifier: u16,
        icmp_sequence_number: u16,
        icmp_payload: &[u8],
        ip_src: &Self,
        ip_dst: &Self,
    ) -> Vec<u8> {
        icmp_echo_request_generation::build_icmpv6_pdu_data_v(
            icmp_identifier,
            icmp_sequence_number,
            icmp_payload,
            ip_src,
            ip_dst,
        )
    }

    fn build_ethernet_pdu_data_v_for_icmp(
        test_target: &TestTarget<Self>,
        id: u16,
        fragment_flag: u8,
        fragment_offset: u16,
        icmp_pdu_data: &[u8],
    ) -> Vec<u8> {
        let ipv6_pdu_payload = ethernet_ip_generation::build_ipv6_pdu_payload(
            test_target,
            id,
            fragment_flag,
            fragment_offset,
            IpNextHeaderProtocols::Icmpv6,
            icmp_pdu_data,
        );
        ethernet_ip_generation::build_ethernet_pdu_payload(test_target, &ipv6_pdu_payload)
    }

    fn build_udp_pdu_data_v(
        udp_src_port: u16,
        udp_payload: &[u8],
        ip_src: &Self,
        ip_dst: &Self,
    ) -> Vec<u8> {
        udp_generation::build_udp_pdu_data_v(
            udp_src_port,
            udp_payload,
            ip_src,
            ip_dst,
            &udp::ipv6_checksum,
        )
    }

    fn build_ethernet_pdu_data_v_for_udp(
        test_target: &TestTarget<Self>,
        id: u16,
        fragment_flag: u8,
        fragment_offset: u16,
        udp_pdu_data: &[u8],
    ) -> Vec<u8> {
        let ipv6_pdu_payload = ethernet_ip_generation::build_ipv6_pdu_payload(
            test_target,
            id,
            fragment_flag,
            fragment_offset,
            IpNextHeaderProtocols::Udp,
            udp_pdu_data,
        );
        ethernet_ip_generation::build_ethernet_pdu_payload(test_target, &ipv6_pdu_payload)
    }

    fn get_bytes_for_checksum_correction(
        //test_target: &TestTarget<Self>,
        local_checksum: usize,
        reference_checksum: usize,
        max_ending_byte_offset: usize,
        local_ending_offset: usize
    ) -> usize {
        local_checksum - reference_checksum - (max_ending_byte_offset - local_ending_offset)
    }

    fn build_ethernet_ip_tcp_data(
        test_target: &TestTarget<Self>,
        source_port: u16,
        destination_port: u16,
        sequence_number: u32,
        acknowledgment_number: u32,
        flags: u16,
        //flags: u8,
        payload: &[u8],
    ) -> Vec<u8> {
        let payload_len = payload.len();
        let tcp_header_len = 20;
        let tcp_packet_len = tcp_header_len + payload_len;

        let mut tcp_data_like_v: Vec<u8> = vec![0; tcp_packet_len];

        let v = payload.to_vec();
        let tcp = Tcp {
            source: source_port,
            destination: destination_port,
            sequence: sequence_number,
            acknowledgement: acknowledgment_number,
            data_offset: 5,
            reserved: 0,
            flags,
            window: 8192,
            checksum: 0,
            urgent_ptr: 0,
            options: vec![],
            payload: v,
        };
        let mut mutable_tcp_packet = MutableTcpPacket::new(&mut tcp_data_like_v[..]).unwrap();
        mutable_tcp_packet.populate(&tcp);
        let mutable_echo_request_packet_u8_s = mutable_tcp_packet.packet();
        let tcp_packet = TcpPacket::new(mutable_echo_request_packet_u8_s).unwrap();
        let checksum = pnet::packet::tcp::ipv6_checksum(
            &tcp_packet,
            &test_target.ip_addr_src,
            &test_target.ip_addr_dst,
        );
        mutable_tcp_packet.set_checksum(checksum);
        let tcp_pdu_data = mutable_tcp_packet.packet().to_vec();

        let ip_id = 0;

        let ipv6_pdu_payload = ethernet_ip_generation::build_ipv6_pdu_payload_tcp_testing(
            test_target,
            ip_id,
            &tcp_pdu_data,
        );

        ethernet_ip_generation::build_ethernet_pdu_payload(test_target, &ipv6_pdu_payload)
    }

        fn display_tcp_packet(
        test_target: &TestTarget<Self>,
        port_destination: u16,
        port_source: u16,
        tcp_tcb: &TcpTcb,
        ip_source: &IpAddr,
        ip_destination: &IpAddr,
        tcp_packet: &TcpPacket,
    ) where Self: PartialEq, Self: Debug, Self: Display {
        let tcp_flags = tcp_packet.get_flags();

        let tcp_payload_sl_length = tcp_packet.payload().len();

        //if ip_packet.get_source() == test_target.ip_addr_dst
        //    && ip_packet.get_destination() == test_target.ip_addr_src
        if ip_source == &test_target.ip_addr_dst
            && ip_destination == &test_target.ip_addr_src
            && tcp_packet.get_source() == port_destination
            && tcp_packet.get_destination() == port_source
        {
            debug!(
                "display_packet: {:?} -> {:?} : {:?} - S={:?} A={:?} - Data: {:?}",
                ip_source,
                ip_destination,
                tcp_flags,
                tcp_packet.get_sequence() - tcp_tcb.iss,
                tcp_packet.get_acknowledgement() - tcp_tcb.irs,
                tcp_payload_sl_length
            );
        } else {
            debug!(
                "display_packet: {}:{} -> {}:{} : {} - S={:?} A={:?} - Data: {:?}",
                ip_source,
                tcp_packet.get_source(),
                ip_destination,
                tcp_packet.get_destination(),
                tcp_flags,
                tcp_packet.get_sequence(),
                tcp_packet.get_acknowledgement(),
                tcp_payload_sl_length
            );
        }
    }
}
