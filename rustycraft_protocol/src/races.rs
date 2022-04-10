use deku::prelude::*;

#[derive(Debug, Clone, DekuWrite)]
#[deku(type = "u8")]
#[repr(u8)]
pub enum Races {
    RaceNone = 0,          // SKIP
    RaceHuman = 1,         // TITLE Human
    RaceOrc = 2,           // TITLE Orc
    RaceDwarf = 3,         // TITLE Dwarf
    RaceNightelf = 4,      // TITLE Night Elf
    RaceUndeadPlayer = 5, // TITLE Undead
    RaceTauren = 6,        // TITLE Tauren
    RaceGnome = 7,         // TITLE Gnome
    RaceTroll = 8,         // TITLE Troll
    RaceGoblin = 9,        // TITLE Goblin
    RaceBloodelf = 10,     // TITLE Blood Elf
    RaceDraenei = 11,      // TITLE Draenei

    // RaceFelOrc = 12,
    // RaceNaga = 13,
    // RaceBroken = 14,
    // RaceSkeleton = 15,
    // RaceVrykul = 16,
    // RaceTuskarr = 17,
    // RaceForestTroll = 18,
    // RaceTaunka = 19,
    // RaceNorthrendSkeleton = 20,
    // RaceIceTroll = 21,

    RaceWorgen = 22, // TITLE Worgen

    // RaceGilnean = 23,

    RacePandarenNeutral = 24, // TITLE Pandaren DESCRIPTION Pandaren (Neutral)
    RacePandarenAlliance = 25, // TITLE Pandaren DESCRIPTION Pandaren (Alliance)
    RacePandarenHorde = 26,   // TITLE Pandaren DESCRIPTION Pandaren (Horde)
    RaceNightborne = 27,       // TITLE Nightborne
    RaceHighmountainTauren = 28, // TITLE Highmountain Tauren
    RaceVoidElf = 29,         // TITLE Void Elf
    RaceLightforgedDraenei = 30, // TITLE Lightforged Draenei
    RaceZandalariTroll = 31,  // TITLE Zandalari Troll
    RaceKulTiran = 32,        // TITLE Kul Tiran

    // RaceThinHuman = 33,

    RaceDarkIronDwarf = 34, // TITLE Dark Iron Dwarf DESCRIPTION Dark Iron Dwarf (RaceMask bit 11)
    RaceVulpera = 35,         // TITLE Vulpera DESCRIPTION Vulpera (RaceMask bit 12)
    RaceMagharOrc = 36,      // TITLE Mag'har Orc DESCRIPTION Mag'har Orc (RaceMask bit 13)
    RaceMechagnome = 37,      // TITLE Mechagnome DESCRIPTION Mechagnome (RaceMask bit 14)
}
