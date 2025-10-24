use core::time::Duration;
use std::cmp::max;
//use std::net::Ipv4Addr;
//use std::net::Ipv6Addr;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::sleep;
use std::thread::JoinHandle;

use cursock::Socket;
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpFlags;
use pnet::packet::{tcp::TcpPacket, Packet};

use crate::byte_time_data::chunk::ChunkC;
use crate::misc::test_target::TestTarget;
use crate::position::payload_mode::PayloadMode;
use crate::tcp_chunk::response_manager::AckManager;
use crate::tcp_chunk::response_manager_mode::ResponseManagerMode;
//use crate::tcp_chunk::tcp_data_builder;
use crate::tcp_chunk::tcp_scenario::TcpScenario;
use crate::tcp_chunk::tcp_sending_error::TcpSendingError;
use crate::tcp_chunk::tcp_tcb::TcpTcb;
use crate::tcp_chunk::tcp_utils;

use std::fmt::Display;
use std::fmt::Debug;
use crate::misc::ip_addr_fragmentation_trait_impl::IpAddrForFragmentationTesting;
// #[derive(Debug, Clone)]
pub struct ChunkSender<I: IpAddrForFragmentationTesting + Clone + Display + Debug + PartialEq + Send + 'static> {
    interface_name: String,
    socket: Socket,
    test_target: TestTarget<I>,
    port_src: u16,
    port_dst: u16,
    tcp_scenario: TcpScenario,
    payload_mode: PayloadMode,
    tcp_tcb_mutex_arc: Arc<Mutex<TcpTcb>>,
    snd_nxt_after_3whs: u32,
}

impl<I: IpAddrForFragmentationTesting + Clone + Display + Debug + PartialEq + Send> ChunkSender<I> {
    pub fn new(
        interface_name: String,
        socket: Socket,
        test_target: TestTarget<I>,
        port_src: u16,
        port_dst: u16,
        tcp_scenario: TcpScenario,
        payload_mode: PayloadMode,
        tcp_tcb_mutex_arc: Arc<Mutex<TcpTcb>>,
        snd_nxt_after_3whs: u32,
    ) -> ChunkSender<I> {
        ChunkSender {
            interface_name,
            socket,
            test_target,
            port_src,
            port_dst,
            tcp_scenario,
            payload_mode,
            tcp_tcb_mutex_arc,
            snd_nxt_after_3whs,
        }
    }

    pub fn init(
        interface_name: String,
        //test_target: TestTarget<Ipv4Addr>,
        //test_target: TestTarget<Ipv6Addr>,
        test_target: TestTarget<I>,
        port_src: u16,
        port_dst: u16,
        tcp_scenario: TcpScenario,
        payload_mode: PayloadMode,
    ) -> ChunkSender<I> {
        debug!("init: start");

        debug!("init: interface_name: {}", interface_name);

        let tcp_tcb_iss = 10;
        let tcp_tcb_snd_nxt = tcp_tcb_iss;

        let tcp_tcb = TcpTcb::new(tcp_tcb_snd_nxt, tcp_tcb_snd_nxt, tcp_tcb_iss, 0, 0);
        let tcp_tcb_mutex_arc = Arc::new(Mutex::new(tcp_tcb));

        #[cfg(target_os = "linux")]
        let socket = Socket::new(interface_name.as_str()).expect("initialize error"); // Linux

        debug!("init: end");

        ChunkSender::<I>::new(
            interface_name,
            socket,
            test_target,
            port_src,
            port_dst,
            tcp_scenario,
            payload_mode,
            tcp_tcb_mutex_arc,
            0,
        )
    }

    pub fn connect(&mut self) -> Result<(), TcpSendingError> {
        debug!("connect: start");

        let snd_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.snd_nxt,
            Err(_e) => {
                // panic!("Could not extract TcpTcb")
                return Err(TcpSendingError::PoisonError);
            }
        };
        // let snd_nxt = self.tcp_tcb_mutex_arc.lock().map(|tcp_tcb| tcp_tcb.snd_nxt);

        // Build and send SYN
        debug!("connect: build SYN");
        let syn_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            snd_nxt,
            0,
            TcpFlags::SYN,
            &[],
        );
        debug!("connect: send SYN");
        self.socket
            .send_raw_packet(&syn_u8_v)
            .map_err(TcpSendingError::Io)?;

        match self.tcp_tcb_mutex_arc.lock() {
            Ok(mut tcp_tcb) => tcp_tcb.snd_nxt += 1,
            Err(_) => {
                // panic!("Could not extract TcpTcb")
                return Err(TcpSendingError::PoisonError);
            }
        };

        let mut packet_u8_sl = [0; 2000];

        let mut waiting_for_synack = true;
        while waiting_for_synack {
            self.socket
                .read_raw_packet(&mut packet_u8_sl)
                .map_err(TcpSendingError::Io)?;

            let ethernet_packet = EthernetPacket::new(&packet_u8_sl)
                .ok_or("Could not build EthernetPacket")
                .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;
            
            
            //if ethernet_packet.get_ethertype() == EtherTypes::Ipv4
            //if ethernet_packet.get_ethertype() == EtherTypes::Ipv6
            //    && ethernet_packet.get_source() == self.test_target.mac_addr_dst
            //    && ethernet_packet.get_destination() == self.test_target.mac_addr_src
            if ethernet_packet.get_source() == self.test_target.mac_addr_dst
                && ethernet_packet.get_destination() == self.test_target.mac_addr_src
            {
                //let ip_packet = Ipv4Packet::new(ethernet_packet.payload())
                //let ip_packet = Ipv6Packet::new(ethernet_packet.payload())
                //    .ok_or("Could not build Ipv6Packet")
                //    .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;
                let (ip_packet_src, ip_packet_dst, ip_packet_next_protocol, ip_packet_payload): (IpAddr,IpAddr,_,_) = match ethernet_packet.get_ethertype() {
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

                //if ip_packet.get_source() == self.test_target.ip_addr_dst
                //    && ip_packet.get_destination() == self.test_target.ip_addr_src
                //    //&& ip_packet.get_next_level_protocol() == IpNextHeaderProtocols::Tcp
                //    //&& ip_packet.get_next_header() == IpNextHeaderProtocols::Tcp
                //    && ( (ethernet_packet.get_ethertype() == EtherTypes::Ipv4 
                //            && ip_packet.get_next_level_protocol() == IpNextHeaderProtocols::Tcp)
                //      || (ethernet_packet.get_ethertype() == EtherTypes::Ipv6 
                //            && ip_packet.get_next_header() == IpNextHeaderProtocols::Tcp)
                //    )
                if ip_packet_src == self.test_target.ip_addr_dst.clone().into()
                    && ip_packet_dst == self.test_target.ip_addr_src.clone().into()
                    && ip_packet_next_protocol == IpNextHeaderProtocols::Tcp
                {
                    //let tcp_packet = TcpPacket::new(ip_packet.payload())
                    let tcp_packet = TcpPacket::new(ip_packet_payload.as_slice())
                        .ok_or("Could not build TcpPacket")
                        .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;

                    let tcp_flags = tcp_packet.get_flags();
                    let syn_b = tcp_flags & TcpFlags::SYN == TcpFlags::SYN;
                    let ack_b = tcp_flags & TcpFlags::ACK == TcpFlags::ACK;

                    if tcp_packet.get_source() == self.port_dst
                        && tcp_packet.get_destination() == self.port_src
                    {
                        if syn_b && ack_b {
                            debug!("connect: packet for us received");

                            waiting_for_synack = false;

                            debug!("connect: tcp_packet: {:?}", tcp_packet);

                            // TODO: manage data in SYNACK
                            match self.tcp_tcb_mutex_arc.lock() {
                                Ok(mut tcp_tcb) => {
                                    tcp_tcb.snd_una = tcp_packet.get_acknowledgement()
                                }
                                Err(_) => {
                                    // panic!("Could not extract TcpTcb")
                                    return Err(TcpSendingError::PoisonError);
                                }
                            };
                            match self.tcp_tcb_mutex_arc.lock() {
                                Ok(mut tcp_tcb) => tcp_tcb.rcv_nxt = tcp_packet.get_sequence() + 1,
                                Err(_) => {
                                    // panic!("Could not extract TcpTcb")
                                    return Err(TcpSendingError::PoisonError);
                                }
                            };
                            match self.tcp_tcb_mutex_arc.lock() {
                                Ok(mut tcp_tcb) => tcp_tcb.irs = tcp_packet.get_sequence(),
                                Err(_) => {
                                    // panic!("Could not extract TcpTcb")
                                    return Err(TcpSendingError::PoisonError);
                                }
                            };

                            match self.tcp_tcb_mutex_arc.lock() {
                                Ok(tcp_tcb) => self.snd_nxt_after_3whs = tcp_tcb.iss + 1,
                                Err(_) => {
                                    // panic!("Could not extract TcpTcb")
                                    return Err(TcpSendingError::PoisonError);
                                }
                            };

                            // NB: Xilinx Onload retransmit the SYNACK if delay is 500ms, so we reduce to 200ms.
                            let synack_to_ack_delay_ms = 200;
                            sleep(Duration::from_millis(synack_to_ack_delay_ms));

                            let snd_nxt = match self.tcp_tcb_mutex_arc.lock() {
                                Ok(tcp_tcb) => tcp_tcb.snd_nxt,
                                Err(_) => {
                                    // panic!("Could not extract TcpTcb")
                                    return Err(TcpSendingError::PoisonError);
                                }
                            };
                            let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                                Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                                Err(_) => {
                                    // panic!("Could not extract TcpTcb")
                                    return Err(TcpSendingError::PoisonError);
                                }
                            };

                            // Build ACK
                            let ack_u8_v = I::build_ethernet_ip_tcp_data(
                                &self.test_target,
                                self.port_src,
                                self.port_dst,
                                snd_nxt,
                                rcv_nxt,
                                TcpFlags::ACK,
                                &[],
                            );
                            // Send ACK
                            self.socket
                                .send_raw_packet(&ack_u8_v)
                                .map_err(TcpSendingError::Io)?;

                            // NB: we send ACK again (and hope that Xilinx Onload can see it).
                            sleep(Duration::from_millis(synack_to_ack_delay_ms));
                            // Send ACK
                            self.socket
                                .send_raw_packet(&ack_u8_v)
                                .map_err(TcpSendingError::Io)?;
                        } else {
                            let rst_b = tcp_flags & TcpFlags::RST == TcpFlags::RST;
                            let urg_b = tcp_flags & TcpFlags::URG == TcpFlags::URG;
                            let fin_b = tcp_flags & TcpFlags::FIN == TcpFlags::FIN;
                            debug!("connect: unexpected flag combination: SYN {} ACK {} FIN {} RST {} URG {} ",
                                syn_b, ack_b, fin_b, rst_b, urg_b);
                            return Err(TcpSendingError::ServerError(format!(
                                "Expected SYNACK but found SYN {} ACK {} FIN {} RST {} URG {} ",
                                syn_b, ack_b, fin_b, rst_b, urg_b
                            )));
                        }
                    }
                }
            }
        }
        debug!("connect: end");

        Ok(())
    }

    pub fn send_data(
        &mut self,
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode,
    ) -> Result<(), TcpSendingError> {
        match self.tcp_scenario {
            TcpScenario::ProgressiveAckProgressive => {
                self.send_data_pep(chunk_c, payload_mode, ResponseManagerMode::UpdateTcbSendAck)
            }
            TcpScenario::ProgressiveAckOnce => {
                self.send_data_pep(chunk_c, payload_mode, ResponseManagerMode::UpdateTcb)
            }

            TcpScenario::OnceStartPrecedesAckProgressive => {
                self.send_data_peosp(chunk_c, payload_mode, ResponseManagerMode::UpdateTcbSendAck)
            }
            TcpScenario::OnceStartPrecedesAckOnce => {
                self.send_data_peosp(chunk_c, payload_mode, ResponseManagerMode::UpdateTcb)
            }

            TcpScenario::OnceEndFollowsAckProgressive => {
                self.send_data_peoef(chunk_c, payload_mode, ResponseManagerMode::UpdateTcbSendAck)
            }
            TcpScenario::OnceEndFollowsAckOnce => {
                self.send_data_peoef(chunk_c, payload_mode, ResponseManagerMode::UpdateTcb)
            }

            TcpScenario::OnceEndPrecedesAckProgressive => {
                self.send_data_peoep(chunk_c, payload_mode, ResponseManagerMode::UpdateTcbSendAck)
            }
            TcpScenario::OnceEndPrecedesAckOnce => {
                self.send_data_peoep(chunk_c, payload_mode, ResponseManagerMode::UpdateTcb)
            }

            TcpScenario::OnceStartPrecedesEndFollowsAckProgressive => {
                self.send_data_peospef(chunk_c, payload_mode, ResponseManagerMode::UpdateTcbSendAck)
            }
            TcpScenario::OnceStartPrecedesEndFollowsAckOnce => {
                self.send_data_peospef(chunk_c, payload_mode, ResponseManagerMode::UpdateTcb)
            }

            TcpScenario::OnceStartPrecedesEndPrecedesAckProgressive => {
                self.send_data_peospep(chunk_c, payload_mode, ResponseManagerMode::UpdateTcbSendAck)
            }
            TcpScenario::OnceStartPrecedesEndPrecedesAckOnce => {
                self.send_data_peospep(chunk_c, payload_mode, ResponseManagerMode::UpdateTcb)
            }

            TcpScenario::OnceEndPrecedesStartPrecedesAckProgressive => {
                self.send_data_peoepsp(chunk_c, payload_mode, ResponseManagerMode::UpdateTcbSendAck)
            }
            TcpScenario::OnceEndPrecedesStartPrecedesAckOnce => {
                self.send_data_peoepsp(chunk_c, payload_mode, ResponseManagerMode::UpdateTcb)
            }

            TcpScenario::OnceStartFollows => {
                self.send_data_peosf(chunk_c, payload_mode, ResponseManagerMode::UpdateTcbSendAck)
            }
            TcpScenario::OnceStartFollowsEndFollows => {
                self.send_data_peosfef(chunk_c, payload_mode, ResponseManagerMode::UpdateTcbSendAck)
            }
            TcpScenario::OnceEndFollowsStartFollows => {
                self.send_data_peoefsf(chunk_c, payload_mode, ResponseManagerMode::UpdateTcbSendAck)
            }
            TcpScenario::OnceEndPrecedesStartFollows => {
                self.send_data_peoepsf(chunk_c, payload_mode, ResponseManagerMode::UpdateTcbSendAck)
            }
        }
    }

    fn send_data_pep(
        &mut self,
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode,
        response_manager_mode: ResponseManagerMode,
    ) -> Result<(), TcpSendingError> {
        debug!("send_data_pep: start");

        let offset_multiplier = tcp_utils::get_offset_multiplier(&self.payload_mode);

        let continue_data_receiving_mutex_arc = Arc::new(Mutex::new(true));
        let continue_data_receiving_mutex_arc_bonus =
            Arc::clone(&continue_data_receiving_mutex_arc);

        let tcp_tcb_mutex_arc_bonus = Arc::clone(&self.tcp_tcb_mutex_arc);

        let mut ack_manager = AckManager::init(
            &self.interface_name,
            self.test_target.clone(),
            self.port_src,
            self.port_dst,
            self.tcp_scenario.clone(),
            continue_data_receiving_mutex_arc_bonus,
            tcp_tcb_mutex_arc_bonus,
            0,
            vec![],
        );

        let thread_join_handle = thread::spawn(move || ack_manager.launch());

        sleep(Duration::from_millis(1000));

        chunk_c.iter().try_for_each(|(i, chunk_d)| {
            debug!("send_data_pep: chunk {}", i);

            let seg_seq = self.snd_nxt_after_3whs + (*chunk_d.offset() * offset_multiplier) as u32;
            debug!("send_data_pep: seg_seq: {}", seg_seq);

            let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                Err(_) => panic!("Could not extract TcpTcb"),
            };
            debug!("send_data_pep: rcv_nxt: {}", rcv_nxt);

            // Build and send data
            let data_u8_v = I::build_ethernet_ip_tcp_data(
                &self.test_target,
                self.port_src,
                self.port_dst,
                seg_seq,
                rcv_nxt,
                TcpFlags::ACK | TcpFlags::PSH,
                &chunk_d.get_chunk_pattern_ascii_v(payload_mode),
            );
            self.socket
                .send_raw_packet(&data_u8_v)
                .map_err(TcpSendingError::Io)?;

            let next_sent_data = seg_seq + chunk_d.internet_checksum_ascii_v().len() as u32;
            if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
                // NB: we use max because a chunk can be the last one time-wise and the data after it may not be located after all previously sent chunks.
                tcp_tcb.snd_nxt = max(tcp_tcb.snd_nxt, next_sent_data);
            };

            debug!("send_data_pep: sleeping");
            // NB: 100ms is too short for the other thread to update the TCB and seems to not prevent reordering.
            sleep(Duration::from_millis(500));

            let r: Result<(), TcpSendingError> = Ok(());
            r
        })?;

        self.handle_common_end(
            thread_join_handle,
            response_manager_mode,
            &continue_data_receiving_mutex_arc,
        )?;

        debug!("send_data_pep: end");

        Ok(())
    }

    fn send_data_peosp(
        &mut self,
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode,
        response_manager_mode: ResponseManagerMode,
    ) -> Result<(), TcpSendingError> {
        debug!("send_data_peosp: start");

        let offset_multiplier = tcp_utils::get_offset_multiplier(&self.payload_mode);

        let continue_data_receiving_mutex_arc = Arc::new(Mutex::new(true));
        let continue_data_receiving_mutex_arc_bonus =
            Arc::clone(&continue_data_receiving_mutex_arc);

        let tcp_tcb_mutex_arc_bonus = Arc::clone(&self.tcp_tcb_mutex_arc);

        let mut ack_manager = AckManager::init(
            &self.interface_name,
            self.test_target.clone(),
            self.port_src,
            self.port_dst,
            self.tcp_scenario.clone(),
            continue_data_receiving_mutex_arc_bonus,
            tcp_tcb_mutex_arc_bonus,
            0,
            vec![],
        );

        let thread_join_handle = thread::spawn(move || ack_manager.launch());

        // NB: 500ms of delay causes the first response ACK to be reordered just after the next sent data.
        sleep(Duration::from_millis(1000));

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        debug!("send_data_peosp: sending first byte-wise chunk as last one");
        // Build and send first byte-wise chunk
        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            self.snd_nxt_after_3whs,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        // important for having consistent seq number for acking if ack progressive scenario
        if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
            tcp_tcb.snd_nxt += 1;
        };

        debug!("send_data_peosp: sleeping");
        // Python: 2000ms
        sleep(Duration::from_millis(500));

        debug!("send_data_peosp: sending data");
        chunk_c.iter().try_for_each(|(i, chunk_d)| {
            debug!("send_data_peosp: chunk {}", i);

            // We add 1 to be able to send the first byte-wise chunk at the beginning.
            let seg_seq =
                self.snd_nxt_after_3whs + (*chunk_d.offset() * offset_multiplier) as u32 + 1;
            debug!("send_data_peosp: seg_seq: {}", seg_seq);

            let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                Err(_) => panic!("Could not extract TcpTcb"),
            };
            debug!("send_data_peosp: rcv_nxt: {}", rcv_nxt);

            // Build and send data
            let data_u8_v = I::build_ethernet_ip_tcp_data(
                &self.test_target,
                self.port_src,
                self.port_dst,
                seg_seq,
                rcv_nxt,
                TcpFlags::ACK | TcpFlags::PSH,
                &chunk_d.get_chunk_pattern_ascii_v(payload_mode),
            );
            self.socket
                .send_raw_packet(&data_u8_v)
                .map_err(TcpSendingError::Io)?;

            let next_sent_data = seg_seq + chunk_d.internet_checksum_ascii_v().len() as u32;
            if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
                // NB: we use max because a chunk can be the last one and the data after it may not be located after all previously sent chunks.
                tcp_tcb.snd_nxt = max(tcp_tcb.snd_nxt, next_sent_data);
            };

            debug!("send_data_peosp: sleeping");
            sleep(Duration::from_millis(500));

            let r: Result<(), TcpSendingError> = Ok(());
            r
        })?;

        self.handle_common_end(
            thread_join_handle,
            response_manager_mode,
            &continue_data_receiving_mutex_arc,
        )?;

        debug!("send_data_peosp: end");

        Ok(())
    }

    fn send_data_peosf(
        &mut self,
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode,
        response_manager_mode: ResponseManagerMode,
    ) -> Result<(), TcpSendingError> {
        debug!("send_data_peosf: start");

        let offset_multiplier = tcp_utils::get_offset_multiplier(&self.payload_mode);

        let continue_data_receiving_mutex_arc = Arc::new(Mutex::new(true));
        let continue_data_receiving_mutex_arc_bonus =
            Arc::clone(&continue_data_receiving_mutex_arc);

        let tcp_tcb_mutex_arc_bonus = Arc::clone(&self.tcp_tcb_mutex_arc);

        let mut ack_manager = AckManager::init(
            &self.interface_name,
            self.test_target.clone(),
            self.port_src,
            self.port_dst,
            self.tcp_scenario.clone(),
            continue_data_receiving_mutex_arc_bonus,
            tcp_tcb_mutex_arc_bonus,
            0,
            vec![],
        );

        let thread_join_handle = thread::spawn(move || ack_manager.launch());

        // NB: 500ms of delay causes the first response ACK to be reordered just after the next sent data.
        sleep(Duration::from_millis(1000));

        debug!("send_data_peosf: sending data");
        chunk_c.iter().try_for_each(|(i, chunk_d)| {
            debug!("send_data_peosf: chunk {}", i);

            // We add 1 to be able to send the first byte-wise chunk at the end.
            let seg_seq =
                self.snd_nxt_after_3whs + (*chunk_d.offset() * offset_multiplier) as u32 + 1;
            debug!("send_data_peosf: seg_seq: {}", seg_seq);

            let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                Err(_) => panic!("Could not extract TcpTcb"),
            };
            debug!("send_data_peosf: rcv_nxt: {}", rcv_nxt);

            // Build and send data
            let data_u8_v = I::build_ethernet_ip_tcp_data(
                &self.test_target,
                self.port_src,
                self.port_dst,
                seg_seq,
                rcv_nxt,
                TcpFlags::ACK | TcpFlags::PSH,
                &chunk_d.get_chunk_pattern_ascii_v(payload_mode),
            );
            self.socket
                .send_raw_packet(&data_u8_v)
                .map_err(TcpSendingError::Io)?;

            let next_sent_data = seg_seq + chunk_d.internet_checksum_ascii_v().len() as u32;
            if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
                // NB: we use max because a chunk can be the last one and the data after it may not be located after all previously sent chunks.
                tcp_tcb.snd_nxt = max(tcp_tcb.snd_nxt, next_sent_data);
            };

            debug!("send_data_peosf: sleeping");
            sleep(Duration::from_millis(500));

            let r: Result<(), TcpSendingError> = Ok(());
            r
        })?;

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        debug!("send_data_peosf: sending first byte-wise chunk as last one");
        // Build and send first byte-wise chunk
        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            self.snd_nxt_after_3whs,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        self.handle_common_end(
            thread_join_handle,
            response_manager_mode,
            &continue_data_receiving_mutex_arc,
        )?;

        debug!("send_data_peosf: end");

        Ok(())
    }

    fn send_data_peoef(
        &mut self,
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode,
        response_manager_mode: ResponseManagerMode,
    ) -> Result<(), TcpSendingError> {
        debug!("send_data_peoef: start");

        let offset_multiplier = tcp_utils::get_offset_multiplier(&self.payload_mode);

        let continue_data_receiving_mutex_arc = Arc::new(Mutex::new(true));
        let continue_data_receiving_mutex_arc_bonus =
            Arc::clone(&continue_data_receiving_mutex_arc);

        let tcp_tcb_mutex_arc_bonus = Arc::clone(&self.tcp_tcb_mutex_arc);

        let mut ack_manager = AckManager::init(
            &self.interface_name,
            self.test_target.clone(),
            self.port_src,
            self.port_dst,
            self.tcp_scenario.clone(),
            continue_data_receiving_mutex_arc_bonus,
            tcp_tcb_mutex_arc_bonus,
            0,
            vec![],
        );

        let thread_join_handle = thread::spawn(move || ack_manager.launch());

        // NB: 500ms of delay causes the first response ACK to be reordered just after the next sent data.
        sleep(Duration::from_millis(1000));

        let _rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        debug!("send_data_peoef: sending data");
        chunk_c.iter().try_for_each(|(i, chunk_d)| {
            debug!("send_data_peoef: chunk {}", i);

            let seg_seq = self.snd_nxt_after_3whs + (*chunk_d.offset() * offset_multiplier) as u32;
            debug!("send_data_peoef: seg_seq: {}", seg_seq);

            let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                Err(_) => panic!("Could not extract TcpTcb"),
            };
            debug!("send_data_peoef: rcv_nxt: {}", rcv_nxt);

            // Build and send data
            let data_u8_v = I::build_ethernet_ip_tcp_data(
                &self.test_target,
                self.port_src,
                self.port_dst,
                seg_seq,
                rcv_nxt,
                TcpFlags::ACK | TcpFlags::PSH,
                &chunk_d.get_chunk_pattern_ascii_v(payload_mode),
            );
            self.socket
                .send_raw_packet(&data_u8_v)
                .map_err(TcpSendingError::Io)?;

            let next_sent_data = seg_seq + chunk_d.internet_checksum_ascii_v().len() as u32;
            if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
                // NB: we use max because a chunk can be the last one and the data after it may not be located after all previously sent chunks.
                tcp_tcb.snd_nxt = max(tcp_tcb.snd_nxt, next_sent_data);
            };

            debug!("send_data_peoef: sleeping");
            sleep(Duration::from_millis(500));

            let r: Result<(), TcpSendingError> = Ok(());
            r
        })?;

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        debug!("send_data_peoef: sending last byte-wise chunk as last one");
        // Build and send last byte-wise chunk

        // Get max ending offset
        let mut max_ending_offset = 0;
        chunk_c.iter().for_each(|(_i, chunk_d)| {
            if max_ending_offset < chunk_d.get_ending_offset() {
                max_ending_offset = chunk_d.get_ending_offset();
            }
        });
        debug!("send_data_peoef: max_ending_offset: {}", max_ending_offset);

        let end_snd_nxt = self.snd_nxt_after_3whs + (max_ending_offset * offset_multiplier) as u32;

        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            end_snd_nxt,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        // update snd_nxt to take into account End extra chunk
        if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
            tcp_tcb.snd_nxt = end_snd_nxt + 1;
        };

        self.handle_common_end(
            thread_join_handle,
            response_manager_mode,
            &continue_data_receiving_mutex_arc,
        )?;

        debug!("send_data_peoef: end");

        Ok(())
    }

    fn send_data_peoep(
        &mut self,
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode,
        response_manager_mode: ResponseManagerMode,
    ) -> Result<(), TcpSendingError> {
        debug!("send_data_peoep: start");

        let offset_multiplier = tcp_utils::get_offset_multiplier(&self.payload_mode);

        let continue_data_receiving_mutex_arc = Arc::new(Mutex::new(true));
        let continue_data_receiving_mutex_arc_bonus =
            Arc::clone(&continue_data_receiving_mutex_arc);

        let tcp_tcb_mutex_arc_bonus = Arc::clone(&self.tcp_tcb_mutex_arc);

        let mut ack_manager = AckManager::init(
            &self.interface_name,
            self.test_target.clone(),
            self.port_src,
            self.port_dst,
            self.tcp_scenario.clone(),
            continue_data_receiving_mutex_arc_bonus,
            tcp_tcb_mutex_arc_bonus,
            0,
            vec![],
        );

        let thread_join_handle = thread::spawn(move || ack_manager.launch());

        // NB: 500ms of delay causes the first response ACK to be reordered just after the next sent data.
        sleep(Duration::from_millis(1000));

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        debug!("send_data_peoep: sending last byte-wise chunk as first one");
        // Build and send last byte-wise chunk

        // Get max ending offset
        let mut max_ending_offset = 0;
        chunk_c.iter().for_each(|(_i, chunk_d)| {
            if max_ending_offset < chunk_d.get_ending_offset() {
                max_ending_offset = chunk_d.get_ending_offset();
            }
        });
        debug!("send_data_peoep: max_ending_offset: {}", max_ending_offset);

        let end_snd_nxt = self.snd_nxt_after_3whs + (max_ending_offset * offset_multiplier) as u32;

        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            end_snd_nxt,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        // update snd_nxt to take into account End extra chunk
        if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
            tcp_tcb.snd_nxt = end_snd_nxt + 1;
        };

        debug!("send_data_peoep: sleeping");
        // Python: 2000ms
        sleep(Duration::from_millis(500));

        debug!("send_data_peoep: sending data");
        chunk_c.iter().try_for_each(|(i, chunk_d)| {
            debug!("send_data_peoep: chunk {}", i);

            let seg_seq = self.snd_nxt_after_3whs + (*chunk_d.offset() * offset_multiplier) as u32;
            debug!("send_data_peoep: seg_seq: {}", seg_seq);

            let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                Err(_) => panic!("Could not extract TcpTcb"),
            };
            debug!("send_data_peoep: rcv_nxt: {}", rcv_nxt);

            // Build and send data
            let data_u8_v = I::build_ethernet_ip_tcp_data(
                &self.test_target,
                self.port_src,
                self.port_dst,
                seg_seq,
                rcv_nxt,
                TcpFlags::ACK | TcpFlags::PSH,
                &chunk_d.get_chunk_pattern_ascii_v(payload_mode),
            );
            self.socket
                .send_raw_packet(&data_u8_v)
                .map_err(TcpSendingError::Io)?;

            let next_sent_data = seg_seq + chunk_d.internet_checksum_ascii_v().len() as u32;
            if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
                // NB: we use max because a chunk can be the last one and the data after it may not be located after all previously sent chunks.
                tcp_tcb.snd_nxt = max(tcp_tcb.snd_nxt, next_sent_data);
            };

            debug!("send_data_peoep: sleeping");
            sleep(Duration::from_millis(500));

            let r: Result<(), TcpSendingError> = Ok(());
            r
        })?;

        self.handle_common_end(
            thread_join_handle,
            response_manager_mode,
            &continue_data_receiving_mutex_arc,
        )?;

        debug!("send_data_peoep: end");

        Ok(())
    }

    fn send_data_peosfef(
        &mut self,
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode,
        response_manager_mode: ResponseManagerMode,
    ) -> Result<(), TcpSendingError> {
        debug!("send_data_peosfef: start");

        let offset_multiplier = tcp_utils::get_offset_multiplier(&self.payload_mode);

        let continue_data_receiving_mutex_arc = Arc::new(Mutex::new(true));
        let continue_data_receiving_mutex_arc_bonus =
            Arc::clone(&continue_data_receiving_mutex_arc);

        let tcp_tcb_mutex_arc_bonus = Arc::clone(&self.tcp_tcb_mutex_arc);

        let mut ack_manager = AckManager::init(
            &self.interface_name,
            self.test_target.clone(),
            self.port_src,
            self.port_dst,
            self.tcp_scenario.clone(),
            continue_data_receiving_mutex_arc_bonus,
            tcp_tcb_mutex_arc_bonus,
            0,
            vec![],
        );

        let thread_join_handle = thread::spawn(move || ack_manager.launch());

        // NB: 500ms of delay causes the first response ACK to be reordered just after the next sent data.
        sleep(Duration::from_millis(1000));

        let _rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        debug!("send_data_peosfef: sending data");
        chunk_c.iter().try_for_each(|(i, chunk_d)| {
            debug!("send_data_peosfef: chunk {}", i);

            let seg_seq =
                self.snd_nxt_after_3whs + (*chunk_d.offset() * offset_multiplier + 1) as u32;
            debug!("send_data_peosfef: seg_seq: {}", seg_seq);

            let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                Err(_) => panic!("Could not extract TcpTcb"),
            };
            debug!("send_data_peosfef: rcv_nxt: {}", rcv_nxt);

            // Build and send data
            let data_u8_v = I::build_ethernet_ip_tcp_data(
                &self.test_target,
                self.port_src,
                self.port_dst,
                seg_seq,
                rcv_nxt,
                TcpFlags::ACK | TcpFlags::PSH,
                &chunk_d.get_chunk_pattern_ascii_v(payload_mode),
            );
            self.socket
                .send_raw_packet(&data_u8_v)
                .map_err(TcpSendingError::Io)?;

            let next_sent_data = seg_seq + chunk_d.internet_checksum_ascii_v().len() as u32;
            if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
                // NB: we use max because a chunk can be the last one and the data after it may not be located after all previously sent chunks.
                tcp_tcb.snd_nxt = max(tcp_tcb.snd_nxt, next_sent_data);
            };

            debug!("send_data_peosfef: sleeping");
            sleep(Duration::from_millis(500));

            let r: Result<(), TcpSendingError> = Ok(());
            r
        })?;

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };
        debug!("send_data_peosfef: rcv_nxt: {}", rcv_nxt);

        debug!("send_data_peosfef: sending first byte-wise chunk as last one");
        // Build and send first byte-wise chunk
        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            self.snd_nxt_after_3whs,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        debug!("send_data_peosf: sleeping");
        // Python: 2000ms
        sleep(Duration::from_millis(500));

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };
        debug!("send_data_peosfef: rcv_nxt: {}", rcv_nxt);

        debug!("send_data_peosfef: sending last byte-wise chunk as last one");
        // Build and send last byte-wise chunk

        // Get max ending offset
        let mut max_ending_offset = 0;
        chunk_c.iter().for_each(|(_i, chunk_d)| {
            if max_ending_offset < chunk_d.get_ending_offset() {
                max_ending_offset = chunk_d.get_ending_offset();
            }
        });
        debug!(
            "send_data_peosfef: max_ending_offset: {}",
            max_ending_offset
        );

        let end_snd_nxt =
            self.snd_nxt_after_3whs + (max_ending_offset * offset_multiplier + 1) as u32;

        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            end_snd_nxt,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        // update snd_nxt to take into account End extra chunk
        if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
            tcp_tcb.snd_nxt = end_snd_nxt + 1;
        };

        self.handle_common_end(
            thread_join_handle,
            response_manager_mode,
            &continue_data_receiving_mutex_arc,
        )?;

        debug!("send_data_peosfef: end");

        Ok(())
    }

    fn send_data_peoefsf(
        &mut self,
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode,
        response_manager_mode: ResponseManagerMode,
    ) -> Result<(), TcpSendingError> {
        debug!("send_data_peoefsf: start");

        let offset_multiplier = tcp_utils::get_offset_multiplier(&self.payload_mode);

        let continue_data_receiving_mutex_arc = Arc::new(Mutex::new(true));
        let continue_data_receiving_mutex_arc_bonus =
            Arc::clone(&continue_data_receiving_mutex_arc);

        let tcp_tcb_mutex_arc_bonus = Arc::clone(&self.tcp_tcb_mutex_arc);

        let mut ack_manager = AckManager::init(
            &self.interface_name,
            self.test_target.clone(),
            self.port_src,
            self.port_dst,
            self.tcp_scenario.clone(),
            continue_data_receiving_mutex_arc_bonus,
            tcp_tcb_mutex_arc_bonus,
            0,
            vec![],
        );

        let thread_join_handle = thread::spawn(move || ack_manager.launch());

        // NB: 500ms of delay causes the first response ACK to be reordered just after the next sent data.
        sleep(Duration::from_millis(1000));

        let _rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        debug!("send_data_peoefsf: sending data");
        chunk_c.iter().try_for_each(|(i, chunk_d)| {
            debug!("send_data_peoefsf: chunk {}", i);

            let seg_seq =
                self.snd_nxt_after_3whs + (*chunk_d.offset() * offset_multiplier + 1) as u32;
            debug!("send_data_peoefsf: seg_seq: {}", seg_seq);

            let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                Err(_) => panic!("Could not extract TcpTcb"),
            };
            debug!("send_data_peoefsf: rcv_nxt: {}", rcv_nxt);

            // Build and send data
            let data_u8_v = I::build_ethernet_ip_tcp_data(
                &self.test_target,
                self.port_src,
                self.port_dst,
                seg_seq,
                rcv_nxt,
                TcpFlags::ACK | TcpFlags::PSH,
                &chunk_d.get_chunk_pattern_ascii_v(payload_mode),
            );
            self.socket
                .send_raw_packet(&data_u8_v)
                .map_err(TcpSendingError::Io)?;

            let next_sent_data = seg_seq + chunk_d.internet_checksum_ascii_v().len() as u32;
            if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
                // NB: we use max because a chunk can be the last one and the data after it may not be located after all previously sent chunks.
                tcp_tcb.snd_nxt = max(tcp_tcb.snd_nxt, next_sent_data);
            };

            debug!("send_data_peoefsf: sleeping");
            sleep(Duration::from_millis(500));

            let r: Result<(), TcpSendingError> = Ok(());
            r
        })?;

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };
        debug!("send_data_peoefsf: rcv_nxt: {}", rcv_nxt);

        debug!("send_data_peoefsf: sending last byte-wise chunk as last one");
        // Build and send last byte-wise chunk

        // Get max ending offset
        let mut max_ending_offset = 0;
        chunk_c.iter().for_each(|(_i, chunk_d)| {
            if max_ending_offset < chunk_d.get_ending_offset() {
                max_ending_offset = chunk_d.get_ending_offset();
            }
        });
        debug!(
            "send_data_peoefsf: max_ending_offset: {}",
            max_ending_offset
        );

        let end_snd_nxt =
            self.snd_nxt_after_3whs + (max_ending_offset * offset_multiplier + 1) as u32;

        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            end_snd_nxt,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        // update snd_nxt to take into account End extra chunk
        if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
            tcp_tcb.snd_nxt = end_snd_nxt + 1;
        };

        debug!("send_data_peosf: sleeping");
        // Python: 2000ms
        sleep(Duration::from_millis(500));

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };
        debug!("send_data_peoefsf: rcv_nxt: {}", rcv_nxt);

        debug!("send_data_peoefsf: sending first byte-wise chunk as last one");
        // Build and send first byte-wise chunk
        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            self.snd_nxt_after_3whs,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        self.handle_common_end(
            thread_join_handle,
            response_manager_mode,
            &continue_data_receiving_mutex_arc,
        )?;

        debug!("send_data_peoefsf: end");

        Ok(())
    }

    fn send_data_peospef(
        &mut self,
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode,
        response_manager_mode: ResponseManagerMode,
    ) -> Result<(), TcpSendingError> {
        debug!("send_data_peospef: start");

        let offset_multiplier = tcp_utils::get_offset_multiplier(&self.payload_mode);

        let continue_data_receiving_mutex_arc = Arc::new(Mutex::new(true));
        let continue_data_receiving_mutex_arc_bonus =
            Arc::clone(&continue_data_receiving_mutex_arc);

        let tcp_tcb_mutex_arc_bonus = Arc::clone(&self.tcp_tcb_mutex_arc);

        let mut ack_manager = AckManager::init(
            &self.interface_name,
            self.test_target.clone(),
            self.port_src,
            self.port_dst,
            self.tcp_scenario.clone(),
            continue_data_receiving_mutex_arc_bonus,
            tcp_tcb_mutex_arc_bonus,
            0,
            vec![],
        );

        let thread_join_handle = thread::spawn(move || ack_manager.launch());

        // NB: 500ms of delay causes the first response ACK to be reordered just after the next sent data.
        sleep(Duration::from_millis(1000));

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        debug!("send_data_peospef: sending first byte-wise chunk as first one");
        // Build and send first byte-wise chunk
        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            self.snd_nxt_after_3whs,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        // important for having consistent seq number for acking if ack progressive scenario
        if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
            tcp_tcb.snd_nxt += 1;
        };

        debug!("send_data_peosf: sleeping");
        // Python: 2000ms
        sleep(Duration::from_millis(500));

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };
        debug!("send_data_peospef: rcv_nxt: {}", rcv_nxt);

        debug!("send_data_peospef: sending data");
        chunk_c.iter().try_for_each(|(i, chunk_d)| {
            debug!("send_data_peospef: chunk {}", i);

            let seg_seq =
                self.snd_nxt_after_3whs + (*chunk_d.offset() * offset_multiplier + 1) as u32;
            debug!("send_data_peospef: seg_seq: {}", seg_seq);

            let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                Err(_) => panic!("Could not extract TcpTcb"),
            };
            debug!("send_data_peospef: rcv_nxt: {}", rcv_nxt);

            // Build and send data
            let data_u8_v = I::build_ethernet_ip_tcp_data(
                &self.test_target,
                self.port_src,
                self.port_dst,
                seg_seq,
                rcv_nxt,
                TcpFlags::ACK | TcpFlags::PSH,
                &chunk_d.get_chunk_pattern_ascii_v(payload_mode),
            );
            self.socket
                .send_raw_packet(&data_u8_v)
                .map_err(TcpSendingError::Io)?;

            let next_sent_data = seg_seq + chunk_d.internet_checksum_ascii_v().len() as u32;
            if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
                // NB: we use max because a chunk can be the last one and the data after it may not be located after all previously sent chunks.
                tcp_tcb.snd_nxt = max(tcp_tcb.snd_nxt, next_sent_data);
            };

            debug!("send_data_peospef: sleeping");
            sleep(Duration::from_millis(500));

            let r: Result<(), TcpSendingError> = Ok(());
            r
        })?;

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };
        debug!("send_data_peospef: rcv_nxt: {}", rcv_nxt);

        debug!("send_data_peospef: sending last byte-wise chunk as last one");
        // Build and send last byte-wise chunk

        // Get max ending offset
        let mut max_ending_offset = 0;
        chunk_c.iter().for_each(|(_i, chunk_d)| {
            if max_ending_offset < chunk_d.get_ending_offset() {
                max_ending_offset = chunk_d.get_ending_offset();
            }
        });
        debug!(
            "send_data_peospef: max_ending_offset: {}",
            max_ending_offset
        );

        let end_snd_nxt =
            self.snd_nxt_after_3whs + (max_ending_offset * offset_multiplier + 1) as u32;

        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            end_snd_nxt,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        // update snd_nxt to take into account End extra chunk
        if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
            tcp_tcb.snd_nxt = end_snd_nxt + 1;
        };

        self.handle_common_end(
            thread_join_handle,
            response_manager_mode,
            &continue_data_receiving_mutex_arc,
        )?;

        debug!("send_data_peospef: end");

        Ok(())
    }

    fn send_data_peoepsf(
        &mut self,
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode,
        response_manager_mode: ResponseManagerMode,
    ) -> Result<(), TcpSendingError> {
        debug!("send_data_peoepsf: start");

        let offset_multiplier = tcp_utils::get_offset_multiplier(&self.payload_mode);

        let continue_data_receiving_mutex_arc = Arc::new(Mutex::new(true));
        let continue_data_receiving_mutex_arc_bonus =
            Arc::clone(&continue_data_receiving_mutex_arc);

        let tcp_tcb_mutex_arc_bonus = Arc::clone(&self.tcp_tcb_mutex_arc);

        let mut ack_manager = AckManager::init(
            &self.interface_name,
            self.test_target.clone(),
            self.port_src,
            self.port_dst,
            self.tcp_scenario.clone(),
            continue_data_receiving_mutex_arc_bonus,
            tcp_tcb_mutex_arc_bonus,
            0,
            vec![],
        );

        let thread_join_handle = thread::spawn(move || ack_manager.launch());

        // NB: 500ms of delay causes the first response ACK to be reordered just after the next sent data.
        sleep(Duration::from_millis(1000));

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        debug!("send_data_peoepsf: sending last byte-wise chunk as first one");
        // Build and send last byte-wise chunk

        // Get max ending offset
        let mut max_ending_offset = 0;
        chunk_c.iter().for_each(|(_i, chunk_d)| {
            if max_ending_offset < chunk_d.get_ending_offset() {
                max_ending_offset = chunk_d.get_ending_offset();
            }
        });
        debug!(
            "send_data_peoepsf: max_ending_offset: {}",
            max_ending_offset
        );

        let end_snd_nxt =
            self.snd_nxt_after_3whs + (max_ending_offset * offset_multiplier + 1) as u32;

        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            end_snd_nxt,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        // update snd_nxt to take into account End extra chunk
        if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
            tcp_tcb.snd_nxt = end_snd_nxt + 1;
        };

        debug!("send_data_peosf: sleeping");
        // Python: 2000ms
        sleep(Duration::from_millis(500));

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };
        debug!("send_data_peoepsf: rcv_nxt: {}", rcv_nxt);

        debug!("send_data_peoepsf: sending data");
        chunk_c.iter().try_for_each(|(i, chunk_d)| {
            debug!("send_data_peoepsf: chunk {}", i);

            let seg_seq =
                self.snd_nxt_after_3whs + (*chunk_d.offset() * offset_multiplier + 1) as u32;
            debug!("send_data_peoepsf: seg_seq: {}", seg_seq);

            let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                Err(_) => panic!("Could not extract TcpTcb"),
            };
            debug!("send_data_peoepsf: rcv_nxt: {}", rcv_nxt);

            // Build and send data
            let data_u8_v = I::build_ethernet_ip_tcp_data(
                &self.test_target,
                self.port_src,
                self.port_dst,
                seg_seq,
                rcv_nxt,
                TcpFlags::ACK | TcpFlags::PSH,
                &chunk_d.get_chunk_pattern_ascii_v(payload_mode),
            );
            self.socket
                .send_raw_packet(&data_u8_v)
                .map_err(TcpSendingError::Io)?;

            let next_sent_data = seg_seq + chunk_d.internet_checksum_ascii_v().len() as u32;
            if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
                // NB: we use max because a chunk can be the last one and the data after it may not be located after all previously sent chunks.
                tcp_tcb.snd_nxt = max(tcp_tcb.snd_nxt, next_sent_data);
            };

            debug!("send_data_peoepsf: sleeping");
            sleep(Duration::from_millis(500));

            let r: Result<(), TcpSendingError> = Ok(());
            r
        })?;

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };
        debug!("send_data_peoepsf: rcv_nxt: {}", rcv_nxt);

        debug!("send_data_peoepsf: sending first byte-wise chunk as last one");
        // Build and send first byte-wise chunk
        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            self.snd_nxt_after_3whs,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        self.handle_common_end(
            thread_join_handle,
            response_manager_mode,
            &continue_data_receiving_mutex_arc,
        )?;

        debug!("send_data_peoepsf: end");

        Ok(())
    }

    fn send_data_peospep(
        &mut self,
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode,
        response_manager_mode: ResponseManagerMode,
    ) -> Result<(), TcpSendingError> {
        debug!("send_data_peospep: start");

        let offset_multiplier = tcp_utils::get_offset_multiplier(&self.payload_mode);

        let continue_data_receiving_mutex_arc = Arc::new(Mutex::new(true));
        let continue_data_receiving_mutex_arc_bonus =
            Arc::clone(&continue_data_receiving_mutex_arc);

        let tcp_tcb_mutex_arc_bonus = Arc::clone(&self.tcp_tcb_mutex_arc);

        let mut ack_manager = AckManager::init(
            &self.interface_name,
            self.test_target.clone(),
            self.port_src,
            self.port_dst,
            self.tcp_scenario.clone(),
            continue_data_receiving_mutex_arc_bonus,
            tcp_tcb_mutex_arc_bonus,
            0,
            vec![],
        );

        let thread_join_handle = thread::spawn(move || ack_manager.launch());

        // NB: 500ms of delay causes the first response ACK to be reordered just after the next sent data.
        sleep(Duration::from_millis(1000));

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        debug!("send_data_peospep2: sending first byte-wise chunk as first one");
        // Build and send first byte-wise chunk
        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            self.snd_nxt_after_3whs,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        // important for having consistent seq number for acking if ack progressive scenario
        if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
            tcp_tcb.snd_nxt += 1;
        };

        debug!("send_data_peosf: sleeping");
        // Python: 2000ms
        sleep(Duration::from_millis(500));

        debug!("send_data_peospep: sending last byte-wise chunk");
        // Build and send last byte-wise chunk

        // Get max ending offset
        let mut max_ending_offset = 0;
        chunk_c.iter().for_each(|(_i, chunk_d)| {
            if max_ending_offset < chunk_d.get_ending_offset() {
                max_ending_offset = chunk_d.get_ending_offset();
            }
        });
        debug!(
            "send_data_peospep: max_ending_offset: {}",
            max_ending_offset
        );

        let end_snd_nxt =
            self.snd_nxt_after_3whs + (max_ending_offset * offset_multiplier + 1) as u32;

        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            end_snd_nxt,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        // update snd_nxt to take into account End extra chunk
        if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
            tcp_tcb.snd_nxt = end_snd_nxt + 1;
        };

        debug!("send_data_peosf: sleeping");
        // Python: 2000ms
        sleep(Duration::from_millis(500));

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };
        debug!("send_data_peospep: rcv_nxt: {}", rcv_nxt);

        debug!("send_data_peospep: sending data");
        chunk_c.iter().try_for_each(|(i, chunk_d)| {
            debug!("send_data_peospep: chunk {}", i);

            let seg_seq =
                self.snd_nxt_after_3whs + (*chunk_d.offset() * offset_multiplier + 1) as u32;
            debug!("send_data_peospep: seg_seq: {}", seg_seq);

            let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                Err(_) => panic!("Could not extract TcpTcb"),
            };
            debug!("send_data_peospep: rcv_nxt: {}", rcv_nxt);

            // Build and send data
            let data_u8_v = I::build_ethernet_ip_tcp_data(
                &self.test_target,
                self.port_src,
                self.port_dst,
                seg_seq,
                rcv_nxt,
                TcpFlags::ACK | TcpFlags::PSH,
                &chunk_d.get_chunk_pattern_ascii_v(payload_mode),
            );
            self.socket
                .send_raw_packet(&data_u8_v)
                .map_err(TcpSendingError::Io)?;

            let next_sent_data = seg_seq + chunk_d.internet_checksum_ascii_v().len() as u32;
            if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
                // NB: we use max because a chunk can be the last one and the data after it may not be located after all previously sent chunks.
                tcp_tcb.snd_nxt = max(tcp_tcb.snd_nxt, next_sent_data);
            };

            debug!("send_data_peospep: sleeping");
            sleep(Duration::from_millis(500));

            let r: Result<(), TcpSendingError> = Ok(());
            r
        })?;

        self.handle_common_end(
            thread_join_handle,
            response_manager_mode,
            &continue_data_receiving_mutex_arc,
        )?;

        debug!("send_data_peospep: end");

        Ok(())
    }

    fn send_data_peoepsp(
        &mut self,
        chunk_c: &ChunkC,
        payload_mode: &PayloadMode,
        response_manager_mode: ResponseManagerMode,
    ) -> Result<(), TcpSendingError> {
        debug!("send_data_peoepsp: start");

        let offset_multiplier = tcp_utils::get_offset_multiplier(&self.payload_mode);

        let continue_data_receiving_mutex_arc = Arc::new(Mutex::new(true));
        let continue_data_receiving_mutex_arc_bonus =
            Arc::clone(&continue_data_receiving_mutex_arc);

        let tcp_tcb_mutex_arc_bonus = Arc::clone(&self.tcp_tcb_mutex_arc);

        let mut ack_manager = AckManager::init(
            &self.interface_name,
            self.test_target.clone(),
            self.port_src,
            self.port_dst,
            self.tcp_scenario.clone(),
            continue_data_receiving_mutex_arc_bonus,
            tcp_tcb_mutex_arc_bonus,
            0,
            vec![],
        );

        let thread_join_handle = thread::spawn(move || ack_manager.launch());

        // NB: 500ms of delay causes the first response ACK to be reordered just after the next sent data.
        sleep(Duration::from_millis(1000));

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        debug!("send_data_peoepsp: sending last byte-wise chunk");
        // Build and send last byte-wise chunk

        // Get max ending offset
        let mut max_ending_offset = 0;
        chunk_c.iter().for_each(|(_i, chunk_d)| {
            if max_ending_offset < chunk_d.get_ending_offset() {
                max_ending_offset = chunk_d.get_ending_offset();
            }
        });
        debug!(
            "send_data_peoepsp: max_ending_offset: {}",
            max_ending_offset
        );

        let end_snd_nxt =
            self.snd_nxt_after_3whs + (max_ending_offset * offset_multiplier + 1) as u32;

        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            end_snd_nxt,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        // update snd_nxt to take into account End extra chunk
        if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
            tcp_tcb.snd_nxt = end_snd_nxt + 1;
        };

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        debug!("send_data_peosf: sleeping");
        // Python: 2000ms
        sleep(Duration::from_millis(500));

        debug!("send_data_peoepsp: sending first byte-wise chunk as first one");
        // Build and send first byte-wise chunk
        let data_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            self.snd_nxt_after_3whs,
            rcv_nxt,
            TcpFlags::ACK | TcpFlags::PSH,
            "0".as_bytes(),
        );
        self.socket
            .send_raw_packet(&data_u8_v)
            .map_err(TcpSendingError::Io)?;

        debug!("send_data_peosf: sleeping");
        // Python: 2000ms
        sleep(Duration::from_millis(500));

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };
        debug!("send_data_peoepsp: rcv_nxt: {}", rcv_nxt);

        debug!("send_data_peoepsp: sending data");
        chunk_c.iter().try_for_each(|(i, chunk_d)| {
            debug!("send_data_peoepsp: chunk {}", i);

            let seg_seq =
                self.snd_nxt_after_3whs + (*chunk_d.offset() * offset_multiplier + 1) as u32;
            debug!("send_data_peoepsp: seg_seq: {}", seg_seq);

            let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                Err(_) => panic!("Could not extract TcpTcb"),
            };
            debug!("send_data_peoepsp: rcv_nxt: {}", rcv_nxt);

            // Build and send data
            let data_u8_v = I::build_ethernet_ip_tcp_data(
                &self.test_target,
                self.port_src,
                self.port_dst,
                seg_seq,
                rcv_nxt,
                TcpFlags::ACK | TcpFlags::PSH,
                &chunk_d.get_chunk_pattern_ascii_v(payload_mode),
            );
            self.socket
                .send_raw_packet(&data_u8_v)
                .map_err(TcpSendingError::Io)?;

            let next_sent_data = seg_seq + chunk_d.internet_checksum_ascii_v().len() as u32;
            if let Ok(mut tcp_tcb) = self.tcp_tcb_mutex_arc.lock() {
                // NB: we use max because a chunk can be the last one and the data after it may not be located after all previously sent chunks.
                tcp_tcb.snd_nxt = max(tcp_tcb.snd_nxt, next_sent_data);
            };

            debug!("send_data_peoepsp: sleeping");
            sleep(Duration::from_millis(500));

            let r: Result<(), TcpSendingError> = Ok(());
            r
        })?;

        self.handle_common_end(
            thread_join_handle,
            response_manager_mode,
            &continue_data_receiving_mutex_arc,
        )?;

        debug!("send_data_peoepsp: end");

        Ok(())
    }

    pub fn send_final_ack(&mut self) -> Result<(), TcpSendingError> {
        debug!("send_final_ack: start");

        let snd_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.snd_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        // TODO: find out why this ack is sometimes not visible.
        let ack_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            snd_nxt,
            rcv_nxt,
            TcpFlags::ACK,
            &[],
        );
        debug!("send_final_ack: sending final Ack");
        self.socket
            .send_raw_packet(&ack_u8_v)
            .map_err(TcpSendingError::Io)?;

        debug!("send_final_ack: end");
        Ok(())
    }

    pub fn build_connection_end_seg_seq(
        &self,
        last_sequence_number_offset_before_hole: u32,
        last_sequence_number_offset: u32,
    ) -> u32 {
        debug!("build_connection_end_seg_seq: start");

        debug!(
            "sendbuild_connection_end_seg_seq_rst: self.snd_nxt_after_3whs: {}",
            self.snd_nxt_after_3whs
        );
        debug!(
            "build_connection_end_seg_seq: last_sequence_number_offset_before_hole: {}",
            last_sequence_number_offset_before_hole
        );
        debug!(
            "build_connection_end_seg_seq: last_sequence_number_offset: {}",
            last_sequence_number_offset
        );
        // We need to add 1 to SEG.SEQ for peos because when we send an additional 0 at the end.

        // let scenario_seg_seq_offset = tcp_utils::build_scenario_seg_seq_offset(
        //     &self.tcp_scenario,
        //     last_sequence_number_offset_before_hole,
        //     last_sequence_number_offset,
        // );
        let sequence_number_offset = tcp_utils::get_sequence_number_offset_from_tcp_scenario(
            &self.tcp_scenario,
            last_sequence_number_offset_before_hole,
            last_sequence_number_offset,
        );
        // assert_eq!(scenario_seg_seq_offset, sequence_number_ofset);

        // if last_sequence_number_offset_before_hole == max_end_seq then we add 1 if there is an extra chunk after (i.e. EndPrecedes or EndFollows).
        let sequence_number_bonus = match self.tcp_scenario {
            TcpScenario::ProgressiveAckOnce => 0,
            TcpScenario::ProgressiveAckProgressive => 0,
            TcpScenario::OnceStartPrecedesAckProgressive => 1,
            TcpScenario::OnceStartPrecedesAckOnce => 1,
            TcpScenario::OnceStartFollows => 1,

            TcpScenario::OnceEndFollowsAckProgressive
            | TcpScenario::OnceEndFollowsAckOnce
            | TcpScenario::OnceEndPrecedesAckProgressive
            | TcpScenario::OnceEndPrecedesAckOnce => {
                if last_sequence_number_offset_before_hole == last_sequence_number_offset {
                    1
                } else {
                    0
                }
            }

            TcpScenario::OnceStartPrecedesEndFollowsAckProgressive
            | TcpScenario::OnceStartPrecedesEndFollowsAckOnce
            | TcpScenario::OnceStartPrecedesEndPrecedesAckProgressive
            | TcpScenario::OnceStartPrecedesEndPrecedesAckOnce
            | TcpScenario::OnceEndPrecedesStartPrecedesAckProgressive
            | TcpScenario::OnceEndPrecedesStartPrecedesAckOnce
            | TcpScenario::OnceStartFollowsEndFollows
            | TcpScenario::OnceEndFollowsStartFollows
            | TcpScenario::OnceEndPrecedesStartFollows => {
                if last_sequence_number_offset_before_hole == last_sequence_number_offset {
                    2
                } else {
                    1
                }
            }
        };
        assert_eq!(sequence_number_bonus, sequence_number_offset);

        // We add one because offset is zero if there is 1 byte of sent data.
        // TODO: check text above (cf proposition below)??
        // We add one because SEG.SEQ needs to be just after the last data.
        // TODO: change offset to make it start at 1 ?
        let seg_seq = self.snd_nxt_after_3whs
            + last_sequence_number_offset_before_hole
            + 1
            + sequence_number_offset;

        debug!("build_connection_end_seg_seq: seg_seq: {}", seg_seq);

        debug!("build_connection_end_seg_seq: end");

        return seg_seq;
    }

    pub fn send_rst(
        &self,
        seg_seq: u32, // last_sequence_number_offset_before_hole: u32,
                      // last_sequence_number_offset: u32,
    ) -> Result<(), TcpSendingError> {
        debug!("send_rst: start");

        // TEST OpenBSD 7.4
        //let seg_seq_2 = self.snd_nxt_after_3whs
        //    + last_sequence_number_offset
        //    + 1
        //    + sequence_number_bonus;

        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_) => panic!("Could not extract TcpTcb"),
        };

        // Build and send RSTACK
        let rst_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            seg_seq,
            rcv_nxt,
            TcpFlags::RST | TcpFlags::ACK,
            &[],
        );
        self.socket
            .send_raw_packet(&rst_u8_v)
            .map_err(TcpSendingError::Io)?;

        // TEST OpenBSD 7.4
        ////if seg_seq != seg_seq_2 {
        //let rst_u8_v = I::build_ethernet_ip_tcp_data(
        //    &self.test_target,
        //    self.port_src,
        //    self.port_dst,
        //    seg_seq_2,
        //    rcv_nxt,
        //    TcpFlags::RST | TcpFlags::ACK,
        //    &[],
        //);
        //self.socket
        //    .send_raw_packet(&rst_u8_v)
        //    .map_err(TcpSendingError::Io)?;
        ////}

        debug!("send_rst: end");

        Ok(())
    }

    pub fn send_multiple_rst(&self, chunk_c: &ChunkC) -> Result<(), TcpSendingError> {
        debug!("send_multiple_rst: start");

        let chunk_seg_seq_offset_v_before_hole =
            tcp_utils::get_chunk_last_seg_seq_offset_v_before_hole(
                chunk_c,
                &self.payload_mode,
                &self.tcp_scenario,
            );
        debug!(
            "send_multiple_rst: chunk_seg_seq_offset_v_before_hole: {:?}",
            chunk_seg_seq_offset_v_before_hole
        );

        // We first send a RST after the handshake in case the target rejects all chunks.
        self.send_rst(self.snd_nxt_after_3whs)?;

        // We then send a RST after each chunk.
        for chunk_seg_seq_offset in chunk_seg_seq_offset_v_before_hole {
            let scenario_start_seg_seq_offset =
                tcp_utils::build_scenario_start_seg_seq_offset(&self.tcp_scenario);
            let seg_seq = self.snd_nxt_after_3whs
                + chunk_seg_seq_offset as u32
                + scenario_start_seg_seq_offset
                + 1;
            debug!("send_multiple_rst: seg_seq: {}", seg_seq);

            self.send_rst(seg_seq)?;
        }

        debug!("send_multiple_rst: end");

        Ok(())
    }

    pub fn close(
        &mut self,
        last_sequence_number_offset_before_hole: u32,
        last_sequence_number_offset: u32,
    ) -> Result<(), TcpSendingError> {
        debug!("close: start");

        let seg_seq = self.build_connection_end_seg_seq(
            last_sequence_number_offset_before_hole,
            last_sequence_number_offset,
        );

        // let snd_nxt = match self.tcp_tcb_mutex_arc.lock() {
        //     Ok(tcp_tcb) => tcp_tcb.snd_nxt,
        //     Err(e) => {
        //         // panic!("Could not extract TcpTcb")
        //         return Err(TcpSendingError::PoisonError);
        //     }
        // };
        let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
            Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
            Err(_e) => {
                // panic!("Could not extract TcpTcb")
                return Err(TcpSendingError::PoisonError);
            }
        };
        // let snd_nxt = self.tcp_tcb_mutex_arc.lock().map(|tcp_tcb| tcp_tcb.snd_nxt);

        // Build and send SYN
        debug!("close: build FINACK");
        let fin_u8_v = I::build_ethernet_ip_tcp_data(
            &self.test_target,
            self.port_src,
            self.port_dst,
            seg_seq,
            rcv_nxt,
            TcpFlags::FIN | TcpFlags::ACK,
            &[],
        );
        debug!("close: send FINACK");
        self.socket
            .send_raw_packet(&fin_u8_v)
            .map_err(TcpSendingError::Io)?;

        let snd_nxt = seg_seq + 1;
        // match self.tcp_tcb_mutex_arc.lock() {
        //     Ok(mut tcp_tcb) => tcp_tcb.snd_nxt += 1,
        //     Err(_) => {
        //         // panic!("Could not extract TcpTcb")
        //         return Err(TcpSendingError::PoisonError);
        //     }
        // };

        let mut packet_u8_sl = [0; 2000];

        let mut waiting_for_finack = true;
        while waiting_for_finack {
            self.socket
                .read_raw_packet(&mut packet_u8_sl)
                .map_err(TcpSendingError::Io)?;

            let ethernet_packet = EthernetPacket::new(&packet_u8_sl)
                .ok_or("Could not build EthernetPacket")
                .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;

            //if ethernet_packet.get_ethertype() == EtherTypes::Ipv4
            //if ethernet_packet.get_ethertype() == EtherTypes::Ipv6
            //    && ethernet_packet.get_source() == self.test_target.mac_addr_dst
            //    && ethernet_packet.get_destination() == self.test_target.mac_addr_src
            if ethernet_packet.get_source() == self.test_target.mac_addr_dst
                && ethernet_packet.get_destination() == self.test_target.mac_addr_src
            {
                //let ip_packet = Ipv4Packet::new(ethernet_packet.payload())
                //let ip_packet = Ipv6Packet::new(ethernet_packet.payload())
                //    .ok_or("Could not build Ipv6Packet")
                //    .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;
                let (ip_packet_src, ip_packet_dst, ip_packet_next_protocol, ip_packet_payload): (IpAddr,IpAddr,_,_) = match ethernet_packet.get_ethertype() {
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

                //if ip_packet.get_source() == self.test_target.ip_addr_dst
                //    && ip_packet.get_destination() == self.test_target.ip_addr_src
                //    //&& ip_packet.get_next_level_protocol() == IpNextHeaderProtocols::Tcp
                //    //&& ip_packet.get_next_header() == IpNextHeaderProtocols::Tcp
                //    && ( (ethernet_packet.get_ethertype() == EtherTypes::Ipv4 
                //            && ip_packet.get_next_level_protocol() == IpNextHeaderProtocols::Tcp)
                //      || (ethernet_packet.get_ethertype() == EtherTypes::Ipv6 
                //            && ip_packet.get_next_header() == IpNextHeaderProtocols::Tcp)
                //    )
                if ip_packet_src == self.test_target.ip_addr_dst.clone().into()
                    && ip_packet_dst == self.test_target.ip_addr_src.clone().into()
                    && ip_packet_next_protocol == IpNextHeaderProtocols::Tcp
                {
                    //let tcp_packet = TcpPacket::new(ip_packet.payload())
                    let tcp_packet = TcpPacket::new(ip_packet_payload.as_slice())
                        .ok_or("Could not build TcpPacket")
                        .map_err(|e| TcpSendingError::Pnet(e.to_string()))?;

                    let tcp_flags = tcp_packet.get_flags();
                    let fin_b = tcp_flags & TcpFlags::FIN == TcpFlags::FIN;
                    let ack_b = tcp_flags & TcpFlags::ACK == TcpFlags::ACK;

                    if tcp_packet.get_source() == self.port_dst
                        && tcp_packet.get_destination() == self.port_src
                    {
                        debug!("close: packet for us received");

                        debug!("close: fin_b: {:?}, ack_b: {:?}", fin_b, ack_b);

                        if !fin_b && ack_b {
                            debug!("close: ack received");
                            // TODO: update RCV_NXT
                        }
                        // TODO: check seg_seq == rcv_nxt
                        else if fin_b && ack_b {
                            debug!("close: FINACK received");

                            waiting_for_finack = false;

                            debug!("close: tcp_packet: {:?}", tcp_packet);

                            // TODO: manage data in FINACK
                            match self.tcp_tcb_mutex_arc.lock() {
                                Ok(mut tcp_tcb) => {
                                    tcp_tcb.snd_una = tcp_packet.get_acknowledgement()
                                }
                                Err(_) => {
                                    // panic!("Could not extract TcpTcb")
                                    return Err(TcpSendingError::PoisonError);
                                }
                            };
                            match self.tcp_tcb_mutex_arc.lock() {
                                Ok(mut tcp_tcb) => tcp_tcb.rcv_nxt = tcp_packet.get_sequence() + 1,
                                Err(_) => {
                                    // panic!("Could not extract TcpTcb")
                                    return Err(TcpSendingError::PoisonError);
                                }
                            };
                            match self.tcp_tcb_mutex_arc.lock() {
                                Ok(mut tcp_tcb) => tcp_tcb.irs = tcp_packet.get_sequence(),
                                Err(_) => {
                                    // panic!("Could not extract TcpTcb")
                                    return Err(TcpSendingError::PoisonError);
                                }
                            };

                            let finack_to_ack_delay_ms = 200;
                            sleep(Duration::from_millis(finack_to_ack_delay_ms));

                            let rcv_nxt = match self.tcp_tcb_mutex_arc.lock() {
                                Ok(tcp_tcb) => tcp_tcb.rcv_nxt,
                                Err(_) => {
                                    // panic!("Could not extract TcpTcb")
                                    return Err(TcpSendingError::PoisonError);
                                }
                            };

                            // Build ACK
                            let ack_u8_v = I::build_ethernet_ip_tcp_data(
                                &self.test_target,
                                self.port_src,
                                self.port_dst,
                                snd_nxt,
                                rcv_nxt,
                                TcpFlags::ACK,
                                &[],
                            );
                            // Send ACK
                            self.socket
                                .send_raw_packet(&ack_u8_v)
                                .map_err(TcpSendingError::Io)?;

                            // sleep(Duration::from_millis(finack_to_ack_delay_ms));
                            // // Send ACK
                            // self.socket
                            //     .send_raw_packet(&ack_u8_v)
                            //     .map_err(TcpSendingError::Io)?;
                        } else {
                            let syn_b = tcp_flags & TcpFlags::SYN == TcpFlags::SYN;
                            let rst_b = tcp_flags & TcpFlags::RST == TcpFlags::RST;
                            let urg_b = tcp_flags & TcpFlags::URG == TcpFlags::URG;
                            debug!("close: unexpected flag combination: SYN {} ACK {} FIN {} RST {} URG {} ",
                                syn_b, ack_b, fin_b, rst_b, urg_b);
                            return Err(TcpSendingError::ServerError(format!(
                                "Expected SYNACK but found SYN {} ACK {} FIN {} RST {} URG {} ",
                                syn_b, ack_b, fin_b, rst_b, urg_b
                            )));
                        }
                    }
                }
            }
        }
        debug!("close: end");

        Ok(())
    }

    pub fn handle_common_end(
        &mut self,
        // Is this a copy of thread_join_handle from the calling function or the same object?
        thread_join_handle: JoinHandle<Result<(), TcpSendingError>>,
        response_manager_mode: ResponseManagerMode,
        continue_data_receiving_mutex_arc: &Arc<Mutex<bool>>,
    ) -> Result<(), TcpSendingError> {
        debug!("handle_common_end: sleeping");
        // NB: 100ms misses some final payload
        let sleep_duration = match response_manager_mode {
            ResponseManagerMode::UpdateTcb => 2000,
            ResponseManagerMode::UpdateTcbSendAck => 1000,
        };
        sleep(Duration::from_millis(sleep_duration));

        debug!("handle_common_end: stopping AckManager thread");
        // We ask the AckManager thread to stop.
        if let Ok(ref mut continue_data_receiving_mutex) = continue_data_receiving_mutex_arc.lock()
        {
            **continue_data_receiving_mutex = false;
        }

        debug!("handle_common_end: waiting AckManager thread");
        // We wait for the AckManager thread to stop.
        let _res = thread_join_handle.join().map_err(TcpSendingError::Truc)?;

        match response_manager_mode {
            ResponseManagerMode::UpdateTcb => self.send_final_ack()?,
            _ => {}
        };

        Ok(())
    }
}
