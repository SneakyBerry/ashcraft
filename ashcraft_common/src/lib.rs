use ashcraft_database::redis::Storable;
use std::collections::HashMap;

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
    pub session_key: Vec<u8>,
}

impl Storable for Account {
    fn key_prefix() -> &'static str {
        "account"
    }
}
