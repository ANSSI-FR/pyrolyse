use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::io;
//use std::sync::MutexGuard;
//use std::sync::PoisonError;

use subprocess::PopenError;

#[derive(Debug)]
pub enum TcpSendingError {
    Cursock(String),
    Io(io::Error),
    IpToInterfaceName(String),
    IpToMac(String),
    LocalIpAddress(local_ip_address::Error),
    PathToString(String),
    Pnet(String),
    Popen(PopenError),
    PoisonError,
    ServerError(String),
    EtherTypesError,
    IpVersionsMismatchError,
    // TODO: clean/rename
    Truc(Box<dyn std::any::Any + Send>),
}

impl Display for TcpSendingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TcpSendingError::Cursock(s) => write!(f, "{}", s),
            TcpSendingError::Io(e) => write!(f, "{}", e),
            TcpSendingError::IpToInterfaceName(s) => write!(f, "{}", s),
            TcpSendingError::IpToMac(s) => write!(f, "{}", s),
            TcpSendingError::LocalIpAddress(e) => write!(f, "{}", e),
            TcpSendingError::PathToString(s) => write!(f, "{}", s),
            TcpSendingError::Pnet(s) => write!(f, "{}", s),
            TcpSendingError::Popen(e) => write!(f, "{}", e),
            TcpSendingError::PoisonError => write!(f, "poison"),
            TcpSendingError::ServerError(e) => write!(f, "{}", e),
            TcpSendingError::EtherTypesError => write!(f, "ether type"),
            TcpSendingError::IpVersionsMismatchError => write!(f, "ip versions mismatch"),
            // TcpSendingError::Truc(e) => write!(f, "{}", e),
            TcpSendingError::Truc(e) => write!(f, "truc error"),
        }
    }
}

impl Error for TcpSendingError {}
