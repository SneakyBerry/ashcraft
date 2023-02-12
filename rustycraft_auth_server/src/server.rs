use crate::packets::logon_challenge::{LogonChallengeRequest, LogonChallengeResponse};
use crate::packets::logon_proof::{LogonProofRequest, LogonProofResponse};
use crate::packets::realm::{
    Population, Realm, RealmCategory, RealmFlag, RealmListResponse, RealmType,
};
use crate::packets::{AuthResult, DekuWriteWithDebug, Opcode, RequestResult, VERSION_CHALLENGE};
use bytes::BytesMut;
use deku::prelude::*;
use rustycraft_common::Account;
use rustycraft_database::redis::RedisClient;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use wow_srp::normalized_string::NormalizedString;
use wow_srp::server::{SrpProof, SrpServer, SrpVerifier};
use wow_srp::{PublicKey, LARGE_SAFE_PRIME_LITTLE_ENDIAN};

pub type OkType = Box<dyn DekuWriteWithDebug>;

enum ClientState {
    Connected,
    OnChallenge(Option<SrpProof>),
    LoggedIn(Option<SrpServer>),
}

pub struct Client {
    redis: Arc<RedisClient>,
    stream: TcpStream,
    state: ClientState,
    username: Option<String>,
}

impl Client {
    pub fn new(stream: TcpStream, redis: Arc<RedisClient>) -> Client {
        Client {
            redis,
            stream,
            state: ClientState::Connected,
            username: None,
        }
    }

    pub async fn serve_connection(mut self) -> Result<(), anyhow::Error> {
        let mut data = BytesMut::with_capacity(1024);
        while self.stream.read_buf(&mut data).await? > 0 {
            let cmd = Opcode::try_from(data[0])?;
            let res = match cmd {
                Opcode::AuthLogonChallenge => self.handle_logon_challenge(&data).await,
                Opcode::AuthLogonProof => self.handle_logon_proof(&data).await,
                Opcode::AuthReconnectChallenge => self.handle_reconnect_challenge(&data).await,
                Opcode::AuthReconnectProof => self.handle_reconnect_proof(&data).await,
                Opcode::RealmList => self.handle_realm_list().await,
                _ => todo!(),
            };

            // Write response to client socket
            match res {
                Ok(result) => {
                    trace!(addr = ?self.stream.peer_addr(), response = result.as_value());
                    let b = result.to_bytes()?;
                    self.stream.write_all(&b).await?;
                }
                Err(err) => {
                    error!(
                        message = "Packet handling error",
                        addr = ?self.stream.peer_addr(),
                        err = ?err
                    );
                    self.stream.write_all(&err.to_bytes()?).await?;
                    break;
                }
            };
            data.clear();
        }
        Ok(())
    }

    pub async fn handle_logon_challenge(
        &mut self,
        data: &BytesMut,
    ) -> Result<OkType, RequestResult> {
        let (_, data) =
            LogonChallengeRequest::from_bytes((data, 0)).map_err(|_| RequestResult {
                cmd: Opcode::AuthLogonChallenge,
                protocol_version: None,
                result: AuthResult::FailDisconnected,
            })?;
        let username = NormalizedString::from(&data.username).map_err(|_| RequestResult {
            cmd: Opcode::AuthLogonChallenge,
            protocol_version: None,
            result: AuthResult::FailDisconnected,
        })?;
        let password = NormalizedString::from(&data.username).map_err(|_| RequestResult {
            cmd: Opcode::AuthLogonChallenge,
            protocol_version: None,
            result: AuthResult::FailDisconnected,
        })?;
        self.username = Some(data.username);
        let proof = SrpVerifier::from_username_and_password(username, password).into_proof();

        let response = LogonChallengeResponse {
            result: RequestResult {
                cmd: Opcode::AuthLogonChallenge,
                protocol_version: Some(0),
                result: AuthResult::Success,
            },
            server_public_key: *proof.server_public_key(),
            generator_length: wow_srp::GENERATOR_LENGTH,
            generator: wow_srp::GENERATOR.to_le_bytes(),
            large_safe_prime_length: LARGE_SAFE_PRIME_LITTLE_ENDIAN.len() as u8,
            large_safe_prime: LARGE_SAFE_PRIME_LITTLE_ENDIAN,
            salt: *proof.salt(),
            crc_salt: VERSION_CHALLENGE,
            security_flags: 0,
        };
        self.state = ClientState::OnChallenge(Some(proof));
        Ok(Box::new(response))
    }

    pub async fn handle_logon_proof(&mut self, data: &BytesMut) -> Result<OkType, RequestResult> {
        let (_, client_proof) =
            LogonProofRequest::from_bytes((data, 0)).map_err(|_| RequestResult {
                cmd: Opcode::AuthLogonProof,
                protocol_version: None,
                result: AuthResult::FailDisconnected,
            })?;
        if let ClientState::OnChallenge(proof) = &mut self.state {
            let (server, server_proof) = proof
                .take()
                .expect("Should be always Some")
                .into_server(
                    PublicKey::from_le_bytes(&client_proof.client_public_key).map_err(|_| {
                        RequestResult {
                            cmd: Opcode::AuthLogonProof,
                            protocol_version: None,
                            result: AuthResult::FailDisconnected,
                        }
                    })?,
                    client_proof.client_proof,
                )
                .map_err(|_| RequestResult {
                    cmd: Opcode::AuthLogonProof,
                    protocol_version: None,
                    result: AuthResult::FailIncorrectPassword,
                })?;
            self.redis
                .set(
                    self.username
                        .as_ref()
                        .expect("Can not be None if we come to this step."),
                    &Account {
                        session_key: server.session_key().to_vec(),
                    },
                )
                .await
                .map_err(|_| RequestResult {
                    cmd: Opcode::AuthLogonProof,
                    protocol_version: None,
                    result: AuthResult::FailDbBusy,
                })?;
            self.state = ClientState::LoggedIn(Some(server));
            Ok(Box::new(LogonProofResponse {
                result: RequestResult {
                    cmd: Opcode::AuthLogonProof,
                    protocol_version: None,
                    result: AuthResult::Success,
                },
                server_proof,
                account_flags: 0,
                hardware_survey_id: 0,
                login_flags: 0,
            }))
        } else {
            Err(RequestResult {
                cmd: Opcode::AuthLogonProof,
                protocol_version: None,
                result: AuthResult::FailDisconnected,
            })
        }
    }

    pub async fn handle_realm_list(&mut self) -> Result<OkType, RequestResult> {
        if let ClientState::LoggedIn(_) = &self.state {
            let mut response = RealmListResponse {
                opcode: Opcode::RealmList,
                size: 0,
                realms_count: 0,
                realms: vec![Realm {
                    realm_type: RealmType::PvP,
                    locked: 0,
                    flag: RealmFlag::ForceGreenRecommended,
                    name: "Test realm".to_string(),
                    address: "127.0.0.1:8085".to_string(),
                    population: Population::GreenRecommended,
                    number_of_characters_on_realm: 0,
                    category: RealmCategory::One,
                    realm_id: 1,
                }],
            };
            response.realms_count = response.realms.len() as u16;
            // TODO: Do not serialize to bytes just for get a length of body.
            // Implement it like in world server
            response.size = (response.to_bytes().expect("It was work, I swear.").len() - 3) as u16;
            Ok(Box::new(response))
        } else {
            Err(RequestResult {
                cmd: Opcode::AuthLogonProof,
                protocol_version: None,
                result: AuthResult::FailDisconnected,
            })
        }
    }

    pub async fn handle_reconnect_challenge(
        &mut self,
        _data: &BytesMut,
    ) -> Result<OkType, RequestResult> {
        todo!()
    }

    pub async fn handle_reconnect_proof(
        &mut self,
        _data: &BytesMut,
    ) -> Result<OkType, RequestResult> {
        todo!()
    }
}
