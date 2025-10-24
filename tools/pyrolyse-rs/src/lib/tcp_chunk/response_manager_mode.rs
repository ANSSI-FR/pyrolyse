use crate::tcp_chunk::tcp_scenario::TcpScenario;

#[derive(Debug, Clone)]
pub enum ResponseManagerMode {
    UpdateTcb,
    UpdateTcbSendAck,
}

impl ResponseManagerMode {
    pub fn from_tcp_scenario(tcp_scenario: TcpScenario) -> ResponseManagerMode {
        match tcp_scenario {
            TcpScenario::ProgressiveAckProgressive => ResponseManagerMode::UpdateTcbSendAck,
            TcpScenario::ProgressiveAckOnce => ResponseManagerMode::UpdateTcb,

            TcpScenario::OnceStartPrecedesAckProgressive => ResponseManagerMode::UpdateTcbSendAck,
            TcpScenario::OnceStartPrecedesAckOnce => ResponseManagerMode::UpdateTcb,

            TcpScenario::OnceEndFollowsAckProgressive => ResponseManagerMode::UpdateTcbSendAck,
            TcpScenario::OnceEndFollowsAckOnce => ResponseManagerMode::UpdateTcb,

            TcpScenario::OnceEndPrecedesAckProgressive => ResponseManagerMode::UpdateTcbSendAck,
            TcpScenario::OnceEndPrecedesAckOnce => ResponseManagerMode::UpdateTcb,

            TcpScenario::OnceStartPrecedesEndFollowsAckProgressive => ResponseManagerMode::UpdateTcbSendAck,
            TcpScenario::OnceStartPrecedesEndFollowsAckOnce => ResponseManagerMode::UpdateTcb,

            TcpScenario::OnceStartPrecedesEndPrecedesAckProgressive => ResponseManagerMode::UpdateTcbSendAck,
            TcpScenario::OnceStartPrecedesEndPrecedesAckOnce => ResponseManagerMode::UpdateTcb,

            TcpScenario::OnceEndPrecedesStartPrecedesAckProgressive => ResponseManagerMode::UpdateTcbSendAck,
            TcpScenario::OnceEndPrecedesStartPrecedesAckOnce => ResponseManagerMode::UpdateTcb,

            TcpScenario::OnceStartFollows => ResponseManagerMode::UpdateTcbSendAck,
            TcpScenario::OnceStartFollowsEndFollows => ResponseManagerMode::UpdateTcbSendAck,
            TcpScenario::OnceEndFollowsStartFollows => ResponseManagerMode::UpdateTcbSendAck,
            TcpScenario::OnceEndPrecedesStartFollows => ResponseManagerMode::UpdateTcbSendAck,
        }
    }
}
