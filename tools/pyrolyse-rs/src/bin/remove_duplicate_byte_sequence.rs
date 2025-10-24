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

use clap::{Arg, Command};

use rst_lib::byte_data::pair_triplet_byte_sequence::PairTripletByteSequence;
// use rst_lib::byte_time_data::byte_time_sequence::ByteTimeSequence;
// use rst_lib::icmp_pcap::all_icmp_trace::AllIcmpTrace;
// use rst_lib::misc::icmp_echo_request_generation;
// use rst_lib::misc::test_target::TestTarget;

fn main() -> io::Result<()> {
    env_logger::init();

    let matches = Command::new("remove-duplicate-byte-sequence")
        .version("0.1")
        .author("Johan Mazel <johan.mazel@ssi.gouv.fr> and Lucas Aubard")
        .about("Remove generated duplicate byte sequence. DEPRECATED")
        .arg(
            Arg::new("input-json-path")
                .short('i')
                .long("input-json-path")
                .help("Input JSON path")
                .required(true),
        )
        .arg(
            Arg::new("output-json-path")
                .short('o')
                .long("output-json-path")
                .help("Output json path")
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

    let file = File::open(input_json_path)?;
    let reader = BufReader::new(file);
    let byte_sequence: PairTripletByteSequence = serde_json::from_reader(reader)?;

    println!("building byte_time_sequence");
    let byte_sequence_wo_duplicate = byte_sequence.remove_duplicate();

    let json_string: String = serde_json::to_string_pretty(&byte_sequence_wo_duplicate).unwrap();
    fs::write(output_json_path, json_string).expect("Unable to write file");

    Ok(())
}
