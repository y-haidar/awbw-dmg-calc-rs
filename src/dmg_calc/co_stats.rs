use strum::IntoEnumIterator;

use crate::{
  dmg_calc::kindle::KindleCityId,
  terrain::{terrain_stars_map, TerrainId, ROAD_TILES_ID},
  units::{air_units, sea_units, vehicle_units, UnitId, FOOTSOLDIER_UNITS, INDIRECT_UNITS},
};

use super::DmgCalcInput;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum ActivePower {
  None,
  Normal,
  Super,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum CoId {
  Andy = 1,
  Hachi = 17,
  Jake = 22,
  Max = 7,
  Nell = 24,
  Rachel = 28,
  Sami = 8,
  Colin = 15,
  Grit = 2,
  Olaf = 9,
  Sasha = 19,
  Drake = 5,
  Eagle = 10,
  Javier = 27,
  Jess = 14,
  Grimm = 20,
  NoCO = 31,
  Kanbei = 3,
  Sensei = 13,
  Sonja = 18,
  Adder = 11,
  Flak = 25,
  Hawke = 12,
  Jugger = 26,
  Kindle = 23,
  Koal = 21,
  Lash = 16,
  Sturm = 29,
  VonBolt = 30,
}

pub fn calc_co_atk(atk: DmgCalcInput, is_counter_attack: bool) -> u32 {
  const BASE_POWER: u32 = 10;

  let co_atk = match atk.co {
    CoId::Andy => match atk.power {
      ActivePower::Super => 110,
      _ => 100,
    },

    CoId::Jake => match (atk.power, atk.terrain_id) {
      (ActivePower::None, TerrainId::Plain) => 110,
      (ActivePower::Normal, TerrainId::Plain) => 120,
      (ActivePower::Super, TerrainId::Plain) => 140,
      (_, _) => 100,
    },

    CoId::Max => match atk.power {
      _ if INDIRECT_UNITS.contains(&atk.units_id) => 90,
      _ if FOOTSOLDIER_UNITS.contains(&atk.units_id) => 100,
      ActivePower::None => 120,
      ActivePower::Normal => 130,
      ActivePower::Super => 150,
    },

    CoId::Sami => match atk.power {
      _ if !FOOTSOLDIER_UNITS.contains(&atk.units_id) => 90,
      ActivePower::None => 130,
      ActivePower::Normal => 160,
      ActivePower::Super => 180,
    },

    CoId::Colin => match atk.power {
      ActivePower::Super => 90 + (atk.funds * 3 / 1000),
      _ => 90,
    },

    CoId::Grit => match atk.power {
      _ if FOOTSOLDIER_UNITS.contains(&atk.units_id) => 100,
      _ if !INDIRECT_UNITS.contains(&atk.units_id) => 80,
      ActivePower::None => 120,
      _ => 140,
    },

    CoId::Drake if air_units().contains(&atk.units_id) => 70,
    CoId::Drake => 100,

    CoId::Eagle if sea_units().contains(&atk.units_id) => 70,
    CoId::Eagle if air_units().contains(&atk.units_id) => match atk.power {
      ActivePower::None => 115,
      _ => 120,
    },
    CoId::Eagle => 100,

    CoId::Jess if vehicle_units().contains(&atk.units_id) => match atk.power {
      ActivePower::None => 110,
      ActivePower::Normal => 120,
      ActivePower::Super => 140,
    },
    CoId::Jess => 90,

    CoId::Grimm => match atk.power {
      ActivePower::None => 130,
      ActivePower::Normal => 150,
      ActivePower::Super => 180,
    },

    CoId::Kanbei => match atk.power {
      ActivePower::None => 130,
      ActivePower::Normal => 140,
      ActivePower::Super if is_counter_attack => 130 / 2 + 140,
      ActivePower::Super => 140,
    },

    CoId::Sensei if FOOTSOLDIER_UNITS.contains(&atk.units_id) => 100,
    CoId::Sensei if atk.units_id == UnitId::BCopter => match atk.power {
      ActivePower::None => 150,
      ActivePower::Normal => 175,
      ActivePower::Super => 175,
    },
    CoId::Sensei if air_units().contains(&atk.units_id) => 100,
    CoId::Sensei => 90,

    CoId::Hawke => 110,

    CoId::Kindle if KindleCityId::iter().any(|city_id| city_id as u32 == atk.terrain_id as u32) => {
      match atk.power {
        ActivePower::None => 140,
        ActivePower::Normal => 180,
        ActivePower::Super => 230,
      }
    }
    CoId::Kindle => 100,

    CoId::Koal if ROAD_TILES_ID.contains(&atk.terrain_id) => match atk.power {
      ActivePower::None => 110,
      ActivePower::Normal => 120,
      ActivePower::Super => 130,
    },
    CoId::Koal => 100,

    CoId::Lash => {
      let stars = *terrain_stars_map().get(&atk.terrain_id).unwrap() as u32;
      let co_atk = match atk.power {
        ActivePower::Super => stars * 20,
        _ => stars * 10,
      };
      co_atk + 100
    }

    CoId::Sturm => 80,
    CoId::VonBolt => 110,
    CoId::Jugger => 100,
    CoId::Flak => 100,
    CoId::Adder => 100,
    CoId::Hachi => 100,
    CoId::Nell => 100,
    CoId::Rachel => 100,
    CoId::Olaf => 100,
    CoId::Sasha => 100,
    CoId::Javier => 100,
    CoId::Sonja => 100,
    CoId::NoCO => 100,
  };

  let co_atk = if atk.power != ActivePower::None {
    co_atk + BASE_POWER
  } else {
    co_atk
  };
  co_atk
}

pub fn calc_co_def(def: DmgCalcInput, atk: DmgCalcInput) -> u32 {
  const BASE_POWER: u32 = 10;

  let co_def = match def.co {
    CoId::Drake if sea_units().contains(&def.units_id) => 110,
    CoId::Drake => 100,

    CoId::Eagle if air_units().contains(&def.units_id) => match def.power {
      ActivePower::None => 110,
      _ => 130,
    },
    CoId::Eagle => 100,

    CoId::Javier if INDIRECT_UNITS.contains(&atk.units_id) => match def.power {
      ActivePower::None => 120 + (10 * def.towers as u32),
      ActivePower::Normal => 140 + (20 * def.towers as u32),
      ActivePower::Super => 180 + (30 * def.towers as u32),
    },
    CoId::Javier => 100,

    CoId::Kanbei => match def.power {
      ActivePower::Super => 150,
      _ => 130,
    },

    CoId::Grimm => 80,
    CoId::Hawke => 110,
    CoId::Sturm => 120,
    CoId::VonBolt => 110,

    CoId::Andy => 100,
    CoId::Hachi => 100,
    CoId::Jake => 100,
    CoId::Max => 100,
    CoId::Nell => 100,
    CoId::Rachel => 100,
    CoId::Sami => 100,
    CoId::Colin => 100,
    CoId::Grit => 100,
    CoId::Olaf => 100,
    CoId::Sasha => 100,
    CoId::Jess => 100,
    CoId::Sensei => 100,
    CoId::Sonja => 100,
    CoId::Adder => 100,
    CoId::Flak => 100,
    CoId::Jugger => 100,
    CoId::Kindle => 100,
    CoId::Koal => 100,
    CoId::Lash => 100,
    CoId::NoCO => 100,
  };

  let co_def = if def.power != ActivePower::None {
    co_def + BASE_POWER
  } else {
    co_def
  };
  co_def
}
