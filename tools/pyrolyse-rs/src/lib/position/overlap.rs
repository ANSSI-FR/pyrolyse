use serde::{Deserialize, Serialize};

use crate::byte_time_data::chunk::ChunkD;
use crate::misc::interval::IntervalD;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::relation::relation_triplet::RelationTripletD;
use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceD;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Overlap {
    byte_relation: AllenIntervalAlgebraRelation,
    start: u16,
    end: u16,
    payload_0: String,
    payload_1: String,
}

impl Overlap {
    pub fn new(
        byte_relation: AllenIntervalAlgebraRelation,
        start: u16,
        end: u16,
        payload_0: String,
        payload_1: String,
    ) -> Overlap {
        Overlap {
            byte_relation,
            start,
            end,
            payload_0,
            payload_1,
        }
    }

    pub fn get_byte_relation(&self) -> &AllenIntervalAlgebraRelation {
        &self.byte_relation
    }

    pub fn get_start(&self) -> u16 {
        self.start
    }

    pub fn get_end(&self) -> u16 {
        self.end
    }

    pub fn get_payload_0(&self) -> &String {
        &self.payload_0
    }

    pub fn get_payload_1(&self) -> &String {
        &self.payload_1
    }

    pub fn of_relation_interval_chunk(
        byte_relation: AllenIntervalAlgebraRelation,
        interval_0: &IntervalD,
        interval_1: &IntervalD,
        chunk_0: &ChunkD,
        chunk_1: &ChunkD,
    ) -> Overlap {
        debug!("of_relation_interval_chunk: start");

        debug!("of_relation_interval_chunk: interval_0: {:?}", interval_0);
        debug!("of_relation_interval_chunk: interval_1: {:?}", interval_1);
        debug!("of_relation_interval_chunk: chunk_0: {:?}", chunk_0);
        debug!("of_relation_interval_chunk: chunk_1: {:?}", chunk_1);

        let string_0 = chunk_0.get_string();
        let string_1 = chunk_1.get_string();
        let offset_0 = chunk_0.get_offset();
        let offset_1 = chunk_1.get_offset();

        let (overlap_start, overlap_end) = interval_0.overlap_index(&interval_1);
        debug!(
            "of_relation_interval_chunk: overlap position: {:?} -> {:?}",
            overlap_start, overlap_end
        );

        // We build the positions of the overlap inside the chunk data.
        let overlap_start_w_offset_0 = overlap_start - offset_0;
        let overlap_end_w_offset_0 = overlap_end - offset_0;
        let overlap_start_w_offset_1 = overlap_start - offset_1;
        let overlap_end_w_offset_1 = overlap_end - offset_1;

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
        debug!(
            "of_relation_interval_chunk: overlap 0 position n: {:?} -> {:?}",
            overlap_start_w_offset_0_n, overlap_end_w_offset_0_n
        );
        debug!(
            "of_relation_interval_chunk: overlap 1 position n: {:?} -> {:?}",
            overlap_start_w_offset_1_n, overlap_end_w_offset_1_n
        );

        debug!("of_relation_interval_chunk: getting overlap_chunk_0");
        let overlap_chunk_0 = string_0
            [overlap_start_w_offset_0_n as usize..overlap_end_w_offset_0_n as usize]
            .to_string();
        debug!("of_relation_interval_chunk: getting overlap_chunk_1");
        let overlap_chunk_1 = string_1
            [overlap_start_w_offset_1_n as usize..overlap_end_w_offset_1_n as usize]
            .to_string();

        debug!("of_relation_interval_chunk: end");

        Overlap::new(
            byte_relation,
            overlap_start,
            overlap_end,
            overlap_chunk_0,
            overlap_chunk_1,
        )
    }

    pub fn of_byte_time_sequence_d_triplet(
        byte_time_sequence_d: &ByteTimeSequenceD<RelationTripletD>,
    ) -> (Overlap, Overlap, Overlap) {
        // We reverse the order of 1 and 2 because the current implemntation send the chunk 1
        // in the last position and we need these position to build policy accurately.
        // TODO: clarify this comment because I do not understand.
        let chunk_c = byte_time_sequence_d.get_chunk_c();
        let chunk_0 = chunk_c.get(0).unwrap();
        let chunk_1 = chunk_c.get(1).unwrap();
        let chunk_2 = chunk_c.get(2).unwrap();

        let interval_c = byte_time_sequence_d.get_interval_c();
        let interval_0 = interval_c.get(0).unwrap();
        let interval_1 = interval_c.get(1).unwrap();
        let interval_2 = interval_c.get(2).unwrap();

        let overlap_01 = Overlap::of_relation_interval_chunk(
            byte_time_sequence_d.get_rc().get_relation_01().clone(),
            interval_0,
            interval_1,
            chunk_0,
            chunk_1,
        );

        let overlap_02 = Overlap::of_relation_interval_chunk(
            byte_time_sequence_d.get_rc().get_relation_02().clone(),
            interval_0,
            interval_2,
            chunk_0,
            chunk_2,
        );

        let overlap_12 = Overlap::of_relation_interval_chunk(
            byte_time_sequence_d.get_rc().get_relation_12().clone(),
            interval_1,
            interval_2,
            chunk_1,
            chunk_2,
        );

        (overlap_01, overlap_02, overlap_12)
    }
}
