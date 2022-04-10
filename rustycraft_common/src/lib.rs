use std::collections::HashMap;

pub struct Realm {
    name: String,
    address: String,
    flags: u32,
    locale: u32,
}

pub struct Character {
    nickname: String,
    realm: Realm,
}

pub struct BattleNetAccount {
    email: String,
    username: String,
    characters: HashMap<Realm, Character>,
    is_authenticated: bool,
}
