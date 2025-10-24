// #[macro_use]
extern crate clap;
extern crate env_logger;
extern crate log;

extern crate itertools;

use std::process::exit;

use itertools::Itertools;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use clap::{Arg, Command};

use rst_lib::misc::sparq_wrapper;
use rst_lib::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use rst_lib::relation::consistent_relation_triplet_pair::ConsistentRelationTripletPair;
use rst_lib::relation::relation_container::RelationContainer;
use rst_lib::relation::relation_triplet::RelationTripletC;

enum Mode {
    Legacy,
    NoInverse,
    All,
}

impl Mode {
    pub fn of_string(s: &str) -> Mode {
        match s {
            "l" => Mode::Legacy,
            "n" => Mode::NoInverse,
            "a" => Mode::All,
            _ => panic!("Invalid mode: {}", s),
        }
    }
}

fn process_overlapping_before_in_triplet(output_json_path: &Path, sparq_path: &Path) {
    let relation_v = AllenIntervalAlgebraRelation::with_overlap();
    println!("relation_v ({}): {:?}", relation_v.len(), relation_v);

    // Generate cartesian product all possible relation pairs
    let cp_data_v = vec![relation_v.clone(), relation_v.clone()];
    let relation_pair_v = cp_data_v
        .iter()
        .multi_cartesian_product()
        .collect::<Vec<Vec<&AllenIntervalAlgebraRelation>>>();
    println!(
        "relation_pair_v ({}): {:?}",
        relation_pair_v.len(),
        relation_pair_v
    );

    let relation_triplet_c = RelationTripletC::of_relation_pair(relation_pair_v);
    println!(
        "relation_triplet_c ({}): {:?}",
        relation_triplet_c.len(),
        relation_triplet_c
    );

    // Use SparQ to check that created generation triplet are possible/valid/consistent.
    let consistent_relation_triplet_c_tmp: RelationTripletC = relation_triplet_c
        .iter()
        .filter(|(_index, relation_triplet_d)| {
            let constraint_s = relation_triplet_d.build_sparq_constraint_string(0);
            sparq_wrapper::get_sparq_consistency(sparq_path, constraint_s)
        })
        .map(|(index, v)| (*index, (*v).clone()))
        .collect();
    // Rebuild index.
    let consistent_relation_triplet_c = consistent_relation_triplet_c_tmp.reindex();
    println!(
        "consistent_relation_triplet_c ({}): {:?}",
        consistent_relation_triplet_c.len(),
        consistent_relation_triplet_c
    );

    // Build consistent relation container with triplets (relation from previous processing) and
    // pairs (relation with overlap).
    let consistent_relation_triplet_pair =
        ConsistentRelationTripletPair::new(relation_v, consistent_relation_triplet_c);
    // Note:
    // The behaviour of a reassembly mechanism maybe be context sensitive.
    // When one consider a single relation, the behaviour may be different between this relation in
    // a isolated position and inside a triplet.
    // We thus do not only analyze relation not in triplets as valid isolated one because relation
    // in triplets but considered in isolation may yield distinct behaviour than inside a triplet.

    // Export
    let json_string: String =
        serde_json::to_string_pretty(&consistent_relation_triplet_pair).unwrap();
    fs::write(output_json_path, json_string).expect("Unable to write file");
}

fn process_no_inverse_relations(output_json_path: &Path, sparq_path: &Path) {
    let relation_v = AllenIntervalAlgebraRelation::no_inverse();
    println!("relation_v ({}): {:?}", relation_v.len(), relation_v);

    // Generate cartesian product all possible relation triplets
    let cp_data_v = vec![relation_v.clone(), relation_v.clone(), relation_v.clone()];
    let relation_triplet_v = cp_data_v
        .iter()
        .multi_cartesian_product()
        .collect::<Vec<Vec<&AllenIntervalAlgebraRelation>>>();
    println!(
        "relation_triplet_v ({}): {:?}",
        relation_triplet_v.len(),
        relation_triplet_v
    );

    let relation_triplet_c = RelationTripletC::of_relation_triplet(relation_triplet_v);
    println!(
        "relation_triplet_c ({}): {:?}",
        relation_triplet_c.len(),
        relation_triplet_c
    );

    // Use SparQ to check that created generation triplet are possible/valid/consistent.
    let consistent_relation_triplet_c_tmp: RelationTripletC = relation_triplet_c
        .iter()
        .filter(|(_index, relation_triplet_d)| {
            let constraint_s = relation_triplet_d.build_sparq_constraint_string(0);
            sparq_wrapper::get_sparq_consistency(sparq_path, constraint_s)
        })
        .map(|(index, v)| (*index, (*v).clone()))
        .collect();
    // Rebuild index.
    let consistent_relation_triplet_c = consistent_relation_triplet_c_tmp.reindex();
    println!(
        "consistent_relation_triplet_c ({}): {:?}",
        consistent_relation_triplet_c.len(),
        consistent_relation_triplet_c
    );

    // Build consistent relation container with triplets (relation from previous processing) and
    // pairs (relation with overlap).
    let consistent_relation_triplet_pair =
        ConsistentRelationTripletPair::new(relation_v, consistent_relation_triplet_c);
    // Note:
    // The behaviour of a reassembly mechanism maybe be context sensitive.
    // When one consider a single relation, the behaviour may be different between this relation in
    // a isolated position and inside a triplet.
    // We thus do not only analyze relation not in triplets as valid isolated one because relation
    // in triplets but considered in isolation may yield distinct behaviour than inside a triplet.

    // Export
    let json_string: String =
        serde_json::to_string_pretty(&consistent_relation_triplet_pair).unwrap();
    // "consistent_relation.json"
    fs::write(output_json_path, json_string).expect("Unable to write file");
}

fn process_all_relations(output_json_path: &Path, sparq_path: &Path) {
    let relation_v = AllenIntervalAlgebraRelation::all();
    println!("relation_v ({}): {:?}", relation_v.len(), relation_v);

    // Generate cartesian product all possible relation triplets
    let cp_data_v = vec![relation_v.clone(), relation_v.clone(), relation_v.clone()];
    let relation_triplet_v = cp_data_v
        .iter()
        .multi_cartesian_product()
        .collect::<Vec<Vec<&AllenIntervalAlgebraRelation>>>();
    println!(
        "relation_triplet_v ({}): {:?}",
        relation_triplet_v.len(),
        relation_triplet_v
    );

    let relation_triplet_c = RelationTripletC::of_relation_triplet(relation_triplet_v);
    println!(
        "relation_triplet_c ({}): {:?}",
        relation_triplet_c.len(),
        relation_triplet_c
    );

    // Use SparQ to check that created generation triplet are possible/valid/consistent.
    let consistent_relation_triplet_c_tmp: RelationTripletC = relation_triplet_c
        .iter()
        .filter(|(_index, relation_triplet_d)| {
            let constraint_s = relation_triplet_d.build_sparq_constraint_string(0);
            sparq_wrapper::get_sparq_consistency(sparq_path, constraint_s)
        })
        .map(|(index, v)| (*index, (*v).clone()))
        .collect();
    // Rebuild index.
    let consistent_relation_triplet_c = consistent_relation_triplet_c_tmp.reindex();
    println!(
        "consistent_relation_triplet_c ({}): {:?}",
        consistent_relation_triplet_c.len(),
        consistent_relation_triplet_c
    );

    // Build consistent relation container with triplets (relation from previous processing) and
    // pairs (relation with overlap).
    let consistent_relation_triplet_pair =
        ConsistentRelationTripletPair::new(relation_v, consistent_relation_triplet_c);
    // Note:
    // The behaviour of a reassembly mechanism maybe be context sensitive.
    // When one consider a single relation, the behaviour may be different between this relation in
    // a isolated position and inside a triplet.
    // We thus do not only analyze relation not in triplets as valid isolated one because relation
    // in triplets but considered in isolation may yield distinct behaviour than inside a triplet.

    // Export
    let json_string: String =
        serde_json::to_string_pretty(&consistent_relation_triplet_pair).unwrap();
    // "consistent_relation.json"
    fs::write(output_json_path, json_string).expect("Unable to write file");
}

fn main() {
    env_logger::init();

    let matches = Command::new("generate-trace")
        .version("0.1")
        .author("Johan Mazel <johan.mazel@ssi.gouv.fr> and Lucas Aubard")
        .about("Generate consistent Allen algebra relation pairs and triplets.")
        .arg(
            Arg::new("json-path")
                .short('o')
                .long("json-path")
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
            Arg::new("mode")
                .short('m')
                .long("mode")
                .help("Relation mode: l (with overlap) | n (no inverse) | a (all).")
                .default_value("a"), // .required(true),
        )
        .get_matches();

    // let output_json_path_s = match matches.value_of("json-path") {
    let output_json_path_s = match matches.get_one::<String>("json-path") {
        Some(s) => s,
        None => {
            eprintln!("No output JSON path provided");
            exit(-1);
        }
    };
    println!("output_json_path_s: {}", output_json_path_s);
    let output_json_path = Path::new(output_json_path_s);

    let mut output_json_directory_path = PathBuf::from(output_json_path);
    output_json_directory_path.pop();
    if !output_json_directory_path.exists() {
        println!(
            "Output JSON directory path ({:?}) does not exists!",
            output_json_directory_path
        );
        std::process::exit(-1)
    } else if !output_json_directory_path.is_dir() {
        println!(
            "Output JSON directory path ({:?}) is not a dir!",
            output_json_directory_path
        );
        std::process::exit(-1)
    }

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

    // let mode_s = match matches.value_of("mode") {
    let mode_s = match matches.get_one::<String>("mode") {
        Some(s) => s,
        None => {
            eprintln!("No value for mode by we setup a default!!!!");
            exit(-1);
        }
    };
    println!("mode_s: {}", mode_s);
    let mode = Mode::of_string(mode_s);

    let mut output_json_directory_path = output_json_path.to_path_buf();
    output_json_directory_path.pop();
    if !output_json_directory_path.exists() {
        panic!(
            "Provided output json path directory ({:?}) does not exists!",
            output_json_directory_path
        )
    }

    if !sparq_path.exists() {
        panic!("Provided SparQ path ({:?}) does not exists!", sparq_path)
    }

    match mode {
        Mode::Legacy => process_overlapping_before_in_triplet(output_json_path, sparq_path),
        Mode::NoInverse => process_no_inverse_relations(output_json_path, sparq_path),
        Mode::All => process_all_relations(output_json_path, sparq_path),
    };
}
