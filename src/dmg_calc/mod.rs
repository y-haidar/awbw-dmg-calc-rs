use crate::{
  terrain::{terrain_stars_map, TerrainId},
  units::UnitId,
};

mod co_stats;
mod kindle;
mod pri_base_dmg_map;
mod sec_base_dmg_map;

pub use co_stats::*;
pub use pri_base_dmg_map::pri_base_dmg_map;
pub use sec_base_dmg_map::sec_base_dmg_map;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct BaseDmgMapKey {
  pub atk: UnitId,
  pub def: UnitId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Damage {
  pub min: u32,
  pub max: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DmgCalcInput {
  pub cities: u32,
  pub co: CoId,
  pub funds: u32,
  pub hp: Option<u8>,
  pub power: ActivePower,
  pub has_ammo: bool,
  pub terrain_id: TerrainId,
  pub towers: u8,
  pub units_id: UnitId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DmgCalcOutput {
  pub atk: Damage,
  pub def_took_max: Damage,
  pub def_took_min: Damage,
}

fn _calc(atk: DmgCalcInput, def: DmgCalcInput) -> Result<Damage, Error> {
  let pri_base_dmg = pri_base_dmg_map().get(&BaseDmgMapKey {
    atk: atk.units_id,
    def: def.units_id,
  });
  let sec_base_dmg = sec_base_dmg_map().get(&BaseDmgMapKey {
    atk: atk.units_id,
    def: def.units_id,
  });

  let pri_base_dmg_ammo_accounted = match atk.has_ammo {
    true => pri_base_dmg,
    false => None,
  };
  let base_dmg = *match (pri_base_dmg_ammo_accounted, sec_base_dmg) {
    (None, None) => return Err(Error::NotAtkable),
    (Some(dmg), None) => dmg,
    (None, Some(dmg)) => dmg,
    (Some(dmg), Some(_)) => dmg,
  } as f64;

  let atk_value = calc_co_atk(atk) as f64 + atk.towers as f64 * 10.;
  let def_value = calc_co_def(def, atk) as f64;
  let def_terrain_stars = *terrain_stars_map().get(&def.terrain_id).unwrap() as f64;
  let def_terrain_stars = if def.co == CoId::Lash && def.power == ActivePower::Super {
    def_terrain_stars * 2.
  } else {
    def_terrain_stars
  };

  let (luck_bad, luck_good) = match atk.co {
    CoId::Rachel if atk.power == ActivePower::Normal => (0, 39),
    CoId::Nell => match atk.power {
      ActivePower::None => (0, 19),
      ActivePower::Normal => (0, 59),
      ActivePower::Super => (0, 99),
    },
    CoId::Sonja => (9, 9),
    CoId::Flak => match atk.power {
      ActivePower::None => (9, 24),
      ActivePower::Normal => (19, 49),
      ActivePower::Super => (39, 89),
    },
    CoId::Jugger => match atk.power {
      ActivePower::None => (14, 29),
      ActivePower::Normal => (24, 54),
      ActivePower::Super => (44, 94),
    },
    _ => (0, 9),
  };

  let dmg_base_atk = ((base_dmg) * atk_value) as f64 / 100.;
  let base_dmg_luck_min = dmg_base_atk - luck_bad as f64;
  let base_dmg_luck_max = dmg_base_atk + luck_good as f64;
  let dmg_hp = {
    let atk_hp_visual = (atk.hp.unwrap() as f64 / 10.).ceil();
    assert!(!(atk_hp_visual as u32 > 10 || ((atk_hp_visual as u32) < 1 && atk.hp.unwrap() != 0)));
    atk_hp_visual / 10.
  };
  let dmg_def = {
    let def_hp_visual = (def.hp.unwrap() as f64 / 100.).ceil();
    assert!(!(def_hp_visual as u32 > 10 || ((def_hp_visual as u32) < 1 && def.hp.unwrap() != 0)));
    (200. - (def_value + (def_terrain_stars * def_hp_visual))) / 100.
  };

  let dmg_min = base_dmg_luck_min * dmg_hp * dmg_def;
  let dmg_min = (dmg_min * 20.).ceil() / 20.;
  let dmg_min = dmg_min.floor() as u32;
  let dmg_max = base_dmg_luck_max * dmg_hp * dmg_def;
  let dmg_max = (dmg_max * 20.).ceil() / 20.;
  let dmg_max = dmg_max.floor() as u32;

  Ok(Damage {
    min: dmg_min,
    max: dmg_max,
  })
}

pub fn calc(atk: DmgCalcInput, def: DmgCalcInput) -> Result<DmgCalcOutput, Error> {
  let (atk, mut def) = if def.co == CoId::Sonja && def.power == ActivePower::Super {
    (def, atk)
  } else {
    (atk, def)
  };
  let atk_dmg = _calc(atk, def)?;
  let def_hp = def.hp.unwrap() as u32;

  let mut _calc_def_dmg = |atk_dmg: u32| {
    if def_hp < atk_dmg {
      def.hp = Some(0);
    } else {
      let def_hp = def_hp - atk_dmg;
      def.hp = Some(def_hp as u8);
    }
    let mut def_dmg = _calc(def, atk).unwrap_or(Damage { min: 0, max: 0 });
    if def.co == CoId::Kanbei && def.power == ActivePower::Super {
      def_dmg.min += def_dmg.min * 2;
      def_dmg.max += def_dmg.max * 2;
    }
    def_dmg
  };
  let def_took_min = _calc_def_dmg(atk_dmg.min);
  let def_took_max = _calc_def_dmg(atk_dmg.max);

  Ok(DmgCalcOutput {
    atk: atk_dmg,
    def_took_min,
    def_took_max,
  })
}

#[derive(Debug)]
pub enum Error {
  NotAtkable,
  NoAmmo,
}

#[cfg(test)]
mod unit_tests;
