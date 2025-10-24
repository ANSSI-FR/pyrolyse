use pnet::util::MacAddr;
use std::net::IpAddr;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TestTarget {
    pub macaddr_src: MacAddr,
    pub macaddr_dst: MacAddr,
    pub ip_src: IpAddr,
    pub ip_dst: IpAddr,
}

impl TestTarget {
    pub fn new(
        macaddr_src: MacAddr,
        macaddr_dst: MacAddr,
        ip_src: IpAddr,
        ip_dst: IpAddr,
    ) -> TestTarget {
        TestTarget {
            macaddr_src,
            macaddr_dst,
            ip_src,
            ip_dst,
        }
    }
}
