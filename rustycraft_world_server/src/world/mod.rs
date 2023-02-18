use crate::core::app::get_app;
use crate::core::prelude::*;
use crate::session_handler::{Connection, ConnectionState};
use bevy_ecs::prelude::*;
use rustycraft_world_packets::prelude::*;
use std::fmt::Debug;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

const TICK_TIME: Duration = Duration::from_millis(1000 / 60);

macro_rules! try_send {
    ($commands:ident, $entity:ident, $connection:ident => $data:expr) => {
        // If we can't send any data to channel that means connection was closed.
        if let Err(_) = (*$connection).sender().send($data) {
            $commands.entity($entity).despawn();
            continue;
        }
    };
}
macro_rules! try_send_box {
    ($commands:tt, $entity:ident, $connection:ident => $data:expr) => {
        // If we can't send any data to channel that means connection was closed.
        try_send!($commands, $entity, $connection => Box::new($data))
    };
}

fn ashbringer_data() -> SmsgItemQuerySingle {
    let mut item_flags2 = ItemFlags2::default();
    item_flags2
        .set_item_flag2_bnet_account_trade_ok()
        .set_item_flag2_faction_alliance()
        .set_item_flag2_no_durability();
    SmsgItemQuerySingle {
        item_id: 13262,
        response: Some(ItemStats {
            class: ItemClass::Weapon(Weapon::Sword2),
            sound_override_subclass: -1,
            name: "Ashbringer SOBOROBON".to_string(),
            display_info_id: 23875,
            quality: ItemQuality::Legendary,
            flags: ItemFlags::default(),
            flags2: ItemFlags2::default(),
            buy_price: 732658,
            sell_price: 146531,
            inventory_type: InventoryTypeU32::WeaponTwoHands,
            allowable_class: 0,
            allowable_race: 0,
            item_level: 76,
            required_level: 60,
            required_skill: 0,
            required_skill_rank: 0,
            required_spell: 0,
            required_honor_rank: 0,
            required_city_rank: 0,
            required_reputation_faction: 0,
            required_reputation_rank: ReputationRank::Hated,
            max_count: 1,
            stackable: 1,
            container_slots: 0,
            stats_count: 0,
            item_stat: vec![],
            scaling_stat_distribution: 0,
            scaling_stat_value: 0,
            damage: [
                ItemDamageData {
                    min: 201.0,
                    max: 247.0,
                    damage_type: DamageSchool::Physical,
                },
                ItemDamageData {
                    min: 30.0,
                    max: 50.0,
                    damage_type: DamageSchool::Holy,
                },
            ],
            resistance: Default::default(),
            delay: 3000,
            ammo_type: 0,
            ranged_mod_range: 0.0,
            spells: [
                ItemSpellData {
                    spell_id: 18112,
                    spell_trigger: 2,
                    spell_charges: 0,
                    spell_cooldown: -1,
                    spell_category: 0,
                    spell_category_cooldown: -1,
                },
                ItemSpellData {
                    spell_id: 0,
                    spell_trigger: 0,
                    spell_charges: 0,
                    spell_cooldown: -1,
                    spell_category: 0,
                    spell_category_cooldown: -1,
                },
                ItemSpellData {
                    spell_id: 0,
                    spell_trigger: 0,
                    spell_charges: 0,
                    spell_cooldown: -1,
                    spell_category: 0,
                    spell_category_cooldown: -1,
                },
                ItemSpellData {
                    spell_id: 0,
                    spell_trigger: 0,
                    spell_charges: 0,
                    spell_cooldown: -1,
                    spell_category: 0,
                    spell_category_cooldown: -1,
                },
                ItemSpellData {
                    spell_id: 0,
                    spell_trigger: 0,
                    spell_charges: 0,
                    spell_cooldown: -1,
                    spell_category: 0,
                    spell_category_cooldown: -1,
                },
            ],
            bonding: ItemBondingType::WhenPickedUp,
            description: "abbobus".to_string(),
            page_text: 0,
            language_id: 0,
            page_material: 0,
            start_quest: 0,
            lock_id: 0,
            material: 1,
            sheath: Sheath::MainHand,
            random_property: 0,
            random_suffix: 0,
            block: 0,
            item_set: 0,
            max_durability: 145,
            area: 0,
            map: 0,
            bag_family: 0,
            totem_category: 0,
            socket: [ItemSocketData {
                color: SocketColor::Unknown,
                content: 0,
            }; 3],
            socket_bonus: 0,
            gem_properties: 0,
            required_disenchant_skill: 0,
            armor_damage_modifier: 0.0,
            duration: 0,
            item_limit_category: 0,
            holiday_id: 0,
        }),
    }
}

pub struct WorldHandler {
    incoming_connections: IncomingConnections,
    realm_server_sender: mpsc::UnboundedSender<Connection>,
}

type IncomingConnections = mpsc::UnboundedReceiver<Connection>;

impl WorldHandler {
    pub fn new(
        incoming_connections: IncomingConnections,
        realm_server_sender: mpsc::UnboundedSender<Connection>,
    ) -> Self {
        Self {
            incoming_connections,
            realm_server_sender,
        }
    }

    pub fn run(mut self) {
        let mut app = get_app(self.incoming_connections, self.realm_server_sender);

        info!("World server started");
        let mut sleep_time = Duration::default();
        loop {
            std::thread::sleep(sleep_time);
            let t = Instant::now();
            app.update();
            sleep_time = TICK_TIME.saturating_sub(t.elapsed());
        }
    }
}
