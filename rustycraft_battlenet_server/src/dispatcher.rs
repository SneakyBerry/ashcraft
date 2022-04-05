// use crate::{Header, Message, WowRpcResponse};
// use bytes::{BufMut, Bytes, BytesMut};
// use log::{debug, error, info, log, trace, warn};
// use prost::Message as _;
// use std::collections::HashMap;
// use std::future::Future;
// use std::pin::Pin;
// use tokio::sync::mpsc;
// use tokio::sync::mpsc::{Receiver, Sender};
// use tokio::task::JoinHandle;
//
// pub const INTO_SOCKET: i64 = i64::MIN;
//
// #[derive(Default, Debug)]
// pub struct MessageDispatcher {
//     channel_in: Option<Receiver<Message>>,
//     channel_out: Option<Sender<Message>>,
//     subscribers: HashMap<i64, Sender<Message>>,
// }
//
// impl MessageDispatcher {
//     pub fn spawn_channel(&mut self) -> Sender<Message> {
//         let (tx, rx) = mpsc::channel(8000);
//         self.channel_in = Some(rx);
//         tx
//     }
//     pub fn register_output(&mut self, out_channel: Sender<Message>) {
//         self.channel_out = Some(out_channel)
//     }
//     pub fn register_subscriber(&mut self, id: i64, subscriber: Sender<Message>) -> &mut Self {
//         self.subscribers.insert(id, subscriber);
//         self
//     }
//     fn response_writer(request_headers: &mut Header, data: &Bytes) -> Option<Bytes> {
//         request_headers.service_id = 0xFE;
//         request_headers.size = Some((data.len() as u32).to_le());
//         request_headers.is_response = Some(true);
//         request_headers.status = Some(WowRpcResponse::Ok as u32);
//
//         let headers_len: usize = request_headers.encoded_len();
//         let mut buffer = BytesMut::new();
//         buffer.extend_from_slice(&(headers_len as u16).to_be_bytes());
//         buffer.extend_from_slice(request_headers.encode_to_vec().as_slice());
//         buffer.extend_from_slice(data.to_vec().as_slice());
//         Some(Bytes::from(buffer))
//     }
//     fn error_writer(token: u32, error: WowRpcResponse) -> Bytes {
//         let mut out = BytesMut::new();
//         let mut headers: Header = Header::default();
//         headers.status = Some(error as u32);
//         headers.token = token;
//         out.extend_from_slice((headers.encoded_len() as u16).to_be_bytes().as_slice());
//         out.extend_from_slice(headers.encode_to_vec().as_slice());
//         Bytes::from(out)
//     }
//
//     fn response(message: &mut Message) -> Bytes {
//         /// Parse service response into bytes
//         let response = match &message.response {
//             Some(resp) => match resp {
//                 Ok(data) => {
//                     if let Some(response) = Self::response_writer(&mut message.headers, data) {
//                         response
//                     } else {
//                         error!(target: "MessageDispatcher", "Encode response failed.");
//                         Self::error_writer(
//                             message.headers.token,
//                             WowRpcResponse::RpcMalformedResponse,
//                         )
//                     }
//                 }
//                 Err(code) => {
//                     info!(target: "MessageDispatcher",
//                         "Bad request. Headers: {:?}, error: {:?}",
//                         &message.headers, &code
//                     );
//                     Self::error_writer(message.headers.token, WowRpcResponse::RpcMalformedResponse)
//                 }
//             },
//             None => {
//                 panic!("Outgoing message without response.")
//             }
//         };
//         trace!(target: "MessageDispatcher", "Response bytes: {} {:?}", response.len(), response);
//         response
//     }
//
//     pub async fn run_forever(&mut self) -> anyhow::Result<()> {
//         debug!(target: "MessageDispatcher", "Dispatcher started");
//         debug!(target: "MessageDispatcher", "Event subscribers: {:?}", &self.subscribers);
//         let mut channel_in = self.channel_in.as_mut().expect("Input channel is unset");
//         let mut channel_out = self.channel_out.as_ref().expect("Output channel is unset");
//         loop {
//             let mut msg = channel_in.recv().await.expect("Message is None");
//             debug!(target: "MessageDispatcher", "New message received {:?}", &msg);
//             if msg.ttl <= 0 {
//                 warn!(target: "MessageDispatcher", "Message exceed TTL limit: {:?}", msg);
//                 msg.result = Some(Self::error_writer(msg.headers.token, WowRpcResponse::Internal));
//                 msg.close = true;
//                 channel_out.send(msg).await;
//                 continue
//             };
//             msg.ttl -= 1;
//             if msg.dest.eq(&INTO_SOCKET) {
//                 msg.result = Some(Self::response(&mut msg));
//                 channel_out
//                     .send(msg)
//                     .await
//                     .expect("Send message failed");
//             } else {
//                 let handler = self.subscribers.get(&msg.dest);
//                 if let Some(handler) = handler {
//                     handler.send(msg).await.expect("Send message failed");
//                 } else {
//                     warn!(target: "MessageDispatcher", "Unhandled message: {:?}", &msg);
//                     msg.result = Some(Self::error_writer(
//                         msg.headers.token,
//                         WowRpcResponse::RpcNotImplemented,
//                     ));
//                     msg.close = true;
//                     channel_out.send(msg).await;
//                 }
//             }
//         }
//     }
//
//     pub async fn stop(mut self) {
//         self.channel_in.unwrap().close();
//     }
// }
