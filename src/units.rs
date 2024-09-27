use std::{collections::BTreeMap, sync::OnceLock};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum UnitId {
  Infantry = 1,
  Mech = 2,
  MdTank = 3,
  Tank = 4,
  Recon = 5,
  APC = 6,
  Artillery = 7,
  Rocket = 8,
  AntiAir = 9,
  Missile = 10,
  Fighter = 11,
  Bomber = 12,
  BCopter = 13,
  TCopter = 14,
  Battleship = 15,
  Cruiser = 16,
  Lander = 17,
  Sub = 18,
  BlackBoat = 28,
  Carrier = 29,
  Stealth = 30,
  // BlackBomb = 968731,
  MegaTank = 1141438,
  Neotank = 46,
  // Piperunner = 960900,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum UnitMovementType {
  Infantry,   // Total units: 1
  Mech,       // Total units: 1
  Tire,       // Total units: 3
  Air,        // Total units: 6
  Tank,       // Total units: 7
  Ship,       // Total units: 4
  ShipShoal,  // Total units: Lander, BB
  Piperunner, // Total units: 1
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Symbol {
  Ground, // G
  Air,    // M
  Sea,    // S
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UnitStatus {
  pub movement_points: u8,
  pub vision: u8,
  pub fuel: u8,
  pub fuel_per_turn: u8,
  pub sub_dive: bool,
  pub ammo: u8,
  pub short_range: u8,
  pub long_range: u8,
  pub second_weapon: bool,
  pub symbol: Symbol,
  pub cost: u32,
  pub movement_type: UnitMovementType,
  pub hit_points: u8,
  pub cargo_units_id: [Option<u8>; 2],
  pub carried: bool,
}

pub fn default_units_settings() -> &'static BTreeMap<UnitId, UnitStatus> {
  static BTREE_MAP: OnceLock<BTreeMap<UnitId, UnitStatus>> = OnceLock::new();
  BTREE_MAP.get_or_init(|| {
    let mut m = BTreeMap::new();
    m.insert(
      UnitId::Infantry,
      UnitStatus {
        movement_points: 3,
        vision: 2,
        fuel: 99,
        fuel_per_turn: 0,
        sub_dive: false,
        ammo: 0,
        short_range: 0,
        long_range: 0,
        second_weapon: false,
        symbol: Symbol::Ground,
        cost: 1000,
        movement_type: UnitMovementType::Infantry,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Mech,
      UnitStatus {
        movement_points: 2,
        vision: 2,
        fuel: 70,
        fuel_per_turn: 0,
        sub_dive: false,
        ammo: 3,
        short_range: 0,
        long_range: 0,
        second_weapon: true,
        symbol: Symbol::Ground,
        cost: 3000,
        movement_type: UnitMovementType::Mech,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::MdTank,
      UnitStatus {
        movement_points: 5,
        vision: 1,
        fuel: 50,
        fuel_per_turn: 0,
        sub_dive: false,
        ammo: 8,
        short_range: 0,
        long_range: 0,
        second_weapon: true,
        symbol: Symbol::Ground,
        cost: 16000,
        movement_type: UnitMovementType::Tank,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Tank,
      UnitStatus {
        movement_points: 6,
        vision: 3,
        fuel: 70,
        fuel_per_turn: 0,
        sub_dive: false,
        ammo: 9,
        short_range: 0,
        long_range: 0,
        second_weapon: true,
        symbol: Symbol::Ground,
        cost: 7000,
        movement_type: UnitMovementType::Tank,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Recon,
      UnitStatus {
        movement_points: 8,
        vision: 5,
        fuel: 80,
        fuel_per_turn: 0,
        sub_dive: false,
        ammo: 0,
        short_range: 0,
        long_range: 0,
        second_weapon: false,
        symbol: Symbol::Ground,
        cost: 4000,
        movement_type: UnitMovementType::Tire,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::APC,
      UnitStatus {
        movement_points: 6,
        vision: 1,
        fuel: 70,
        fuel_per_turn: 0,
        sub_dive: false,
        ammo: 0,
        short_range: 0,
        long_range: 0,
        second_weapon: false,
        symbol: Symbol::Ground,
        cost: 5000,
        movement_type: UnitMovementType::Tank,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Artillery,
      UnitStatus {
        movement_points: 5,
        vision: 1,
        fuel: 50,
        fuel_per_turn: 0,
        sub_dive: false,
        ammo: 9,
        short_range: 2,
        long_range: 3,
        second_weapon: false,
        symbol: Symbol::Ground,
        cost: 6000,
        movement_type: UnitMovementType::Tank,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Rocket,
      UnitStatus {
        movement_points: 5,
        vision: 1,
        fuel: 50,
        fuel_per_turn: 0,
        sub_dive: false,
        ammo: 6,
        short_range: 3,
        long_range: 5,
        second_weapon: false,
        symbol: Symbol::Ground,
        cost: 15000,
        movement_type: UnitMovementType::Tire,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::AntiAir,
      UnitStatus {
        movement_points: 6,
        vision: 2,
        fuel: 60,
        fuel_per_turn: 0,
        sub_dive: false,
        ammo: 9,
        short_range: 0,
        long_range: 0,
        second_weapon: false,
        symbol: Symbol::Ground,
        cost: 8000,
        movement_type: UnitMovementType::Tank,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Missile,
      UnitStatus {
        movement_points: 4,
        vision: 5,
        fuel: 50,
        fuel_per_turn: 0,
        sub_dive: false,
        ammo: 6,
        short_range: 3,
        long_range: 5,
        second_weapon: false,
        symbol: Symbol::Ground,
        cost: 12000,
        movement_type: UnitMovementType::Tire,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Fighter,
      UnitStatus {
        movement_points: 9,
        vision: 2,
        fuel: 99,
        fuel_per_turn: 5,
        sub_dive: false,
        ammo: 9,
        short_range: 0,
        long_range: 0,
        second_weapon: false,
        symbol: Symbol::Air,
        cost: 20000,
        movement_type: UnitMovementType::Air,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Bomber,
      UnitStatus {
        movement_points: 7,
        vision: 2,
        fuel: 99,
        fuel_per_turn: 5,
        sub_dive: false,
        ammo: 9,
        short_range: 0,
        long_range: 0,
        second_weapon: false,
        symbol: Symbol::Air,
        cost: 22000,
        movement_type: UnitMovementType::Air,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::BCopter,
      UnitStatus {
        movement_points: 6,
        vision: 3,
        fuel: 99,
        fuel_per_turn: 2,
        sub_dive: false,
        ammo: 6,
        short_range: 0,
        long_range: 0,
        second_weapon: true,
        symbol: Symbol::Air,
        cost: 9000,
        movement_type: UnitMovementType::Air,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::TCopter,
      UnitStatus {
        movement_points: 6,
        vision: 2,
        fuel: 99,
        fuel_per_turn: 2,
        sub_dive: false,
        ammo: 0,
        short_range: 0,
        long_range: 0,
        second_weapon: false,
        symbol: Symbol::Air,
        cost: 5000,
        movement_type: UnitMovementType::Air,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Battleship,
      UnitStatus {
        movement_points: 5,
        vision: 2,
        fuel: 99,
        fuel_per_turn: 1,
        sub_dive: false,
        ammo: 9,
        short_range: 2,
        long_range: 6,
        second_weapon: false,
        symbol: Symbol::Sea,
        cost: 28000,
        movement_type: UnitMovementType::Ship,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Cruiser,
      UnitStatus {
        movement_points: 6,
        vision: 3,
        fuel: 99,
        fuel_per_turn: 1,
        sub_dive: false,
        ammo: 9,
        short_range: 0,
        long_range: 0,
        second_weapon: false,
        symbol: Symbol::Sea,
        cost: 18000,
        movement_type: UnitMovementType::Ship,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Lander,
      UnitStatus {
        movement_points: 6,
        vision: 1,
        fuel: 99,
        fuel_per_turn: 1,
        sub_dive: false,
        ammo: 0,
        short_range: 0,
        long_range: 0,
        second_weapon: false,
        symbol: Symbol::Sea,
        cost: 12000,
        movement_type: UnitMovementType::ShipShoal,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Sub,
      UnitStatus {
        movement_points: 5,
        vision: 5,
        fuel: 60,
        fuel_per_turn: 1,
        sub_dive: false,
        ammo: 6,
        short_range: 0,
        long_range: 0,
        second_weapon: false,
        symbol: Symbol::Sea,
        cost: 20000,
        movement_type: UnitMovementType::Ship,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::BlackBoat,
      UnitStatus {
        movement_points: 7,
        vision: 1,
        fuel: 60,
        fuel_per_turn: 1,
        sub_dive: false,
        ammo: 0,
        short_range: 0,
        long_range: 0,
        second_weapon: false,
        symbol: Symbol::Sea,
        cost: 7500,
        movement_type: UnitMovementType::ShipShoal,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Carrier,
      UnitStatus {
        movement_points: 5,
        vision: 4,
        fuel: 99,
        fuel_per_turn: 1,
        sub_dive: false,
        ammo: 9,
        short_range: 3,
        long_range: 8,
        second_weapon: false,
        symbol: Symbol::Sea,
        cost: 30000,
        movement_type: UnitMovementType::Ship,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Stealth,
      UnitStatus {
        movement_points: 6,
        vision: 4,
        fuel: 60,
        fuel_per_turn: 5,
        sub_dive: false,
        ammo: 6,
        short_range: 0,
        long_range: 0,
        second_weapon: false,
        symbol: Symbol::Air,
        cost: 24000,
        movement_type: UnitMovementType::Air,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::MegaTank,
      UnitStatus {
        movement_points: 4,
        vision: 1,
        fuel: 50,
        fuel_per_turn: 0,
        sub_dive: false,
        ammo: 3,
        short_range: 0,
        long_range: 0,
        second_weapon: true,
        symbol: Symbol::Ground,
        cost: 28000,
        movement_type: UnitMovementType::Tank,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m.insert(
      UnitId::Neotank,
      UnitStatus {
        movement_points: 6,
        vision: 1,
        fuel: 99,
        fuel_per_turn: 0,
        sub_dive: false,
        ammo: 9,
        short_range: 0,
        long_range: 0,
        second_weapon: true,
        symbol: Symbol::Ground,
        cost: 22000,
        movement_type: UnitMovementType::Tank,
        hit_points: 10,
        cargo_units_id: [None, None],
        carried: false,
      },
    );
    m
  })
}

// TODO: can use default_units_settings to extract this
pub const INDIRECT_UNITS: [UnitId; 4] = [
  UnitId::Battleship,
  UnitId::Carrier,
  UnitId::Missile,
  UnitId::Rocket,
  // Piperunner
];

// TODO: can use default_units_settings to extract this
pub const FOOTSOLDIER_UNITS: [UnitId; 2] = [UnitId::Infantry, UnitId::Mech];

pub fn sea_units() -> &'static Vec<UnitId> {
  static SEA_UNITS: OnceLock<Vec<UnitId>> = OnceLock::new();
  SEA_UNITS.get_or_init(|| {
    default_units_settings()
      .iter()
      .filter_map(|(k, v)| match v.symbol {
        Symbol::Sea => Some(*k),
        _ => None,
      })
      .collect()
  })
}

pub fn air_units() -> &'static Vec<UnitId> {
  static AIR_UNITS: OnceLock<Vec<UnitId>> = OnceLock::new();
  AIR_UNITS.get_or_init(|| {
    default_units_settings()
      .iter()
      .filter_map(|(k, v)| match v.symbol {
        Symbol::Air => Some(*k),
        _ => None,
      })
      .collect()
  })
}

pub fn vehicle_units() -> &'static Vec<UnitId> {
  static VEHICLE_UNITS: OnceLock<Vec<UnitId>> = OnceLock::new();
  VEHICLE_UNITS.get_or_init(|| {
    default_units_settings()
      .iter()
      .filter_map(|(k, v)| match v.movement_type {
        UnitMovementType::Tank => Some(*k),
        UnitMovementType::Tire => Some(*k),
        _ => None,
      })
      .collect()
  })
}
