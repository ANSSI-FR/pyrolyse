use std::cmp::max;
use std::net::IpAddr;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;

use cursock::Socket;
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpFlags;
use pnet::packet::{tcp::TcpPacket, Packet};

use crate::misc::test_target::TestTarget;
use crate::tcp_chunk::response_manager_mode::ResponseManagerMode;
use crate::tcp_chunk::tcp_scenario::TcpScenario;
use crate::tcp_chunk::tcp_sending_error::TcpSendingError;
use crate::tcp_chunk::tcp_tcb::TcpTcb;

use std::fmt::Display;
use std::fmt::Debug;
use crate::misc::ip_addr_fragmentation_trait_impl::IpAddrForFragmentationTesting;

// NB: cannot derive Debug because Socket does not implement Debug.
pub struct AckManager<I: IpAddrForFragmentationTesting + Clone + Display + Debug + PartialEq> {
    socket: Socket,
    test_target: TestTarget<I>,
    port_src: u16,
    port_dst: u16,
    // tcp_scenario: TcpScenario,
    response_manager_mode: ResponseManagerMode,
    continue_data_receiving_mutex_arc: Arc<Mutex<bool>>,
    tcp_tcb_mutex_arc: Arc<Mutex<TcpTcb>>,
    ack_index: u32,
    last_echo_replay_u8_v: Vec<u8>,
}

impl<I: IpAddrForFragmentationTesting + Clone + Display + Debug + PartialEq> AckManager<I> {
    pub fn new(
        socket: Socket,
        test_target: TestTarget<I>,
        port_src: u16,
        port_dst: u16,
        response_manager_mode: ResponseManagerMode,
        continue_data_receiving_mutex_arc: Arc<Mutex<bool>>,
        tcp_tcb_mutex_arc: Arc<Mutex<TcpTcb>>,
        ack_index: u32,
        last_echo_replay_u8_v: Vec<u8>,
    ) -> AckManager<I> {
        AckManager {
            socket,
            test_target,
            port_src,
            port_dst,
            response_manager_mode,
            continue_data_receiving_mutex_arc,
            tcp_tcb_mutex_arc,
            ack_index,
            last_echo_replay_u8_v,
        }
    }

    pub fn init(
        interface_name: &String,
        test_target: TestTarget<I>,
        port_src: u16,
        port_dst: u16,
        tcp_scenario: TcpScenario,
        continue_data_receiving_mutex_arc: Arc<Mutex<bool>>,
        tcp_tcb_mutex_arc: Arc<Mutex<TcpTcb>>,
        ack_index: u32,
        last_echo_replay_u8_v: Vec<u8>,
    ) -> AckManager<I> {
        debug!("init: start");

        debug!("init: interface_name: {}", interface_name);

        #[cfg(target_os = "linux")]
        let socket = Socket::new(interface_name.as_str()).expect("initialize error");
        debug!("init: interface MAC: {}", socket.get_src_mac());
        debug!("init: interface MAC: {}", socket.get_adapter().get_mac());
        debug!(
            "init: interface Ipv4: {:?}",
            socket
                .get_adapter()
                .get_ipv4()
                .ok_or("Could not extract IPv4")
        );

        let response_manager_mode = ResponseManagerMode::from_tcp_scenario(tcp_scenario);

        AckManager {
            socket,
            test_target,
            port_src,
            port_dst,
            response_manager_mode,
            continue_data_receiving_mutex_arc,
            tcp_tcb_mutex_arc,
            ack_index,
            last_echo_replay_u8_v,
        }
    }

    fn update_tcb_send_ack_to_packet(
        &mut self,
        tcp_packet: &TcpPacket,
    ) -> Result<(), TcpSendingError> {
        debug!("update_tcb_send_ack_to_packet: start");

        let tcp_payload_sl_length = tcp_packet.payload().len();

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTCb for rcv_nxt"),
        };

        let next_data_to_receive = tcp_packet.get_sequence() + tcp_payload_sl_length as u32;
        debug!(
            "update_tcb_send_ack_to_packet: next_data_to_receive: {}",
            next_data_to_receive
        );

        if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
            tcp_tcb.rcv_nxt = max(*tcp_tcb.rcv_nxt(), next_data_to_receive);
        }

        // TODO: check this new stuff
        // If there is no hole and there is new data, ...
        // This should work because the Echo server does not send data with hole and we hope that there is no reordering.
        if tcp_packet.get_sequence() <= rcv_nxt && rcv_nxt < next_data_to_receive {
            debug!("update_tcb_send_ack_to_packet: new data => sending ack");

            let snd_nxt = match self.tcp_tcb_mutex_arc.lock() {
                Ok(tcp_tcb) => tcp_tcb.snd_nxt,
                Err(_) => panic!("Could not extract TcpTCb for snd_nxt"),
            };

            debug!("update_tcb_send_ack_to_packet: building ack");
            let ack_u8_v = I::build_ethernet_ip_tcp_data(
                &self.test_target,
                self.port_src,
                self.port_dst,
                snd_nxt,
                next_data_to_receive,
                TcpFlags::ACK,
                &[],
            );
            debug!("update_tcb_send_ack_to_packet: sending ack");
            // self.tx.send_to(&ack_u8_v, None);
            self.socket
                .send_raw_packet(&ack_u8_v)
                .map_err(TcpSendingError::Io)?;

            self.ack_index += 1;
        }

        debug!("update_tcb_send_ack_to_packet: end");

        Ok(())
    }

    fn update_last_data(&mut self) -> Result<(), TcpSendingError> {
        debug!("update_last_data: start");

        let mut packet_u8_sl = [0; 2000];

        while self.continue_data_receiving() {
            // debug!("update_last_data: loop start");

            self.socket
                .read_raw_packet(&mut packet_u8_sl)
                .map_err(TcpSendingError::Io)?;

            let ethernet_packet = EthernetPacket::new(&packet_u8_sl)
                .ok_or("Could not build EthernetPacket")
                .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;

            // NB: these checks are extra important because cursock does not bind the socket to a single interface.
            if ethernet_packet.get_source() == self.test_target.mac_addr_dst
                && ethernet_packet.get_destination() == self.test_target.mac_addr_src
            {
                let (ip_packet_src, ip_packet_dst, ip_packet_next_protocol, ip_packet_payload) = match ethernet_packet.get_ethertype() {
                    EtherTypes::Ipv4 => {
                    let ipv4_packet = Ipv4Packet::new(ethernet_packet.payload())
                        .ok_or("Could not build Ipv4Packet")
                        .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;
                    (
                       IpAddr::V4(ipv4_packet.get_source()), 
                       IpAddr::V4(ipv4_packet.get_destination()), 
                       ipv4_packet.get_next_level_protocol(),
                       ipv4_packet.payload().to_vec(), 
                    )

                },
                    EtherTypes::Ipv6 => {
                        let ipv6_packet = Ipv6Packet::new(ethernet_packet.payload())
                            .ok_or("Could not build Ipv6Packet")
                            .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;
                        (
                           IpAddr::V6(ipv6_packet.get_source()), 
                           IpAddr::V6(ipv6_packet.get_destination()), 
                           ipv6_packet.get_next_header(), 
                           ipv6_packet.payload().to_vec(), 
                        )
                    },
                    _ => return Err(TcpSendingError::EtherTypesError)
                };

                if ip_packet_src == self.test_target.ip_addr_dst.clone().into()
                    && ip_packet_dst == self.test_target.ip_addr_src.clone().into()
                    && ip_packet_next_protocol == IpNextHeaderProtocols::Tcp
                {
                    //let tcp_packet = TcpPacket::new(ip_packet.payload())
                    let tcp_packet = TcpPacket::new(ip_packet_payload.as_slice())
                        .ok_or("Could not build TcpPacket")
                        .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;

                    let tcp_flags = tcp_packet.get_flags();
                    let ack_b = tcp_flags & TcpFlags::ACK == TcpFlags::ACK;
                    let push_b = tcp_flags & TcpFlags::PSH == TcpFlags::PSH;

                    if tcp_packet.get_source() == self.port_dst
                        && tcp_packet.get_destination() == self.port_src
                        && ack_b
                        && push_b
                    {
                        debug!("update_last_data_send_ack: received packet from target");

                        {
                            let tcp_tcb_for_display = match self.tcp_tcb_mutex_arc.lock() {
                                Ok(tcp_tcb) => tcp_tcb,
                                Err(_) => panic!("Could not extract TcpTCb"),
                            };
                            I::display_tcp_packet(
                                &self.test_target,
                                self.port_src,
                                self.port_dst,
                                &tcp_tcb_for_display,
                                //&ip_packet,
                                &ip_packet_src,
                                &ip_packet_dst,
                                &tcp_packet,
                            );
                        }

                        let tcp_payload_v = tcp_packet.payload();
                        let tcp_payload_length = tcp_payload_v.len();
                        // let tcp_payload_v_clean = tcp_utils::clean_tcp_payload(tcp_payload_v.to_vec());
                        // let tcp_payload_v_clean_length = tcp_payload_v_clean.len();

                        let snd_nxt = match self.tcp_tcb_mutex_arc.lock() {
                            Ok(tcp_tcb) => tcp_tcb.snd_nxt,
                            Err(_) => panic!("Could not extract TcpTCb for snd_nxt"),
                        };
                        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                            Err(_) => panic!("Could not extract TcpTCb for rcv_nxt"),
                        };

                        debug!(
                            "update_last_data: snd_nxt: {} ; rcv_nxt: {}",
                            snd_nxt, rcv_nxt
                        );

                        // debug!("update_last_data: SEG.SEQ: {}", tcp_packet.get_sequence());

                        //bytes_data = get_tcp_payload(received_packet['TCP'])
                        debug!(
                            "update_last_data: tcp_payload_length: {:?}",
                            tcp_payload_length
                        );

                        let next_data_to_receive =
                            tcp_packet.get_sequence() + tcp_payload_length as u32;
                        debug!(
                            "update_last_data: next_data_to_receive: {:?}",
                            next_data_to_receive
                        );

                        // TODO: check this new stuff
                        // If there is no hole and there is new data, ...
                        // This should work because the Echo server does not send data with hole and we hope that there is no reordering.
                        if tcp_packet.get_sequence() <= rcv_nxt && rcv_nxt < next_data_to_receive {
                            // self.tcp_tcb.rcv_nxt = seq_number_after_data;
                            match self.tcp_tcb_mutex_arc.lock() {
                                Ok(mut tcp_tcb) => {
                                    tcp_tcb.rcv_nxt = max(tcp_tcb.rcv_nxt, next_data_to_receive)
                                }
                                Err(_) => panic!("Could not extract TcpTCb for rcv_nxt"),
                            };

                            self.last_echo_replay_u8_v = packet_u8_sl.to_vec();
                            // self.index += 1
                            debug!("update_last_data: rcv_nxt: {:?}", next_data_to_receive);

                            debug!("update_last_data: end")
                        }
                    }
                }
            }
        }

        debug!("update_last_data: end");

        Ok(())
    }

    fn continue_data_receiving(&self) -> bool {
        return match self.continue_data_receiving_mutex_arc.lock() {
            Ok(b) => *b.deref(),
            Err(_) => panic!("Could not extract continue_data_receiving_mutex_arc"),
        };
    }

    pub fn update_last_data_send_ack(&mut self) -> Result<(), TcpSendingError> {
        debug!("update_last_data_send_ack: start");

        let mut packet_u8_sl = [0; 2000];

        while self.continue_data_receiving() {
            self.socket
                .read_raw_packet(&mut packet_u8_sl)
                .map_err(TcpSendingError::Io)?;

            let ethernet_packet = EthernetPacket::new(&packet_u8_sl)
                .ok_or("Could not build EthernetPacket")
                .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;

                if ethernet_packet.get_source() == self.test_target.mac_addr_dst
                && ethernet_packet.get_destination() == self.test_target.mac_addr_src
            {
                let (ip_packet_src, ip_packet_dst, ip_packet_next_protocol, ip_packet_payload) = match ethernet_packet.get_ethertype() {
                    EtherTypes::Ipv4 => {
                    let ipv4_packet = Ipv4Packet::new(ethernet_packet.payload())
                        .ok_or("Could not build Ipv4Packet")
                        .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;
                    (
                       IpAddr::V4(ipv4_packet.get_source()), 
                       IpAddr::V4(ipv4_packet.get_destination()), 
                       ipv4_packet.get_next_level_protocol(),
                       ipv4_packet.payload().to_vec(), 
                    )

                },
                    EtherTypes::Ipv6 => {
                        let ipv6_packet = Ipv6Packet::new(ethernet_packet.payload())
                            .ok_or("Could not build Ipv6Packet")
                            .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;
                        (
                           IpAddr::V6(ipv6_packet.get_source()), 
                           IpAddr::V6(ipv6_packet.get_destination()), 
                           ipv6_packet.get_next_header(), 
                           ipv6_packet.payload().to_vec(), 
                        )
                    },
                    _ => return Err(TcpSendingError::EtherTypesError)
                };

                if ip_packet_src == self.test_target.ip_addr_dst.clone().into()
                    && ip_packet_dst == self.test_target.ip_addr_src.clone().into()
                    && ip_packet_next_protocol == IpNextHeaderProtocols::Tcp
                {
                    let tcp_packet = TcpPacket::new(ip_packet_payload.as_slice())
                        .ok_or("Could not build TcpPacket")
                        .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;

                    if tcp_packet.get_source() == self.port_dst
                        && tcp_packet.get_destination() == self.port_src
                    {
                        debug!("update_last_data_send_ack: received packet from target");

                        {
                            let tcp_tcb_for_display = match self.tcp_tcb_mutex_arc.lock() {
                                Ok(tcp_tcb) => tcp_tcb,
                                Err(_) => panic!("Could not extract TcpTCb"),
                            };
                            I::display_tcp_packet(
                                &self.test_target,
                                self.port_src,
                                self.port_dst,
                                &tcp_tcb_for_display,
                                //&ip_packet,
                                &ip_packet_src,
                                &ip_packet_dst,
                                &tcp_packet,
                            );
                        }
                        {
                            // debug!("update_last_data_send_ack: received packet from target");

                            // self.update_tcb_send_ack_to_packet(&u8_v)?;
                            self.update_tcb_send_ack_to_packet(&tcp_packet)?;
                        }
                    }
                }
            }
        }

        debug!("update_last_data_send_ack: end");

        Ok(())
    }

    pub fn launch(&mut self) -> Result<(), TcpSendingError> {
        debug!("launch: start");
        match self.response_manager_mode {
            ResponseManagerMode::UpdateTcb => self.update_last_data()?,
            ResponseManagerMode::UpdateTcbSendAck => self.update_last_data_send_ack()?,
        }
        debug!("launch: end");

        Ok(())
    }
}
