use crate::constants::{
    AUTH_CHECK_SEED, CLIENT_TO_SERVER_CONNECTION, CONTINUED_SESSION_SEED, ENCRYPTION_KEY_SEED,
    REALM_WIN_AUTH_SEED, SERVER_TO_CLIENT_CONNECTION, SESSION_KEY_SEED,
};
use crate::crypt::{AES128Companion, INITIALIZED_RSA};
use crate::opcodes::OpcodeClient;
use crate::packets::auth::{AuthChallenge, AuthSession, EncryptedMode};
use crate::packets::{ClientPacket, ServerPacket};
use crate::realm_server::{Error, RealmServer};
use crate::utils::generate_session_key;
use crate::world_session::WorldSession;
use crate::{IntoServerPacket, OpcodeServer};
use bytes::BytesMut;
use deku::{DekuContainerRead, DekuContainerWrite};
use hmac::{Hmac, Mac};
use rand::Rng;
use rustycraft_common::Account;
use rustycraft_database::redis::Storable;
use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionKey(pub Vec<u8>);

impl Storable for SessionKey {
    fn key_prefix() -> &'static str {
        "session_key"
    }
}

#[async_trait::async_trait]
pub trait EncryptedModeTrait {
    fn get_writer(&mut self) -> &mut WriteHalf<TcpStream>;
    fn get_reader(&mut self) -> &mut ReadHalf<TcpStream>;
    fn get_aes_companion(&mut self) -> &mut AES128Companion;
    fn get_addr(&self) -> &SocketAddr;
    fn get_encryption_key(&self) -> &[u8; 16];
    async fn init_encryption_state(&mut self) -> Result<(), Error>;
    async fn read_client_packet(&mut self) -> Result<ClientPacket, Error> {
        let reader = self.get_reader();
        let pkt_size = reader.read_u32_le().await?;
        let mut aes_token = Vec::with_capacity(12);
        reader.read_buf(&mut aes_token).await?;
        let mut packet = BytesMut::with_capacity(pkt_size as usize);
        reader.read_buf(&mut packet).await?;

        let decrypted = self.get_aes_companion().decrypt(&packet, &aes_token)?;
        let result = ClientPacket::try_from(decrypted)?;
        trace!(
            "[ {:?} ] New packet from client: {:?}",
            self.get_addr(),
            &result
        );
        Ok(result)
    }

    async fn write_to_socket(&mut self, data: Box<dyn IntoServerPacket>) -> anyhow::Result<()> {
        trace!("[ {:?} ]  Plain packet: {:?}", self.get_addr(), &data);
        let result = data.serialize()?;
        let encrypted = self.get_aes_companion().encrypt(&result)?;
        let pkt = ServerPacket::new(encrypted.aes_tag, encrypted.cipher_text);
        self.get_writer().write(&pkt.serialize()?).await?;
        Ok(())
    }

    async fn initial_packets(&mut self) -> Result<(), Error> {
        self.get_writer().write(SERVER_TO_CLIENT_CONNECTION).await?;
        let mut buf = BytesMut::with_capacity(CLIENT_TO_SERVER_CONNECTION.len());
        self.get_reader().read_buf(&mut buf).await?;
        if buf.to_vec() != CLIENT_TO_SERVER_CONNECTION.to_vec() {
            return Err(Error::Anyhow(anyhow!("Bad response from client.")));
        };
        self.init_encryption_state().await
    }
    async fn enter_encrypted_mode(&mut self) -> Result<(), Error> {
        let encryption_key = self.get_encryption_key().clone();
        let encrypted_mode = EncryptedMode::new(&INITIALIZED_RSA, &encryption_key);
        self.write_to_socket(Box::new(encrypted_mode)).await?;
        let packet = self.read_client_packet().await?;
        if let ClientPacket::EnterEncryptedModeAck = packet {
            self.get_aes_companion().init(&encryption_key)?;
            Ok(())
        } else {
            Err(Error::Anyhow(anyhow!(
                "Expected AuthSession packet, got: {:?}",
                packet
            )))
        }
    }
}

#[async_trait::async_trait]
impl EncryptedModeTrait for RealmServer {
    fn get_writer(&mut self) -> &mut WriteHalf<TcpStream> {
        &mut self.client_socket_writer
    }

    fn get_reader(&mut self) -> &mut ReadHalf<TcpStream> {
        &mut self.client_socket_reader
    }

    fn get_aes_companion(&mut self) -> &mut AES128Companion {
        &mut self.aes_companion
    }

    fn get_addr(&self) -> &SocketAddr {
        &self.addr
    }

    fn get_encryption_key(&self) -> &[u8; 16] {
        &self.encryption_key
    }

    async fn init_encryption_state(&mut self) -> Result<(), Error> {
        let dos_challenge = rand::thread_rng().gen();
        let challenge = AuthChallenge::new(self.server_challenge.clone(), dos_challenge);
        self.write_to_socket(Box::new(challenge)).await?;

        let packet = self.read_client_packet().await?;
        if let ClientPacket::AuthSession(session_pkt) = packet {
            let acc: Account = self
                .redis
                .get(&session_pkt.realm_join_ticket)
                .await
                .unwrap();
            let mut session_secret = acc.client_secret;
            session_secret.extend(acc.server_secret);

            let mut key_hasher = sha2::Sha256::new();
            key_hasher.update(&session_secret);
            key_hasher.update(&REALM_WIN_AUTH_SEED);
            let digest_key_hash = key_hasher.finalize();

            let mut hmac_digester = HmacSha256::new_from_slice(&digest_key_hash).unwrap();
            hmac_digester.update(&session_pkt.local_challenge);
            hmac_digester.update(&self.server_challenge);
            hmac_digester.update(&AUTH_CHECK_SEED);
            let res = hmac_digester.finalize().into_bytes();
            assert_eq!(session_pkt.digest.as_slice(), &res[..24]);

            let mut key_data_hasher = sha2::Sha256::new();
            key_data_hasher.update(&session_secret);
            let key_data_hash = key_data_hasher.finalize();

            let mut session_key_hasher = HmacSha256::new_from_slice(&key_data_hash).unwrap();
            session_key_hasher.update(&self.server_challenge);
            session_key_hasher.update(&session_pkt.local_challenge);
            session_key_hasher.update(&SESSION_KEY_SEED);
            let session_key_seed = session_key_hasher.finalize().into_bytes();

            let session_key = generate_session_key::<Sha256>(&session_key_seed, 40);
            for (i, byte) in session_key.iter().enumerate() {
                self.session_key[i] = *byte;
            }
            println!("SESSION KEY: {:?}", &self.session_key);
            let s_key = SessionKey(self.session_key.to_vec());
            self.redis.set("0", &s_key).await?;
            let mut encryption_key_hasher = HmacSha256::new_from_slice(&self.session_key).unwrap();
            encryption_key_hasher.update(&session_pkt.local_challenge);
            encryption_key_hasher.update(&self.server_challenge);
            encryption_key_hasher.update(&ENCRYPTION_KEY_SEED);
            let encryption_key = encryption_key_hasher.finalize().into_bytes();
            for (i, b) in encryption_key[..16].iter().enumerate() {
                self.encryption_key[i] = *b;
            }
            self.enter_encrypted_mode().await
        } else {
            Err(Error::Anyhow(anyhow!(
                "Expected AuthSession packet, got: {:?}",
                packet
            )))
        }
    }
}

#[async_trait::async_trait]
impl EncryptedModeTrait for WorldSession {
    fn get_writer(&mut self) -> &mut WriteHalf<TcpStream> {
        &mut self.client_socket_writer
    }

    fn get_reader(&mut self) -> &mut ReadHalf<TcpStream> {
        &mut self.client_socket_reader
    }

    fn get_aes_companion(&mut self) -> &mut AES128Companion {
        &mut self.aes_companion
    }

    fn get_addr(&self) -> &SocketAddr {
        &self.addr
    }

    fn get_encryption_key(&self) -> &[u8; 16] {
        &self.encryption_key
    }

    async fn init_encryption_state(&mut self) -> Result<(), Error> {
        let dos_challenge = rand::thread_rng().gen();
        let challenge = AuthChallenge::new(self.server_challenge.clone(), dos_challenge);
        self.write_to_socket(Box::new(challenge)).await?;

        let packet = self.read_client_packet().await?;
        if let ClientPacket::AuthContinuedSession(session_pkt) = packet {
            let s_key: SessionKey = self.redis.get("0").await.unwrap();
            self.session_key = s_key.0.try_into().unwrap();

            let acc_id = session_pkt.raw & 0xFFFFFFFF;
            let conn_t = (session_pkt.raw >> 32) & 1;
            let key = session_pkt.raw >> 33;

            let mut hmac_digester = HmacSha256::new_from_slice(&self.session_key).unwrap();

            hmac_digester.update(&session_pkt.raw.to_le_bytes());
            hmac_digester.update(&session_pkt.local_challenge);
            hmac_digester.update(&self.server_challenge);
            hmac_digester.update(&CONTINUED_SESSION_SEED);
            let res = &hmac_digester.finalize().into_bytes()[..24];
            assert_eq!(session_pkt.digest.as_slice(), res);

            let mut session_key_digest = HmacSha256::new_from_slice(&self.session_key).unwrap();
            session_key_digest.update(&session_pkt.local_challenge);
            session_key_digest.update(&self.server_challenge);
            session_key_digest.update(&ENCRYPTION_KEY_SEED);
            self.encryption_key = session_key_digest.finalize().into_bytes().to_vec()[..16]
                .try_into()
                .unwrap();
            self.enter_encrypted_mode().await
        } else {
            Err(Error::Anyhow(anyhow!(
                "Expected AuthContinuedSession packet, got: {:?}",
                packet
            )))
        }
    }
}
