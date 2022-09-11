use crate::realmlist::json::realm_list::{
    ClientVersion, IpAddress, RealmCharacterCountEntry, RealmCharacterCountList, RealmEntry,
    RealmIpAddressFamily, RealmListServerIpAddresses, RealmListTicketClientInformation,
    RealmListTicketIdentity, RealmListUpdates, RealmState,
};
use crate::Server;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use rand::Rng;
use rustycraft_common::Account;
use rustycraft_protocol::bgs::protocol::game_utilities::v1::{
    ClientRequest, ClientResponse, GameUtilitiesService, GetAllValuesForAttributeRequest,
    GetAllValuesForAttributeResponse,
};
use rustycraft_protocol::bgs::protocol::{Attribute, Variant};
use rustycraft_protocol::rpc_responses::WowRpcResponse;
use rustycraft_protocol::rpc_responses::WowRpcResponse::NotImplemented;
use serde::Serialize;
use serde_json::to_string;
use std::io::Write;

fn compress<T>(name: &str, data: &T) -> Vec<u8>
where
    T: Serialize,
{
    let payload = format!("{}:{}\0", name, to_string(data).unwrap());
    let mut out = Vec::new();
    out.extend_from_slice(&(payload.as_bytes().len() as u32).to_le_bytes());
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write(payload.as_bytes()).unwrap();
    out.extend(encoder.finish().unwrap());
    out
}

fn extract_json_from_blob(blob: Vec<u8>) -> String {
    let blob = String::from_utf8(blob).unwrap();
    let sep_idx = blob.find(":").unwrap();
    let blob_ready = &blob[sep_idx + 1..];
    blob_ready[..blob_ready.len() - 1].to_owned()
}

impl Server {
    async fn handle_realm_list_ticket_request(
        &mut self,
        request: ClientRequest,
    ) -> Result<ClientResponse, WowRpcResponse> {
        let param_identity = request.get_param("Param_Identity").unwrap();
        let param_identity_blob_ready =
            extract_json_from_blob(param_identity.blob_value.as_ref().unwrap().to_vec());
        let _ticket_identity: RealmListTicketIdentity =
            serde_json::from_str(&param_identity_blob_ready).unwrap();
        let client_info = request.get_param("Param_ClientInfo").unwrap();
        let param_client_info_blob_ready =
            extract_json_from_blob(client_info.blob_value.as_ref().unwrap().to_vec());
        let abiba: RealmListTicketClientInformation =
            serde_json::from_str(&param_client_info_blob_ready).unwrap();
        self.client_secret = abiba.info.secret;
        Ok(ClientResponse {
            attribute: vec![Attribute {
                name: "Param_RealmListTicket".to_owned(),
                value: Variant {
                    bool_value: None,
                    int_value: None,
                    float_value: None,
                    string_value: None,
                    blob_value: Some(b"AuthRealmListTicket".to_vec()),
                    message_value: None,
                    fourcc_value: None,
                    uint_value: None,
                    entity_id_value: None,
                },
            }],
        })
    }

    async fn handle_last_char_played_request(
        &mut self,
        _request: ClientRequest,
    ) -> Result<ClientResponse, WowRpcResponse> {
        Ok(ClientResponse { attribute: vec![] })
    }

    async fn handle_realm_list_request(
        &mut self,
        _request: ClientRequest,
    ) -> Result<ClientResponse, WowRpcResponse> {
        let rl = RealmListUpdates {
            updates: vec![RealmState {
                update: Some(RealmEntry {
                    wow_realm_address: 1024,
                    cfg_timezones_id: 1,
                    population_state: 1,
                    cfg_categories_id: 1,
                    version: ClientVersion {
                        version_major: 9,
                        version_minor: 2,
                        version_revision: 0,
                        version_build: 43206,
                    },
                    cfg_realms_id: 1,
                    flags: 0,
                    name: "ABOBA".to_string(),
                    cfg_configs_id: 1,
                    cfg_languages_id: 1,
                }),
                deleting: false,
            }],
        };

        let cc = RealmCharacterCountList {
            counts: vec![RealmCharacterCountEntry {
                wow_realm_address: 1024,
                count: 0,
            }],
        };

        Ok(ClientResponse {
            attribute: vec![
                Attribute {
                    name: "Param_RealmList".to_string(),
                    value: Variant {
                        bool_value: None,
                        int_value: None,
                        float_value: None,
                        string_value: None,
                        blob_value: Some(compress("JSONRealmListUpdates", &rl)),
                        message_value: None,
                        fourcc_value: None,
                        uint_value: None,
                        entity_id_value: None,
                    },
                },
                Attribute {
                    name: "Param_CharacterCountList".to_owned(),
                    value: Variant {
                        bool_value: None,
                        int_value: None,
                        float_value: None,
                        string_value: None,
                        blob_value: Some(compress("JSONRealmCharacterCountList", &cc)),
                        message_value: None,
                        fourcc_value: None,
                        uint_value: None,
                        entity_id_value: None,
                    },
                },
            ],
        })
    }

    async fn handle_realm_join_request(
        &mut self,
        _request: ClientRequest,
    ) -> Result<ClientResponse, WowRpcResponse> {
        let resp = RealmListServerIpAddresses {
            families: vec![RealmIpAddressFamily {
                family: 1,
                addresses: vec![IpAddress {
                    ip: "127.0.0.1".to_string(),
                    port: 9900,
                }],
            }],
        };
        let ticket = uuid::Uuid::new_v4().to_string();
        let server_secret = rand::thread_rng().gen::<[u8; 32]>().to_vec();
        let acc_data = Account {
            server_secret: server_secret.clone(),
            client_secret: self.client_secret.clone(),
        };
        self.redis.set(&ticket.clone(), &acc_data).await.unwrap();
        Ok(ClientResponse {
            attribute: vec![
                Attribute {
                    name: "Param_RealmJoinTicket".to_owned(),
                    value: Variant {
                        bool_value: None,
                        int_value: None,
                        float_value: None,
                        string_value: None,
                        blob_value: Some(ticket.as_bytes().to_vec()),
                        message_value: None,
                        fourcc_value: None,
                        uint_value: None,
                        entity_id_value: None,
                    },
                },
                Attribute {
                    name: "Param_ServerAddresses".to_owned(),
                    value: Variant {
                        bool_value: None,
                        int_value: None,
                        float_value: None,
                        string_value: None,
                        blob_value: Some(compress("JSONRealmListServerIPAddresses", &resp)),
                        message_value: None,
                        fourcc_value: None,
                        uint_value: None,
                        entity_id_value: None,
                    },
                },
                Attribute {
                    name: "Param_JoinSecret".to_owned(),
                    value: Variant {
                        bool_value: None,
                        int_value: None,
                        float_value: None,
                        string_value: None,
                        blob_value: Some(server_secret),
                        message_value: None,
                        fourcc_value: None,
                        uint_value: None,
                        entity_id_value: None,
                    },
                },
            ],
        })
    }
}

#[async_trait::async_trait]
impl GameUtilitiesService for Server {
    async fn process_client_request(
        &mut self,
        request: ClientRequest,
    ) -> Result<ClientResponse, WowRpcResponse> {
        if request
            .get_param("Command_RealmListTicketRequest_v1_b9")
            .is_some()
        {
            self.handle_realm_list_ticket_request(request).await
        } else if request
            .get_param("Command_LastCharPlayedRequest_v1_b9")
            .is_some()
        {
            self.handle_last_char_played_request(request).await
        } else if request
            .get_param("Command_RealmListRequest_v1_b9")
            .is_some()
        {
            self.handle_realm_list_request(request).await
        } else if request
            .get_param("Command_RealmJoinRequest_v1_b9")
            .is_some()
        {
            self.handle_realm_join_request(request).await
        } else {
            Err(NotImplemented)
        }
    }

    async fn get_all_values_for_attribute(
        &mut self,
        _: GetAllValuesForAttributeRequest,
    ) -> Result<GetAllValuesForAttributeResponse, WowRpcResponse> {
        Ok(GetAllValuesForAttributeResponse {
            attribute_value: vec![Variant {
                bool_value: None,
                int_value: None,
                float_value: None,
                string_value: Some("1-1-0".to_owned()),
                blob_value: None,
                message_value: None,
                fourcc_value: None,
                uint_value: None,
                entity_id_value: None,
            }],
        })
    }
}
