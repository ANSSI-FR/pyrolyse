use crate::byte_time_data::chunk::ChunkC;
use crate::byte_time_data::chunk::ChunkD;
use crate::position::payload_mode::PayloadMode;
use crate::tcp_chunk::tcp_scenario::TcpScenario;

pub fn get_offset_multiplier(payload_mode: &PayloadMode) -> u16 {
    match payload_mode {
        PayloadMode::VariableChecksum1Byte(_) => 1,
        PayloadMode::InvariantChecksumFixedLength8Byte(_)
        | PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(_)
        | PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(_) => 8,
    }
}

pub fn build_scenario_start_seg_seq_offset(
    tcp_scenario: &TcpScenario,
    // last_sequence_number_offset_before_hole: u32,
    // last_sequence_number_offset: u32,
) -> u32 {
    debug!("build_scenario_start_seg_seq_offset: start");

    // if last_sequence_number_offset_before_hole == max_end_seq then we add 1 if there is an extra chunk after (i.e. EndPrecedes or EndFollows).
    let sequence_number_bonus = match tcp_scenario {
        TcpScenario::ProgressiveAckOnce => 0,
        TcpScenario::ProgressiveAckProgressive => 0,
        TcpScenario::OnceStartPrecedesAckProgressive => 1,
        TcpScenario::OnceStartPrecedesAckOnce => 1,
        TcpScenario::OnceStartFollows => 1,

        TcpScenario::OnceEndFollowsAckProgressive
        | TcpScenario::OnceEndFollowsAckOnce
        | TcpScenario::OnceEndPrecedesAckProgressive
        | TcpScenario::OnceEndPrecedesAckOnce => 0,

        TcpScenario::OnceStartPrecedesEndFollowsAckProgressive
        | TcpScenario::OnceStartPrecedesEndFollowsAckOnce
        | TcpScenario::OnceStartPrecedesEndPrecedesAckProgressive
        | TcpScenario::OnceStartPrecedesEndPrecedesAckOnce
        | TcpScenario::OnceEndPrecedesStartPrecedesAckProgressive
        | TcpScenario::OnceEndPrecedesStartPrecedesAckOnce
        | TcpScenario::OnceStartFollowsEndFollows
        | TcpScenario::OnceEndFollowsStartFollows
        | TcpScenario::OnceEndPrecedesStartFollows => 1,
    };

    debug!("build_scenario_start_seg_seq_offset: end");

    sequence_number_bonus
}

pub fn build_scenario_end_seg_seq_offset(
    tcp_scenario: &TcpScenario,
    last_sequence_number_offset_before_hole: u32,
    last_sequence_number_offset: u32,
) -> u32 {
    debug!("build_scenario_end_seg_seq_offset: start");

    // if last_sequence_number_offset_before_hole == max_end_seq then
    // we add 1 if there is an extra chunk after (i.e. EndPrecedes or EndFollows).
    let hole_not_present = last_sequence_number_offset_before_hole == last_sequence_number_offset;
    debug!(
        "build_scenario_end_seg_seq_offset: hole_not_present: {}",
        hole_not_present
    );

    let sequence_number_offset = match tcp_scenario {
        TcpScenario::ProgressiveAckOnce => 0,
        TcpScenario::ProgressiveAckProgressive => 0,
        TcpScenario::OnceStartPrecedesAckProgressive => 0,
        TcpScenario::OnceStartPrecedesAckOnce => 0,
        TcpScenario::OnceStartFollows => 0,

        TcpScenario::OnceEndFollowsAckProgressive
        | TcpScenario::OnceEndFollowsAckOnce
        | TcpScenario::OnceEndPrecedesAckProgressive
        | TcpScenario::OnceEndPrecedesAckOnce => {
            if hole_not_present {
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
            if hole_not_present {
                1
            } else {
                0
            }
        }
    };

    debug!("build_scenario_end_seg_seq_offset: end");

    sequence_number_offset
}

pub fn get_sequence_number_offset_from_tcp_scenario(
    tcp_scenario: &TcpScenario,
    last_sequence_number_offset_before_hole: u32,
    last_sequence_number_offset: u32,
) -> u32 {
    // if last_sequence_number_offset_before_hole == max_end_seq then
    // we add 1 if there is an extra chunk after (i.e. EndPrecedes or EndFollows).
    let hole_not_present = last_sequence_number_offset_before_hole == last_sequence_number_offset;

    let sequence_number_offset = match tcp_scenario {
        TcpScenario::ProgressiveAckOnce => 0,
        TcpScenario::ProgressiveAckProgressive => 0,
        TcpScenario::OnceStartPrecedesAckProgressive => 1,
        TcpScenario::OnceStartPrecedesAckOnce => 1,
        TcpScenario::OnceStartFollows => 1,

        TcpScenario::OnceEndFollowsAckProgressive
        | TcpScenario::OnceEndFollowsAckOnce
        | TcpScenario::OnceEndPrecedesAckProgressive
        | TcpScenario::OnceEndPrecedesAckOnce => {
            if hole_not_present {
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
            if hole_not_present {
                2
            } else {
                1
            }
        }
    };

    sequence_number_offset
}

fn last_seg_seq_at_chunk_offset(payload_mode: &PayloadMode, offset: u16) -> u32 {
    let offset_multiplier = get_offset_multiplier(payload_mode);

    // We add 7 to reach the last byte inside the data located at the considered offset.
    offset as u32 * offset_multiplier as u32 + 7
}

pub fn get_chunk_last_seg_seq_offset_v_before_hole(
    chunk_c: &ChunkC,
    payload_mode: &PayloadMode,
    tcp_scenario: &TcpScenario,
) -> Vec<u32> {
    debug!("get_seg_seq_v_before_hole: start");

    let last_offset_before_hole = get_last_offset_before_hole(&chunk_c);
    debug!(
        "get_seg_seq_v_before_hole: last_offset_before_hole: {}",
        last_offset_before_hole
    );

    let mut chunk_last_offset_v = chunk_c
        .iter()
        .map(|(_i, chunk_d)| chunk_d.get_ending_offset() - 1)
        .collect::<Vec<u16>>();
    assert_eq!(chunk_c.len(), chunk_last_offset_v.len());
    chunk_last_offset_v.sort();
    debug!(
        "get_seg_seq_v_before_hole: chunk_last_offset_v: {:?}",
        chunk_last_offset_v
    );

    let last_offset = *(chunk_last_offset_v.iter().max().unwrap());

    let chunk_last_offset_v_before_hole = chunk_last_offset_v
        .into_iter()
        .filter(|last_chunk_data_offset| *last_chunk_data_offset <= last_offset_before_hole)
        .collect::<Vec<u16>>();
    debug!(
        "get_seg_seq_v_before_hole: chunk_last_offset_v_before_hole: {:?}",
        chunk_last_offset_v_before_hole
    );

    // let offset_multiplier = get_offset_multiplier(payload_mode);

    let mut chunk_last_seg_seq_offset_v_before_hole = chunk_last_offset_v_before_hole
        .into_iter()
        .map(|offset| last_seg_seq_at_chunk_offset(payload_mode, offset))
        .collect::<Vec<u32>>();
    debug!(
        "get_seg_seq_v_before_hole: chunk_last_seg_seq_offset_v_before_hole: {:?}",
        chunk_last_seg_seq_offset_v_before_hole
    );

    let scenario_end_seg_seq_offset = build_scenario_end_seg_seq_offset(
        tcp_scenario,
        // Using offset instead of sequence number is not very clean, but it should work. :)
        last_offset_before_hole as u32,
        last_offset as u32,
    );
    debug!(
        "get_seg_seq_v_before_hole: scenario_end_seg_seq_offset: {}",
        scenario_end_seg_seq_offset
    );
    let v_len = chunk_last_seg_seq_offset_v_before_hole.len();
    chunk_last_seg_seq_offset_v_before_hole[v_len - 1] += scenario_end_seg_seq_offset;

    debug!("get_seg_seq_v_before_hole: end");

    chunk_last_seg_seq_offset_v_before_hole
}

pub fn get_offset_v_before_hole(chunk_c: &ChunkC) -> Vec<u16> {
    debug!("get_offset_v_before_hole: start");

    let last_offset_before_hole = get_last_offset_before_hole(&chunk_c);

    let chunk_last_offset_v = chunk_c
        .iter()
        .map(|(_i, chunk_d)| chunk_d.get_ending_offset() as u16 - 1_u16)
        .collect::<Vec<u16>>();
    assert_eq!(chunk_c.len(), chunk_last_offset_v.len());

    let chunk_last_offset_v_before_hole = chunk_last_offset_v
        .into_iter()
        .filter(|last_chunk_data_offset| *last_chunk_data_offset <= last_offset_before_hole)
        .collect::<Vec<u16>>();

    debug!("get_offset_v_before_hole: end");

    chunk_last_offset_v_before_hole
}

pub fn get_last_offset_before_hole(chunk_c: &ChunkC) -> u16 {
    debug!("get_last_offset_before_hole: start");

    debug!("get_last_offset_before_hole: chunk_c: {:?}", chunk_c);

    let mut offset_chunk_d_v = chunk_c
        .iter()
        .map(|(_i, chunk_d)| (*chunk_d.offset(), (*chunk_d).clone()))
        .collect::<Vec<(u16, ChunkD)>>();
    assert_eq!(chunk_c.len(), offset_chunk_d_v.len());

    offset_chunk_d_v.sort_by_key(|(offset, _)| *offset);

    debug!(
        "get_last_offset_before_hole: offset_chunk_d_v ({}): {:?}",
        offset_chunk_d_v.len(),
        offset_chunk_d_v
    );

    let last_offset_before_hole_init = 0_u16;

    let offset_at_hole_beginning = offset_chunk_d_v.iter().fold(
        last_offset_before_hole_init,
        |last_offset_before_hole_acc, (offset, chunk_d)| {
            // let chunk_byte_len = chunk_d.internet_checksum_ascii_v().len() as u16 / 8;
            let offset_after_chunk = chunk_d.get_ending_offset();
            debug!("get_last_offset_before_hole: offset {}, chunk_len: {}", offset, offset_after_chunk);

            let first_offset = *chunk_d.offset();
            debug!(
                "get_last_offset_before_hole: first_offset ({}) > last_offset_before_hole_acc ({}) + 1",
                first_offset,
                last_offset_before_hole_acc
            );
            // We got a hole
            if first_offset > last_offset_before_hole_acc + 1 {
                debug!("get_last_offset_before_hole: we have a hole");
                debug!(
                    "get_last_offset_before_hole: last_sequence_number_before_hole_acc: {}",
                    last_offset_before_hole_acc
                );
                last_offset_before_hole_acc
            } else {
                debug!("get_last_offset_before_hole: we do not have a hole");

                // We update max_offset_wihtout_hole only if current segment finishes after hole.
                let last_data_in_current_chunk = offset_after_chunk - 1_u16;
                debug!(
                    "get_last_offset_before_hole: last_data_in_current_chunk: {}",
                    last_data_in_current_chunk
                );
                if last_data_in_current_chunk > last_offset_before_hole_acc {
                    debug!("get_last_offset_before_hole: last_offset_before_hole_acc update");
                    let new_acc =last_data_in_current_chunk;

                    debug!(
                        "get_last_offset_before_hole: last_offset_before_hole_acc (new): {}",
                        new_acc
                    );
                    new_acc
                } else {
                    last_offset_before_hole_acc
                }
            }
        },
    );

    debug!("get_last_offset_before_hole: end");

    offset_at_hole_beginning
}

pub fn get_sequence_number_after_every_chunk(
    payload_mode: &PayloadMode,
    chunk_c: &ChunkC,
) -> Vec<u16> {
    debug!("get_sequence_number_offset_after_every_chunk: start");

    let offset_after_chunk_v = chunk_c
        .iter()
        .map(|(_i, chunk_d)| {
            // TODO: add check for overflow
            let offset_after_chunk = chunk_d.offset() + chunk_d.get_simple_ascii_v().len() as u16;
            offset_after_chunk
        })
        .collect::<Vec<u16>>();

    let offset_multiplier = get_offset_multiplier(payload_mode);

    let sequence_number_after_every_v = offset_after_chunk_v
        .iter()
        .map(|offset| offset * offset_multiplier)
        .collect::<Vec<u16>>();

    debug!("get_sequence_number_offset_after_every_chunk: end");

    sequence_number_after_every_v
}

// check if there is a hole in test case and, if so, return the maximum seq number before hole
pub fn get_last_sequence_number_offset_before_hole(
    payload_mode: &PayloadMode,
    chunk_c: &ChunkC,
) -> u16 {
    debug!("get_last_sequence_number_offset_before_hole: start");

    let offset_multiplier = get_offset_multiplier(payload_mode);

    debug!(
        "get_last_sequence_number_offset_before_hole: chunk_c: {:?}",
        chunk_c
    );

    let mut offset_chunk_d_v = chunk_c
        .iter()
        .map(|(_i, chunk_d)| (*chunk_d.offset(), (*chunk_d).clone()))
        .collect::<Vec<(u16, ChunkD)>>();
    assert_eq!(chunk_c.len(), offset_chunk_d_v.len());

    offset_chunk_d_v.sort_by_key(|(offset, _)| *offset);

    debug!(
        "get_last_sequence_number_offset_before_hole: offset_chunk_d_v ({}): {:?}",
        offset_chunk_d_v.len(),
        offset_chunk_d_v
    );

    let last_sequence_number_before_hole_init = 0_u16;

    let last_sequence_number_before_hole = offset_chunk_d_v.iter().fold(
        last_sequence_number_before_hole_init,
        |last_sequence_number_before_hole_acc, (offset, chunk_d)| {
            debug!("get_last_sequence_number_offset_before_hole: offset {}, chunk_d: {:?}", offset, chunk_d);

            let first_sequence_number = *chunk_d.offset() *offset_multiplier;
            debug!(
                "get_last_sequence_number_offset_before_hole: first_sequence_number ({}) > last_sequence_number_before_hole_acc ({}) + 1",
                first_sequence_number,
                last_sequence_number_before_hole_acc
            );
            // We got a hole
            if first_sequence_number > last_sequence_number_before_hole_acc + 1 {
                debug!("get_last_sequence_number_offset_before_hole: we have a hole");
                debug!(
                    "get_last_sequence_number_offset_before_hole: last_sequence_number_before_hole_acc: {}",
                    last_sequence_number_before_hole_acc
                );
                last_sequence_number_before_hole_acc
            } else {
                debug!("get_last_sequence_number_offset_before_hole: we do not have a hole");

                // We update max_offset_wihtout_hole only if current segment finishes after hole.
                let last_data_in_current_chunk = first_sequence_number + chunk_d.internet_checksum_ascii_v().len() as u16 - 1_u16;
                debug!(
                    "get_last_sequence_number_offset_before_hole: last_data_in_current_chunk: {}",
                    last_data_in_current_chunk
                );
                if last_data_in_current_chunk > last_sequence_number_before_hole_acc {
                    debug!("get_last_sequence_number_offset_before_hole: last_sequence_number_before_hole_acc update");
                    let new_acc = last_data_in_current_chunk;

                    debug!(
                        "get_last_sequence_number_offset_before_hole: last_sequence_number_before_hole_acc (new): {}",
                        new_acc
                    );
                    new_acc
                } else {
                    last_sequence_number_before_hole_acc
                }
            }
        },
    );

    let last_offset_before_hole = get_last_offset_before_hole(&chunk_c);
    let last_sequence_number_before_hole_to_compare =
        last_seg_seq_at_chunk_offset(payload_mode, last_offset_before_hole);
    assert_eq!(
        last_sequence_number_before_hole as u32,
        last_sequence_number_before_hole_to_compare
    );

    debug!("get_last_sequence_number_offset_before_hole: end");

    last_sequence_number_before_hole
}

pub fn get_last_sequence_number_offset(payload_mode: &PayloadMode, chunk_c: &ChunkC) -> u16 {
    debug!("get_last_sequence_number_offset: start");

    let offset_multiplier = get_offset_multiplier(payload_mode);

    debug!("get_last_sequence_number_offset: chunk_c: {:?}", chunk_c);

    let mut offset_chunk_d_v = chunk_c
        .iter()
        .map(|(_i, chunk_d)| (*chunk_d.offset(), (*chunk_d).clone()))
        .collect::<Vec<(u16, ChunkD)>>();
    assert_eq!(chunk_c.len(), offset_chunk_d_v.len());

    offset_chunk_d_v.sort_by_key(|(offset, _)| *offset);

    debug!(
        "get_last_sequence_number_offset: offset_chunk_d_v ({}): {:?}",
        offset_chunk_d_v.len(),
        offset_chunk_d_v
    );

    let last_sequence_number_init = 0_u16;

    let last_sequence_number = offset_chunk_d_v.iter().fold(
        last_sequence_number_init,
        |last_sequence_number_acc, (offset, chunk_d)| {
            debug!("get_last_sequence_number_offset: offset {}, chunk_d: {:?}", offset, chunk_d);

            let first_sequence_number = *chunk_d.offset() *offset_multiplier;
            debug!(
                "get_last_sequence_number_offset_before_hole: first_sequence_number ({}) > last_sequence_number_acc ({}) + 1",
                first_sequence_number,
                last_sequence_number_acc
            );

            let last_data_in_current_chunk = first_sequence_number + chunk_d.internet_checksum_ascii_v().len() as u16 - 1_u16;
            if last_data_in_current_chunk > last_sequence_number_acc {
                last_data_in_current_chunk
            }
            else {
                last_sequence_number_acc
            }
        },
    );

    debug!("get_last_sequence_number_offset: end");

    last_sequence_number
}

//pub fn display_packet<I: Display + Debug + PartialEq + Clone + IpAddrForFragmentationTesting>(
//    //test_target: &TestTarget<Ipv4Addr>,
//    test_target: &TestTarget<I>,
//    port_destination: u16,
//    port_source: u16,
//    tcp_tcb: &TcpTcb,
//    //ip_packet: &Ipv4Packet,
//    //ip_packet: &Ipv6Packet,
//    ip_source: &IpAddr,
//    ip_destination: &IpAddr,
//    tcp_packet: &TcpPacket,
//) {
//    let tcp_flags = tcp_packet.get_flags();
//
//    let tcp_payload_sl_length = tcp_packet.payload().len();
//
//    //if ip_packet.get_source() == test_target.ip_addr_dst
//    //    && ip_packet.get_destination() == test_target.ip_addr_src
//    if ip_source == test_target.ip_addr_dst
//        && ip_destination == test_target.ip_addr_src
//        && tcp_packet.get_source() == port_destination
//        && tcp_packet.get_destination() == port_source
//    {
//        debug!(
//            "display_packet: {:?} -> {:?} : {:?} - S={:?} A={:?} - Data: {:?}",
//            ip_source,
//            ip_destination,
//            tcp_flags,
//            tcp_packet.get_sequence() - tcp_tcb.iss,
//            tcp_packet.get_acknowledgement() - tcp_tcb.irs,
//            tcp_payload_sl_length
//        );
//    } else {
//        debug!(
//            "display_packet: {}:{} -> {}:{} : {} - S={:?} A={:?} - Data: {:?}",
//            ip_source,
//            tcp_packet.get_source(),
//            ip_destination,
//            tcp_packet.get_destination(),
//            tcp_flags,
//            tcp_packet.get_sequence(),
//            tcp_packet.get_acknowledgement(),
//            tcp_payload_sl_length
//        );
//    }
//}
