// #[macro_use]
extern crate env_logger;
extern crate log;

extern crate itertools;
extern crate pcap_parser;
extern crate pnet;

extern crate clap;

use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

use clap::{Arg, ArgAction, Command};

use rst_lib::byte_time_data::pair_triplet_byte_time_sequence::PairTripletByteTimeSequence;
//use rst_lib::complicated_policy::all_position_policy::AllPositionPolicy;
use rst_lib::position::all_position_data::AllPositionData;
use rst_lib::reply_payload::reply_payload::ReplyPayloadC;
use rst_lib::reply_payload::reply_payload_hex::ReplyPayloadHexC;
use rst_lib::reply_payload::reply_payload_string::ReplyPayloadStringC;

fn main() -> io::Result<()> {
    env_logger::init();

    let matches = Command::new("extract-reassembly-policies")
        .version("0.1")
        .author("Johan Mazel <johan.mazel@ssi.gouv.fr> and Lucas Aubard")
        .about("Extract policies using test cases and observed payload")
        .arg(
            Arg::new("payload-mode")
                .long("payload-mode")
                .help("Payload mode: vc1b (variable checksum 1 byte)|icfl8b (invariant checksum fixed length 8 byte)|icvl8i4 (invariant checksum variable length 8 byte ICMPv4)|icvl8i6 (invariant checksum variable length 8 byte ICMPv6)")
                .required(true),
        )
        .arg(
            Arg::new("all-chunk-json-path")
                .long("all-chunk-json-path")
                .help("All chunk JSON path")
                .required(true),
        )
        .arg(
            Arg::new("payload-json-path").long("payload-json-path").help("Payload JSON path").required(true)
        )
        .arg(
            Arg::new("complicated-policy-json-path")
                .long("complicated-policy-json-path")
                .help("Complicated policy JSON path")
                .required(true),
        )
        .arg(
            Arg::new("payload-string")
                .short('s')
                .long("payload-string")
                .help("Payload as ASCII string (not hex)")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    // let payload_mode_s = match matches.value_of("payload-mode") {
    let payload_mode_s = match matches.get_one::<String>("payload-mode") {
        Some(s) => s,
        None => {
            eprintln!("No payload mode provided.");
            exit(-1);
        }
    };
    println!("payload_mode_s: {}", payload_mode_s);
    let payload_mode = payload_mode_s
        .parse()
        .expect("Invalid input for PayloadMode");

    // let all_chunk_json_path_s = match matches.value_of("all-chunk-json-path") {
    let all_chunk_json_path_s = match matches.get_one::<String>("all-chunk-json-path") {
        Some(s) => s,
        None => {
            eprintln!("No JSON path provided");
            exit(-1);
        }
    };
    println!("all_chunk_json_path_s: {}", all_chunk_json_path_s);
    let all_chunk_json_path = Path::new(all_chunk_json_path_s);

    // let payload_json_path_s = match matches.value_of("payload-json-path") {
    let payload_json_path_s = match matches.get_one::<String>("payload-json-path") {
        Some(s) => s,
        None => {
            eprintln!("No payload JSON path provided");
            exit(-1);
        }
    };
    println!("payload_json_path_s: {}", payload_json_path_s);
    let payload_json_path = Path::new(payload_json_path_s);

    // let complicated_policy_json_path_s = match matches.value_of("complicated-policy-json-path") {
    let complicated_policy_json_path_s = match matches.get_one::<String>("complicated-policy-json-path") {
        Some(s) => s,
        None => {
            eprintln!("No complicated policy JSON path provided");
            exit(-1);
        }
    };
    println!("complicated_policy_json_path_s: {}", complicated_policy_json_path_s);
    let complicated_policy_json_path = Path::new(complicated_policy_json_path_s);

    let payload_string = matches.get_flag("payload-string");
    println!("payload_string: {}", payload_string);

    println!("Reading pair_triplet_byte_time_sequence");
    // Get pair_triplet_byte_time_sequence from json
    let file = File::open(all_chunk_json_path)?;
    let reader = BufReader::new(file);
    let pair_triplet_byte_time_sequence: PairTripletByteTimeSequence =
        serde_json::from_reader(reader)?;

    println!("Reading reply_payload");
    // Get reply from json
    let file = File::open(payload_json_path)?;
    let reader = BufReader::new(file);
    let reply_payload_c = if payload_string {
        let reply_payload_string_c: ReplyPayloadStringC = serde_json::from_reader(reader)?;
        println!("reply_payload_string_c: {:?}", reply_payload_string_c);
        let reply_payload_c = ReplyPayloadC::of_replay_payload_string(reply_payload_string_c);
        println!("reply_payload_c: {:?}", reply_payload_c);
        reply_payload_c
    } else {
        let reply_payload_hex_c: ReplyPayloadHexC = serde_json::from_reader(reader)?;
        println!("reply_payload_hex_c: {:?}", reply_payload_hex_c);
        let reply_payload_c = ReplyPayloadC::of_replay_payload_hex(reply_payload_hex_c);
        println!("reply_payload_c: {:?}", reply_payload_c);
        reply_payload_c
    };

    println!("Building test_overlap");
    // Build overlap zone indexes
    let all_position_data =
        AllPositionData::of_byte_time_sequence(&payload_mode, &pair_triplet_byte_time_sequence);

    println!("Building overlap_policy");
    // Compare overlap zones between chunks and replies
    let all_position_policy = rst_lib::tcp_full_policy::all_policy::AllPolicy::of_data(
        &payload_mode,
        &pair_triplet_byte_time_sequence,
        &all_position_data,
        &reply_payload_c,
        false,
    );

    println!("Exporting overlap_policy");
    let json_string: String = serde_json::to_string_pretty(&all_position_policy).unwrap();
    fs::write(complicated_policy_json_path, json_string).expect("Unable to write file");

    Ok(())
}
