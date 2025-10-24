// #[macro_use]
extern crate clap;
extern crate env_logger;
extern crate itertools;
extern crate log;
extern crate pcap_parser;
extern crate pnet;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::path::Path;
use std::process::exit;

use clap::{value_parser, Arg, Command};

use rst_lib::byte_time_data::byte_time_sequence::ByteTimeSequenceD;
use rst_lib::byte_time_data::chunk::ChunkC;
use rst_lib::byte_time_data::pair_triplet_byte_time_sequence::PairTripletByteTimeSequence;
use rst_lib::misc::test_index::TestIndex;
use rst_lib::misc::test_target;
use rst_lib::misc::test_target::TestTarget;
use rst_lib::relation::allen_interval_algebra_relation::AllenIntervalAlgebraRelation;
use rst_lib::relation::relation_triplet::RelationTripletD;
use rst_lib::relation::relation_custom::RelationCustomD;
use rst_lib::tcp_chunk::connection_end_mode::ConnectionEndMode;
use rst_lib::tcp_chunk::input_mode::InputMode;
use rst_lib::position::payload_mode::PayloadMode;
use rst_lib::tcp_chunk::tcp_manager;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let matches = Command::new("send-tcp-chunk")
        .version("0.1")
        .author("Johan Mazel <johan.mazel@ssi.gouv.fr> and Lucas Aubard")
        .about("Send TCP chunks.")
        .arg(
            Arg::new("input-json-path")
                .short('j')
                .long("input-json-path")
                .help("Input JSON path")
                .required(true),
        )
        .arg(
            Arg::new("output-pcap-path")
                .short('o')
                .long("output-pcap-path")
                .help("Output PCAP path")
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
            Arg::new("mac-src")
                .long("mac-src")
                .help("Source MAC address")
                .required(false),
        )
        .arg(
            Arg::new("mac-dst")
                .long("mac-dst")
                .help("Destination MAC address")
                .requires("mac-src"),
        )
        .arg(
            Arg::new("port-destination")
                .short('p')
                .long("port-destination")
                .help("TCP destination port")
                .default_value("7")
                .value_parser(clap::value_parser!(u16)),
        )
        .arg(
            Arg::new("test-index-offset")
                .long("tio")
                .help("Test index offset (value added to the TCP src port)")
                .default_value("0")
                .value_parser(value_parser!(u16)),
        )
        .arg(
            Arg::new("test-index")
                .short('i')
                .long("test-index")
                .help("Test index")
                .value_parser(clap::value_parser!(u16))
                .required(true),
        )
        .arg(
            Arg::new("tcp-scenario")
                .short('c')
                .long("tcp-scenario")
                .help("TCP scenario")
                .required(true),
        )
        .arg(
            Arg::new("input-mode")
                .long("input-mode")
                .default_value("scc")
                .help("Input mode: scc (single chunk)|sbts (single byte time sequence)|abts (all byte time sequence)")
        )
        .arg(
            Arg::new("payload-mode")
                .long("payload-mode")
                .default_value("icfl8b")
                .help("Payload mode: vc1b (variable checksum 1 byte)|icfl8b (invariant checksum fixed length 8 byte)|icvl8i4 (invariant checksum variable length 8 byte ICMPv4)|icvl8i6 (invariant checksum variable length 8 byte ICMPv6) ")
        )
        .arg(
            Arg::new("connection-end-mode")
                .long("connection-end-mode")
                .default_value("mrst")
                .help("Connection end mode: rst (RST)| mrst (RST after each chunk)|fhs (FIN/FINACK/ACK handshake) ")
        )
        
        .get_matches();

    let input_json_path_s = match matches.get_one::<String>("input-json-path") {
        Some(s) => s,
        None => {
            eprintln!("No input JSON path provided");
            exit(-1);
        }
    };
    println!("input_json_path_s: {}", input_json_path_s);
    let input_json_path = Path::new(input_json_path_s);

    let output_pcap_path_s = match matches.get_one::<String>("output-pcap-path") {
        Some(s) => s,
        None => {
            eprintln!("No output PCAP path provided");
            exit(-1);
        }
    };
    println!("output_pcap_path_s: {}", output_pcap_path_s);
    let output_pcap_path = Path::new(output_pcap_path_s);

    let port_destination = match matches.get_one::<u16>("port-destination") {
        Some(v) => v,
        None => {
            eprintln!("No destination port provided");
            exit(-1);
        }
    };
    println!("port_destination: {}", port_destination);

    let test_index_offset = match matches.get_one::<u16>("test-index-offset") {
        Some(s) => s,
        None => {
            eprintln!("No test index offset provided");
            exit(-1);
        }
    };
    println!("test_index_offset: {}", test_index_offset);

    let test_index = match matches.get_one::<u16>("test-index") {
        Some(v) => v,
        None => {
            eprintln!("No test index provided");
            exit(-1);
        }
    };
    println!("test_index: {}", test_index);

    let tcp_scenario_s = match matches.get_one::<String>("tcp-scenario") {
        Some(s) => s,
        None => {
            eprintln!("No TCP scenario provided");
            exit(-1);
        }
    };
    println!("tcp_scenario_s: {}", tcp_scenario_s);
    let tcp_scenario = tcp_scenario_s
        .parse()
        .expect("tcp-scenario is not a valid TcpScenario");

    let input_mode_s = match matches.get_one::<String>("input-mode") {
        Some(s) => s,
        None => {
            eprintln!("No input mode provided");
            exit(-1);
        }
    };
    println!("input_mode_s: {}", input_mode_s);
    let input_mode = input_mode_s
        .parse()
        .expect("input-mode is not a valid TcpScenario");

    let payload_mode_s = match matches.get_one::<String>("payload-mode") {
        Some(s) => s,
        None => {
            eprintln!("No input mode provided");
            exit(-1);
        }
    };
    println!("payload_mode_s: {}", payload_mode_s);
    let payload_mode: PayloadMode = payload_mode_s
        .parse()
        .expect("Invalid input for PayloadMode");

    //let test_target = if let Some(macaddr_src_s) = matches.get_one::<String>("mac-src") {
    //    let macaddr_dst_s = matches.get_one::<String>("mac-dst").unwrap();
    //    let macaddr_src = macaddr_src_s
    //        .parse()
    //        .expect("mac-src is not a valid MacAddr");
    //    let macaddr_dst = macaddr_dst_s
    //        .parse()
    //        .expect("mac-dst is not a valid MacAddr");
//
    //    //TestTarget::new(macaddr_src, macaddr_dst, ipv4addr_src, ipv4addr_dst)
    //    //TestTarget::new(macaddr_src, macaddr_dst, ipv6addr_src, ipv6addr_dst)
    //    TestTarget::new(macaddr_src, macaddr_dst, ipaddr_src, ipaddr_dst)
    //} else {
    //    //TestTarget::from_source_destination_ip_addr(ipv4addr_src, ipv4addr_dst)?
    //    //TestTarget::from_source_destination_ip_addr(ipv6addr_src, ipv6addr_dst)?
    //    TestTarget::from_source_destination_ip_addr(ipaddr_src, ipaddr_dst)?
    //};
    
    let connection_end_mode_s = match matches.get_one::<String>("connection-end-mode") {
        Some(s) => s,
        None => {
            eprintln!("No input mode provided");
            exit(-1);
        }
    };
    println!("payload_mode_s: {}", payload_mode_s);
    let connection_end_mode: ConnectionEndMode = connection_end_mode_s
        .parse()
        .expect("Invalid input for PayloadMode");

    //let interface_name = test_target::ip_addr_to_interface_name(&ipv4addr_src)?;
    //let interface_name = test_target::ip_addr_to_interface_name(&ipv6addr_src)?;
    //let interface_name = test_target::ip_addr_to_interface_name(&ipaddr_src)?;

    let file = File::open(input_json_path)?;
    let reader = BufReader::new(file);

    // TODO: encode this value somewhere
    let triplet_index_offset = 100;
    let custom_index_offset = 600;

    let chunk_c: ChunkC = match input_mode {
        InputMode::SingleChunkC => {
            let chunk_c: ChunkC = serde_json::from_reader(reader)?;
            chunk_c
        }
        InputMode::SingleByteTimeSequence => {
            if *test_index < triplet_index_offset {
                let byte_time_sequence_d: ByteTimeSequenceD<AllenIntervalAlgebraRelation> =
                    serde_json::from_reader(reader)?;
                (*byte_time_sequence_d.get_chunk_c()).clone()
            } else if *test_index < custom_index_offset {
                let byte_time_sequence_d: ByteTimeSequenceD<RelationTripletD> =
                    serde_json::from_reader(reader)?;
                (*byte_time_sequence_d.get_chunk_c()).clone()
            }
            else {
                let byte_time_sequence_d: ByteTimeSequenceD<RelationCustomD> =
                    serde_json::from_reader(reader)?;
                (*byte_time_sequence_d.get_chunk_c()).clone()
            }
        }
        InputMode::AllByteTimeSequence => {
            let pair_triplet_byte_time_sequence: PairTripletByteTimeSequence =
                serde_json::from_reader(reader)?;

            if *test_index < triplet_index_offset {
                (*pair_triplet_byte_time_sequence
                    .get_byte_time_sequence_c_pair()
                    .get(&TestIndex(*test_index))
                    .unwrap()
                    .get_chunk_c())
                .clone()
            } else {
                (*pair_triplet_byte_time_sequence
                    .get_byte_time_sequence_c_triplet()
                    .get(&TestIndex(*test_index))
                    .unwrap()
                    .get_chunk_c())
                .clone()
            }
        }
    };

    let port_source = test_index_offset + test_index;

    let ip_version = match matches.get_one::<String>("ip-version") {
        Some(s) => s,
        None => {
            eprintln!("No IP version provided");
            exit(-1);
        }
    };

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
        let ipv4addr_dst: Ipv4Addr = ipv4addr_dst_s
            .parse()
            .expect("ipv4-dst is not a valid ipAddr");

        let test_target = if let Some(macaddr_src_s) = matches.get_one::<String>("mac-src") {
            let macaddr_dst_s = matches.get_one::<String>("mac-dst").unwrap();
            let macaddr_src = macaddr_src_s
                .parse()
                .expect("mac-src is not a valid MacAddr");
            let macaddr_dst = macaddr_dst_s
                .parse()
                .expect("mac-dst is not a valid MacAddr");

            TestTarget::new(macaddr_src, macaddr_dst, ipv4addr_src, ipv4addr_dst)
        } else {
            TestTarget::from_source_destination_ip_addr(ipv4addr_src, ipv4addr_dst)?
        };

        let interface_name = test_target::ip_addr_to_interface_name(&ipv4addr_src)?;

        tcp_manager::process(
            output_pcap_path,
            interface_name,
            &test_target,
            port_source,
            *port_destination,
            tcp_scenario,
            payload_mode,
            &chunk_c,
            &connection_end_mode,
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
        let ipv6addr_dst: Ipv6Addr = ipv6addr_dst_s
            .parse()
            .expect("ipv6-dst is not a valid ipAddr");
        
        let test_target = if let Some(macaddr_src_s) = matches.get_one::<String>("mac-src") {
            let macaddr_dst_s = matches.get_one::<String>("mac-dst").unwrap();
            let macaddr_src = macaddr_src_s
                .parse()
                .expect("mac-src is not a valid MacAddr");
            let macaddr_dst = macaddr_dst_s
                .parse()
                .expect("mac-dst is not a valid MacAddr");

            TestTarget::new(macaddr_src, macaddr_dst, ipv6addr_src, ipv6addr_dst)
        } else {
            TestTarget::from_source_destination_ip_addr(ipv6addr_src, ipv6addr_dst)?
        };

        let interface_name = test_target::ip_addr_to_interface_name(&ipv6addr_src)?;

        tcp_manager::process(
            output_pcap_path,
            interface_name,
            &test_target,
            port_source,
            *port_destination,
            tcp_scenario,
            payload_mode,
            &chunk_c,
            &connection_end_mode,
        )?

    } else {
        eprintln!("Bad ip version provided");
        std::process::exit(-1)
    };

    Ok(())
}
