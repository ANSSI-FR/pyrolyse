use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum ConnectionEndMode {
    Rst,
    MultipleRst,
    FinHandshake,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseConnectionEndModeError(String);

impl FromStr for ConnectionEndMode {
    type Err = ParseConnectionEndModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rst" => Ok(ConnectionEndMode::Rst),
            "mrst" => Ok(ConnectionEndMode::MultipleRst),
            "fhs" => Ok(ConnectionEndMode::FinHandshake),
            _ => Err(ParseConnectionEndModeError(s.to_string())),
        }
    }
}
