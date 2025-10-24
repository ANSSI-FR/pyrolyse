use std::str::FromStr;

use serde::{Deserialize, Serialize};
use crate::position::pattern::{PatternD,ChunkBasedPatternC};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayloadMode {
    // One-byte pattern mode 
    VariableChecksum1Byte(PatternD), // previously Simple
    // 8-byte long patterns - the IP upper layer checksum is valid for a unique fixed reassembled payload length for IP testing (used by Novak et al.)  
    InvariantChecksumFixedLength8Byte(PatternD), // previously InternetChecksum 
    // 8-byte long patterns - the IPv4 upper layer checksum is valid for multiple reassembled payload lengths for IPv4 testing 
    InvariantChecksumVariableLength8ByteICMPv4(ChunkBasedPatternC), // previously Ipv4InvariantChecksum 
    // 8-byte long patterns - the IPv6 upper layer checksum is valid for multiple reassembled payload lengths for IPv6 testing   
    InvariantChecksumVariableLength8ByteICMPv6(ChunkBasedPatternC), // previously Ipv6InvariantChecksum 
}

impl PayloadMode {
    pub fn get_factor(&self) -> u16 {
        match self {
            // Each pattern contains a single character.
            PayloadMode::VariableChecksum1Byte(_) => 1,
            // Each pattern contains 8 characters.
            PayloadMode::InvariantChecksumFixedLength8Byte(_) 
            | PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(_) 
            | PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(_) => 8,
        }
    }

}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePayloadModeError(String);

impl FromStr for PayloadMode {
    type Err = ParsePayloadModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vc1b" => Ok(PayloadMode::VariableChecksum1Byte(PatternD::new())),
            "icfl8b" => Ok(PayloadMode::InvariantChecksumFixedLength8Byte(PatternD::new())),
            "icvl8i4" => Ok(PayloadMode::InvariantChecksumVariableLength8ByteICMPv4(ChunkBasedPatternC::new())),
            "icvl8i6" => Ok(PayloadMode::InvariantChecksumVariableLength8ByteICMPv6(ChunkBasedPatternC::new())),
            _ => Err(ParsePayloadModeError(s.to_string())),
        }
    }

}
