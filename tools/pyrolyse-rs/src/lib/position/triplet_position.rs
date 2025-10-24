use serde::{Deserialize, Serialize};

use crate::byte_time_data::byte_time_sequence::ByteTimeSequenceD;
use crate::position::payload_mode::PayloadMode;
use crate::position::triplet_overlap::TripletOverlap;
use crate::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use crate::relation::relation_triplet::RelationTripletD;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TripletPosition {
    Disjoint(RelationTripletD),
    Overlap(TripletOverlap),
}

impl TripletPosition {

    pub fn of_byte_time_sequence_d_triplet(
        payload_mode: &PayloadMode,
        byte_time_sequence_d_triplet: &ByteTimeSequenceD<RelationTripletD>,
    ) -> TripletPosition {
        debug!("of_byte_time_sequence_d_triplet: start");

        let byte_relation_01 = byte_time_sequence_d_triplet
            .get_rc()
            .get_relation_01()
            .clone();
        let byte_relation_02 = byte_time_sequence_d_triplet
            .get_rc()
            .get_relation_02()
            .clone();
        let byte_relation_12 = byte_time_sequence_d_triplet
            .get_rc()
            .get_relation_12()
            .clone();

        let chunk_c = byte_time_sequence_d_triplet.get_chunk_c();
        let chunk_0 = chunk_c.get(&0).unwrap();
        let chunk_1 = chunk_c.get(&1).unwrap();
        let chunk_2 = chunk_c.get(&2).unwrap();

        let interval_c = byte_time_sequence_d_triplet.get_interval_c();
        let interval_0 = interval_c.get(&0).unwrap();
        let interval_1 = interval_c.get(&1).unwrap();
        let interval_2 = interval_c.get(&2).unwrap();


        let triplet_position = if interval_0.intersect(interval_1)
            && interval_0.intersect(interval_2)
            && interval_1.intersect(interval_2)
        {
            debug!("of_byte_time_sequence_d_triplet: triple overlap detected");
            TripletPosition::Overlap(TripletOverlap::of_relation_interval_chunk(
                payload_mode,
                byte_time_sequence_d_triplet.get_rc(),
                interval_0,
                interval_1,
                interval_2,
                chunk_0,
                chunk_1,
                chunk_2,
            ))
        } else {
            // Sanity check
            assert!(
                byte_relation_01 == AllenIntervalAlgebraRelation::B
                    || byte_relation_01 == AllenIntervalAlgebraRelation::Bi
                    || byte_relation_01 == AllenIntervalAlgebraRelation::M
                    || byte_relation_01 == AllenIntervalAlgebraRelation::Mi
                    || byte_relation_02 == AllenIntervalAlgebraRelation::B
                    || byte_relation_02 == AllenIntervalAlgebraRelation::Bi
                    || byte_relation_02 == AllenIntervalAlgebraRelation::M
                    || byte_relation_02 == AllenIntervalAlgebraRelation::Mi
                    || byte_relation_12 == AllenIntervalAlgebraRelation::B
                    || byte_relation_12 == AllenIntervalAlgebraRelation::Bi
                    || byte_relation_12 == AllenIntervalAlgebraRelation::M
                    || byte_relation_12 == AllenIntervalAlgebraRelation::Mi
            );

            TripletPosition::Disjoint(byte_time_sequence_d_triplet.get_rc())


            
        };

        debug!("of_byte_time_sequence_d_triplet: end");

        triplet_position
    }
    
    pub fn get_relation_triplet_d(&self) -> &RelationTripletD {
        match self {
            TripletPosition::Disjoint(relation_triplet_d) => relation_triplet_d,
            TripletPosition::Overlap(triplet_overlap) => triplet_overlap.get_relation_triplet_d(),
        }
    }
}
