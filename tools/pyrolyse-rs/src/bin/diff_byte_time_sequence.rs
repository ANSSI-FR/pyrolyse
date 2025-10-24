// #[macro_use]
extern crate clap;
extern crate env_logger;
extern crate itertools;
extern crate log;
extern crate pcap_parser;
extern crate pnet;
// extern crate rand;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

use clap::{Arg, Command};
use serde::Serialize;

use rst_lib::byte_time_data::byte_time_sequence::ByteTimeSequenceC;
use rst_lib::byte_time_data::pair_triplet_byte_time_sequence::PairTripletByteTimeSequence;
use rst_lib::misc::collections_utils;
use rst_lib::misc::interval::IntervalD;
use rst_lib::misc::test_index::TestIndex;
use rst_lib::relation::relation_container::RelationContainer;

fn build_hm<D: Clone + Serialize + RelationContainer>(
    byte_time_sequence_c: &ByteTimeSequenceC<D>,
) -> HashMap<Vec<IntervalD>, TestIndex> {
    println!("build_hm: start");

    let data_v = byte_time_sequence_c
        .iter()
        .map(|(i, byte_time_sequence)| {
            let temporal_position_v = byte_time_sequence.get_temporal_position_v();
            let mut interval_data_v = byte_time_sequence
                .get_interval_c()
                .iter()
                // .map(|i, interval| (i, interval, temporal_position_v[i]))
                .collect::<Vec<_>>();
            interval_data_v.sort_by_key(|(i, _)| temporal_position_v[**i as usize]);

            let interval_v = interval_data_v
                .into_iter()
                .map(|(_, interval_d)| (*interval_d).clone())
                .collect::<Vec<_>>();

            (*i, interval_v)
        })
        .collect::<Vec<_>>();

    let interval_v_i_v_v =
        collections_utils::group_iter_to_hm(data_v.iter(), &|(_, v)| v, &|(i, _)| i);
    let interval_v_i_v_v_more_than_2 = interval_v_i_v_v
        .into_iter()
        .filter(|(_, v)| v.len() >= 2)
        .collect::<Vec<_>>();
    println!(
        "build_hm: interval_v_i_v_v_more_than_2 ({}):\n{:?}",
        interval_v_i_v_v_more_than_2.len(),
        interval_v_i_v_v_more_than_2
    );

    // let interval_v_counter = data_v
    //     .clone()
    //     .into_iter()
    //     .map(|(_, v)| v)
    //     .collect::<Counter<_>>();
    // let interval_v_counter_v = interval_v_counter
    //     .iter()
    //     .filter(|(_, c)| **c >= 2)
    //     .map(|(v, c)| {
    //         let i_v = v
    //             .iter()
    //             .map(|interval_v| interval_v_i_hm.get(interval_v.clone()))
    //             .collect();
    //         (v, c, i_v)
    //     })
    //     .collect::<Vec<_>>();
    // println!(
    //     "build_hm: interval_v_counter_v:\n{:?}",
    //     interval_v_counter_v
    // );

    let interval_v_i_hm = data_v
        .clone()
        .into_iter()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<_, _>>();

    println!("build_hm: end");

    interval_v_i_hm
}

fn process(
    byte_time_sequence_0: PairTripletByteTimeSequence,
    byte_time_sequence_1: PairTripletByteTimeSequence,
) {
    let byte_time_triplet_sequence_c_0 = byte_time_sequence_0.get_byte_time_sequence_c_triplet();
    let byte_time_triplet_sequence_c_1 = byte_time_sequence_1.get_byte_time_sequence_c_triplet();

    let interval_v_i_hm_0 = build_hm(byte_time_triplet_sequence_c_0);
    let interval_v_i_hm_1 = build_hm(byte_time_triplet_sequence_c_1);

    println!("interval_v_i_hm_0 len: {:?}", interval_v_i_hm_0.len());
    println!("interval_v_i_hm_1 len: {:?}", interval_v_i_hm_1.len());

    let interval_v_hs_0 = interval_v_i_hm_0.keys().collect::<HashSet<_>>();
    let interval_v_hs_1 = interval_v_i_hm_1.keys().collect::<HashSet<_>>();

    let interval_v_hs_0_not_1 = interval_v_hs_0
        .difference(&interval_v_hs_1)
        .collect::<Vec<_>>();
    let i_0_not_1_v = interval_v_hs_0_not_1
        .iter()
        .map(|interval_v| interval_v_i_hm_0.get(**interval_v))
        .collect::<Vec<_>>();

    println!("i_0_not_1_v: {:?}", i_0_not_1_v);
}

fn main() -> io::Result<()> {
    env_logger::init();

    let matches = Command::new("diff-byte-time-sequence")
        .version("0.1")
        .author("Johan Mazel <johan.mazel@ssi.gouv.fr> and Lucas Aubard")
        .about("Diff byte time sequence.")
        .arg(
            Arg::new("input-json-0-path")
                .short('i')
                .long("input-json-0-path")
                .help("Input JSON 0 path")
                .required(true),
        )
        .arg(
            Arg::new("input-json-1-path")
                .short('j')
                .long("input-json-1-path")
                .help("Input JSON 1 path")
                .required(true),
        )
        .get_matches();

    // let input_json_0_path_s = match matches.value_of("input-json-0-path") {
    let input_json_0_path_s = match matches.get_one::<String>("input-json-0-path") {
        Some(s) => s,
        None => {
            eprintln!("No input JSON path 0 provided");
            exit(-1);
        }
    };
    println!("input_json_path_0_s: {}", input_json_0_path_s);
    let input_json_0_path = Path::new(input_json_0_path_s);

    // let input_json_1_path_s = match matches.value_of("input-json-1-path") {
    let input_json_1_path_s = match matches.get_one::<String>("input-json-1-path") {
        Some(s) => s,
        None => {
            eprintln!("No input JSON path 0 provided");
            exit(-1);
        }
    };
    println!("input_json_path_1_s: {}", input_json_1_path_s);
    let input_json_1_path = Path::new(input_json_1_path_s);

    // let output_json_path_s = match matches.value_of("output-json-path") {
    //     Some(s) => s,
    //     None => {
    //         eprintln!("No output JSON path provided");
    //         exit(-1);
    //     }
    // };
    // println!("output_directory_path_s: {}", output_json_path_s);
    // let output_json_path = Path::new(output_json_path_s);

    // // Simple examples for debug ; TODO: move to test
    // icmp_echo_request_generation::simple_fragmentation_test(&test_target, 10, 0)?;
    // icmp_echo_request_generation::simple_fragmentation_overlap_test(&test_target, 10, 0)?;

    let file = File::open(input_json_0_path)?;
    let reader = BufReader::new(file);
    let byte_time_sequence_0: PairTripletByteTimeSequence = serde_json::from_reader(reader)?;

    let file = File::open(input_json_1_path)?;
    let reader = BufReader::new(file);
    let byte_time_sequence_1: PairTripletByteTimeSequence = serde_json::from_reader(reader)?;

    process(byte_time_sequence_0, byte_time_sequence_1);

    // println!("building byte_time_sequence");
    // let pair_triplet_byte_time_sequence = PairTripletByteTimeSequence::of_all_byte_sequence(
    //     deactivate_temporal_shuffling,
    //     &all_byte_sequence,
    // );

    // let json_string: String =
    //     serde_json::to_string_pretty(&pair_triplet_byte_time_sequence).unwrap();
    // fs::write(output_json_path, json_string).expect("Unable to write file");

    Ok(())
}
