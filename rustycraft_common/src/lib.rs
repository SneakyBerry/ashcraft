use std::collections::HashMap;
use rustycraft_database::redis::Storable;

#[macro_use]
extern crate serde;

pub struct Realm {
    pub name: String,
    pub address: String,
    pub flags: u32,
    pub locale: u32,
}

pub struct Character {
    pub nickname: String,
    pub realm: Realm,
}

pub struct BattleNetAccount {
    pub email: String,
    pub username: String,
    pub characters: HashMap<Realm, Character>,
    pub is_authenticated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub server_secret: Vec<u8>,
    pub client_secret: Vec<u8>,
}

impl Storable for Account {
    fn key_prefix() -> &'static str {
        "account"
    }
}
