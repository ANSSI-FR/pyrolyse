use derive_getters::Getters;

#[derive(Debug, Clone, Getters)]
pub struct TcpTcb {
    pub snd_una: u32, // SND.UNA - send unacknowledged
    pub snd_nxt: u32, // SND.NXT - send next
    // pub snd_nxt_after_3whs: u32,
    // snd_wnd : u32, // SND.WND - send window
    // snd_up : u32, // SND.UP  - send urgent pointer
    // snd_nxt : u32, // SND.WL1 - segment sequence number used for last window update
    // snd_nxt : u32, // SND.WL2 - segment acknowledgment number used for last window update
    pub iss: u32, // ISS     - initial send sequence number

    pub rcv_nxt: u32, // RCV.NXT - receive next
    // rcv_wnd : u32, // RCV.WND - receive window
    // rcv_up : u32, // RCV.UP  - receive urgent pointer
    pub irs: u32, // IRS     - initial receive sequence number
}

impl TcpTcb {
    pub fn new(
        snd_una: u32,
        snd_nxt: u32,
        iss: u32,
        rcv_nxt: u32,
        irs: u32,
    ) -> TcpTcb {
        TcpTcb {
            snd_una,
            snd_nxt,
            iss,

            rcv_nxt,
            irs,
        }
    }
}
