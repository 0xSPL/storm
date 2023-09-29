#![allow(non_camel_case_types)]

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum WeaponId {
  // ===========================================================================
  // Classic
  // ===========================================================================
  GaussRifle = 0,                     // (Normal)
  GaussRifle_JimRaynor = 1,           // (Jim Raynor-Marine)
  C10CanisterRifle = 2,               // (Normal)
  C10CanisterRifle_SarahKerrigan = 3, // (Sarah Kerrigan)
  FragmentationGrenade = 4,           // (Normal)
  FragmentationGrenade_JimRaynor = 5, // (Jim Raynor-Vulture)
  SpiderMines = 6,
  TwinAutocannons = 7,                  // (Normal)
  HellfireMissilePack = 8,              // (Normal)
  TwinAutocannons_AlanSchezar = 9,      // (Alan Schezar)
  HellfireMissilePack_AlanSchezar = 10, // (Alan Schezar)
  ArcliteCannon = 11,                   // (Normal)
  ArcliteCannon_EdmundDuke = 12,        // (Edmund Duke)
  FusionCutter = 13,
  FusionCutter_Harvest = 14,          // (Harvest)
  GeminiMissiles = 15,                // (Normal)
  BurstLasers = 16,                   // (Normal)
  GeminiMissiles_TomKazansky = 17,    // (Tom Kazansky)
  BurstLasers_TomKazansky = 18,       // (Tom Kazansky)
  ATSLaserBattery = 19,               // (Normal)
  ATALaserBattery = 20,               // (Normal)
  ATSLaserBattery_Norad = 21,         // (Norad II+Mengsk+DuGalle)
  ATALaserBattery_Norad = 22,         // (Norad II+Mengsk+DuGalle)
  ATSLaserBattery_Hyperion = 23,      // (Hyperion)
  ATALaserBattery_Hyperion = 24,      // (Hyperion)
  FlameThrower = 25,                  // (Normal)
  FlameThrower_GuiMontag = 26,        // (Gui Montag)
  ArcliteShockCannon = 27,            // (Normal)
  ArcliteShockCannon_EdmundDuke = 28, // (Edmund Duke)
  LongboltMissile = 29,
  YamatoGun = 30,
  NuclearMissile = 31,
  Lockdown = 32,
  EMPShockwave = 33,
  Irradiate = 34,
  Claws = 35,                     // (Normal)
  Claws_DevouringOne = 36,        // (Devouring One)
  Claws_InfestedKerrigan = 37,    // (Infested Kerrigan)
  NeedleSpines = 38,              // (Normal)
  NeedleSpines_HunterKiller = 39, // (Hunter Killer)
  KaiserBlades = 40,              // (Normal)
  KaiserBlades_Torrasque = 41,    // (Torrasque)
  ToxicSpores = 42,               // (Broodling)
  Spines = 43,
  Spines_Harvest = 44,    // (Harvest)
  AcidSpray = 45,         // (Unused)
  AcidSpore = 46,         // (Normal)
  AcidSpore_Kukulza = 47, // (Kukulza-Guardian)
  GlaveWurm = 48,         // (Normal)
  GlaveWurm_Kukulza = 49, // (Kukulza-Mutalisk)
  Venom = 50,             // (Unused-Defiler)
  Venom_Hero = 51,        // (Unused-Defiler Hero)
  SeekerSpores = 52,
  SubterraneanTentacle = 53,
  Suicide_InfestedTerran = 54, // (Infested Terran)
  Suicide_Scourge = 55,        // (Scourge)
  Parasite = 56,
  SpawnBroodlings = 57,
  Ensnare = 58,
  DarkSwarm = 59,
  Plague = 60,
  Consume = 61,
  ParticleBeam = 62,
  ParticleBeam_Harvest = 63,      // (Harvest)
  PsiBlades = 64,                 // (Normal)
  PsiBlades_Fenix = 65,           // (Fenix-Zealot)
  PhaseDisruptor = 66,            // (Normal)
  PhaseDisruptor_Fenix = 67,      // (Fenix-Dragoon)
  PsiAssault = 68,                // (Normal-Unused)
  PsiAssault_Tassadar = 69,       // (Tassadar+Aldaris)
  PsionicShockwave = 70,          // (Normal)
  PsionicShockwave_Tassadar = 71, // (Tassadar/Zeratul Archon)
  Unknown72 = 72,
  DualPhotonBlasters = 73,            // (Normal)
  AntiMatterMissiles = 74,            // (Normal)
  DualPhotonBlasters_Mojo = 75,       // (Mojo)
  AntiMatterMissiles_Mojo = 76,       // (Mojo)
  PhaseDisruptorCannon = 77,          // (Normal)
  PhaseDisruptorCannon_Danimoth = 78, // (Danimoth)
  PulseCannon = 79,
  STSPhotonCannon = 80,
  STAPhotonCannon = 81,
  Scarab = 82,
  StasisField = 83,
  PsiStorm = 84,
  WarpBlades_Zeratul = 85,            // (Zeratul)
  WarpBlades_Hero = 86,               // (Dark Templar Hero)
  Missiles = 87,                      // (Unused)
  LaserBattery1 = 88,                 // (Unused)
  TormentorMissiles = 89,             // (Unused)
  Bombs = 90,                         // (Unused)
  RaiderGun = 91,                     // (Unused)
  LaserBattery2 = 92,                 // (Unused)
  LaserBattery3 = 93,                 // (Unused)
  DualPhotonBlasters_Unused = 94,     // (Unused)
  FlechetteGrenade = 95,              // (Unused)
  TwinAutocannons_FloorTrap = 96,     // (Floor Trap)
  HellfireMissilePack_WallTrap = 97,  // (Wall Trap)
  FlameThrower_WallTrap = 98,         // (Wall Trap)
  HellfireMissilePack_FloorTrap = 99, // (Floor Trap)
  // ===========================================================================
  // Brood War
  // ===========================================================================
  NeutronFlare = 100,
  DisruptionWeb = 101,
  Restoration = 102,
  HaloRockets = 103,
  CorrosiveAcid = 104,
  MindControl = 105,
  Feedback = 106,
  OpticalFlare = 107,
  Maelstrom = 108,
  SubterraneanSpines = 109,
  GaussRifle0 = 110,                    // (Unused)
  WarpBlades = 111,                     // (Normal)
  C10CanisterRifle_SamirDuran = 112,    // (Samir Duran)
  C10CanisterRifle_InfestedDuran = 113, // (Infested Duran)
  DualPhotonBlasters_Artanis = 114,     // (Artanis)
  AntiMatterMissiles_Artanis = 115,     // (Artanis)
  C10CanisterRifle_AlexeiStukov = 116,  // (Alexei Stukov)
  GaussRifle1 = 117,                    // (Unused)
  GaussRifle2 = 118,                    // (Unused)
  GaussRifle3 = 119,                    // (Unused)
  GaussRifle4 = 120,                    // (Unused)
  GaussRifle5 = 121,                    // (Unused)
  GaussRifle6 = 122,                    // (Unused)
  GaussRifle7 = 123,                    // (Unused)
  GaussRifle8 = 124,                    // (Unused)
  GaussRifle9 = 125,                    // (Unused)
  GaussRifle10 = 126,                   // (Unused)
  GaussRifle11 = 127,                   // (Unused)
  GaussRifle12 = 128,                   // (Unused)
  GaussRifle13 = 129,                   // (Unused)
  // None = 130,
}

impl WeaponId {
  /// Total number of classic weapons.
  pub const CLASSIC: usize = 0x64;

  /// Total number of brood war weapons.
  pub const BROOD_WAR: usize = 0x82;

  pub const fn is_classic(&self) -> bool {
    (*self as usize) < Self::CLASSIC
  }
}
