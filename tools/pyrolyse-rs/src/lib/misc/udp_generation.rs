use pnet::packet::udp;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;

pub fn build_udp_pdu_data_v<I>(
    // test_index_offset: u16,
    udp_src_port: u16,
    payload: &[u8],
    ip_src: &I,
    ip_dst: &I,
    checksum: &dyn Fn(&UdpPacket, &I, &I) -> u16,
) -> Vec<u8> {
    let payload_len = payload.len();
    let udp_header_len = 8;
    let udp_packet_len = udp_header_len + payload_len;

    let mut udp_data_like_v: Vec<u8> = vec![0; udp_packet_len];

    let udp = udp::Udp {
        source: udp_src_port,
        destination: 7,
        length: udp_packet_len as u16,
        checksum: 0,
        payload: payload.to_vec(),
    };
    let mut mutable_udp_packet = udp::MutableUdpPacket::new(&mut udp_data_like_v[..]).unwrap();
    mutable_udp_packet.populate(&udp);
    let mutable_echo_request_packet_u8_s = mutable_udp_packet.packet();
    let udp_packet = UdpPacket::new(mutable_echo_request_packet_u8_s).unwrap();
    // let checksum = udp::ipv4_checksum(&udp_packet, ip_src, ip_dst);
    let checksum = checksum(&udp_packet, ip_src, ip_dst);
    mutable_udp_packet.set_checksum(checksum);
    mutable_udp_packet.packet().to_vec()
}
