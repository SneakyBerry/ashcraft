use deku::prelude::*;

#[derive(Debug, Clone, DekuWrite)]
#[deku(type = "u8")]
#[repr(u8)]
pub enum Expansions {
    ExpansionClassic = 0,
    ExpansionTheBurningCrusade = 1,
    ExpansionWrathOfTheLichKing = 2,
    ExpansionCataclysm = 3,
    ExpansionMistsOfPandaria = 4,
    ExpansionWarlordsOfDraenor = 5,
    ExpansionLegion = 6,
    ExpansionBattleForAzeroth = 7,
    ExpansionShadowlands = 8,
}
