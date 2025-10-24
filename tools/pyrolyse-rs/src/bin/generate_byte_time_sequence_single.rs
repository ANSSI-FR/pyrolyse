// #[macro_use]
extern crate clap;
extern crate env_logger;
extern crate itertools;
extern crate log;
extern crate pcap_parser;
extern crate pnet;
// extern crate rand;

use std::error::Error;
use std::fs;
use std::fs::File;
// use std::io;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;
use log::debug;

use clap::{value_parser, Arg, ArgAction, Command};
use serde::Serialize;

use rst_lib::byte_data::byte_sequence::ByteSequenceD;

use rst_lib::byte_time_data::byte_time_sequence::ByteTimeSequenceD;
use rst_lib::byte_time_data::export_mode::ExportMode;

use rst_lib::misc::test_index::TestIndex;
//use rst_lib::misc::invariant_checksum_chunk_pattern::InvariantChecksumChunkPatternC;
use rst_lib::position::pattern::{PatternD,ChunkBasedPatternC};
use rst_lib::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use rst_lib::relation::relation_container::RelationContainer;
use rst_lib::relation::relation_custom::RelationCustomD;
use rst_lib::relation::relation_triplet::RelationTripletD;
use rst_lib::relation::relation_type::RelationType;

fn build_export<Rc: Clone + Serialize + RelationContainer>(
    test_index: TestIndex,
    export_mode: ExportMode,
    simple_chunk_pattern_v: PatternD,
    internet_checksum_chunk_pattern_v: PatternD,
    ipv4_invariant_checksum_chunk_pattern: ChunkBasedPatternC,
    ipv6_invariant_checksum_chunk_pattern: ChunkBasedPatternC,
    byte_sequence_d: ByteSequenceD<Rc>,
    temporal_position_v: Vec<u16>,
    output_json_path: &Path,
) {
    println!("build_export: building byte_time_sequence");
    let byte_time_sequence = ByteTimeSequenceD::<Rc>::of_data(
        test_index,
        &export_mode,
        &simple_chunk_pattern_v,
        &internet_checksum_chunk_pattern_v,
        &ipv4_invariant_checksum_chunk_pattern,
        &ipv6_invariant_checksum_chunk_pattern,
        &byte_sequence_d,
        &temporal_position_v,
    );

    let json_string: String = serde_json::to_string_pretty(&byte_time_sequence).unwrap();
    println!("build_export: write to file");
    fs::write(output_json_path, json_string).expect("Unable to write file");
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let matches = Command::new("generate-byte-time-sequence")
        .version("0.1")
        .author("Johan Mazel <johan.mazel@ssi.gouv.fr> and Lucas Aubard")
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
                .long("vc1b")
                .help("Simple chunk pattern JSON path")
                .required(true),
        )
        .arg(
            Arg::new("internet-checksum-chunk-pattern-json-path")
                .long("icfl8b")
                .help("Internet checksum chunk pattern JSON path")
                .required(true),
        )
        .arg(
            Arg::new("internet-invariant-ipv4-checksum-chunk-pattern-json-path")
                .long("icvl8i4")
                .help("Internet invariant ipv4 checksum chunk pattern JSON path")
                .required(true),
        )
        .arg(
            Arg::new("internet-invariant-ipv6-checksum-chunk-pattern-json-path")
                .long("icvl8i6")
                .help("Internet invariant ipv6 checksum chunk pattern JSON path")
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
                .long("ts")
                .help("Enable temporal shuffling")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("relation-type")
                .long("rt")
                .help("Relation type")
                .required(true),
        )
        .arg(
            Arg::new("test-index")
                .long("ti")
                .help("Test index")
                .value_parser(value_parser!(u16))
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

    let internet_invariant_ipv4_checksum_chunk_pattern_json_path_s =
        match matches.get_one::<String>("internet-invariant-ipv4-checksum-chunk-pattern-json-path") {
            Some(s) => s,
            None => {
                eprintln!("No internet invariant ipv4 checksum chunk pattern JSON path provided");
                exit(-1);
            }
        };
    println!(
        "internet_invariant_ipv4_checksum_chunk_pattern_json_path_s: {}",
        internet_invariant_ipv4_checksum_chunk_pattern_json_path_s
    );
    let internet_invariant_ipv4_checksum_chunk_pattern_json_path =
        Path::new(internet_invariant_ipv4_checksum_chunk_pattern_json_path_s);

    
    let internet_invariant_ipv6_checksum_chunk_pattern_json_path_s =
        match matches.get_one::<String>("internet-invariant-ipv6-checksum-chunk-pattern-json-path") {
            Some(s) => s,
            None => {
                eprintln!("No internet invariant ipv6 checksum chunk pattern JSON path provided");
                exit(-1);
            }
        };
    println!(
        "internet_invariant_ipv6_checksum_chunk_pattern_json_path_s: {}",
        internet_invariant_ipv6_checksum_chunk_pattern_json_path_s
    );
    let internet_invariant_ipv6_checksum_chunk_pattern_json_path =
        Path::new(internet_invariant_ipv6_checksum_chunk_pattern_json_path_s);


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

    let relation_type_s = match matches.get_one::<String>("relation-type") {
        Some(s) => s,
        None => {
            eprintln!("No relation type provided");
            exit(-1);
        }
    };
    println!("relation_type_s: {}", relation_type_s);
    let relation_type = RelationType::of_string(relation_type_s).unwrap();

    let test_index = match matches.get_one::<u16>("test-index") {
        Some(s) => s,
        None => {
            eprintln!("No test index provided");
            exit(-1);
        }
    };
    println!("test_index: {}", test_index);

    let export_mode = ExportMode::Isolated;

    let file = File::open(simple_chunk_pattern_json_path)?;
    let reader = BufReader::new(file);
    let simple_chunk_pattern_v: PatternD = serde_json::from_reader(reader)?;
    debug!("simple_chunk_pattern_v: ok");

    let file = File::open(internet_checksum_chunk_pattern_json_path)?;
    let reader = BufReader::new(file);
    let internet_checksum_chunk_pattern_v: PatternD = serde_json::from_reader(reader)?;
    debug!("internet_checksum_chunk_pattern_v: ok");

    let file = File::open(internet_invariant_ipv4_checksum_chunk_pattern_json_path)?;
    let reader = BufReader::new(file);
    let ipv4_invariant_checksum_chunk_pattern: ChunkBasedPatternC = serde_json::from_reader(reader)?;
    debug!("ipv4_invariant_checksum_chunk_pattern: ok");

    let file = File::open(internet_invariant_ipv6_checksum_chunk_pattern_json_path)?;
    let reader = BufReader::new(file);
    let ipv6_invariant_checksum_chunk_pattern: ChunkBasedPatternC = serde_json::from_reader(reader)?;
    debug!("ipv6_invariant_checksum_chunk_pattern: ok");

    // let simple_chunk_pattern_v =
    //     internet_checksum_pattern_generator::build_simple_chunk_pattern_v();
    // let internet_checksum_chunk_pattern_v =
    //     internet_checksum_pattern_generator::build_internet_checksum_chunk_pattern_v();

    println!("opening input_json_path: {:?}", input_json_path);
    let file = File::open(input_json_path)?;
    let reader = BufReader::new(file);

    match relation_type {
        RelationType::Pair => {
            println!("reading ByteSequenceD");
            let byte_sequence_d: ByteSequenceD<AllenIntervalAlgebraRelation> =
                serde_json::from_reader(reader)?;

            let temporal_position_v = vec![0, 1];

            build_export(
                TestIndex(*test_index),
                export_mode,
                simple_chunk_pattern_v,
                internet_checksum_chunk_pattern_v,
                ipv4_invariant_checksum_chunk_pattern,
                ipv6_invariant_checksum_chunk_pattern,
                byte_sequence_d,
                temporal_position_v,
                output_json_path,
            )
        }
        RelationType::Triplet => {
            println!("reading ByteSequenceD");
            let byte_sequence_d: ByteSequenceD<RelationTripletD> = serde_json::from_reader(reader)?;

            let temporal_position_v = vec![0, 1, 2];

            build_export(
                TestIndex(*test_index),
                export_mode,
                simple_chunk_pattern_v,
                internet_checksum_chunk_pattern_v,
                ipv4_invariant_checksum_chunk_pattern,
                ipv6_invariant_checksum_chunk_pattern,
                byte_sequence_d,
                temporal_position_v,
                output_json_path,
            )
        }
        RelationType::Custom => {
            println!("reading ByteSequenceD");
            let byte_sequence_d: ByteSequenceD<RelationCustomD> = serde_json::from_reader(reader)?;

            let chunk_nb = byte_sequence_d.get_base_interval_c().len();
            let temporal_position_v = (0..chunk_nb).map(|u| u as u16).collect::<Vec<_>>();

            build_export(
                TestIndex(*test_index),
                export_mode,
                simple_chunk_pattern_v,
                internet_checksum_chunk_pattern_v,
                ipv4_invariant_checksum_chunk_pattern,
                ipv6_invariant_checksum_chunk_pattern,
                byte_sequence_d,
                temporal_position_v,
                output_json_path,
            )
        }
    }

    Ok(())
}
