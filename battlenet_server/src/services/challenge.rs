use protocol::bgs::protocol::challenge::v1::{
    ChallengeExternalRequest, ChallengeListener,
};
use protocol::bgs::protocol::NoResponse;
use crate::{Header, OutgoingMessage, Server, SocketEvents, WowRpcResponse};

#[async_trait::async_trait]
impl ChallengeListener for Server {
    async fn on_external_challenge(
        &mut self,
        request: ChallengeExternalRequest,
    ) -> Result<NoResponse, WowRpcResponse> {
        let headers = Header {
            method_id: Some(Self::ON_EXTERNAL_CHALLENGE as u32),
            token: self.token as u32,
            service_hash: Some(Self::ORIGINAL_HASH),
            ..Default::default()
        };
        let mut msg = OutgoingMessage {
            headers: headers.clone(),
            message: Some(request),
        };
        self.tx.send(SocketEvents::Send(msg.encode(false))).await?;
        Ok(NoResponse::default())
    }
}
