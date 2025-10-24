// #[macro_use]
extern crate clap;
extern crate env_logger;
extern crate itertools;
extern crate log;
extern crate pcap_parser;
extern crate pnet;

use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

use clap::{Command, Arg};

use rst_lib::byte_time_data::pair_triplet_byte_time_sequence::PairTripletByteTimeSequence;
use rst_lib::icmp_pcap::all_icmp_trace_deprecated::AllIcmpTrace;
use rst_lib::icmp_pcap::policy_evaluation::PolicyEvaluation;
// use rst_lib::misc::icmp_echo_request_generation;
// use rst_lib::misc::icmpv6_echo_request_generation;
use rst_lib::misc::test_target_deprecated::TestTarget;

fn main() -> io::Result<()> {
    env_logger::init();

    let matches = Command::new("generate-trace-ip-fragmentation-icmp")
        .version("0.1")
        .author("Johan Mazel <johan.mazel@ssi.gouv.fr>")
        .about("Generate ICMP PCAP traces to test IP fragmentation reassambly policies.")
        .arg(
            Arg::new("input-json-path")
                .short('i')
                .long("input-json-path")
                .help("Input JSON path")
                .required(true),
        )
        .arg(
            Arg::new("output-pcap-directory-path")
                .short('o')
                .long("output-pcap-directory-path")
                .help("Output PCAP directory path")
                .required(true),
        )
        .arg(
            Arg::new("mac-src")
                .long("mac-src")
                .help("Source MAC address")
                .required(true),
        )
        .arg(
            Arg::new("mac-dst")
                .long("mac-dst")
                .help("Destination MAC address")
                .required(true),
        )
        .arg(
            Arg::new("ip-version")
                .long("ip-version")
                .help("Version of ip (4 or 6)")
                .required(true),
        )
        .arg(
            Arg::new("ipv4-src")
                .long("ipv4-src")
                .required_if_eq("ip-version", "4")
                .help("Source IPv4 address")
        )
        .arg(
            Arg::new("ipv4-dst")
                .long("ipv4-dst")
                .required_if_eq("ip-version", "4")
                .help("Destination IPv4 address")
        )
        .arg(
            Arg::new("ipv6-src")
                .long("ipv6-src")
                .required_if_eq("ip-version", "6")
                .help("Source IPv6 address")
        )
        .arg(
            Arg::new("ipv6-dst")
                .long("ipv6-dst")
                .required_if_eq("ip-version", "6")
                .help("Destination IPv6 address")
        )
        .arg(
            Arg::new("policy-evaluation")
                .short('p')
                .long("policy-evaluation")
                .help("Policy evaluation: p (Progressive) | os (Once with byte-wise starting chunk sent last) | os (Once with byte-wise starting and ending chunks sent last in this order) | os (Oncebyte-wise ending and starting chunks sent last in this order)")
                
                .default_value("p")
                .required(false),
        )
        .get_matches();

    // let policy_evaluation_s = match matches.value_of("policy-evaluation") {
        let policy_evaluation_s = match matches.get_one::<String>("policy-evaluation") {
        Some(s) => s,
        None => {
            eprintln!("No policy evaluation not provided");
            exit(-1);
        }
    };
    println!("policy_evaluation_s: {}", policy_evaluation_s);
    let policy_evaluation = PolicyEvaluation::of_string(policy_evaluation_s);
    println!("policy_evaluation: {:?}", policy_evaluation);

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

    // let output_pcap_directory_path_s = match matches.value_of("output-pcap-directory-path") {
        let output_pcap_directory_path_s = match matches.get_one::<String>("output-pcap-directory-path") {
        Some(s) => s,
        None => {
            eprintln!("No output PCAP directory path provided");
            exit(-1);
        }
    };
    println!(
        "output_pcap_directory_path_s: {}",
        output_pcap_directory_path_s
    );
    let output_pcap_directory_path = Path::new(output_pcap_directory_path_s);

    // let macaddr_src_s = match matches.value_of("mac-src") {
        let macaddr_src_s = match matches.get_one::<String>("mac-src") {
        Some(s) => s,
        None => {
            eprintln!("No source MAC address path provided");
            exit(-1);
        }
    };
    println!("mac_addr_src_s: {}", macaddr_src_s);
    let macaddr_src = macaddr_src_s
        .parse()
        .expect("mac-src is not a valid MacAddr");

    // let macaddr_dst_s = match matches.value_of("mac-dst") {
    let macaddr_dst_s = match matches.get_one::<String>("mac-dst") {
        Some(s) => s,
        None => {
            eprintln!("No destination MAC address path provided");
            exit(-1);
        }
    };
    println!("macaddr_dst_s: {}", macaddr_dst_s);
    let macaddr_dst = macaddr_dst_s
        .parse()
        .expect("mac-dst is not a valid MacAddr");

    if !input_json_path.exists() {
            println!("Input JSON path ({:?}) does not exists!", input_json_path);
            std::process::exit(-1)
    }

    if !input_json_path.exists() {
        println!("Input JSON path ({:?}) does not exists!", input_json_path);
        std::process::exit(-1)
    }

    if !output_pcap_directory_path.exists() {
        println!(
            "Output PCAP directory path ({:?}) does not exists!",
            output_pcap_directory_path
        );
        std::process::exit(-1)
    }

    let file = File::open(input_json_path)?;
    let reader = BufReader::new(file);
    let pair_triplet_byte_time_sequence: PairTripletByteTimeSequence =
        serde_json::from_reader(reader)?;

    let ip_version = match matches.get_one::<String>("ip-version") {
        Some(s) => s,
        None => {
            eprintln!("No IP version provided");
            exit(-1);
        }
    };

    println!("building all_icmp_trace");
    if ip_version == "4" {
        let ipv4addr_src_s = match matches.get_one::<String>("ipv4-src") {
            Some(s) => s,
            None => {
                eprintln!("No source IP address path provided");
                exit(-1);
            }
        };
        println!("ipv4addr_src_s: {}", ipv4addr_src_s);
        let ipv4addr_src = ipv4addr_src_s
            .parse()
            .expect("ipv4-src is not a valid ipv4Addr");
    
        let ipv4addr_dst_s = match matches.get_one::<String>("ipv4-dst") {
            Some(s) => s,
            None => {
                eprintln!("No destination IP address path provided");
                exit(-1);
            }
        };
        println!("ipaddr_dst_s: {}", ipv4addr_dst_s);
        let ipv4addr_dst = ipv4addr_dst_s
            .parse()
            .expect("ipv4-dst is not a valid ipAddr");
        
        let test_target = TestTarget::new(macaddr_src, macaddr_dst, ipv4addr_src, ipv4addr_dst);
        let all_icmp_trace = AllIcmpTrace::of_data(
            &test_target,
            &policy_evaluation,
            &pair_triplet_byte_time_sequence,
        );
        println!("generating PCAPs");
        all_icmp_trace.export(output_pcap_directory_path);
    
    } else if ip_version == "6" {

        let ipv6addr_src_s = match matches.get_one::<String>("ipv6-src") {
            Some(s) => s,
            None => {
                eprintln!("No source IP address path provided");
                exit(-1);
            }
        };
        println!("ipv6addr_src_s: {}", ipv6addr_src_s);
        let ipv6addr_src = ipv6addr_src_s
            .parse()
            .expect("ipv6-src is not a valid ipv6Addr");
    
        let ipv6addr_dst_s = match matches.get_one::<String>("ipv6-dst") {
            Some(s) => s,
            None => {
                eprintln!("No destination IP address path provided");
                exit(-1);
            }
        };
        println!("ipaddr_dst_s: {}", ipv6addr_dst_s);
        let ipv6addr_dst = ipv6addr_dst_s
            .parse()
            .expect("ipv6-dst is not a valid ipv6Addr");

        let test_target = TestTarget::new(macaddr_src, macaddr_dst, ipv6addr_src, ipv6addr_dst);
        let all_icmp_trace = AllIcmpTrace::of_data(
            &test_target,
            &policy_evaluation,
            &pair_triplet_byte_time_sequence,
        );
        println!("generating PCAPs");
        all_icmp_trace.export(output_pcap_directory_path);

    } else {
        eprintln!("Bad ip version provided");
        std::process::exit(-1)
    }

    println!("exporting all_chunk");
    let json_string: String =
        serde_json::to_string_pretty(&pair_triplet_byte_time_sequence).unwrap();
    fs::write("byte_time_sequence.json", json_string).expect("Unable to write file");

    Ok(())
}