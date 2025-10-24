// #[macro_use]
extern crate clap;
extern crate env_logger;
extern crate log;

extern crate itertools;
extern crate serde;

use std::net::{Ipv4Addr,Ipv6Addr};
use std::net::IpAddr::{V4,V6};

use std::process::exit;

// use std::error::Error;
use std::fs;
// use std::fs::File;
// use std::io::BufReader;
use std::path::Path;
// use std::path::PathBuf;

use clap::{Arg, Command};

//use rst_lib::misc::internet_checksum_pattern_generator;
use rst_lib::position::pattern::{PatternD,ChunkBasedPatternC};
// use rst_lib::byte_data::pair_triplet_byte_sequence::PairTripletByteSequence;
// use rst_lib::relation::consistent_relation_triplet_pair::ConsistentRelationTripletPair;

fn main() {
    env_logger::init();

    let matches = Command::new("generate-byte-sequence")
        .version("0.1")
        .author("Johan Mazel <johan.mazel@ssi.gouv.fr> and Lucas Aubard")
        .about("Generate byte sequence from consistent relation pairs and triplets.")
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
        .get_matches();

    // TODO: change type from String to PathBuf
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

    let v = PatternD::of_simple_chunk_pattern_v();
    let json_string: String = serde_json::to_string_pretty(&v).unwrap();
    fs::write(simple_chunk_pattern_json_path, json_string).expect("Unable to write file");

    let v = PatternD::of_internet_checksum_chunk_pattern_v();
    let json_string: String = serde_json::to_string_pretty(&v).unwrap();
    fs::write(internet_checksum_chunk_pattern_json_path, json_string)
        .expect("Unable to write file");

    // XXX pass build_invariant_checksum_chunk_pattern_c() chunk_number and pattern_nb options as this script parameter ?
    let v = ChunkBasedPatternC::of_invariant_checksum(
        //5,
        //8,
        25,
        25,
        V4(Ipv4Addr::UNSPECIFIED)
    );
    let json_string: String = serde_json::to_string_pretty(&v).unwrap();
    fs::write(internet_invariant_ipv4_checksum_chunk_pattern_json_path, json_string)
        .expect("Unable to write file");

    // XXX pass build_invariant_checksum_chunk_pattern_c() chunk_number and pattern_nb options as this script parameter ?
    let v = ChunkBasedPatternC::of_invariant_checksum(
        //5,
        //8,
        25,
        25,
        V6(Ipv6Addr::UNSPECIFIED)
    );
    let json_string: String = serde_json::to_string_pretty(&v).unwrap();
    fs::write(internet_invariant_ipv6_checksum_chunk_pattern_json_path, json_string)
        .expect("Unable to write file");
}
