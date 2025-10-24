use serde::{Deserialize, Serialize};

use crate::byte_time_data::chunk::ChunkD;
use crate::misc::interval::IntervalD;
use crate::position::payload_mode::PayloadMode;
use crate::relation::relation_triplet::RelationTripletD;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TripletOverlap {
    relation_triplet_d: RelationTripletD,
    //payload_0: String,
    //payload_1: String,
    //payload_2: String,
    payload_0: Vec<u8>,
    payload_1: Vec<u8>,
    payload_2: Vec<u8>,
    start: u16,
    end: u16,
}

impl TripletOverlap {
    pub fn new(
        relation_triplet_d: RelationTripletD,
        //payload_0: String,
        //payload_1: String,
        //payload_2: String,
        payload_0: Vec<u8>,
        payload_1: Vec<u8>,
        payload_2: Vec<u8>,
        start: u16,
        end: u16,
    ) -> TripletOverlap {
        TripletOverlap {
            relation_triplet_d,
            payload_0,
            payload_1,
            payload_2,
            start,
            end,
        }
    }

    pub fn get_relation_triplet_d(&self) -> &RelationTripletD {
        &self.relation_triplet_d
    }

    pub fn get_start(&self) -> u16 {
        self.start
    }

    pub fn get_end(&self) -> u16 {
        self.end
    }

    //pub fn get_payload_0(&self) -> &String {
    pub fn get_payload_0(&self) -> &Vec<u8> {
        &self.payload_0
    }

    //pub fn get_payload_1(&self) -> &String {
    pub fn get_payload_1(&self) -> &Vec<u8> {
        &self.payload_1
    }

    //pub fn get_payload_2(&self) -> &String {
    pub fn get_payload_2(&self) -> &Vec<u8> {
        &self.payload_2
    }

    pub fn of_relation_interval_chunk(
        payload_mode: &PayloadMode,
        relation_triplet_d: RelationTripletD,
        interval_0: &IntervalD,
        interval_1: &IntervalD,
        interval_2: &IntervalD,
        chunk_0: &ChunkD,
        chunk_1: &ChunkD,
        chunk_2: &ChunkD,
    ) -> TripletOverlap {
        debug!("of_relation_interval_chunk: start");

        debug!("of_relation_interval_chunk: interval_0: {:?}", interval_0);
        debug!("of_relation_interval_chunk: interval_1: {:?}", interval_1);
        debug!("of_relation_interval_chunk: interval_2: {:?}", interval_2);
        debug!("of_relation_interval_chunk: chunk_0: {:?}", chunk_0);
        debug!("of_relation_interval_chunk: chunk_1: {:?}", chunk_1);
        debug!("of_relation_interval_chunk: chunk_2: {:?}", chunk_2);

        // let string_0 = chunk_0.get_string();
        // let string_1 = chunk_1.get_string();
        // let string_2 = chunk_2.get_string();
        //let string_0 = match payload_mode {
        //    PayloadMode::VariableChecksum1Byte(_) => chunk_0.get_simple_s(),
        //    PayloadMode::InvariantChecksumFixedLength8Byte(_) => chunk_0.get_internet_checksum_s(),
        //    PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(_) => todo!(),
        //    PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(_) => todo!(),
        //};
        //let string_1 = match payload_mode {
        //    PayloadMode::VariableChecksum1Byte(_) => chunk_1.get_simple_s(),
        //    PayloadMode::InvariantChecksumFixedLength8Byte(_) => chunk_1.get_internet_checksum_s(),
        //    PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(_) => todo!(),
        //    PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(_) => todo!(),
        //};
        //let string_2 = match payload_mode {
        //    PayloadMode::VariableChecksum1Byte(_) => chunk_2.get_simple_s(),
        //    PayloadMode::InvariantChecksumFixedLength8Byte(_) => chunk_2.get_internet_checksum_s(),
        //    PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(_) => todo!(),
        //    PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(_) => todo!(),
        //};
        let ascii_0 = chunk_0.get_chunk_pattern_ascii_v(payload_mode);
        let ascii_1 = chunk_1.get_chunk_pattern_ascii_v(payload_mode);
        let ascii_2 = chunk_2.get_chunk_pattern_ascii_v(payload_mode);

        let offset_0 = chunk_0.get_offset();
        let offset_1 = chunk_1.get_offset();
        let offset_2 = chunk_2.get_offset();

        let inter_tmp = interval_0.intersection(interval_1).unwrap();
        let inter = inter_tmp.intersection(interval_2).unwrap();
        let overlap_start = inter.get_start();
        let overlap_end = inter.get_end();
        debug!(
            "of_relation_interval_chunk: overlap position: {:?} -> {:?}",
            overlap_start, overlap_end
        );

        // We build the positions of the overlap inside the chunk data.
        let overlap_start_w_offset_0 = overlap_start - offset_0;
        let overlap_end_w_offset_0 = overlap_end - offset_0;
        let overlap_start_w_offset_1 = overlap_start - offset_1;
        let overlap_end_w_offset_1 = overlap_end - offset_1;
        let overlap_start_w_offset_2 = overlap_start - offset_2;
        let overlap_end_w_offset_2 = overlap_end - offset_2;

        //let factor = match payload_mode {
        //    // Each pattern contains a single character.
        //    PayloadMode::VariableChecksum1Byte(_) => 1,
        //    // Each pattern contains 8 characters.
        //    PayloadMode::InvariantChecksumFixedLength8Byte(_) 
        //    | PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(_) 
        //    | PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(_) => 8,
        //};
        let factor = payload_mode.get_factor();

        let overlap_start_w_offset_0_n = overlap_start_w_offset_0 * factor;
        let overlap_end_w_offset_0_n = (overlap_end_w_offset_0 + 1) * factor;
        let overlap_start_w_offset_1_n = overlap_start_w_offset_1 * factor;
        let overlap_end_w_offset_1_n = (overlap_end_w_offset_1 + 1) * factor;
        let overlap_start_w_offset_2_n = overlap_start_w_offset_2 * factor;
        let overlap_end_w_offset_2_n = (overlap_end_w_offset_2 + 1) * factor;
        debug!(
            "of_relation_interval_chunk: overlap 0 position n: {:?} -> {:?}",
            overlap_start_w_offset_0_n, overlap_end_w_offset_0_n
        );
        debug!(
            "of_relation_interval_chunk: overlap 1 position n: {:?} -> {:?}",
            overlap_start_w_offset_1_n, overlap_end_w_offset_1_n
        );
        debug!(
            "of_relation_interval_chunk: overlap 2 position n: {:?} -> {:?}",
            overlap_start_w_offset_2_n, overlap_end_w_offset_2_n
        );

        debug!("of_relation_interval_chunk: getting overlap_chunk_0");
        //let overlap_chunk_0 = string_0
        //    [overlap_start_w_offset_0_n as usize..overlap_end_w_offset_0_n as usize]
        //    .to_string();
        let overlap_chunk_0 = ascii_0
            [overlap_start_w_offset_0_n as usize..overlap_end_w_offset_0_n as usize].to_vec();
        debug!("of_relation_interval_chunk: getting overlap_chunk_1");
        //let overlap_chunk_1 = string_1
        //    [overlap_start_w_offset_1_n as usize..overlap_end_w_offset_1_n as usize]
        //    .to_string();
        let overlap_chunk_1 = ascii_1
            [overlap_start_w_offset_1_n as usize..overlap_end_w_offset_1_n as usize].to_vec();
        debug!("of_relation_interval_chunk: getting overlap_chunk_2");
        //let overlap_chunk_2 = string_2
        //    [overlap_start_w_offset_2_n as usize..overlap_end_w_offset_2_n as usize]
        //    .to_string();
        let overlap_chunk_2 = ascii_2
            [overlap_start_w_offset_2_n as usize..overlap_end_w_offset_2_n as usize].to_vec();
        debug!("of_relation_interval_chunk: end");

        TripletOverlap::new(
            relation_triplet_d,
            overlap_chunk_0,
            overlap_chunk_1,
            overlap_chunk_2,
            overlap_start,
            overlap_end,
        )
    }
}
