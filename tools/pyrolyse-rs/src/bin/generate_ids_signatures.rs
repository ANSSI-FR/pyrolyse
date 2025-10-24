// #[macro_use]
extern crate clap;
extern crate env_logger;
extern crate log;

extern crate itertools;
extern crate serde;

use std::process::exit;

use std::path::Path;
use std::io;
use std::fs::File;
use std::io::BufReader;

use clap::{Arg, Command};

use rst_lib::position::pattern::{PatternD,ChunkBasedPatternC};
use rst_lib::misc::ids_sig_generator;

fn main() -> io::Result<()> {
    env_logger::init();

    let matches = Command::new("generate-ids-signatures")
        .version("0.1")
        .author("Johan Mazel <johan.mazel@ssi.gouv.fr> and Lucas Aubard")
        .about("Generate IDS signature files across payload modes.")
        .arg(
            Arg::new("vc1b-pattern-json-path")
                .long("vc1b")
                .help("Simple chunk pattern JSON path")
                .required(true),
        )
        .arg(
            Arg::new("icfl8b-pattern-json-path")
                .long("icfl8b")
                .help("Internet checksum chunk pattern JSON path")
                .required(true),
        )
        .arg(
            Arg::new("icvl8i4-pattern-json-path")
                .long("icvl8i4")
                .help("Internet invariant ipv4 checksum chunk pattern JSON path")
                .required(true),
        )
        .arg(
            Arg::new("icvl8i6-pattern-json-path")
                .long("icvl8i6")
                .help("Internet invariant ipv6 checksum chunk pattern JSON path")
                .required(true),
        )
        .arg(
            Arg::new("ids-code-path")
                .long("ids-code-path")
                .help("Path to IDS base directory")
                .required(true),
        )
        .get_matches();

    // TODO: change type from String to PathBuf
    let vc1b_pattern_json_path_s =
        match matches.get_one::<String>("vc1b-pattern-json-path") {
            Some(s) => s,
            None => {
                eprintln!("No simple chunk pattern JSON path provided");
                exit(-1);
            }
        };
    println!(
        "vc1b_pattern_json_path_s: {}",
        vc1b_pattern_json_path_s
    );
    let vc1b_pattern_json_path = Path::new(vc1b_pattern_json_path_s);
    let file = File::open(vc1b_pattern_json_path)?;
    let reader = BufReader::new(file);
    let vc1b_pattern_d: PatternD = serde_json::from_reader(reader)?;

    let icfl8b_pattern_json_path_s =
        match matches.get_one::<String>("icfl8b-pattern-json-path") {
            Some(s) => s,
            None => {
                eprintln!("No internet checksum chunk pattern JSON path provided");
                exit(-1);
            }
        };
    println!(
        "icfl8b_pattern_json_path_s: {}",
        icfl8b_pattern_json_path_s
    );
    let icfl8b_pattern_json_path =
        Path::new(icfl8b_pattern_json_path_s);
    let file = File::open(icfl8b_pattern_json_path)?;
    let reader = BufReader::new(file);
    let icfl8b_pattern_d: PatternD = serde_json::from_reader(reader)?;
    
    let icvl8i4_pattern_json_path_s =
        match matches.get_one::<String>("icvl8i4-pattern-json-path") {
            Some(s) => s,
            None => {
                eprintln!("No internet invariant ipv4 checksum chunk pattern JSON path provided");
                exit(-1);
            }
        };
    println!(
        "icvl8i4_pattern_json_path_s: {}",
        icvl8i4_pattern_json_path_s
    );
    let icvl8i4_pattern_json_path =
        Path::new(icvl8i4_pattern_json_path_s);
    let file = File::open(icvl8i4_pattern_json_path)?;
    let reader = BufReader::new(file);
    let icvl8i4_pattern_d: ChunkBasedPatternC = serde_json::from_reader(reader)?;
    
    let icvl8i6_pattern_json_path_s =
        match matches.get_one::<String>("icvl8i6-pattern-json-path") {
            Some(s) => s,
            None => {
                eprintln!("No internet invariant ipv6 checksum chunk pattern JSON path provided");
                exit(-1);
            }
        };
    println!(
        "icvl8i6_pattern_json_path_s: {}",
        icvl8i6_pattern_json_path_s
    );
    let icvl8i6_pattern_json_path =
        Path::new(icvl8i6_pattern_json_path_s);
    let file = File::open(icvl8i6_pattern_json_path)?;
    let reader = BufReader::new(file);
    let icvl8i6_pattern_d: ChunkBasedPatternC = serde_json::from_reader(reader)?;

    let ids_code_path_s =
        match matches.get_one::<String>("ids-code-path") {
            Some(s) => s,
            None => {
                eprintln!("No IDS base path provided");
                exit(-1);
            }
        };
    println!("ids_code_path_s: {}", ids_code_path_s);

    let _ = ids_sig_generator::build_snort_suricata_sig(
        ids_code_path_s,
        "snort",
        &vc1b_pattern_d,
        &icfl8b_pattern_d,
        &icvl8i4_pattern_d,
        &icvl8i6_pattern_d
    );

    let _ = ids_sig_generator::build_snort_suricata_sig(
        ids_code_path_s,
        "suricata",
        &vc1b_pattern_d,
        &icfl8b_pattern_d,
        &icvl8i4_pattern_d,
        &icvl8i6_pattern_d
    );

    let _ = ids_sig_generator::build_zeek_sig(
        ids_code_path_s,
        &vc1b_pattern_d,
        &icfl8b_pattern_d,
        &icvl8i4_pattern_d,
        &icvl8i6_pattern_d
    );

    Ok(())
}
