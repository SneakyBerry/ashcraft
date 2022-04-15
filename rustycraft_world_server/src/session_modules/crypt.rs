use crate::constants::{
    AUTH_CHECK_SEED, CLIENT_TO_SERVER_CONNECTION, ENCRYPTION_KEY_SEED, REALM_WIN_AUTH_SEED,
    SERVER_TO_CLIENT_CONNECTION, SESSION_KEY_SEED,
};
use crate::opcodes::OpcodeClient;
use crate::packets::auth::{AuthChallenge, AuthSession, EncryptedMode};
use crate::packets::ClientPacket;
use crate::utils::generate_session_key;
use crate::world_session::WorldClientSession;
use crate::OpcodeServer;
use bytes::BytesMut;
use deku::{DekuContainerRead, DekuContainerWrite};
use hmac::{Hmac, Mac};
use rand::Rng;
use rustycraft_common::Account;
use sha2::{Digest, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

type HmacSha256 = Hmac<Sha256>;

impl WorldClientSession {
    pub(crate) async fn initial_packets(&mut self) -> anyhow::Result<()> {
        self.client_socket_writer
            .write(SERVER_TO_CLIENT_CONNECTION)
            .await?;
        let mut buf = BytesMut::with_capacity(CLIENT_TO_SERVER_CONNECTION.len());
        self.client_socket_reader.read_buf(&mut buf).await?;
        if buf.to_vec() != CLIENT_TO_SERVER_CONNECTION.to_vec() {
            return Err(anyhow!("Bad response from client."));
        };
        Ok(())
    }

    pub(crate) async fn init_encryption_state(&mut self) -> anyhow::Result<()> {
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

            let mut encryption_key_hasher = HmacSha256::new_from_slice(&self.session_key).unwrap();
            encryption_key_hasher.update(&session_pkt.local_challenge);
            encryption_key_hasher.update(&self.server_challenge);
            encryption_key_hasher.update(&ENCRYPTION_KEY_SEED);
            let encryption_key = encryption_key_hasher.finalize().into_bytes();
            for (i, b) in encryption_key[..16].iter().enumerate() {
                self.encryption_key[i] = *b;
            }
            Ok(())
        } else {
            Err(anyhow!("Expected AuthSession packet, got: {:?}", packet))
        }
    }

    pub(crate) async fn enter_encrypted_mode(&mut self) -> anyhow::Result<()> {
        let encrypted_mode = EncryptedMode::new(&self.rsa, &self.encryption_key);
        self.write_to_socket(Box::new(encrypted_mode)).await?;
        let packet = self.read_client_packet().await?;
        if let ClientPacket::EnterEncryptedModeAck = packet {
            self.aes_companion.init(&self.encryption_key)?;
            Ok(())
        } else {
            Err(anyhow!("Expected AuthSession packet, got: {:?}", packet))
        }
    }
}
