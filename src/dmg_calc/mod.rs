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
pub struct DmgCalc {
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

pub fn calc(atk: DmgCalc, def: DmgCalc) -> Result<(u32, u32), Error> {
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
  let dmg_luck_min = dmg_base_atk - luck_bad as f64;
  let dmg_luck_max = dmg_base_atk + luck_good as f64;
  let dmg_hp = atk.hp.unwrap() as f64 / 10.;
  let dmg_def = (200. - (def_value + (def_terrain_stars * def.hp.unwrap() as f64))) / 100.;

  let dmg_min = dmg_luck_min * dmg_hp * dmg_def;
  let dmg_min = (dmg_min * 20.).ceil() / 20.;
  let dmg_min = dmg_min.floor();
  let dmg_max = dmg_luck_max * dmg_hp * dmg_def;
  let dmg_max = (dmg_max * 20.).ceil() / 20.;
  let dmg_max = dmg_max.floor();

  Ok((dmg_min as u32, dmg_max as u32))
}

#[derive(Debug)]
pub enum Error {
  NotAtkable,
  NoAmmo,
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_adder_vs_adder_aa_vs_inf() {
    let atk = DmgCalc {
      cities: 21,
      co: CoId::Adder,
      funds: 10_000,
      hp: Some(10),
      power: ActivePower::None,
      has_ammo: true,
      terrain_id: TerrainId::CRoad,
      towers: 0,
      units_id: UnitId::AntiAir,
    };
    let def = DmgCalc {
      cities: 21,
      co: CoId::Adder,
      funds: 0,
      hp: Some(10),
      power: ActivePower::None,
      has_ammo: true,
      terrain_id: TerrainId::CRoad,
      towers: 0,
      units_id: UnitId::Infantry,
    };
    insta::assert_debug_snapshot!(calc(atk, def), @r###"
    Ok(
        (
            105,
            114,
        ),
    )
    "###);
  }
  #[test]
  fn test_adder_vs_adder_aa_vs_inf_cop() {
    let atk = DmgCalc {
      cities: 21,
      co: CoId::Adder,
      funds: 10_000,
      hp: Some(10),
      power: ActivePower::Normal,
      has_ammo: true,
      terrain_id: TerrainId::CRoad,
      towers: 0,
      units_id: UnitId::AntiAir,
    };
    let def = DmgCalc {
      cities: 21,
      co: CoId::Adder,
      funds: 0,
      hp: Some(10),
      power: ActivePower::None,
      has_ammo: true,
      terrain_id: TerrainId::CRoad,
      towers: 0,
      units_id: UnitId::Infantry,
    };
    insta::assert_debug_snapshot!(calc(atk, def), @r###"
    Ok(
        (
            115,
            124,
        ),
    )
    "###);
  }

  #[test]
  fn test_colin_vs_adder_aa_vs_inf() {
    let atk = DmgCalc {
      cities: 21,
      co: CoId::Colin,
      funds: 10_000,
      hp: Some(10),
      power: ActivePower::None,
      has_ammo: true,
      terrain_id: TerrainId::CRoad,
      towers: 0,
      units_id: UnitId::AntiAir,
    };
    let def = DmgCalc {
      cities: 21,
      co: CoId::Adder,
      funds: 0,
      hp: Some(10),
      power: ActivePower::None,
      has_ammo: true,
      terrain_id: TerrainId::CRoad,
      towers: 0,
      units_id: UnitId::Infantry,
    };
    insta::assert_debug_snapshot!(calc(atk, def), @r###"
    Ok(
        (
            94,
            103,
        ),
    )
    "###);
  }
  #[test]
  fn test_colin_vs_adder_aa_vs_inf_scop() {
    let atk = DmgCalc {
      cities: 21,
      co: CoId::Colin,
      funds: 10_000,
      hp: Some(10),
      power: ActivePower::Super,
      has_ammo: true,
      terrain_id: TerrainId::CRoad,
      towers: 0,
      units_id: UnitId::AntiAir,
    };
    let def = DmgCalc {
      cities: 21,
      co: CoId::Adder,
      funds: 0,
      hp: Some(10),
      power: ActivePower::None,
      has_ammo: true,
      terrain_id: TerrainId::CRoad,
      towers: 0,
      units_id: UnitId::Infantry,
    };
    insta::assert_debug_snapshot!(calc(atk, def), @r###"
    Ok(
        (
            136,
            145,
        ),
    )
    "###);
  }
}
