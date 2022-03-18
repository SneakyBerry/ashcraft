use log::debug;
use protocol::bgs::protocol::challenge::v1::{ChallengeExternalRequest, ChallengeListener};
use protocol::bgs::protocol::{EntityId, Header, NoData, NoResponse};
use protocol::bgs::protocol::authentication::v1::{AuthenticationListener, AuthenticationService, LogonRequest, LogonResult, VerifyWebCredentialsRequest};
use protocol::errors::WowRpcResponse;
use protocol::messages::OutgoingMessage;
use crate::{Server, SocketEvents};


#[async_trait::async_trait]
impl AuthenticationService for Server {
    async fn logon(&mut self, _: LogonRequest) -> Result<NoData, WowRpcResponse> {
        let request = ChallengeExternalRequest {
            request_token: None,
            payload_type: Some("web_auth_url".to_owned()),
            payload: Some(b"https://127.0.0.1:9990/bnetserver/login/".to_vec()),
        };
        self.on_external_challenge(request).await?;
        Ok(NoData::default())
    }

    async fn verify_web_credentials(&mut self, request: VerifyWebCredentialsRequest) -> Result<NoData, WowRpcResponse> {
        debug!("{:?}", String::from_utf8(request.web_credentials.unwrap()).unwrap());
        let logon_result = LogonResult {
            error_code: WowRpcResponse::Ok as u32,
            account_id: Some(EntityId { high: u64::MAX, low: 1 }),
            game_account_id: vec![EntityId { high: u64::MAX, low: 1 }],
            email: None,
            available_region: vec![],
            connected_region: None,
            battle_tag: None,
            geoip_country: None,
            session_key: Some((0..64).map(|_| { rand::random::<u8>() }).collect()),
            restricted_mode: None,
            client_id: None
        };
        self.on_logon_complete(logon_result).await?;
        Ok(NoData::default())
    }
}


#[async_trait::async_trait]
impl AuthenticationListener for Server {
    async fn on_logon_complete(&mut self, request: LogonResult) -> Result<NoResponse, WowRpcResponse> {
        let headers = Header {
            method_id: Some(Self::ON_LOGON_COMPLETE as u32),
            token: self.token as u32,
            service_hash: Some(<Self as AuthenticationListener>::ORIGINAL_HASH),
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