use crate::realmlist::json::realm_list::{
    ClientVersion, IpAddress, RealmCharacterCountEntry, RealmCharacterCountList, RealmEntry,
    RealmIpAddressFamily, RealmListServerIpAddresses, RealmListUpdates, RealmState,
};
use crate::Server;
use flate2::write::ZlibEncoder;
use flate2::{Compression, FlushCompress};
use prost::Message;
use protocol::bgs::protocol::game_utilities::v1::{
    ClientRequest, ClientResponse, GameUtilitiesService, GetAllValuesForAttributeRequest,
    GetAllValuesForAttributeResponse,
};
use protocol::bgs::protocol::{Attribute, Variant};
use protocol::errors::WowRpcResponse;
use protocol::errors::WowRpcResponse::NotImplemented;
use serde::Serialize;
use serde_json::to_string;
use std::io::Write;
use rand::Rng;

fn compress<T>(name: &str, data: &T) -> Vec<u8>
where
    T: Serialize,
{
    let payload = format!("{}:{}\0", name, to_string(data).unwrap());
    let mut out = Vec::new();
    out.extend_from_slice(&(payload.as_bytes().len() as u32).to_le_bytes());
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write(payload.as_bytes());
    out.extend(encoder.finish().unwrap());
    out
}

#[async_trait::async_trait]
impl GameUtilitiesService for Server {
    async fn process_client_request(
        &mut self,
        request: ClientRequest,
    ) -> Result<ClientResponse, WowRpcResponse> {
        for attr in &request.attribute {
            return match attr {
                attr if attr.name == "Command_RealmListTicketRequest_v1_b9" => Ok(ClientResponse {
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
                }),
                attr if attr.name == "Command_LastCharPlayedRequest_v1_b9" => {
                    Ok(ClientResponse { attribute: vec![] })
                }
                attr if attr.name == "Command_RealmListRequest_v1_b9" => {
                    let rl = RealmListUpdates {
                        updates: vec![RealmState {
                            update: Some(RealmEntry {
                                wow_realm_address: 16842753,
                                cfg_timezones_id: 1,
                                population_state: 1,
                                cfg_categories_id: 1,
                                version: ClientVersion {
                                    version_major: 9,
                                    version_minor: 2,
                                    version_revision: 0,
                                    version_build: 42698,
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
                            wow_realm_address: 16842753,
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
                attr if attr.name == "Command_RealmJoinRequest_v1_b9" => {
                    let resp = RealmListServerIpAddresses {
                        families: vec![RealmIpAddressFamily {
                            family: 1,
                            addresses: vec![IpAddress {
                                ip: "127.0.0.1".to_string(),
                                port: 9900,
                            }],
                        }],
                    };

                    Ok(ClientResponse {
                        attribute: vec![
                            Attribute {
                                name: "Param_RealmJoinTicket".to_owned(),
                                value: Variant {
                                    bool_value: None,
                                    int_value: None,
                                    float_value: None,
                                    string_value: None,
                                    blob_value: Some(b"Handlefa1".to_vec()),
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
                                    blob_value: Some(compress(
                                        "JSONRealmListServerIPAddresses",
                                        &resp,
                                    )),
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
                                    blob_value: Some(rand::thread_rng().gen::<[u8; 32]>().to_vec()),
                                    message_value: None,
                                    fourcc_value: None,
                                    uint_value: None,
                                    entity_id_value: None,
                                },
                            },
                        ],
                    })
                }
                _ => Err(NotImplemented),
            };
        }
        Err(NotImplemented)
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
