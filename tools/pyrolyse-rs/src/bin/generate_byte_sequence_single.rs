// #[macro_use]
extern crate clap;
extern crate env_logger;
extern crate log;

extern crate itertools;
extern crate serde;

use std::process::exit;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use clap::{Arg, Command};

use rst_lib::byte_data::byte_sequence::ByteSequenceD;
// use rst_lib::byte_data::pair_triplet_byte_sequence::PairTripletByteSequence;
use rst_lib::misc::interval::IntervalC;
use rst_lib::misc::sparq_wrapper;
// use rst_lib::relation::allen_interval_algebra_relation;
use rst_lib::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
// use rst_lib::relation::consistent_relation_triplet_pair::ConsistentRelationTripletPair;
use rst_lib::relation::relation_container::RelationContainer;
use rst_lib::relation::relation_custom::RelationCustomD;
use rst_lib::relation::relation_triplet::RelationTripletD;
use rst_lib::relation::relation_type::RelationType;

fn process(
    relation_type: RelationType,
    sparq_path: &Path,
    input_json_path: &Path,
    output_json_path: &Path,
) -> Result<(), Box<dyn Error>> {
    println!("process: start");

    let file = File::open(input_json_path)?;
    let reader = BufReader::new(file);

    let index_offset = 0;
    let data_offset = 0;

    // Read the JSON contents of the file as an instance of `ConsistentRelationTripletPair`.

    let test_path = Path::new("toto.json");
    let json_string: String = serde_json::to_string_pretty(&AllenIntervalAlgebraRelation::S).unwrap();
    fs::write(test_path, json_string).expect("Unable to write file");

    match relation_type {
        RelationType::Pair => {
            let allen_interval_algebra_relation: AllenIntervalAlgebraRelation =
                serde_json::from_reader(reader)?;
            println!(
                "process: allen_interval_algebra_relation: {:?}",
                allen_interval_algebra_relation
            );

            // Quantify triplet/pair using SparQ
            let constraint_s = allen_interval_algebra_relation.build_sparq_constraint_string(0);
            println!("constraint_s: {}", constraint_s);
            let sparq_output_s = sparq_wrapper::get_sparq_quantification(sparq_path, constraint_s);
            let interval_c = IntervalC::of_sparq_string(sparq_output_s);

            // Build sequence from consitent relation pairs and triplets.
            let byte_sequence = ByteSequenceD::of_data(
                &allen_interval_algebra_relation,
                &interval_c,
                index_offset,
                data_offset,
            );
            println!("byte_sequence: {:?}", byte_sequence);

            let mut output_directory_path = PathBuf::from(output_json_path);
            output_directory_path.pop();

            // let mut csv_file_path = PathBuf::new();
            // csv_file_path.push(output_directory_path);
            // csv_file_path.push("interval_all.csv");
            // pair_triplet_byte_sequence.export_csv(&csv_file_path);

            let json_string: String = serde_json::to_string_pretty(&byte_sequence).unwrap();
            fs::write(output_json_path, json_string).expect("Unable to write file");
        }
        RelationType::Triplet => {
            let relation_triplet_d: RelationTripletD = serde_json::from_reader(reader)?;
            println!("process: relation_triplet_d: {:?}", relation_triplet_d);

            // Quantify triplet/pair using SparQ
            let constraint_s = relation_triplet_d.build_sparq_constraint_string(0);
            println!("constraint_s: {}", constraint_s);
            let sparq_output_s = sparq_wrapper::get_sparq_quantification(sparq_path, constraint_s);
            let interval_c = IntervalC::of_sparq_string(sparq_output_s);

            // Build sequence from consitent relation pairs and triplets.
            let byte_sequence =
                ByteSequenceD::of_data(&relation_triplet_d, &interval_c, index_offset, data_offset);
            println!("byte_sequence: {:?}", byte_sequence);

            let mut output_directory_path = PathBuf::from(output_json_path);
            output_directory_path.pop();

            // let mut csv_file_path = PathBuf::new();
            // csv_file_path.push(output_directory_path);
            // csv_file_path.push("interval_all.csv");
            // pair_triplet_byte_sequence.export_csv(&csv_file_path);

            let json_string: String = serde_json::to_string_pretty(&byte_sequence).unwrap();
            fs::write(output_json_path, json_string).expect("Unable to write file");
        }
        RelationType::Custom => {
            let relation_custom_d: RelationCustomD = serde_json::from_reader(reader)?;
            println!("process: relation_custom_d: {:?}", relation_custom_d);

            // Quantify triplet/pair using SparQ
            let constraint_s = relation_custom_d.build_sparq_constraint_string(0);
            println!("constraint_s: {}", constraint_s);
            let sparq_output_s = sparq_wrapper::get_sparq_quantification(sparq_path, constraint_s);
            let interval_c = IntervalC::of_sparq_string(sparq_output_s);

            // Build sequence from consitent relation pairs and triplets.
            let byte_sequence =
                ByteSequenceD::of_data(&relation_custom_d, &interval_c, index_offset, data_offset);
            println!("byte_sequence: {:?}", byte_sequence);

            let mut output_directory_path = PathBuf::from(output_json_path);
            output_directory_path.pop();

            // let mut csv_file_path = PathBuf::new();
            // csv_file_path.push(output_directory_path);
            // csv_file_path.push("interval_all.csv");
            // pair_triplet_byte_sequence.export_csv(&csv_file_path);

            let json_string: String = serde_json::to_string_pretty(&byte_sequence).unwrap();
            fs::write(output_json_path, json_string).expect("Unable to write file");
        }
    }

    println!("process: end");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let matches = Command::new("generate-byte-sequence")
        .version("0.1")
        .author("Johan Mazel <johan.mazel@ssi.gouv.fr> and Lucas Aubard")
        .about("Generate byte sequence from consistent relation pairs and triplets.")
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
                .help("Output JSON path")
                .required(true),
        )
        .arg(
            Arg::new("sparq-path")
                .short('s')
                .long("sparq-path")
                .help("SparQ executable path")
                .required(true),
        )
        .arg(
            Arg::new("relation-type")
                .long("rt")
                .help("Relation type")
                .required(true),
        )
        .get_matches();

    // TODO: change type from String to PathBuf
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
            eprintln!("No output json path provided");
            exit(-1);
        }
    };
    println!("output_json_path_s: {}", output_json_path_s);
    let output_json_path = Path::new(output_json_path_s);

    // let sparq_path_s = match matches.value_of("sparq-path") {
    let sparq_path_s = match matches.get_one::<String>("sparq-path") {
        Some(s) => s,
        None => {
            eprintln!("No SparQ path provided");
            exit(-1);
        }
    };
    println!("sparq_path_s: {}", sparq_path_s);
    let sparq_path = Path::new(sparq_path_s);

    let relation_type_s = match matches.get_one::<String>("relation-type") {
        Some(s) => s,
        None => {
            eprintln!("No Relation type provided");
            exit(-1);
        }
    };
    println!("relation_type_s: {}", relation_type_s);
    let relation_type = RelationType::of_string(relation_type_s).unwrap();

    process(relation_type, sparq_path, input_json_path, output_json_path)?;

    Ok(())
}
