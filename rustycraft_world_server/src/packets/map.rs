use deku::prelude::*;

#[derive(Debug, DekuWrite, DekuRead)]
#[deku(type = "u32", endian = "little")]
pub enum Map {
    EasternKingdoms = 0x0,
    Kalimdor = 0x1,
    Testing = 0xd,
    ScottTest = 0x19,
    AlteracValley = 0x1e,
    ShadowfangKeep = 0x21,
    StormwindStockade = 0x22,
    Stormwindprison = 0x23,
    Deadmines = 0x24,
    AzsharaCrater = 0x25,
    CollinsTest = 0x2a,
    WailingCaverns = 0x2b,
    Monastery = 0x2c,
    RazorfenKraul = 0x2f,
    BlackfathomDeeps = 0x30,
    Uldaman = 0x46,
    Gnomeregan = 0x5a,
    SunkenTemple = 0x6d,
    RazorfenDowns = 0x81,
    EmeraldDream = 0xa9,
    ScarletMonastery = 0xbd,
    ZulFarrak = 0xd1,
    BlackrockSpire = 0xe5,
    BlackrockDepths = 0xe6,
    OnyxiasLair = 0xf9,
    OpeningOfTheDarkPortal = 0x10d,
    Scholomance = 0x121,
    ZulGurub = 0x135,
    Stratholme = 0x149,
    Maraudon = 0x15d,
    DeeprunTram = 0x171,
    RagefireChasm = 0x185,
    MoltenCore = 0x199,
    DireMaul = 0x1ad,
    AlliancePvpBarracks = 0x1c1,
    HordePvpBarracks = 0x1c2,
    DevelopmentLand = 0x1c3,
    BlackwingLair = 0x1d5,
    WarsongGulch = 0x1e9,
    RuinsOfAhnQiraj = 0x1fd,
    ArathiBasin = 0x211,
    Outland = 0x212,
    AhnQirajTemple = 0x213,
    Karazhan = 0x214,
    Naxxramas = 0x215,
    TheBattleForMountHyjal = 0x216,
    HellfireCitadelTheShatteredHalls = 0x21c,
    HellfireCitadelTheBloodFurnace = 0x21e,
    HellfireCitadelRamparts = 0x21f,
    MagtheridonsLair = 0x220,
    CoilfangTheSteamvault = 0x221,
    CoilfangTheUnderbog = 0x222,
    CoilfangTheSlavePens = 0x223,
    CoilfangSerpentshrineCavern = 0x224,
    TempestKeep = 0x226,
    TempestKeepTheArcatraz = 0x228,
    TempestKeepTheBotanica = 0x229,
    TempestKeepTheMechanar = 0x22a,
    AuchindounShadowLabyrinth = 0x22b,
    AuchindounSethekkHalls = 0x22c,
    AuchindounManaTombs = 0x22d,
    AuchindounAuchenaiCrypts = 0x22e,
    NagrandArena = 0x22f,
    TheEscapeFromDurnholde = 0x230,
    BladesEdgeArena = 0x232,
    BlackTemple = 0x234,
    GruulsLair = 0x235,
    EyeOfTheStorm = 0x236,
    ZulAman = 0x238,
    Northrend = 0x23b,
    RuinsOfLordaeron = 0x23c,
    Exteriortest = 0x23d,
    UtgardeKeep = 0x23e,
    UtgardePinnacle = 0x23f,
    TheNexus = 0x240,
    TheOculus = 0x242,
    TheSunwell = 0x244,
    TransportRutTheranToAuberdine = 0x246,
    TransportMenethilToTheramore = 0x248,
    MagistersTerrace = 0x249,
    TransportExodarToAuberdine = 0x24a,
    TransportFeathermoonFerry = 0x24b,
    TransportMenethilToAuberdine = 0x24c,
    TransportOrgrimmarToGromGol = 0x24d,
    TransportGromGolToUndercity = 0x24e,
    TransportUndercityToOrgrimmar = 0x24f,
    TransportBoreanTundraTest = 0x250,
    TransportBootyBayToRatchet = 0x251,
    TransportHowlingFjordSisterMercyQuest = 0x252,
    TheCullingOfStratholme = 0x253,
    TransportNaglfar = 0x254,
    CraigTest = 0x255,
    SunwellFix = 0x256,
    HallsOfStone = 0x257,
    DrakTharonKeep = 0x258,
    AzjolNerub = 0x259,
    HallsOfLightning = 0x25a,
    Ulduar = 0x25b,
    Gundrak = 0x25c,
    DevelopmentLandNonWeightedTextures = 0x25d,
    QaAndDvd = 0x25e,
    StrandOfTheAncients = 0x25f,
    VioletHold = 0x260,
    EbonHold = 0x261,
    TransportTirisfalToVengeanceLanding = 0x262,
    TransportMenethilToValgarde = 0x264,
    TransportOrgrimmarToWarsongHold = 0x265,
    TransportStormwindToValianceKeep = 0x266,
    TheObsidianSanctum = 0x267,
    TheEyeOfEternity = 0x268,
    DalaranSewers = 0x269,
    TheRingOfValor = 0x26a,
    AhnKahetTheOldKingdom = 0x26b,
    TransportMoaKiToUnuPe = 0x26c,
    TransportMoaKiToKamagua = 0x26d,
    TransportOrgrimsHammer = 0x26e,
    TransportTheSkybreaker = 0x26f,
    VaultOfArchavon = 0x270,
    IsleOfConquest = 0x274,
    IcecrownCitadel = 0x277,
    TheForgeOfSouls = 0x278,
    TransportAllianceAirshipBg = 0x281,
    TransportHordeairshipbg = 0x282,
    TransportOrgrimmarToThunderBluff = 0x287,
    TrialOfTheCrusader = 0x289,
    TrialOfTheChampion = 0x28a,
    PitOfSaron = 0x292,
    HallsOfReflection = 0x29c,
    TransportTheSkybreakerIcecrownCitadelRaid = 0x2a0,
    TransportOrgrimsHammerIcecrownCitadelRaid = 0x2a1,
    TransportTheSkybreakerIcDungeon = 0x2c8,
    TransportOrgrimsHammerIcDungeon = 0x2c9,
    TrasnportTheMightyWindIcecrownCitadelRaid = 0x2ce,
    Stormwind = 0x2d3,
    TheRubySanctum = 0x2d4,
}
