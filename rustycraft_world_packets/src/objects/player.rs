use crate::guid::Guid;
use crate::objects::private;
use deku::prelude::*;

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Bytes1 {
    pub skin: u8,
    pub face: u8,
    pub hairstyle: u8,
    pub hair_color: u8,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Bytes2 {
    pub facial: u8,
    pub party: u8,
    pub bank_bag_slots: u8,
    pub rest_state: u8,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Bytes3 {
    pub gender: u8,
    pub inebriation: u8,
    pub pvp_title: u8,
    pub arena_faction: u8,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct QuestLogItem {
    pub id: u32,
    pub state: u32,
    pub counts1: u16,
    pub counts2: u16,
    pub counts3: u16,
    pub counts4: u16,
    pub time: u32,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct PlayerEnchantment {
    pub id: u32,
    pub permanent: u16,
    pub temporary: u16,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
pub struct Bytes4 {
    pub flags: u8,
    pub raf_grantable_level: u8,
    pub action_bar_toggles: u8,
    pub lifetime_max_pvp_rank: u8,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct PlayerFieldBytes2Offsets {
    pub override_spells_id: u16,
    pub ignore_power_regen_prediction_mask: u8,
    pub aura_vision: u8,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum PlayerFlags {
    GroupLeader = 0x00000001,
    Afk = 0x00000002,
    Dnd = 0x00000004,
    Gm = 0x00000008,
    Ghost = 0x00000010,
    Resting = 0x00000020,
    Unk6 = 0x00000040,
    Unk7 = 0x00000080,         // pre-3.0.3 FLAGS_FFA_PVP flag for FFA PVP state
    ContestedPvp = 0x00000100, // Player has been involved in a PvP combat and will be attacked by contested guards
    InPvp = 0x00000200,
    HideHelm = 0x00000400,
    HideCloak = 0x00000800,
    PlayedLongTime = 0x00001000, // played long time
    PlayedTooLong = 0x00002000,  // played too long time
    IsOutOfBounds = 0x00004000,
    Developer = 0x00008000,     // <Dev> prefix for something?
    Unk16 = 0x00010000,         // pre-3.0.3 FLAGS_SANCTUARY flag for player entered sanctuary
    TaxiBenchmark = 0x00020000, // taxi benchmark mode (on/off) (2.0.1)
    PvpTimer = 0x00040000,      // 3.0.2, pvp timer active (after you disable pvp manually)
    Uber = 0x00080000,
    Unk20 = 0x00100000,
    Unk21 = 0x00200000,
    Commentator2 = 0x00400000,
    PlayerAllowOnlyAbility = 0x00800000, // used by bladestorm and killing spree, allowed only spells with SPELL_ATTR0_REQ_AMMO, SPELL_EFFECT_ATTACK, checked only for active player
    Unk24 = 0x01000000,                  // disabled all melee ability on tab include autoattack
    NoXpGain = 0x02000000,
    Unk26 = 0x04000000,
    Unk27 = 0x08000000,
    Unk28 = 0x10000000,
    Unk29 = 0x20000000,
    Unk30 = 0x40000000,
    Unk31 = 0x80000000,
}

#[derive(Debug, Clone, Copy)]
pub enum EquipmentSlots
// 19 slots
{
    Head = 0,
    Neck = 1,
    Shoulders = 2,
    Body = 3,
    Chest = 4,
    Waist = 5,
    Legs = 6,
    Feet = 7,
    Wrists = 8,
    Hands = 9,
    Finger1 = 10,
    Finger2 = 11,
    Trinket1 = 12,
    Trinket2 = 13,
    Back = 14,
    Mainhand = 15,
    Offhand = 16,
    Ranged = 17,
    Tabard = 18,
    End = 19,
}

#[derive(Debug, Clone, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct Rune {
    pub blood: u32,
    pub unholy: u32,
    pub frost: u32,
    pub death: u32,
}

pub trait Player: private::Object<0x0094> {
    fn set_player_duel_arbiter(&mut self, val: Guid) -> &mut Self {
        self.set_value(val, 0x0000)
    }
    fn get_player_duel_arbiter(&self) -> Option<Guid> {
        self.get_value(0x0000)
    }
    fn set_player_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x0002, mask)
    }
    fn unset_player_flags(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x0002, !mask)
    }
    fn get_player_flags(&mut self) -> Option<u32> {
        self.get_value(0x0002)
    }
    fn set_player_guild_id(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0003)
    }
    fn get_player_guild_id(&self) -> Option<u32> {
        self.get_value(0x0003)
    }

    fn set_player_guild_rank(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0004)
    }
    fn get_player_guild_rank(&self) -> Option<u32> {
        self.get_value(0x0004)
    }

    fn set_player_bytes(&mut self, val: Bytes1) -> &mut Self {
        self.set_value(val, 0x0005)
    }
    fn get_player_bytes(&self) -> Option<Bytes1> {
        self.get_value(0x0005)
    }

    fn set_player_bytes_2(&mut self, val: Bytes2) -> &mut Self {
        self.set_value(val, 0x0006)
    }
    fn get_player_bytes_2(&self) -> Option<Bytes2> {
        self.get_value(0x0006)
    }

    fn set_player_bytes_3(&mut self, val: Bytes3) -> &mut Self {
        self.set_value(val, 0x0007)
    }
    fn get_player_bytes_3(&self) -> Option<Bytes3> {
        self.get_value(0x0007)
    }

    fn set_player_duel_team(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0008)
    }
    fn get_player_duel_team(&self) -> Option<u32> {
        self.get_value(0x0008)
    }

    fn set_player_guild_timestamp(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0009)
    }
    fn get_player_guild_timestamp(&self) -> Option<u32> {
        self.get_value(0x0009)
    }

    fn set_player_quests(&mut self, val: QuestLogItem, index: usize) -> &mut Self {
        assert!(index < 25, "Index is out of range");
        self.set_value(val, 0x000A + index * 5)
    }
    fn get_player_quests(&self, index: usize) -> Option<QuestLogItem> {
        assert!(index < 25, "Index is out of range");
        self.get_value(0x000A + index * 5)
    }

    fn set_player_visible_items(&mut self, val: PlayerEnchantment, index: usize) -> &mut Self {
        assert!(index < 19, "Index is out of range");
        self.set_value(val, 0x0087 + index * 2)
    }
    fn get_player_visible_items(&self, index: usize) -> Option<PlayerEnchantment> {
        assert!(index < 19, "Index is out of range");
        self.get_value(0x0087 + index * 2)
    }

    fn set_player_chosen_title(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x00AD)
    }
    fn get_player_chosen_title(&self) -> Option<u32> {
        self.get_value(0x00AD)
    }

    fn set_player_fake_inebriation(&mut self, val: i32) -> &mut Self {
        self.set_value(val, 0x00AE)
    }
    fn get_player_fake_inebriation(&self) -> Option<i32> {
        self.get_value(0x00AE)
    }

    fn set_player_field_inv_slot_head(&mut self, val: Guid, index: usize) -> &mut Self {
        assert!(index < 23, "Index is out of range");
        self.set_value(val, 0x00B0 + index * 2)
    }
    fn get_player_field_inv_slot_head(&self, index: usize) -> Option<Guid> {
        assert!(index < 23, "Index is out of range");
        self.get_value(0x00B0 + index * 2)
    }

    fn set_player_field_pack_slot(&mut self, val: Guid, index: usize) -> &mut Self {
        assert!(index < 16, "Index is out of range");
        self.set_value(val, 0x00DE + index * 2)
    }
    fn get_player_field_pack_slot(&self, index: usize) -> Option<Guid> {
        assert!(index < 16, "Index is out of range");
        self.get_value(0x00DE + index * 2)
    }

    fn set_player_field_bank_slot(&mut self, val: Guid, index: usize) -> &mut Self {
        assert!(index < 28, "Index is out of range");
        self.set_value(val, 0x00FE + index * 2)
    }
    fn get_player_field_bank_slot(&self, index: usize) -> Option<Guid> {
        assert!(index < 28, "Index is out of range");
        self.get_value(0x00FE + index * 2)
    }

    fn set_player_field_bankbag_slot(&mut self, val: Guid, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x0136 + index * 2)
    }
    fn get_player_field_bankbag_slot(&self, index: usize) -> Option<Guid> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x0136 + index * 2)
    }

    fn set_player_field_vendorbuyback_slot(&mut self, val: Guid, index: usize) -> &mut Self {
        assert!(index < 12, "Index is out of range");
        self.set_value(val, 0x0144 + index * 2)
    }
    fn get_player_field_vendorbuyback_slot(&self, index: usize) -> Option<Guid> {
        assert!(index < 12, "Index is out of range");
        self.get_value(0x0144 + index * 2)
    }

    fn set_player_field_keyring_slot(&mut self, val: Guid, index: usize) -> &mut Self {
        assert!(index < 32, "Index is out of range");
        self.set_value(val, 0x015C + index * 2)
    }
    fn get_player_field_keyring_slot(&self, index: usize) -> Option<Guid> {
        assert!(index < 32, "Index is out of range");
        self.get_value(0x015C + index * 2)
    }

    fn set_player_field_currencytoken_slot(&mut self, val: Guid, index: usize) -> &mut Self {
        assert!(index < 32, "Index is out of range");
        self.set_value(val, 0x019C + index * 2)
    }
    fn get_player_field_currencytoken_slot(&self, index: usize) -> Option<Guid> {
        assert!(index < 32, "Index is out of range");
        self.get_value(0x019C + index * 2)
    }

    fn set_player_farsight(&mut self, val: Guid) -> &mut Self {
        self.set_value(val, 0x01DC)
    }
    fn get_player_farsight(&self) -> Option<Guid> {
        self.get_value(0x01DC)
    }

    fn set_player_field_known_titles(&mut self, val: u64, index: usize) -> &mut Self {
        assert!(index < 3, "Index is out of range");
        self.set_value(val, 0x01DE + index * 2)
    }
    fn get_player_field_known_titles(&self, index: usize) -> Option<u64> {
        assert!(index < 3, "Index is out of range");
        self.get_value(0x01DE + index * 2)
    }

    fn set_player_field_known_currencies(&mut self, val: u64) -> &mut Self {
        self.set_value(val, 0x01E4)
    }
    fn get_player_field_known_currencies(&self) -> Option<u64> {
        self.get_value(0x01E4)
    }

    fn set_player_xp(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x01E6)
    }
    fn get_player_xp(&self) -> Option<u32> {
        self.get_value(0x01E6)
    }

    fn set_player_next_level_xp(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x01E7)
    }
    fn get_player_next_level_xp(&self) -> Option<u32> {
        self.get_value(0x01E7)
    }

    fn set_player_skills_info(&mut self, val: (u16, u16), index: usize) -> &mut Self {
        assert!(index < 384, "Index is out of range");
        self.set_value(val, 0x01E8 + index)
    }
    fn get_player_skills_info(&self, index: usize) -> Option<(u16, u16)> {
        assert!(index < 384, "Index is out of range");
        self.get_value(0x01E8 + index)
    }

    fn set_player_character_points(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 2, "Index is out of range");
        self.set_value(val, 0x0368 + index)
    }
    fn get_player_character_points(&self, index: usize) -> Option<u32> {
        assert!(index < 2, "Index is out of range");
        self.get_value(0x0368 + index)
    }

    fn set_player_track_creatures(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x036A, mask)
    }
    fn unset_player_track_creatures(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x036A, !mask)
    }
    fn get_player_track_creatures(&mut self) -> Option<u32> {
        self.get_value(0x036A)
    }
    fn set_player_track_resources(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x036B, mask)
    }
    fn unset_player_track_resources(&mut self, mask: u32) -> &mut Self {
        self.apply_and(0x036B, !mask)
    }
    fn get_player_track_resources(&mut self) -> Option<u32> {
        self.get_value(0x036B)
    }

    //
    fn set_player_block_percentage(&mut self, val: f32) -> &mut Self {
        self.set_value(val, 0x036C)
    }
    fn get_player_block_percentage(&self) -> Option<f32> {
        self.get_value(0x036C)
    }

    fn set_player_dodge_percentage(&mut self, val: f32) -> &mut Self {
        self.set_value(val, 0x036D)
    }
    fn get_player_dodge_percentage(&self) -> Option<f32> {
        self.get_value(0x036D)
    }

    fn set_player_parry_percentage(&mut self, val: f32) -> &mut Self {
        self.set_value(val, 0x036E)
    }
    fn get_player_parry_percentage(&self) -> Option<f32> {
        self.get_value(0x036E)
    }

    //
    fn set_player_expertise(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x036F)
    }
    fn get_player_expertise(&self) -> Option<u32> {
        self.get_value(0x036F)
    }

    fn set_player_offhand_expertise(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0370)
    }
    fn get_player_offhand_expertise(&self) -> Option<u32> {
        self.get_value(0x0370)
    }

    //
    fn set_player_crit_percentage(&mut self, val: f32) -> &mut Self {
        self.set_value(val, 0x0371)
    }
    fn get_player_crit_percentage(&self) -> Option<f32> {
        self.get_value(0x0371)
    }

    fn set_player_ranged_crit_percentage(&mut self, val: f32) -> &mut Self {
        self.set_value(val, 0x0372)
    }
    fn get_player_ranged_crit_percentage(&self) -> Option<f32> {
        self.get_value(0x0372)
    }

    fn set_player_offhand_crit_percentage(&mut self, val: f32) -> &mut Self {
        self.set_value(val, 0x0373)
    }
    fn get_player_offhand_crit_percentage(&self) -> Option<f32> {
        self.get_value(0x0373)
    }

    fn set_player_spell_crit_percentage(&mut self, val: f32, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x0374 + index)
    }
    fn get_player_spell_crit_percentage(&self, index: usize) -> Option<f32> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x0374 + index)
    }

    fn set_player_shield_block(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x037B)
    }
    fn get_player_shield_block(&self) -> Option<u32> {
        self.get_value(0x037B)
    }

    fn set_player_shield_block_crit_percentage(&mut self, val: f32) -> &mut Self {
        self.set_value(val, 0x037C)
    }
    fn get_player_shield_block_crit_percentage(&self) -> Option<f32> {
        self.get_value(0x037C)
    }

    fn set_player_explored_zones(&mut self, mask: u32, index: usize) -> &mut Self {
        assert!(index < 128, "Index is out of range");
        self.apply_and(0x037D + index, mask)
    }
    fn unset_player_explored_zones(&mut self, mask: u32, index: usize) -> &mut Self {
        assert!(index < 128, "Index is out of range");
        self.apply_and(0x037D + index, !mask)
    }
    fn get_player_explored_zones(&self, index: usize) -> Option<u32> {
        assert!(index < 128, "Index is out of range");
        self.get_value(0x037D + index)
    }

    //
    fn set_player_rest_state_experience(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x03FD)
    }
    fn get_player_rest_state_experience(&self) -> Option<u32> {
        self.get_value(0x03FD)
    }

    fn set_player_field_coinage(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x03FE)
    }
    fn get_player_field_coinage(&self) -> Option<u32> {
        self.get_value(0x03FE)
    }

    fn set_player_field_mod_damage_done_pos(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x03FF + index)
    }
    fn get_player_field_mod_damage_done_pos(&self, index: usize) -> Option<u32> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x03FF + index)
    }
    // TODO: map it
    fn set_player_field_mod_damage_done_neg(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x0406 + index)
    }
    fn get_player_field_mod_damage_done_neg(&self, index: usize) -> Option<u32> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x0406 + index)
    }

    fn set_player_field_mod_damage_done_pct(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 7, "Index is out of range");
        self.set_value(val, 0x040D + index)
    }
    fn get_player_field_mod_damage_done_pct(&self, index: usize) -> Option<u32> {
        assert!(index < 7, "Index is out of range");
        self.get_value(0x040D + index)
    }

    fn set_player_field_mod_healing_done_pos(&mut self, val: f32) -> &mut Self {
        self.set_value(val, 0x0414)
    }
    fn get_player_field_mod_healing_done_pos(&self) -> Option<f32> {
        self.get_value(0x0414)
    }

    fn set_player_field_mod_healing_pct(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0415)
    }
    fn get_player_field_mod_healing_pct(&self) -> Option<u32> {
        self.get_value(0x0415)
    }

    fn set_player_field_mod_healing_done_pct(&mut self, val: f32) -> &mut Self {
        self.set_value(val, 0x0416)
    }
    fn get_player_field_mod_healing_done_pct(&self) -> Option<f32> {
        self.get_value(0x0416)
    }

    fn set_player_field_mod_target_resistance(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0417)
    }
    fn get_player_field_mod_target_resistance(&self) -> Option<u32> {
        self.get_value(0x0417)
    }

    fn set_player_field_mod_target_physical_resistance(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0418)
    }
    fn get_player_field_mod_target_physical_resistance(&self) -> Option<u32> {
        self.get_value(0x0418)
    }

    //
    fn set_player_field_bytes(&mut self, val: Bytes4) -> &mut Self {
        self.set_value(val, 0x0419)
    }
    fn get_player_field_bytes(&self) -> Option<Bytes4> {
        self.get_value(0x0419)
    }

    //
    fn set_player_ammo_id(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x041A)
    }
    fn get_player_ammo_id(&self) -> Option<u32> {
        self.get_value(0x041A)
    }

    fn set_player_self_res_spell(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x041B)
    }
    fn get_player_self_res_spell(&self) -> Option<u32> {
        self.get_value(0x041B)
    }

    fn set_player_field_pvp_medals(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x041C)
    }
    fn get_player_field_pvp_medals(&self) -> Option<u32> {
        self.get_value(0x041C)
    }

    fn set_player_field_buyback_price(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 12, "Index is out of range");
        self.set_value(val, 0x041D + index)
    }
    fn get_player_field_buyback_price(&self, index: usize) -> Option<u32> {
        assert!(index < 12, "Index is out of range");
        self.get_value(0x041D + index)
    }

    fn set_player_field_buyback_timestamp(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 12, "Index is out of range");
        self.set_value(val, 0x0429 + index)
    }
    fn get_player_field_buyback_timestamp(&self, index: usize) -> Option<u32> {
        assert!(index < 12, "Index is out of range");
        self.get_value(0x0429 + index)
    }

    fn set_player_field_kills(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0435)
    }
    fn get_player_field_kills(&self) -> Option<u32> {
        self.get_value(0x0435)
    }

    fn set_player_field_today_contribution(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0436)
    }
    fn get_player_field_today_contribution(&self) -> Option<u32> {
        self.get_value(0x0436)
    }

    fn set_player_field_yesterday_contribution(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0437)
    }
    fn get_player_field_yesterday_contribution(&self) -> Option<u32> {
        self.get_value(0x0437)
    }

    fn set_player_field_lifetime_honorable_kills(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0438)
    }
    fn get_player_field_lifetime_honorable_kills(&self) -> Option<u32> {
        self.get_value(0x0438)
    }

    //
    fn set_player_field_bytes2(&mut self, val: PlayerFieldBytes2Offsets) -> &mut Self {
        self.set_value(val, 0x0439)
    }
    fn get_player_field_bytes2(&self) -> Option<PlayerFieldBytes2Offsets> {
        self.get_value(0x0439)
    }

    //
    fn set_player_field_watched_faction_index(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x043A)
    }
    fn get_player_field_watched_faction_index(&self) -> Option<u32> {
        self.get_value(0x043A)
    }

    fn set_player_field_combat_rating(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 25, "Index is out of range");
        self.set_value(val, 0x043B + index)
    }
    fn get_player_field_combat_rating(&self, index: usize) -> Option<u32> {
        assert!(index < 25, "Index is out of range");
        self.get_value(0x043B + index)
    }

    fn set_player_field_arena_team_info(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 21, "Index is out of range");
        self.set_value(val, 0x0454 + index)
    }
    fn get_player_field_arena_team_info(&self, index: usize) -> Option<u32> {
        assert!(index < 21, "Index is out of range");
        self.get_value(0x0454 + index)
    }

    fn set_player_field_honor_currency(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0469)
    }
    fn get_player_field_honor_currency(&self) -> Option<u32> {
        self.get_value(0x0469)
    }

    fn set_player_field_arena_currency(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x046A)
    }
    fn get_player_field_arena_currency(&self) -> Option<u32> {
        self.get_value(0x046A)
    }

    fn set_player_field_max_level(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x046B)
    }
    fn get_player_field_max_level(&self) -> Option<u32> {
        self.get_value(0x046B)
    }

    fn set_player_field_daily_quests(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 25, "Index is out of range");
        self.set_value(val, 0x046C + index)
    }
    fn get_player_field_daily_quests(&self, index: usize) -> Option<u32> {
        assert!(index < 25, "Index is out of range");
        self.get_value(0x046C + index)
    }

    fn set_player_rune_regen(&mut self, val: Rune) -> &mut Self {
        self.set_value(val, 0x0485)
    }
    fn get_player_rune_regen(&self) -> Option<Rune> {
        self.get_value(0x0485)
    }

    fn set_player_no_reagent_cost(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 3, "Index is out of range");
        self.set_value(val, 0x0489 + index)
    }
    fn get_player_no_reagent_cost(&self, index: usize) -> Option<u32> {
        assert!(index < 3, "Index is out of range");
        self.get_value(0x0489 + index)
    }

    fn set_player_field_glyph_slots(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 6, "Index is out of range");
        self.set_value(val, 0x048C + index)
    }
    fn get_player_field_glyph_slots(&self, index: usize) -> Option<u32> {
        assert!(index < 6, "Index is out of range");
        self.get_value(0x048C + index)
    }

    fn set_player_field_glyphs(&mut self, val: u32, index: usize) -> &mut Self {
        assert!(index < 6, "Index is out of range");
        self.set_value(val, 0x0492 + index)
    }
    fn get_player_field_glyphs(&self, index: usize) -> Option<u32> {
        assert!(index < 6, "Index is out of range");
        self.get_value(0x0492 + index)
    }

    fn set_player_glyphs_enabled(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0498)
    }
    fn get_player_glyphs_enabled(&self) -> Option<u32> {
        self.get_value(0x0498)
    }

    fn set_player_pet_spell_power(&mut self, val: u32) -> &mut Self {
        self.set_value(val, 0x0499)
    }
    fn get_player_pet_spell_power(&self) -> Option<u32> {
        self.get_value(0x0499)
    }
}
