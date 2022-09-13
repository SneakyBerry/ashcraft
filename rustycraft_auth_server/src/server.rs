use crate::packets::logon_challenge::{LogonChallengeRequest, LogonChallengeResponse};
use crate::packets::logon_proof::{LogonProofRequest, LogonProofResponse};
use crate::packets::realm::{
    Population, Realm, RealmCategory, RealmFlag, RealmListResponse, RealmType,
};
use crate::packets::{AuthResult, DekuWriteDebug, Opcode, RequestResult, VERSION_CHALLENGE};
use bytes::BytesMut;
use deku::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use wow_srp::normalized_string::NormalizedString;
use wow_srp::server::{SrpProof, SrpServer, SrpVerifier};
use wow_srp::{PublicKey, LARGE_SAFE_PRIME_LITTLE_ENDIAN};

pub type OkType = Box<dyn DekuWriteDebug>;

enum ClientState {
    Connected,
    OnChallenge(Option<SrpProof>),
    LoggedIn(Option<SrpServer>),
}

pub struct Client {
    stream: TcpStream,
    state: ClientState,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Client {
            stream,
            state: ClientState::Connected,
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
            match res {
                Ok(result) => {
                    trace!("[{:?}] Response: {:?}", self.stream.peer_addr(), result);
                    let b = result.to_bytes()?;
                    self.stream.write_all(&b).await?;
                }
                Err(err) => {
                    info!(
                        "[{:?}] Packet handling error: {:#?}",
                        self.stream.peer_addr(),
                        err
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
            LogonChallengeRequest::from_bytes((&data, 0)).map_err(|_| RequestResult {
                cmd: Opcode::AuthLogonChallenge,
                protocol_version: None,
                result: AuthResult::WowFailDisconnected,
            })?;
        trace!(
            "[{:?}] LogonChallengeRequest {:?}",
            self.stream.peer_addr(),
            &data
        );
        let username = NormalizedString::from(&data.username).map_err(|_| RequestResult {
            cmd: Opcode::AuthLogonChallenge,
            protocol_version: None,
            result: AuthResult::WowFailDisconnected,
        })?;
        let password = NormalizedString::from(&data.username).map_err(|_| RequestResult {
            cmd: Opcode::AuthLogonChallenge,
            protocol_version: None,
            result: AuthResult::WowFailDisconnected,
        })?;
        let proof = SrpVerifier::from_username_and_password(username, password).into_proof();

        let response = LogonChallengeResponse {
            result: RequestResult {
                cmd: Opcode::AuthLogonChallenge,
                protocol_version: Some(0),
                result: AuthResult::WowSuccess,
            },
            server_public_key: *proof.server_public_key(),
            generator_length: wow_srp::GENERATOR_LENGTH,
            generator: wow_srp::GENERATOR.to_le_bytes(),
            large_safe_prime_length: LARGE_SAFE_PRIME_LITTLE_ENDIAN.len() as u8,
            large_safe_prime: LARGE_SAFE_PRIME_LITTLE_ENDIAN.into(),
            salt: *proof.salt(),
            crc_salt: VERSION_CHALLENGE,
            security_flags: 0,
        };
        self.state = ClientState::OnChallenge(Some(proof));
        Ok(Box::new(response))
    }

    pub async fn handle_logon_proof(&mut self, data: &BytesMut) -> Result<OkType, RequestResult> {
        let (_, client_proof) =
            LogonProofRequest::from_bytes((&data, 0)).map_err(|_| RequestResult {
                cmd: Opcode::AuthLogonProof,
                protocol_version: None,
                result: AuthResult::WowFailDisconnected,
            })?;
        trace!(
            "[{:?}] LogonProofRequest {:?}",
            self.stream.peer_addr(),
            &client_proof
        );
        if let ClientState::OnChallenge(proof) = &mut self.state {
            let (server, server_proof) = proof
                .take()
                .unwrap() // always Some
                .into_server(
                    PublicKey::from_le_bytes(&client_proof.client_public_key).map_err(|_| {
                        RequestResult {
                            cmd: Opcode::AuthLogonProof,
                            protocol_version: None,
                            result: AuthResult::WowFailDisconnected,
                        }
                    })?,
                    client_proof.client_proof,
                )
                .map_err(|_| RequestResult {
                    cmd: Opcode::AuthLogonProof,
                    protocol_version: None,
                    result: AuthResult::WowFailIncorrectPassword,
                })?;
            self.state = ClientState::LoggedIn(Some(server));
            Ok(Box::new(LogonProofResponse {
                result: RequestResult {
                    cmd: Opcode::AuthLogonProof,
                    protocol_version: None,
                    result: AuthResult::WowSuccess,
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
                result: AuthResult::WowFailDisconnected,
            })
        }
    }

    pub async fn handle_realm_list(&mut self) -> Result<OkType, RequestResult> {
        if let ClientState::LoggedIn(_) = &self.state {
            trace!("[{:?}] Got RealmList request", self.stream.peer_addr());
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
            response.size = (response.to_bytes().unwrap().len() - 3) as u16;
            Ok(Box::new(response))
        } else {
            Err(RequestResult {
                cmd: Opcode::AuthLogonProof,
                protocol_version: None,
                result: AuthResult::WowFailDisconnected,
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
