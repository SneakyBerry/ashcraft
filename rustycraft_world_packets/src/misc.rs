use deku::prelude::*;

use rustycraft_derive::ServerPacket;

use crate::prelude::{Guid, Map, Vector3d};

pub mod client {
    use super::*;

    #[derive(Debug, Clone, DekuRead)]
    pub struct CompleteCinematic;

    #[derive(Debug, Clone, DekuRead)]
    pub struct NextCinematicCamera;

    #[derive(Debug, Clone, DekuRead)]
    pub struct CompleteMovie;

    #[derive(Debug, Clone, DekuRead)]
    pub struct OpeningCinematic;

    #[derive(Debug, Clone, DekuRead)]
    pub struct RandomRollClient {
        pub min: u32,
        pub max: u32,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct TogglePvP {
        pub enable: bool,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct WorldTeleport {
        pub time: u32,
        pub map_id: Map,
        pub pos: Vector3d,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct ReclaimCorpse {
        pub corpse: Guid,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct RepopRequest {
        pub check_instance: bool,
    }

    #[derive(Debug, Clone, DekuRead)]
    pub struct ResurrectResponse {
        pub resurrected_by: Guid,
        pub response: bool,
    }
}

pub mod server {
    use crate::prelude::{Area, Opcode, PackedGuid, PowerIndexed, StatsIndexed};

    use super::*;

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgBindPointUpdate)]
    pub struct BindPointUpdate {
        pub bind_position: Vector3d,
        pub bind_map_id: Map,
        pub bind_area_id: Area,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgPlayerBound)]
    pub struct PlayerBound {
        pub binder: Guid,
        pub bind_area_id: Area,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgBinderConfirm)]
    pub struct BinderConfirm {
        pub unit: Guid,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgStartMirrorTimer)]
    pub struct StartMirrorTimer {
        pub timer: u32,
        pub value: u32,
        pub max_value: u32,
        pub scale: i32,
        pub is_paused: bool,
        pub spell_id: u32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgPauseMirrorTimer)]
    pub struct PauseMirrorTimer {
        pub timer: u32,
        pub is_paused: bool,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgStopMirrorTimer)]
    pub struct StopMirrorTimer {
        pub timer: u32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgInvalidatePlayer)]
    pub struct InvalidatePlayer {
        pub guid: Guid,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgLoginSetTimeSpeed)]
    pub struct LoginSetTimeSpeed {
        pub game_time: u32, // packed time
        pub new_speed: f32,
        pub game_time_holiday_offset: u32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgTriggerMovie)]
    pub struct TriggerMovie {
        pub movie_id: u32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgTriggerCinematic)]
    pub struct TriggerCinematic {
        pub cinematic_id: u32,
    }

    #[derive(Debug, Clone, DekuWrite)]
    #[deku(type = "u32")]
    pub enum WeatherState {
        Fine = 0,
        Fog = 1,
        Drizzle = 2,
        LightRain = 3,
        MediumRain = 4,
        HeavyRain = 5,
        LightSnow = 6,
        MediumSnow = 7,
        HeavySnow = 8,
        LightSandstorm = 22,
        MediumSandstorm = 41,
        HeavySandstorm = 42,
        Thunders = 86,
        Blackrain = 90,
        Blacksnow = 106,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgWeather)]
    pub struct Weather {
        pub weather_state: WeatherState,
        pub intensity: f32,
        pub abrupt: bool,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgLevelUpInfo)]
    pub struct LevelUpInfo {
        pub level: u32,
        pub health_delta: u32,
        pub power_delta: PowerIndexed<u32>,
        pub stat_delta: StatsIndexed<u32>,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgPlayMusic)]
    pub struct PlayMusic {
        pub sound_kit_id: u32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgPlayObjectSound)]
    pub struct PlayObjectSound {
        pub source_guid: Guid,
        pub sound_kit_id: u32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgPlaySound)]
    pub struct PlaySound {
        pub sound_kit_id: u32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgCrossedInebriationThreshold)]
    pub struct CrossedInebriationThreshold {
        pub guid: Guid,
        pub threshold: u32,
        pub item_id: u32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgOverrideLight)]
    pub struct OverrideLight {
        pub area_light_id: i32,
        pub override_light_id: i32,
        pub transition_ms: i32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::MsgRandomRoll)]
    pub struct RandomRoll {
        pub min: u32,
        pub max: u32,
        pub result: u32,
        pub roller: Guid,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgWorldStateUiTimerUpdate)]
    pub struct UITime {
        pub time: u32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgCorpseReclaimDelay)]
    pub struct CorpseReclaimDelay {
        pub remaining: u32,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgDeathReleaseLoc)]
    pub struct DeathReleaseLoc {
        pub map: Map,
        pub position: Vector3d,
    }

    #[derive(Debug, Clone, DekuWrite, ServerPacket)]
    #[opcode(Opcode::SmsgPreResurrect)]
    pub struct PreResurrect {
        pub guid: PackedGuid,
    }
}
