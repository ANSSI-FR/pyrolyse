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

use log::debug;

use clap::{value_parser, Arg, Command};
use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::ip::IpNextHeaderProtocols;
use serde::Serialize;

use rst_lib::byte_time_data::byte_time_sequence::ByteTimeSequenceD;
use rst_lib::misc::ip_addr_fragmentation_trait_impl::IpAddrForFragmentationTesting;
use rst_lib::misc::policy_evaluation::PolicyEvaluation;
use rst_lib::misc::test_index::TestIndex;
use rst_lib::misc::test_target::TestTarget;
//use rst_lib::misc::invariant_checksum_chunk_pattern::InvariantChecksumChunkPatternC;
use rst_lib::misc::test_more_chunk;
use rst_lib::pcap::testing_trace::TestingTraceC;
use rst_lib::pcap::testing_trace::TestingTraceD;
use rst_lib::position::pattern::{ChunkBasedPatternC, PatternD};
use rst_lib::position::payload_mode::PayloadMode;
use rst_lib::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use rst_lib::relation::relation_container::RelationContainer;
use rst_lib::relation::relation_custom::RelationCustomD;
use rst_lib::relation::relation_triplet::RelationTripletD;
use rst_lib::relation::relation_type::RelationType;

fn build_trace_export<
    I: Debug + Copy + IpAddrForFragmentationTesting,
    Rc: Debug + Clone + Serialize + RelationContainer,
>(
    output_protocol: IpNextHeaderProtocol,
    policy_evaluation: PolicyEvaluation,
    internet_checksum_chunk_pattern_v: Vec<u8>,
    test_index_offset: TestIndex,
    test_target: TestTarget<I>,
    byte_time_sequence_d: ByteTimeSequenceD<Rc>,
    output_pcap_directory_path: &Path,
    payload_mode: PayloadMode,
    generate_only_unique_test_cases: bool,
) {
    let test_index = byte_time_sequence_d.get_byte_sequence_index();
    let temporal_position_v = byte_time_sequence_d.get_temporal_position_v();
    let interval_c = byte_time_sequence_d.get_interval_c();

    if test_more_chunk::is_test_case_original(
        &policy_evaluation,
        temporal_position_v,
        interval_c,
        generate_only_unique_test_cases,
    ) == false
    {
        debug!(
            "TestIndex {:?} not generated for {:?} policy_evaluation",
            test_index, policy_evaluation
        );
        exit(0);
    }

    let payload_header_data_v = match output_protocol {
        IpNextHeaderProtocols::Icmp => {
            let byte_sequence_test_index = byte_time_sequence_d.get_byte_sequence_index();
            let icmp_identifier = test_index_offset.0 + test_index.0;
            let icmp_sequence_number = test_index_offset.0 + byte_sequence_test_index.0;
            TestingTraceC::build_icmp_header_data_v(
                &policy_evaluation,
                &payload_mode,
                &internet_checksum_chunk_pattern_v,
                icmp_identifier,
                icmp_sequence_number,
                // byte_time_sequence_d.get_byte_sequence_index(),
                byte_time_sequence_d.get_interval_c(),
                test_target.ip_addr_src,
                test_target.ip_addr_dst,
            )
        }
        IpNextHeaderProtocols::Udp => {
            let udp_src_port = test_index_offset.0 + test_index.0;
            TestingTraceC::build_udp_header_data_v(
                &policy_evaluation,
                &payload_mode,
                &internet_checksum_chunk_pattern_v,
                // test_index_offset,
                // test_index,
                // byte_time_sequence_d.get_byte_sequence_index(),
                udp_src_port,
                byte_time_sequence_d.get_interval_c(),
                test_target.ip_addr_src,
                test_target.ip_addr_dst,
            )
        }
        _ => panic!("Unsupported IP payload"),
    };

    let testing_trace_d = TestingTraceD::of_data_byte_time_sequence_d(
        //&internet_checksum_chunk_pattern_v,
        &test_target,
        &policy_evaluation,
        &payload_header_data_v,
        // test_index,
        &byte_time_sequence_d,
        &payload_mode,
    );

    println!("generating PCAPs");
    // all_icmp_trace.export(output_pcap_directory_path);
    testing_trace_d.export(
        output_protocol,
        output_pcap_directory_path,
        test_index_offset,
        test_index,
    );
}

fn build_data_trace_export<I: Debug + Copy + IpAddrForFragmentationTesting>(
    output_protocol: IpNextHeaderProtocol,
    policy_evaluation: PolicyEvaluation,
    internet_checksum_chunk_pattern_v: Vec<u8>,
    test_index_offset: TestIndex,
    test_target: TestTarget<I>,
    relation_type: RelationType,
    sequence_json_path: &Path,
    output_pcap_directory_path: &Path,
    payload_mode: PayloadMode,
    generate_only_unique_test_cases: bool,
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
                test_index_offset,
                // byte_time_sequence_d.get_byte_sequence_index(),
                test_target,
                byte_time_sequence_d,
                output_pcap_directory_path,
                payload_mode,
                generate_only_unique_test_cases,
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
                test_index_offset,
                // byte_time_sequence_d.get_byte_sequence_index(),
                test_target,
                byte_time_sequence_d,
                output_pcap_directory_path,
                payload_mode,
                generate_only_unique_test_cases,
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
                test_index_offset,
                // byte_time_sequence_d.get_byte_sequence_index(),
                test_target,
                byte_time_sequence_d,
                output_pcap_directory_path,
                payload_mode,
                generate_only_unique_test_cases,
            );

            Ok(())
        }
    }
}

fn main() -> io::Result<()> {
    env_logger::init();

    let matches = Command::new("generate-trace-ip-fragmentation-icmp")
        .version("0.1")
        .author("Johan Mazel <johan.mazel@ssi.gouv.fr> and Lucas Aubard")
        .about("Generate an ICMP PCAP trace from a chunk sequence to test IP fragmentation reassembly policies.")
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
        .arg(
            Arg::new("test-index-offset")
                .long("tio")
                .help("Test index offset (value added to the test index inside IP ID, ICMP id/sequence number, and UDP src port)")
                .default_value("0")
                // .required(true)
                .value_parser(value_parser!(u16)),
        )
        .arg(
            Arg::new("payload-mode")
                .long("payload-mode")
                .help("Payload mode: vc1b (variable checksum 1 byte)|icfl8b (invariant checksum fixed length 8 byte)|icvl8i4 (invariant checksum variable length 8 byte ICMPv4)|icvl8i6 (invariant checksum variable length 8 byte ICMPv6)")
                .required(true),
        )
        .arg(
            Arg::new("generate-only-unique-test-cases")
                .long("goutc")
                .help("Indicates whether the generated test cases should be unique across testing scenarii (yes: utc | false: nutc)")
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

    let internet_checksum_chunk_pattern_json_path_s =
        match matches.get_one::<String>("chunk-pattern-json-path") {
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
    let internet_checksum_chunk_pattern_json_path: &Path =
        Path::new(internet_checksum_chunk_pattern_json_path_s);

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

    let generate_only_unique_test_cases_s =
        match matches.get_one::<String>("generate-only-unique-test-cases") {
            Some(s) => s,
            None => {
                eprintln!("No output unique test cases flag provided");
                exit(-1);
            }
        };
    println!(
        "generate_only_unique_test_cases_s: {}",
        generate_only_unique_test_cases_s
    );
    let generate_only_unique_test_cases = match generate_only_unique_test_cases_s.as_str() {
        "utc" => Ok(true),
        "nutc" => Ok(false),
        _ => Err(io::ErrorKind::InvalidData),
    }?;

    let policy_evaluation_s = match matches.get_one::<String>("policy-evaluation") {
        Some(s) => s,
        None => {
            eprintln!("No policy evaluation provided");
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

    let test_index_offset = match matches.get_one::<u16>("test-index-offset") {
        Some(s) => s,
        None => {
            eprintln!("No test index offset provided");
            exit(-1);
        }
    };
    println!("test_index_offset: {}", test_index_offset);

    let payload_mode_s = match matches.get_one::<String>("payload-mode") {
        Some(s) => s,
        None => {
            eprintln!("No payload mode provided.");
            exit(-1);
        }
    };
    println!("payload_mode_s: {}", payload_mode_s);
    let payload_mode_e = payload_mode_s
        .parse()
        .expect("Invalid input for PayloadMode");

    if !sequence_json_path.exists() {
        println!(
            "Sequence JSON path ({:?}) does not exists!",
            sequence_json_path
        );
        std::process::exit(-1)
    }

    if !internet_checksum_chunk_pattern_json_path.exists() {
        println!(
            "Internet checksum chunk pattern JSON path ({:?}) does not exists!",
            internet_checksum_chunk_pattern_json_path
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

    let file = File::open(internet_checksum_chunk_pattern_json_path)?;
    let reader = BufReader::new(file);
    ////let internet_checksum_chunk_pattern_v: Vec<String> = match payload_mode {
    //let internet_checksum_chunk_pattern_v: Vec<u8> = match payload_mode {
    //    PayloadMode::VariableChecksum1Byte(_)
    //    | PayloadMode::InvariantChecksumFixedLength8Byte(_) => {
    //        let internet_checksum_chunk_pattern_s_v: Vec<String> = serde_json::from_reader(reader)?;
    //        internet_checksum_chunk_pattern_s_v.join("").as_bytes().to_vec()
    //    }
    //    PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(_)
    //    | PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(_) => {
    //        let invariant_checksum_chunk_pattern_c: InvariantChecksumChunkPatternC = serde_json::from_reader(reader)?;
    //        println!("desrialization ok");
    //        invariant_checksum_chunk_pattern_c.to_fake_payload_v()
    //    }
    //
    //};
    let payload_mode: PayloadMode = match payload_mode_e {
        PayloadMode::VariableChecksum1Byte(_) => {
            let pattern_d: PatternD = serde_json::from_reader(reader)?;
            PayloadMode::VariableChecksum1Byte(pattern_d)
        }
        PayloadMode::InvariantChecksumFixedLength8Byte(_) => {
            let pattern_d: PatternD = serde_json::from_reader(reader)?;
            PayloadMode::InvariantChecksumFixedLength8Byte(pattern_d)
        }
        PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(_) => {
            let chunk_based_pattern_c: ChunkBasedPatternC = serde_json::from_reader(reader)?;
            PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(chunk_based_pattern_c)
        }
        PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(_) => {
            let chunk_based_pattern_c: ChunkBasedPatternC = serde_json::from_reader(reader)?;
            PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(chunk_based_pattern_c)
        }
    };

    let internet_checksum_chunk_pattern_v: Vec<u8> = match payload_mode {
        PayloadMode::VariableChecksum1Byte(ref pattern_d)
        | PayloadMode::InvariantChecksumFixedLength8Byte(ref pattern_d) => {
            pattern_d.to_fake_payload_ascii_v()
        }
        PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(ref chunk_based_pattern_c)
        | PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(ref chunk_based_pattern_c) => {
            chunk_based_pattern_c.to_fake_payload_ascii_v()
        }
    };

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
            TestIndex(*test_index_offset),
            test_target,
            relation_type,
            sequence_json_path,
            output_pcap_directory_path,
            payload_mode,
            generate_only_unique_test_cases,
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
            TestIndex(*test_index_offset),
            test_target,
            relation_type,
            sequence_json_path,
            output_pcap_directory_path,
            payload_mode,
            generate_only_unique_test_cases,
        )?
    } else {
        eprintln!("Bad ip version provided");
        std::process::exit(-1)
    }

    Ok(())
}
