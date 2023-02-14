use deku::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u32")]
pub enum Emote {
    OneshotNone = 0, // SKIP
    OneshotTalk = 1,
    OneshotBow = 2,
    OneshotWave = 3,
    OneshotCheer = 4,
    OneshotExclamation = 5,
    OneshotQuestion = 6,
    OneshotEat = 7,
    StateDance = 10,
    OneshotLaugh = 11,
    StateSleep = 12,
    StateSit = 13,
    OneshotRude = 14,
    OneshotRoar = 15,
    OneshotKneel = 16,
    OneshotKiss = 17,
    OneshotCry = 18,
    OneshotChicken = 19,
    OneshotBeg = 20,
    OneshotApplaud = 21,
    OneshotShout = 22,
    OneshotFlex = 23,
    OneshotShy = 24,
    OneshotPoint = 25,
    StateStand = 26,
    StateReadyUnarmed = 27,
    StateWorkSheathed = 28,
    StatePoint = 29,
    StateNone = 30,
    OneshotWound = 33,
    OneshotWoundCritical = 34,
    OneshotAttackUnarmed = 35,
    OneshotAttack1h = 36,
    OneshotAttack2htight = 37,
    OneshotAttack2hLoose = 38,
    OneshotParryUnarmed = 39,
    OneshotParryShield = 43,
    OneshotReadyUnarmed = 44,
    OneshotReady1h = 45,
    OneshotReadyBow = 48,
    OneshotSpellPrecast = 50,
    OneshotSpellCast = 51,
    OneshotBattleRoar = 53,
    OneshotSpecialattack1h = 54,
    OneshotKick = 60,
    OneshotAttackThrown = 61,
    StateStun = 64,
    StateDead = 65,
    OneshotSalute = 66,
    StateKneel = 68,
    StateUseStanding = 69,
    OneshotWaveNoSheathe = 70,
    OneshotCheerNoSheathe = 71,
    OneshotEatNoSheathe = 92,
    StateStunNoSheathe = 93,
    OneshotDance = 94,
    OneshotSaluteNoSheath = 113,
    StateUseStandingNoSheathe = 133,
    OneshotLaughNoSheathe = 153,
    StateWork = 173,
    StateSpellPrecast = 193,
    OneshotReadyRifle = 213,
    StateReadyRifle = 214,
    StateWorkMining = 233,
    StateWorkChopwood = 234,
    StateApplaud = 253,
    OneshotLiftoff = 254,
    OneshotYes = 273,
    OneshotNo = 274,
    OneshotTrain = 275,
    OneshotLand = 293,
    StateAtEase = 313,
    StateReady1h = 333,
    StateSpellKneelStart = 353,
    StateSubmerged = 373,
    OneshotSubmerge = 374,
    StateReady2h = 375,
    StateReadyBow = 376,
    OneshotMountSpecial = 377,
    StateTalk = 378,
    StateFishing = 379,
    OneshotFishing = 380,
    OneshotLoot = 381,
    StateWhirlwind = 382,
    StateDrowned = 383,
    StateHoldBow = 384,
    StateHoldRifle = 385,
    StateHoldThrown = 386,
    OneshotDrown = 387,
    OneshotStomp = 388,
    OneshotAttackOff = 389,
    OneshotAttackOffPierce = 390,
    StateRoar = 391,
    StateLaugh = 392,
    OneshotCreatureSpecial = 393,
    OneshotJumplandrun = 394,
    OneshotJumpend = 395,
    OneshotTalkNoSheathe = 396,
    OneshotPointNoSheathe = 397,
    StateCannibalize = 398,
    OneshotJumpstart = 399,
    StateDancespecial = 400,
    OneshotDancespecial = 401,
    OneshotCustomSpell01 = 402,
    OneshotCustomSpell02 = 403,
    OneshotCustomSpell03 = 404,
    OneshotCustomSpell04 = 405,
    OneshotCustomSpell05 = 406,
    OneshotCustomSpell06 = 407,
    OneshotCustomSpell07 = 408,
    OneshotCustomSpell08 = 409,
    OneshotCustomSpell09 = 410,
    OneshotCustomSpell10 = 411,
    StateExclaim = 412,
    StateDanceCustom = 413,
    StateSitChairMed = 415,
    StateCustomSpell01 = 416,
    StateCustomSpell02 = 417,
    StateEat = 418,
    StateCustomSpell04 = 419,
    StateCustomSpell03 = 420,
    StateCustomSpell05 = 421,
    StateSpelleffectHold = 422,
    StateEatNoSheathe = 423,
    StateMount = 424,
    StateReady2hl = 425,
    StateSitChairHigh = 426,
    StateFall = 427,
    StateLoot = 428,
    StateSubmergedNew = 429,
    OneshotCower = 430,
    StateCower = 431,
    OneshotUseStanding = 432,
    StateStealthStand = 433,
    OneshotOmnicastGhoul = 434,
    OneshotAttackBow = 435,
    OneshotAttackRifle = 436,
    StateSwimIdle = 437,
    StateAttackUnarmed = 438,
    OneshotSpellCastWSound = 439,
    OneshotDodge = 440,
    OneshotParry1h = 441,
    OneshotParry2h = 442,
    OneshotParry2hl = 443,
    StateFlyfall = 444,
    OneshotFlydeath = 445,
    StateFlyFall = 446,
    OneshotFlySitGroundDown = 447,
    OneshotFlySitGroundUp = 448,
    OneshotEmerge = 449,
    OneshotDragonSpit = 450,
    StateSpecialUnarmed = 451,
    OneshotFlygrab = 452,
    StateFlygrabclosed = 453,
    OneshotFlygrabthrown = 454,
    StateFlySitGround = 455,
    StateWalkBackwards = 456,
    OneshotFlytalk = 457,
    OneshotFlyattack1h = 458,
    StateCustomSpell08 = 459,
    OneshotFlyDragonSpit = 460,
    StateSitChairLow = 461,
    OneshotStun = 462,
    OneshotSpellCastOmni = 463,
    StateReadyThrown = 465,
    OneshotWorkChopwood = 466,
    OneshotWorkMining = 467,
    StateSpellChannelOmni = 468,
    StateSpellChannelDirected = 469,
    StandStateNone = 470,
    StateReadyjoust = 471,
    StateStrangulate = 473,
    StateReadySpellOmni = 474,
    StateHoldJoust = 475,
    OneshotCryJaina = 476,
}
