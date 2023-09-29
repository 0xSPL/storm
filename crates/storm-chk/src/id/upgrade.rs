#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum UpgradeId {
  // ===========================================================================
  // Classic
  // ===========================================================================
  TerranInfantryArmor = 0,
  TerranVehiclePlating = 1,
  TerranShipPlating = 2,
  ZergCarapace = 3,
  ZergFlyerCaparace = 4,
  ProtossArmor = 5,
  ProtossPlating = 6,
  TerranInfantryWeapons = 7,
  TerranVehicleWeapons = 8,
  TerranShipWeapons = 9,
  ZergMeleeAttacks = 10,
  ZergMissileAttacks = 11,
  ZergFlyerAttacks = 12,
  ProtossGroundWeapons = 13,
  ProtossAirWeapons = 14,
  ProtossPlasmaShields = 15,
  U238Shells = 16,
  IonThrusters = 17,
  BurstLasers = 18,  // (Unused)
  TitanReactor = 19, // (SV +50)
  OcularImplants = 20,
  MoebiusReactor = 21,  // (Ghost +50)
  ApolloReactor = 22,   // (Wraith +50)
  ColossusReactor = 23, // (BC +50)
  VentralSacs = 24,
  Antennae = 25,
  PneumatizedCarapace = 26,
  MetabolicBoost = 27,
  AdrenalGlands = 28,
  MuscularAugments = 29,
  GroovedSpines = 30,
  GameteMeiosis = 31,    // (Queen +50)
  MetasynapticNode = 32, // (Defiler +50)
  SingularityCharge = 33,
  LegEnhancements = 34,
  ScarabDamage = 35,
  ReaverCapacity = 36,
  GraviticDrive = 37,
  SensorArray = 38,
  GraviticBoosters = 39,
  KhaydarinAmulet = 40, // (HT +50)
  ApialSensors = 41,
  GraviticThrusters = 42,
  CarrierCapacity = 43,
  KhaydarinCore = 44, // (Arbiter +50)
  UnknownUpgrade45 = 45,
  // ===========================================================================
  // Brood War
  // ===========================================================================
  UnknownUpgrade46 = 46,
  ArgusJewel = 47, // (Corsair +50)
  UnknownUpgrade48 = 48,
  ArgusTalisman = 49, // (DA +50)
  UnknownUpgrade50 = 50,
  CaduceusReactor = 51, // (Medic +50)
  ChitinousPlating = 52,
  AnabolicSynthesis = 53,
  CharonBooster = 54,
  UnknownUpgrade55 = 55,
  UnknownUpgrade56 = 56,
  UnknownUpgrade57 = 57,
  UnknownUpgrade58 = 58,
  UnknownUpgrade59 = 59,
  UnknownUpgrade60 = 60,
}

impl UpgradeId {
  /// Total number of classic upgrades.
  pub const CLASSIC: usize = 0x2E;

  /// Total number of brood war upgrades.
  pub const BROOD_WAR: usize = 0x3D;

  pub const fn is_classic(&self) -> bool {
    (*self as usize) < Self::CLASSIC
  }
}
