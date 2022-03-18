use crate::Server;
use protocol::bgs::protocol::connection::v1::{
    ConnectRequest, ConnectResponse, ConnectionService, DisconnectRequest,
};
use protocol::bgs::protocol::{NoData, NoResponse};
use protocol::errors::WowRpcResponse;

#[async_trait::async_trait]
impl ConnectionService for Server {
    async fn connect(
        &mut self,
        request: ConnectRequest,
    ) -> Result<ConnectResponse, WowRpcResponse> {
        let mut response = ConnectResponse::get_default();
        response.use_bindless_rpc = request.use_bindless_rpc;
        response.client_id = request.client_id;
        Ok(response)
    }
    async fn keep_alive(&mut self, _: NoData) -> Result<NoResponse, WowRpcResponse> {
        Ok(NoResponse::default())
    }
    async fn request_disconnect(
        &mut self,
        _: DisconnectRequest,
    ) -> Result<NoResponse, WowRpcResponse> {
        Err(WowRpcResponse::SessionDisconnected)
    }
}
