use std::cmp::max;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::iter::Iterator;
use std::path::Path;

use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceC;
use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceD;
use crate::byte_time_data::chunk::ChunkC;
use crate::icmp_pcap::icmp_packet::IcmpPacket;
use crate::icmp_pcap::policy_evaluation::PolicyEvaluation;
use crate::misc::icmp_echo_request_generation;
use crate::misc::internet_checksum_pattern_generator;
use crate::misc::interval::IntervalC;
use crate::misc::test_target::TestTarget;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::relation::relation_triplet::RelationTripletD;

use std::net::IpAddr;


#[derive(Debug, Clone)]
pub struct IcmpTraceD {
    v: Vec<IcmpPacket>,
}

impl IcmpTraceD {
    pub fn new(v: Vec<IcmpPacket>) -> IcmpTraceD {
        IcmpTraceD { v }
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&IcmpPacket> {
        self.v.get(index)
    }

    // fn extract_chunk_data(temporal_position_sl: &[u16], chunk_d: &ChunkD) -> (u16, u16) {
    //     let chunk_index = chunk_d.get_index();
    //     let temporal_position = temporal_position_sl.get(chunk_index as usize).unwrap();
    //     (chunk_index, *temporal_position)
    // }

    pub fn icmp_packet_v_of_data(
        test_target: &TestTarget,
        policy_evaluation: &PolicyEvaluation,
        temporal_position_sl: &[u16],
        icmp_header: &[u8],
        test_index: u16,
        sequence_index: u16,
        chunk_c: &ChunkC,
    ) -> Vec<IcmpPacket> {
        debug!("icmp_packet_v_of_data: start");

        let chunk_data_hm = match policy_evaluation {
            PolicyEvaluation::Progressive => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        // We shift temporal position by one to leave space for the header at the beginning.
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap() + 1;
                        let data = chunk_d.get_internet_checksum_ascii_v().clone();
                        let more_chunk = chunk_d.get_more_chunk();
                        let offset = chunk_d.get_offset() + 1;
                        (
                            *i,
                            (chunk_index, temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                let chunk_nb = chunk_c.len() as u16;

                // We insert the header chunk at the beginning.
                let starting_chunk_index = chunk_nb;
                let starting_chunk_temporal_position = 0;

                hm.insert(
                    starting_chunk_index,
                    (
                        starting_chunk_index,
                        starting_chunk_temporal_position,
                        true,
                        0,
                        icmp_header.to_vec(),
                    ),
                );
                hm
            }
            PolicyEvaluation::OnceStart => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap();
                        let more_chunk = chunk_d.get_more_chunk();
                        let offset = chunk_d.get_offset() + 1;
                        let data = chunk_d.get_internet_checksum_ascii_v().clone();

                        (
                            *i,
                            (chunk_index, *temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();
                let chunk_nb = chunk_c.len() as u16;

                // We insert the header chunk at the end.
                let starting_chunk_index = chunk_nb;
                let starting_chunk_temporal_position = chunk_nb;

                hm.insert(
                    starting_chunk_index,
                    (
                        starting_chunk_index,
                        starting_chunk_temporal_position,
                        true,
                        0,
                        icmp_header.to_vec(),
                    ),
                );
                hm
            }
            PolicyEvaluation::OnceEnd => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap() + 1;
                        // More_chunk is set to true because there will be a new last chunk.
                        let more_chunk = true;
                        let offset = chunk_d.get_offset() + 1;
                        let data = chunk_d.get_internet_checksum_ascii_v().clone();

                        (
                            *i,
                            (chunk_index, temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                // We process the ending chunk offset as the biggest offset + data length of all chunks.
                let ending_chunk_offset = hm.iter().fold(
                    0,
                    |acc, (_i, (_chunk_index, _temporal_position, _more_chunk, offset, data))| {
                        // let offset = chunk_d.get_offset();
                        let data_length = data.len();
                        let offset_after_chunk = offset + (data_length / 8) as u16;

                        // debug!(
                        //     "icmp_packet_v_of_data: {}: {} {}",
                        //     chunk_index,
                        //     offset,
                        //     data_length,
                        // );

                        max(acc, offset_after_chunk)
                    },
                );

                // debug!(
                //     "icmp_packet_v_of_data: ending_chunk_offset: {}",
                //     ending_chunk_offset
                // );

                let chunk_nb = chunk_c.len() as u16;

                // We insert the header chunk at the beginning.
                let starting_chunk_index = chunk_nb;
                let starting_chunk_temporal_position = 0;
                let starting_chunk_payload = icmp_header.to_vec();

                // We insert the tail chunk at the end.
                let ending_chunk_index = chunk_nb + 1;
                let ending_chunk_temporal_position = chunk_nb + 1;
                let internet_checksum_chunk_pattern_v =
                    internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();
                let ending_chunk_payload = internet_checksum_chunk_pattern_v
                    [internet_checksum_chunk_pattern_v.len() - 1]
                    .as_bytes()
                    .to_vec();

                hm.insert(
                    starting_chunk_index,
                    (
                        starting_chunk_index,
                        starting_chunk_temporal_position,
                        true,
                        0,
                        starting_chunk_payload,
                    ),
                );
                hm.insert(
                    ending_chunk_index,
                    (
                        ending_chunk_index,
                        ending_chunk_temporal_position,
                        false,
                        ending_chunk_offset,
                        ending_chunk_payload,
                    ),
                );
                hm
            }
            PolicyEvaluation::OnceStartEnd => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap();
                        // More_chunk is set to true because there will be a new last chunk.
                        let more_chunk = true;
                        let offset = chunk_d.get_offset() + 1;
                        let data = chunk_d.get_internet_checksum_ascii_v().clone();

                        (
                            *i,
                            (chunk_index, *temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                // We process the ending chunk offset as the biggest offset + data length of all chunks.
                let ending_chunk_offset = hm.iter().fold(
                    0,
                    |acc, (_i, (_chunk_index, _temporal_position, _more_chunk, offset, data))| {
                        // let offset = chunk_d.get_offset();
                        let data_length = data.len();
                        let offset_after_chunk = offset + (data_length / 8) as u16;

                        // debug!(
                        //     "icmp_packet_v_of_data: {}: {} {}",
                        //     chunk_index,
                        //     offset,
                        //     data_length,
                        // );

                        max(acc, offset_after_chunk)
                    },
                );

                // debug!(
                //     "icmp_packet_v_of_data: ending_chunk_offset: {}",
                //     ending_chunk_offset
                // );

                let chunk_nb = chunk_c.len() as u16;

                // We insert the header chunk just before the end.
                let starting_chunk_index = chunk_nb;
                let starting_chunk_temporal_position = chunk_nb;
                let starting_chunk_payload = icmp_header.to_vec();

                // We insert the tail chunk at the end.
                let ending_chunk_index = chunk_nb + 1;
                let ending_chunk_temporal_position = chunk_nb + 1;
                let internet_checksum_chunk_pattern_v =
                    internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();
                let ending_chunk_payload = internet_checksum_chunk_pattern_v
                    [internet_checksum_chunk_pattern_v.len() - 1]
                    .as_bytes()
                    .to_vec();

                hm.insert(
                    starting_chunk_index,
                    (
                        starting_chunk_index,
                        starting_chunk_temporal_position,
                        true,
                        0,
                        starting_chunk_payload,
                    ),
                );
                hm.insert(
                    ending_chunk_index,
                    (
                        ending_chunk_index,
                        ending_chunk_temporal_position,
                        false,
                        ending_chunk_offset,
                        ending_chunk_payload,
                    ),
                );
                hm
            }
            PolicyEvaluation::OnceEndStart => {
                let mut hm = chunk_c
                    .iter()
                    .map(|(i, chunk_d)| {
                        let chunk_index = chunk_d.get_index();
                        let temporal_position =
                            temporal_position_sl.get(chunk_index as usize).unwrap();
                        // More_chunk is set to true because there will be a new last chunk.
                        let more_chunk = true;
                        let offset = chunk_d.get_offset() + 1;
                        let data = chunk_d.get_internet_checksum_ascii_v().clone();

                        (
                            *i,
                            (chunk_index, *temporal_position, more_chunk, offset, data),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                let chunk_nb = chunk_c.len() as u16;

                // We process the ending chunk offset as the biggest offset + data length of all chunks.
                let ending_chunk_offset = hm.iter().fold(
                    0,
                    |acc, (_i, (_chunk_index, _temporal_position, _more_chunk, offset, data))| {
                        // let offset = chunk_d.get_offset();
                        let data_length = data.len();
                        let offset_after_chunk = offset + (data_length / 8) as u16;

                        // debug!(
                        //     "icmp_packet_v_of_data: {}: {} {}",
                        //     chunk_index,
                        //     offset,
                        //     data_length,
                        // );

                        max(acc, offset_after_chunk)
                    },
                );

                // We insert the header chunk at the end.
                let starting_chunk_index = chunk_nb;
                let starting_chunk_temporal_position = chunk_nb + 1;
                let starting_chunk_payload = icmp_header.to_vec();

                // We insert the tail chunk just before the end.
                let ending_chunk_index = chunk_nb + 1;
                let ending_chunk_temporal_position = chunk_nb;
                let internet_checksum_chunk_pattern_v =
                    internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();
                let ending_chunk_payload = internet_checksum_chunk_pattern_v
                    [internet_checksum_chunk_pattern_v.len() - 1]
                    .as_bytes()
                    .to_vec();

                hm.insert(
                    starting_chunk_index,
                    (
                        starting_chunk_index,
                        starting_chunk_temporal_position,
                        true,
                        0,
                        starting_chunk_payload,
                    ),
                );
                hm.insert(
                    ending_chunk_index,
                    (
                        ending_chunk_index,
                        ending_chunk_temporal_position,
                        false,
                        ending_chunk_offset,
                        ending_chunk_payload,
                    ),
                );
                hm
            }
        };

        let mut index_position_ethernet_data_v = chunk_data_hm
            .iter()
            .map(
                |(_, (chunk_index, temporal_position, more_chunk, offset, data))| {
                    // .map(|chunk_d| {
                    // debug!("icmp_packet_v_of_data: chunk_d: {:?}", chunk_d);
                    debug!("icmp_packet_v_of_data: chunk_index: {:?}", chunk_index);

                    // let chunk_index = chunk_d.get_index();
                    // let temporal_position = temporal_position_sl.get(chunk_index as usize).unwrap();

                    // If this chunk starts the pattern:
                    // let offset = if chunk_d.get_start() {
                    //     // offset should be 0 for isolated
                    //     assert_eq!(chunk_d.get_offset(), 0);
                    //     chunk_d.get_offset()
                    // } else {
                    //     // We add 1 to offset because we are not at the beginning and we need
                    //     // to add the ICMP header to the first fragment.
                    //     chunk_d.get_offset() + 1
                    // };
                    debug!(
                        "icmp_packet_v_of_data: chunk_d data ({:?}): {:?}",
                        data.len() / 8,
                        data
                    );
                    debug!(
                        "icmp_packet_v_of_data: chunk_d data hex ({:?}): {:x?}",
                        data.len() / 8,
                        data
                    );

                    // // If this chunk starts the pattern:
                    // let data = if chunk_d.get_start() {
                    //     // We add the ICMP header in front, and use chunk data after.
                    //     let fragment_data_tmp_v = chunk_d.get_internet_checksum_ascii_v().clone();
                    //     [icmp_header, &fragment_data_tmp_v].concat()
                    // } else {
                    //     // otherwise, we only use data.
                    //     chunk_d.get_internet_checksum_ascii_v().clone()
                    // };
                    // This is especially important if fragment 0 and 1 start at the same
                    // position.

                    // debug!("generate_isolated_pcap: data: {:?}", data);
                    // debug!(
                    //     "icmp_packet_v_of_data: data ({:?}): {:x?}",
                    //     data.len() / 8,
                    //     data
                    // );

                    // debug!("generate_isolated_pcap: \n\n\n\n");
                    (
                        *chunk_index,
                        *temporal_position,
                        IcmpPacket::new(
                            test_target.macaddr_src,
                            test_target.macaddr_dst,
                            test_target.ip_src,
                            test_target.ip_dst,
                            test_index,
                            *more_chunk,
                            // TODO: clean this
                            *offset,
                            test_index,
                            sequence_index,
                            data.to_vec(),
                        ),
                    )
                },
            )
            .collect::<Vec<(u16, u16, IcmpPacket)>>();

        debug!(
            "icmp_packet_v_of_data: index_position_ethernet_data_v: {:?}",
            index_position_ethernet_data_v
        );
        index_position_ethernet_data_v.sort_by_key(|tuple| {
            let (_, temporal_position, _) = tuple;
            *temporal_position
        });
        debug!(
            "icmp_packet_v_of_data: index_position_ethernet_data_v: {:?}",
            index_position_ethernet_data_v
        );

        let icmp_packet_v = index_position_ethernet_data_v
            .into_iter()
            .map(|(_, _, icmp_packet)| icmp_packet)
            .collect();

        debug!("icmp_packet_v_of_data: end");

        icmp_packet_v
    }

    pub fn of_data_byte_time_sequence_d_pair(
        test_target: &TestTarget,
        policy_evaluation: &PolicyEvaluation,
        icmp_header: &[u8],
        test_index: u16,
        byte_time_sequence_d: &ByteTimeSequenceD<AllenIntervalAlgebraRelation>,
    ) -> IcmpTraceD {
        debug!("of_data_byte_time_sequence_d_pair: start");

        let temporal_position_v = byte_time_sequence_d.get_temporal_position_v();

        let byte_sequence_index = byte_time_sequence_d.get_byte_sequence_index();

        let icmp_packet_v = IcmpTraceD::icmp_packet_v_of_data(
            test_target,
            policy_evaluation,
            temporal_position_v,
            icmp_header,
            test_index,
            byte_sequence_index,
            byte_time_sequence_d.get_chunk_c(),
        );

        debug!("of_data_byte_time_sequence_d_pair: end");

        IcmpTraceD::new(icmp_packet_v)
    }

    pub fn of_data_byte_time_sequence_d_triplet(
        test_target: &TestTarget,
        policy_evaluation: &PolicyEvaluation,
        icmp_header: &[u8],
        test_index: u16,
        byte_time_sequence_d: &ByteTimeSequenceD<RelationTripletD>,
    ) -> IcmpTraceD {
        debug!("of_data_byte_time_sequence_d_triplet: start");

        let temporal_position_v = byte_time_sequence_d.get_temporal_position_v();

        let sequence_index = byte_time_sequence_d.get_byte_sequence_index();

        let icmp_packet_v = IcmpTraceD::icmp_packet_v_of_data(
            test_target,
            policy_evaluation,
            temporal_position_v,
            icmp_header,
            test_index,
            sequence_index,
            byte_time_sequence_d.get_chunk_c(),
        );

        debug!("of_data_byte_time_sequence_d_triplet: end");

        // IcmpTrace::new(index_icmp_packet_v)
        IcmpTraceD::new(icmp_packet_v)
    }

    pub fn export(
        &self,
        // test_target: &TestTarget,
        pcap_directory_path: &Path,
        // icmp_header: &Vec<u8>,
        test_index: u16,
        // chunk_c: &ChunkC,
        // temporal_position_v: &Vec<u32>,
    ) {
        debug!("export_chunk_c: start");
        // debug!(
        //     "export_chunk_c: temporal_position_v: {:?}",
        //     temporal_position_v
        // );

        // let output_path_s = format!("{}/test_{}.pcap",pcap_directory_path, test_index);
        // let output_path = Path::new(&output_path_s);

        // let output_path = PathBuf::new();
        // output_path.push(pcap_directory_path);
        // output_path.push(format!("test_{}.pcap",test_index));
        let output_path = pcap_directory_path.join(format!("test_{}.pcap", test_index));

        // let output_path_s = format!("{}/test_{}.pcap",pcap_directory_path, test_index);
        debug!("export_chunk_c: output_path: {:?}", output_path);

        let mut output_file = if output_path
            .to_str()
            .expect("Could not convert output_path to string")
            == "-"
        {
            Box::new(io::stdout())
        } else {
            let file = File::create(output_path).unwrap();
            Box::new(file) as Box<dyn io::Write>
        };

        debug!("export_chunk_c: pcap_write_header");
        icmp_echo_request_generation::pcap_write_header(&mut output_file, 1500).unwrap();

        // index_position_ethernet_data_v.iter().enumerate().for_each(
        self.v.iter().enumerate().for_each(|(i, icmp_packet)| {
            let ts_sec = i as u32;
            let ts_usec = 0;

            let data = icmp_packet.build_ethernet();

            let written = icmp_echo_request_generation::pcap_write_packet(
                &mut output_file,
                ts_sec,
                ts_usec,
                &data,
            )
            .unwrap();
            debug!("export_chunk_c: written: {}", written);
        });
        debug!("export_chunk_c: end");
    }
}

#[cfg(test)]
mod tests {
    use crate::byte_data::byte_sequence::ByteSequenceD;
    use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceD;
    use crate::byte_time_data::chunk::ChunkC;
    use crate::byte_time_data::chunk::ChunkD;
    use crate::byte_time_data::export_mode::ExportMode;
    use crate::misc::internet_checksum_pattern_generator;
    use crate::misc::interval::IntervalC;
    use crate::misc::interval::IntervalD;
    use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;

    #[test]
    fn pair_o() {
        let relation = AllenIntervalAlgebraRelation::O;
        let interval_c = IntervalC::new(
            vec![
                // IntervalD::new(0, false, 0, 1),
                // IntervalD::new(1, false, 1, 2),
                (0, IntervalD::new(0, 1)),
                (1, IntervalD::new(1, 2)),
            ]
            .into_iter()
            .collect(),
        );

        let index_offset = 0;
        let data_offset = 0;
        let pair_sequence_d =
            ByteSequenceD::of_data(&relation, &interval_c, index_offset, data_offset);

        let sequence_index = 0;
        let simple_chunk_pattern_v =
            internet_checksum_pattern_generator::build_simple_chunk_pattern_v();
        let internet_checksum_chunk_pattern_v =
            internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();
        let temporal_position_v = vec![0, 1];
        let byte_time_pair_sequence_d =
            ByteTimeSequenceD::<AllenIntervalAlgebraRelation>::of_data(
                sequence_index,
                &ExportMode::Isolated,
                &simple_chunk_pattern_v,
                &internet_checksum_chunk_pattern_v,
                &pair_sequence_d,
                &temporal_position_v,
            );

        let simple_payload_0 = "AB".to_string();
        let simple_payload_1 = "CD".to_string();
        let internet_checksum_payload_0 = "AABBCCDDAABBDDCC".to_string();
        let internet_checksum_payload_1 = "AACCBBDDAACCDDBB".to_string();
        let simple_payload_byte_length = 3;
        let internet_checksum_payload_byte_length = 24;
        let chunk_c = ChunkC::new(
            vec![
                (
                    0,
                    ChunkD::new(
                        0,
                        true,
                        true,
                        0,
                        simple_payload_0.clone(),
                        simple_payload_0.as_bytes().to_vec(),
                        internet_checksum_payload_0.clone(),
                        internet_checksum_payload_0.as_bytes().to_vec(),
                    ),
                ),
                (
                    1,
                    ChunkD::new(
                        1,
                        false,
                        false,
                        1,
                        simple_payload_1.clone(),
                        simple_payload_1.as_bytes().to_vec(),
                        internet_checksum_payload_1.clone(),
                        internet_checksum_payload_1.as_bytes().to_vec(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        );
        let byte_time_pair_sequence_d_ref = ByteTimeSequenceD::<AllenIntervalAlgebraRelation>::new(
            sequence_index,
            relation,
            interval_c,
            chunk_c,
            temporal_position_v,
            simple_payload_byte_length,
            internet_checksum_payload_byte_length,
        );

        assert_eq!(byte_time_pair_sequence_d, byte_time_pair_sequence_d_ref);
    }
}

#[derive(Debug, Clone)]
pub struct IcmpTraceC {
    hm: HashMap<u16, IcmpTraceD>,
}

impl FromIterator<(u16, IcmpTraceD)> for IcmpTraceC {
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = (u16, IcmpTraceD)>,
    {
        Self {
            hm: HashMap::from_iter(iter),
        }
    }
}

impl IcmpTraceC {
    pub fn len(&self) -> usize {
        self.hm.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hm.is_empty()
    }

    pub fn get(&self, index: &u16) -> Option<&IcmpTraceD> {
        self.hm.get(index)
    }

    pub fn iter(&self) -> Iter<u16, IcmpTraceD> {
        self.hm.iter()
    }

    /// Build an ICMP header.
    fn build_icmp_header(
        policy_evaluation: &PolicyEvaluation,
        internet_checksum_chunk_pattern_sl: &[String],
        test_index: u16,
        sequence_index: u16,
        interval_c: &IntervalC,
        ip_src: IpAddr,
        ip_dst: IpAddr,
    ) -> Vec<u8> {
        // We build a fake packet to obtain a valid header.
        // Get the total payload length from generated relations
        let total_length = interval_c.get_total_length();
        // We increase by one if we add another interval/chunk at the end.
        let chunk_nb_for_checksum = match policy_evaluation {
            PolicyEvaluation::Progressive => total_length,
            PolicyEvaluation::OnceStart => total_length,
            PolicyEvaluation::OnceEnd => total_length + 1,
            PolicyEvaluation::OnceStartEnd => total_length + 1,
            PolicyEvaluation::OnceEndStart => total_length + 1,
        };
        // Get the data for this total length
        let pattern_v = &internet_checksum_chunk_pattern_sl[0..chunk_nb_for_checksum as usize];
        let data = pattern_v.join("").as_bytes().to_vec();
        // Build the ICMP packet with this (and thus correct checksum)
        let icmp_packet_data = icmp_echo_request_generation::build_icmp_data(
            test_index,
            sequence_index,
            &data,
            &ip_src,
            &ip_dst,
        );
        // Extract the ICMP header (type, code, identifier, sequence number) for the first
        // fragmentation block (it corresponds to a fragmentation offset of one, 8bytes)
        let icmp_header = &icmp_packet_data[0..8].to_vec();

        (*icmp_header).clone()
    }

    pub fn of_data_byte_time_pair_sequence_c(
        test_target: &TestTarget,
        policy_evaluation: &PolicyEvaluation,
        // export_mode: &ExportMode,
        internet_checksum_chunk_pattern_sl: &[String],
        byte_time_pair_sequence_c: &ByteTimeSequenceC<AllenIntervalAlgebraRelation>,
    ) -> IcmpTraceC {
        debug!("of_data: start");

        // debug!("of_triplet_sequence_c: building position permutation");
        // let temporal_position_v_v = std::ops::Range { start: 0, end: 2 }
        //     .permutations(2)
        //     .collect::<Vec<Vec<u32>>>();
        // debug!(
        //     "of_pair_sequence_c: temporal_position_v_v: {:?}",
        //     temporal_position_v_v
        // );

        let mut tuple_v: Vec<(&u16, &ByteTimeSequenceD<AllenIntervalAlgebraRelation>)> =
            byte_time_pair_sequence_c.iter().collect();
        tuple_v.sort_by_key(|(k, _v)| *k);

        debug!("of_data: building v");
        let map = tuple_v
            .iter()
            .map(|(test_index, byte_time_pair_sequence_d)| {
                debug!("of_data: sequence_index: {}", test_index);

                let icmp_header = IcmpTraceC::build_icmp_header(
                    policy_evaluation,
                    internet_checksum_chunk_pattern_sl,
                    **test_index,
                    byte_time_pair_sequence_d.get_byte_sequence_index(),
                    byte_time_pair_sequence_d.get_interval_c(),
                    test_target.ip_src,
                    test_target.ip_dst,
                );

                (
                    **test_index,
                    IcmpTraceD::of_data_byte_time_sequence_d_pair(
                        test_target,
                        policy_evaluation,
                        &icmp_header,
                        **test_index,
                        byte_time_pair_sequence_d,
                    ),
                )
            })
            // .collect::<Vec<(u16, IcmpTraceD)>>()
            ;

        debug!("of_data: building hm");
        let hm = map.into_iter().collect();

        debug!("of_data: end");
        IcmpTraceC { hm }
    }

    pub fn of_data_byte_time_triplet_sequence_c(
        test_target: &TestTarget,
        policy_evaluation: &PolicyEvaluation,
        chunk_pattern_sl: &[String],
        byte_time_triplet_sequence_c: &ByteTimeSequenceC<RelationTripletD>,
    ) -> IcmpTraceC {
        debug!("of_data_byte_time_triplet_sequence_c: start");

        // debug!("of_triplet_sequence_c: building position permutation");
        // let temporal_position_v_v = std::ops::Range { start: 0, end: 2 }
        //     .permutations(2)
        //     .collect::<Vec<Vec<u32>>>();
        // debug!(
        //     "of_pair_sequence_c: temporal_position_v_v: {:?}",
        //     temporal_position_v_v
        // );

        let mut tuple_v: Vec<(&u16, &ByteTimeSequenceD<RelationTripletD>)> =
            byte_time_triplet_sequence_c.iter().collect();
        tuple_v.sort_by_key(|(k, _v)| *k);

        debug!("of_data_byte_time_triplet_sequence_c: building v");
        let map = tuple_v
            .iter()
            .map(|(test_index, byte_time_triplet_sequence_d)| {
                debug!(
                    "of_data_byte_time_triplet_sequence_c: sequence_index: {}",
                    test_index
                );

                let icmp_header = IcmpTraceC::build_icmp_header(
                    policy_evaluation,
                    chunk_pattern_sl,
                    **test_index,
                    byte_time_triplet_sequence_d.get_byte_sequence_index(),
                    byte_time_triplet_sequence_d.get_interval_c(),
                    test_target.ip_src,
                    test_target.ip_dst,
                );

                (
                    **test_index,
                    IcmpTraceD::of_data_byte_time_sequence_d_triplet(
                        test_target,
                        policy_evaluation,
                        &icmp_header,
                        **test_index,
                        byte_time_triplet_sequence_d,
                    ),
                )
            })
            // .collect::<Vec<(u16, IcmpTraceD)>>()
            ;

        debug!("of_data_byte_time_triplet_sequence_c: building hm");
        let hm = map.into_iter().collect();

        debug!("of_data_byte_time_triplet_sequence_c: end");
        IcmpTraceC { hm }
    }

    pub fn export(&self, pcap_directory_path: &Path) {
        self.hm.iter().for_each(|(test_index, icmp_trace_d)| {
            icmp_trace_d.export(pcap_directory_path, *test_index)
        });
    }

    // pub fn export(
    //     &self,
    //     test_target: &TestTarget,
    //     pcap_directory_path: &Path,
    //     icmp_header: &Vec<u8>,
    //     test_index: u32,
    //     // chunk_c: &ChunkC,
    //     temporal_position_v: &Vec<u32>,
    // ) {
    //     debug!("export_chunk_c: start");
    //
    //     let index_data_v = self.hm.iter().map(|i, icmp_trace_d| {
    //         icmp_trace_d.to_data
    //     }).collect::<Vec<(u16, Vec<u8>)>>();
    //
    //     // let output_path_s = format!("{}/test_{}.pcap",pcap_directory_path, test_index);
    //     // let output_path = Path::new(&output_path_s);
    //
    //     // let output_path = PathBuf::new();
    //     // output_path.push(pcap_directory_path);
    //     // output_path.push(format!("test_{}.pcap",test_index));
    //     let output_path = pcap_directory_path.join(format!("test_{}.pcap", test_index));
    //
    //     // let output_path_s = format!("{}/test_{}.pcap",pcap_directory_path, test_index);
    //     debug!("export_chunk_c: output_path: {:?}", output_path);
    //
    //     let mut output_file = if output_path
    //         .to_str()
    //         .expect("Could not convert output_path to string")
    //         == "-"
    //     {
    //         Box::new(io::stdout())
    //     } else {
    //         let file = File::create(output_path).unwrap();
    //         Box::new(file) as Box<dyn io::Write>
    //     };
    //
    //     debug!("export_chunk_c: pcap_write_header");
    //     icmp_echo_request_generation::pcap_write_header(&mut output_file, 1500 as usize).unwrap();
    //
    //     index_position_ethernet_data_v.iter().enumerate().for_each(
    //         |(i, (_index, _position, ethernet_data))| {
    //             let ts_sec = i as u32;
    //             let ts_usec = 0;
    //             let written = icmp_echo_request_generation::pcap_write_packet(
    //                 &mut output_file,
    //                 ts_sec,
    //                 ts_usec,
    //                 &ethernet_data,
    //             )
    //             .unwrap();
    //             debug!("export_chunk_c: written: {}", written);
    //         },
    //     );
    //     debug!("export_chunk_c: end");
    // }
}
