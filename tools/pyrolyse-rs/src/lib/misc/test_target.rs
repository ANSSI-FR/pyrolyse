use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::net::IpAddr;

use cursock::Socket;
use local_ip_address::list_afinet_netifas;
use pnet::{packet::ethernet::EtherType, util::MacAddr};

use crate::misc::ip_addr_fragmentation_trait_impl::IpAddrForFragmentationTesting;
use crate::tcp_chunk::tcp_sending_error::TcpSendingError;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TestTarget<I: Into<IpAddr>> {
    pub mac_addr_src: MacAddr,
    pub mac_addr_dst: MacAddr,
    pub ip_addr_src: I,
    pub ip_addr_dst: I,
}

pub fn ip_addr_to_interface_name<
    I: Display + Into<IpAddr> + Clone + IpAddrForFragmentationTesting,
>(
    ip_addr: &I,
) -> Result<String, TcpSendingError> {
    let ip_addr_cloned = ip_addr.clone().into();
    let interface_ip_addr_v = list_afinet_netifas().map_err(TcpSendingError::LocalIpAddress)?;
    let ip_addr_interface_hm: HashMap<_, _> = interface_ip_addr_v
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect();
    let interface_name = ip_addr_interface_hm
        .get(&ip_addr_cloned)
        .ok_or(format!(
            "Could not find interface for {} from ARP tables",
            ip_addr_cloned
        ))
        .map_err(TcpSendingError::IpToInterfaceName)?;
    Ok((*interface_name).clone())
}

impl<I: Display + Debug + PartialEq + Into<IpAddr> + Clone + IpAddrForFragmentationTesting>
    TestTarget<I>
{
    pub fn new(
        mac_addr_src: MacAddr,
        mac_addr_dst: MacAddr,
        ip_addr_src: I,
        ip_addr_dst: I,
    ) -> TestTarget<I> {
        TestTarget {
            mac_addr_src,
            mac_addr_dst,
            ip_addr_src,
            ip_addr_dst,
        }
    }

    pub fn get_ether_type(&self) -> EtherType {
        IpAddrForFragmentationTesting::get_ether_type(&self.ip_addr_src)
    }

    pub fn from_source_destination_ip_addr(
        ip_addr_src: I,
        ip_addr_dst: I,
    ) -> Result<TestTarget<I>, TcpSendingError> {
        // Src
        let interface_name = ip_addr_to_interface_name(&ip_addr_src)?;

        #[cfg(target_os = "linux")]
        let socket = Socket::new(&interface_name).expect("initialize error"); // Linux

        let adapter = socket.get_adapter();

        let mac = adapter.get_mac();
        let u8_sl: [u8; 6] = mac.clone().into();
        let mac_addr_src = MacAddr::from(u8_sl);

        let ip_addr_src_from_cursock = (*I::get_from_adapter(adapter))
            .clone()
            .ok_or(format!(
                "Could not find IPv4 address for {}",
                interface_name
            ))
            .map_err(TcpSendingError::Cursock)?;
        assert_eq!(ip_addr_src, ip_addr_src_from_cursock);

        // Dst
        let v = netneighbours::get_table();
        let hm = v.into_iter().collect::<HashMap<_, _>>();
        let ip_addr_dst_cloned = ip_addr_dst.clone().into();
        let macaddr6 = hm
            .get(&ip_addr_dst_cloned)
            .ok_or(format!(
                "Could not find DST MAC for {} from ARP tables",
                ip_addr_dst
            ))
            .map_err(TcpSendingError::IpToMac)?;
        let mac_addr_dst = MacAddr::from(macaddr6.into_array());

        Ok(TestTarget::new(
            mac_addr_src,
            mac_addr_dst,
            ip_addr_src,
            ip_addr_dst,
        ))
    }
}
