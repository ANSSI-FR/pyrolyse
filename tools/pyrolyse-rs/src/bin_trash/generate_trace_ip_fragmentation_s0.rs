// #[macro_use]
extern crate clap;
extern crate env_logger;
extern crate itertools;
extern crate log;
extern crate pcap_parser;
extern crate pnet;

// use std::fs;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::path::Path;
use std::process::exit;

use clap::{Arg, Command};
use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::ip::IpNextHeaderProtocols;
use serde::Serialize;

use rst_lib::byte_time_data::byte_time_sequence::ByteTimeSequenceD;
use rst_lib::icmp_pcap::icmp_trace::IcmpTraceC;
use rst_lib::icmp_pcap::icmp_trace::IcmpTraceD;
use rst_lib::icmp_pcap::policy_evaluation::PolicyEvaluation;
use rst_lib::misc::ip_addr_container_generic::IpAddrGeneric;
use rst_lib::misc::test_target::TestTarget;
use rst_lib::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use rst_lib::relation::relation_container::RelationContainer;
use rst_lib::relation::relation_custom::RelationCustomD;
use rst_lib::relation::relation_triplet::RelationTripletD;
use rst_lib::relation::relation_type::RelationType;
use rst_lib::udp_pcap::udp_trace::UdpTraceC;
use rst_lib::udp_pcap::udp_trace::UdpTraceD;

fn build_trace_export<
    I: Debug + Copy + IpAddrGeneric,
    Rc: Debug + Clone + Serialize + RelationContainer,
>(
    output_protocol: IpNextHeaderProtocol,
    policy_evaluation: PolicyEvaluation,
    internet_checksum_chunk_pattern_v: Vec<String>,
    test_index: u16,
    test_target: TestTarget<I>,
    byte_time_sequence_d: ByteTimeSequenceD<Rc>,
    output_pcap_directory_path: &Path,
) {
    match output_protocol {
        IpNextHeaderProtocols::Icmp => {
            let icmp_header = IcmpTraceC::build_icmp_header(
                &policy_evaluation,
                &internet_checksum_chunk_pattern_v,
                test_index,
                byte_time_sequence_d.get_byte_sequence_index(),
                byte_time_sequence_d.get_interval_c(),
                test_target.ip_src,
                test_target.ip_dst,
            );
            let icmp_trace_d = IcmpTraceD::of_data_byte_time_sequence_d(
                &test_target,
                &policy_evaluation,
                &icmp_header,
                test_index,
                &byte_time_sequence_d,
            );

            println!("generating PCAPs");
            // all_icmp_trace.export(output_pcap_directory_path);
            icmp_trace_d.export(output_pcap_directory_path, test_index);
        }
        IpNextHeaderProtocols::Udp => {
            let udp_header = UdpTraceC::build_udp_header(
                &policy_evaluation,
                &internet_checksum_chunk_pattern_v,
                test_index,
                // byte_time_sequence_d.get_byte_sequence_index(),
                byte_time_sequence_d.get_interval_c(),
                test_target.ip_src,
                test_target.ip_dst,
            );
            let udp_trace_d = UdpTraceD::of_data_byte_time_sequence_d(
                &test_target,
                &policy_evaluation,
                &udp_header,
                test_index,
                &byte_time_sequence_d,
            );

            println!("generating PCAPs");
            // all_icmp_trace.export(output_pcap_directory_path);
            udp_trace_d.export(output_pcap_directory_path, test_index);
        }
        _ => {
            panic!("Unsupported protocol (should never happen => probable bug in argument parsing)")
        }
    }
}

fn build_data_trace_export<I: Debug + Copy + IpAddrGeneric>(
    output_protocol: IpNextHeaderProtocol,
    policy_evaluation: PolicyEvaluation,
    internet_checksum_chunk_pattern_v: Vec<String>,
    // test_index: u16,
    test_target: TestTarget<I>,
    relation_type: RelationType,
    sequence_json_path: &Path,
    output_pcap_directory_path: &Path,
) -> io::Result<()> {
    let file = File::open(sequence_json_path)?;
    let reader = BufReader::new(file);

    match relation_type {
        RelationType::Pair => {
            let byte_time_sequence_d: ByteTimeSequenceD<AllenIntervalAlgebraRelation> =
                serde_json::from_reader(reader)?;

            build_trace_export(
                output_protocol,
                policy_evaluation,
                internet_checksum_chunk_pattern_v,
                byte_time_sequence_d.get_byte_sequence_index(),
                test_target,
                byte_time_sequence_d,
                output_pcap_directory_path,
            );

            Ok(())
        }
        RelationType::Triplet => {
            let byte_time_sequence_d: ByteTimeSequenceD<RelationTripletD> =
                serde_json::from_reader(reader)?;

            build_trace_export(
                output_protocol,
                policy_evaluation,
                internet_checksum_chunk_pattern_v,
                byte_time_sequence_d.get_byte_sequence_index(),
                test_target,
                byte_time_sequence_d,
                output_pcap_directory_path,
            );

            Ok(())
        }
        RelationType::Custom => {
            let byte_time_sequence_d: ByteTimeSequenceD<RelationCustomD> =
                serde_json::from_reader(reader)?;

            build_trace_export(
                output_protocol,
                policy_evaluation,
                internet_checksum_chunk_pattern_v,
                byte_time_sequence_d.get_byte_sequence_index(),
                test_target,
                byte_time_sequence_d,
                output_pcap_directory_path,
            );

            Ok(())
        }
    }
}

fn main() -> io::Result<()> {
    env_logger::init();

    let matches = Command::new("generate-trace-ip-fragmentation-icmp")
        .version("0.1")
        .author("Johan Mazel <johan.mazel@ssi.gouv.fr>")
        .about("Generate ICMP PCAP traces to test IP fragmentation reassambly policies.")
        .arg(
            Arg::new("sequence-json-path")
                .short('s')
                .long("sequence-json-path")
                .help("Sequence JSON path")
                .required(true),
        )
        .arg(
            Arg::new("chunk-pattern-json-path")
                .long("cp")
                .help("Chunk pattern JSON path")
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
            Arg::new("output-protocol")
                // .short('p')
                .long("op")
                .help("Output protocol: icmp | udp")
                .required(true),
        )
        .arg(
            Arg::new("policy-evaluation")
                // .short('p')
                .long("pe")
                .help("Policy evaluation: p (Progressive) | os (Once with byte-wise starting chunk sent last) | os (Once with byte-wise starting and ending chunks sent last in this order) | os (Oncebyte-wise ending and starting chunks sent last in this order)")
                .default_value("p")
                .required(false),
        )
        .arg(
            Arg::new("relation-type")
                .long("rt")
                .help("Relation type: pair | triplet")
                .required(true),
        )
        .get_matches();

    let sequence_json_path_s = match matches.get_one::<String>("sequence-json-path") {
        Some(s) => s,
        None => {
            eprintln!("No sequence JSON path provided");
            exit(-1);
        }
    };
    println!("sequence_json_path_s: {}", sequence_json_path_s);
    let sequence_json_path = Path::new(sequence_json_path_s);

    let chunk_pattern_json_path_s = match matches.get_one::<String>("chunk-pattern-json-path") {
        Some(s) => s,
        None => {
            eprintln!("No chunk pattern JSON path provided");
            exit(-1);
        }
    };
    println!("chunk_pattern_json_path_s: {}", chunk_pattern_json_path_s);
    let chunk_pattern_json_path = Path::new(chunk_pattern_json_path_s);

    let output_pcap_directory_path_s = match matches.get_one::<String>("output-pcap-directory-path")
    {
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

    let outut_protocol_s = match matches.get_one::<String>("output-protocol") {
        Some(s) => s,
        None => {
            eprintln!("No output protocol type provided");
            exit(-1);
        }
    };
    println!("outut_protocol_s: {}", outut_protocol_s);
    let output_protocol = match outut_protocol_s.as_str() {
        "icmp" => Ok(IpNextHeaderProtocols::Icmp),
        "udp" => Ok(IpNextHeaderProtocols::Udp),
        _ => Err(io::ErrorKind::InvalidData),
    }?;

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

    let relation_type_s = match matches.get_one::<String>("relation-type") {
        Some(s) => s,
        None => {
            eprintln!("No relation type provided");
            exit(-1);
        }
    };
    println!("relation_type_s: {}", relation_type_s);
    let relation_type = RelationType::of_string(relation_type_s).unwrap();

    if !sequence_json_path.exists() {
        println!(
            "Sequence JSON path ({:?}) does not exists!",
            sequence_json_path
        );
        std::process::exit(-1)
    }

    if !chunk_pattern_json_path.exists() {
        println!(
            "Chunk pattern JSON path ({:?}) does not exists!",
            chunk_pattern_json_path
        );
        std::process::exit(-1)
    }

    if !output_pcap_directory_path.exists() {
        println!(
            "Output PCAP directory path ({:?}) does not exists!",
            output_pcap_directory_path
        );
        std::process::exit(-1)
    }

    let file = File::open(chunk_pattern_json_path)?;
    let reader = BufReader::new(file);
    let internet_checksum_chunk_pattern_v: Vec<String> = serde_json::from_reader(reader)?;

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
        let ipv4addr_src: Ipv4Addr = ipv4addr_src_s
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
        // let all_icmp_trace = IcmpTrace::of_data(
        //     &test_target,
        //     &policy_evaluation,
        //     &pair_triplet_byte_time_sequence,
        // );

        build_data_trace_export(
            output_protocol,
            policy_evaluation,
            internet_checksum_chunk_pattern_v,
            // test_index,
            test_target,
            relation_type,
            sequence_json_path,
            output_pcap_directory_path,
        )?
    } else if ip_version == "6" {
        let ipv6addr_src_s = match matches.get_one::<String>("ipv6-src") {
            Some(s) => s,
            None => {
                eprintln!("No source IP address path provided");
                exit(-1);
            }
        };
        println!("ipv6addr_src_s: {}", ipv6addr_src_s);
        let ipv6addr_src: Ipv6Addr = ipv6addr_src_s
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

        build_data_trace_export(
            output_protocol,
            policy_evaluation,
            internet_checksum_chunk_pattern_v,
            test_target,
            relation_type,
            sequence_json_path,
            output_pcap_directory_path,
        )?
    } else {
        eprintln!("Bad ip version provided");
        std::process::exit(-1)
    }

    Ok(())
}
