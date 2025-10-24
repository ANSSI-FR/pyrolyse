use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum InputMode {
    SingleChunkC,
    SingleByteTimeSequence,
    AllByteTimeSequence,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseInputModeError(String);

impl FromStr for InputMode {
    type Err = ParseInputModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "scc" => Ok(InputMode::SingleChunkC),
            "sbts" => Ok(InputMode::SingleByteTimeSequence),
            "abts" => Ok(InputMode::AllByteTimeSequence),
            _ => Err(ParseInputModeError(s.to_string())),
        }
    }
}

// impl TcpTcb {
//     pub fn new(
//         snd_una: u32,
//         snd_nxt: u32,
//         snd_nxt_after_3whs: u32,
//         iss: u32,
//         rcv_nxt: u32,
//         irs: u32,
//     ) -> TcpTcb {
//         TcpTcb {
//             snd_una,
//             snd_nxt,
//             snd_nxt_after_3whs,

//             iss,

//             rcv_nxt,
//             irs,
//         }
//     }
// }
