#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum TechId {
  // ===========================================================================
  // Classic
  // ===========================================================================
  StimPacks = 0,
  Lockdown = 1,
  EMPShockwave = 2,
  SpiderMines = 3,
  ScannerSweep = 4,
  SiegeMode = 5,
  DefensiveMatrix = 6,
  Irradiate = 7,
  YamatoGun = 8,
  CloakingField = 9,
  PersonnelCloaking = 10,
  Burrowing = 11,
  Infestation = 12,
  SpawnBroodling = 13,
  DarkSwarm = 14,
  Plague = 15,
  Consume = 16,
  Ensnare = 17,
  Parasite = 18,
  PsionicStorm = 19,
  Hallucination = 20,
  Recall = 21,
  StasisField = 22,
  ArchonWarp = 23,
  // ===========================================================================
  // Brood War
  // ===========================================================================
  Restoration = 24,
  DisruptionWeb = 25,
  Unused26 = 26,
  MindControl = 27,
  DarkArchonMeld = 28,
  Feedback = 29,
  OpticalFlare = 30,
  Maelstorm = 31,
  LurkerAspect = 32,
  Unused33 = 33,
  Healing = 34,
  Unused35 = 35,
  Unused36 = 36,
  Unused37 = 37,
  Unused38 = 38,
  Unused39 = 39,
  Unused40 = 40,
  Unused41 = 41,
  Unused42 = 42,
  Unused43 = 43,
}

impl TechId {
  /// Total number of classic technologies.
  pub const CLASSIC: usize = 0x18;

  /// Total number of brood war technologies.
  pub const BROOD_WAR: usize = 0x2C;

  pub const fn is_classic(&self) -> bool {
    (*self as usize) < Self::CLASSIC
  }
}
