use core::time::Duration;
use std::path::Path;
use std::thread::sleep;

use subprocess::Popen;
use subprocess::PopenConfig;
use subprocess::Redirection;

use crate::byte_time_data::chunk::ChunkC;
use crate::misc::test_target::TestTarget;
use crate::position::payload_mode::PayloadMode;
use crate::tcp_chunk::chunk_sender::ChunkSender;
use crate::tcp_chunk::tcp_scenario::TcpScenario;
use crate::tcp_chunk::tcp_sending_error::TcpSendingError;
use crate::tcp_chunk::tcp_utils;

use super::connection_end_mode::ConnectionEndMode;

use std::fmt::Display;
use std::fmt::Debug;
use crate::misc::ip_addr_fragmentation_trait_impl::IpAddrForFragmentationTesting;

pub fn process<I: Display + Debug + PartialEq + Clone + IpAddrForFragmentationTesting + Send + 'static>(
    output_pcap_path: &Path,
    interface_name: String,
    test_target: &TestTarget<I>,
    port_source: u16,
    port_destination: u16,
    tcp_scenario: TcpScenario,
    payload_mode: PayloadMode,
    chunk_c: &ChunkC,
    connection_end_mode: &ConnectionEndMode,
) -> Result<(), TcpSendingError> {
    debug!("process: start");

    let last_sequence_number_offset_before_hole =
        tcp_utils::get_last_sequence_number_offset_before_hole(&payload_mode, chunk_c);
    let last_sequence_number_offset =
        tcp_utils::get_last_sequence_number_offset(&payload_mode, chunk_c);
    debug!(
        "process: last_sequence_number_offset_before_hole: {}",
        last_sequence_number_offset_before_hole
    );
    debug!(
        "process: last_sequence_number_offset: {}",
        last_sequence_number_offset
    );
    let mut chunk_sender = ChunkSender::init(
        interface_name,
        (*test_target).clone(),
        port_source,
        port_destination,
        tcp_scenario,
        payload_mode.clone(),
    );

    debug!("process: launching tcpdump");
    let mut p = Popen::create(
        &[
            "tcpdump",
            "-U",
            "-i",
            "eth1",
            "-w",
            output_pcap_path
                .to_str()
                .ok_or("Could not convert PCAP path to string")
                .map_err(|e| TcpSendingError::PathToString(e.to_string()))?,
            "-nn",
            &format!("host {} and port {}", test_target.ip_addr_dst, port_source),
        ],
        PopenConfig {
            stdout: Redirection::Pipe,
            ..Default::default()
        },
    )
    .map_err(TcpSendingError::Popen)?;

    // TODO: add way to exit from main in case of error without panic ; panic closes the main thread without waiting for the tcpdump thread and everything is stuck after

    // NB: 500ms often miss the TCP handshake.
    sleep(Duration::from_millis(1000));

    debug!("process: TCP connect");
    // TODO: add this for all other use of ? until p.terminate
    // chunk_sender.connect()?;
    match chunk_sender.connect() {
        Ok(_) => (),
        Err(e) => {
            sleep(Duration::from_millis(2000));
            p.terminate().map_err(TcpSendingError::Io)?;
            return Err(e);
        }
    };

    debug!("send: sending data packet");
    chunk_sender.send_data(chunk_c, &payload_mode)?;
    // chunk_data_sender_functions::send_data(&mut chunk_sender, chunk_c, &payload_mode)?;

    // NB: 100ms often miss some echoed data.
    sleep(Duration::from_millis(2000));

    match connection_end_mode {
        ConnectionEndMode::Rst => {
            debug!("send: sending RST");

            let seg_seq = chunk_sender.build_connection_end_seg_seq(
                last_sequence_number_offset_before_hole as u32,
                last_sequence_number_offset as u32,
            );

            chunk_sender.send_rst(
                seg_seq, // last_sequence_number_offset_before_hole as u32,
                        // last_sequence_number_offset as u32,
            )?;
        }
        ConnectionEndMode::MultipleRst => {
            debug!("send: sending multiple RST");
            chunk_sender.send_multiple_rst(
                // last_sequence_number_offset_before_hole as u32,
                // last_sequence_number_offset as u32,
                chunk_c
            )?;
        }
        ConnectionEndMode::FinHandshake => {
            debug!("send: sending FIN handshake");
            chunk_sender.close(
                last_sequence_number_offset_before_hole as u32,
                last_sequence_number_offset as u32,
            )?;
        }
    }
    // NB: 1000ms often miss the RST.
    sleep(Duration::from_millis(2000));

    p.terminate().map_err(TcpSendingError::Io)?;

    debug!("process: end");

    Ok(())
}
