// #[macro_use]
extern crate clap;
extern crate env_logger;
extern crate itertools;
extern crate log;
extern crate pcap_parser;
extern crate pnet;
// extern crate rand;

use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

use clap::{Arg, ArgAction, Command};

use rst_lib::byte_data::pair_triplet_byte_sequence::PairTripletByteSequence;
use rst_lib::byte_time_data::pair_triplet_byte_time_sequence::PairTripletByteTimeSequence;

fn main() -> io::Result<()> {
    env_logger::init();

    let matches = Command::new("generate-byte-time-sequence")
        .version("0.1")
        .author("Johan Mazel <johan.mazel@ssi.gouv.fr>")
        .about("Generate byte time sequence from byte sequence.")
        .arg(
            Arg::new("input-json-path")
                .short('i')
                .long("input-json-path")
                .help("Input JSON path")
                .required(true),
        )
        .arg(
            Arg::new("simple-chunk-pattern-json-path")
                .long("scp")
                .help("Simple chunk pattern JSON path")
                .required(true),
        )
        .arg(
            Arg::new("internet-checksum-chunk-pattern-json-path")
                .long("iccp")
                .help("Internet checksum chunk pattern JSON path")
                .required(true),
        )
        .arg(
            Arg::new("output-json-path")
                .short('o')
                .long("output-json-path")
                .help("Output json path")
                .required(true),
        )
        .arg(
            Arg::new("temporal-shuffling")
                .short('t')
                .long("temporal-shuffling")
                .help("Enable temporal shuffling")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("more-fragment-mode")
                .long("mfm")
                .help("More fragment mode")
                .required(true),
        )
        .get_matches();

    // let input_json_path_s = match matches.value_of("input-json-path") {
    let input_json_path_s = match matches.get_one::<String>("input-json-path") {
        Some(s) => s,
        None => {
            eprintln!("No input JSON path provided");
            exit(-1);
        }
    };
    println!("input_json_path_s: {}", input_json_path_s);
    let input_json_path = Path::new(input_json_path_s);

    let simple_chunk_pattern_json_path_s =
        match matches.get_one::<String>("simple-chunk-pattern-json-path") {
            Some(s) => s,
            None => {
                eprintln!("No simple chunk pattern JSON path provided");
                exit(-1);
            }
        };
    println!(
        "simple_chunk_pattern_json_path_s: {}",
        simple_chunk_pattern_json_path_s
    );
    let simple_chunk_pattern_json_path = Path::new(simple_chunk_pattern_json_path_s);

    let internet_checksum_chunk_pattern_json_path_s =
        match matches.get_one::<String>("internet-checksum-chunk-pattern-json-path") {
            Some(s) => s,
            None => {
                eprintln!("No internet checksum chunk pattern JSON path provided");
                exit(-1);
            }
        };
    println!(
        "internet_checksum_chunk_pattern_json_path_s: {}",
        internet_checksum_chunk_pattern_json_path_s
    );
    let internet_checksum_chunk_pattern_json_path =
        Path::new(internet_checksum_chunk_pattern_json_path_s);

    // let output_json_path_s = match matches.value_of("output-json-path") {
    let output_json_path_s = match matches.get_one::<String>("output-json-path") {
        Some(s) => s,
        None => {
            eprintln!("No output JSON path provided");
            exit(-1);
        }
    };
    println!("output_directory_path_s: {}", output_json_path_s);
    let output_json_path = Path::new(output_json_path_s);

    let temporal_shuffling = matches.get_flag("temporal-shuffling");
    println!("temporal_shuffling: {}", temporal_shuffling);

    // // Simple examples for debug ; TODO: move to test
    // icmp_echo_request_generation::simple_fragmentation_test(&test_target, 10, 0)?;
    // icmp_echo_request_generation::simple_fragmentation_overlap_test(&test_target, 10, 0)?;

    let file = File::open(input_json_path)?;
    let reader = BufReader::new(file);
    let all_byte_sequence: PairTripletByteSequence = serde_json::from_reader(reader)?;

    let file = File::open(simple_chunk_pattern_json_path)?;
    let reader = BufReader::new(file);
    let simple_chunk_pattern_v: Vec<String> = serde_json::from_reader(reader)?;

    let file = File::open(internet_checksum_chunk_pattern_json_path)?;
    let reader = BufReader::new(file);
    let internet_checksum_chunk_pattern_v: Vec<String> = serde_json::from_reader(reader)?;

    println!("building byte_time_sequence");
    let pair_triplet_byte_time_sequence = PairTripletByteTimeSequence::of_all_byte_sequence(
        simple_chunk_pattern_v,
        internet_checksum_chunk_pattern_v,
        temporal_shuffling,
        &all_byte_sequence,
    );

    let json_string: String =
        serde_json::to_string_pretty(&pair_triplet_byte_time_sequence).unwrap();
    fs::write(output_json_path, json_string).expect("Unable to write file");

    Ok(())
}
