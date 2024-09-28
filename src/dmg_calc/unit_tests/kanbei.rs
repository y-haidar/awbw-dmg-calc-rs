use super::*;

#[test]
fn test_colin_vs_kanbei_aa_vs_aa_city() {
  let atk = DmgCalcInput {
    cities: 21,
    co: CoId::Colin,
    funds: 10_000,
    hp: Some(100),
    power: ActivePower::None,
    has_ammo: true,
    terrain_id: TerrainId::GreySkyCity,
    towers: 0,
    units_id: UnitId::AntiAir,
  };
  let def = DmgCalcInput {
    cities: 21,
    co: CoId::Kanbei,
    funds: 0,
    hp: Some(100),
    power: ActivePower::None,
    has_ammo: true,
    terrain_id: TerrainId::CRoad,
    towers: 0,
    units_id: UnitId::AntiAir,
  };
  assert_eq!(
    calc(atk, def).unwrap(),
    DmgCalcOutput {
      atk: Damage { min: 28, max: 34 },
      def_took_max: Damage { min: 28, max: 33 },
      def_took_min: Damage { min: 32, max: 37 },
    }
  );
}
#[test]
fn test_colin_vs_kanbei_aa_vs_aa_scop_vs_scop() {
  let atk = DmgCalcInput {
    cities: 21,
    co: CoId::Colin,
    funds: 10_000,
    hp: Some(100),
    power: ActivePower::Super,
    has_ammo: true,
    terrain_id: TerrainId::CRoad,
    towers: 0,
    units_id: UnitId::AntiAir,
  };
  let def = DmgCalcInput {
    cities: 21,
    co: CoId::Kanbei,
    funds: 0,
    hp: Some(100),
    power: ActivePower::Super,
    has_ammo: true,
    terrain_id: TerrainId::CRoad,
    towers: 0,
    units_id: UnitId::AntiAir,
  };
  assert_eq!(
    calc(atk, def).unwrap(),
    DmgCalcOutput {
      atk: Damage { min: 23, max: 27 },
      def_took_max: Damage { min: 69, max: 76 },
      def_took_min: Damage { min: 69, max: 76 },
    }
  );
}
#[test]
fn test_kanbei_vs_colin_aa_vs_aa_scop_vs_scop() {
  let atk = DmgCalcInput {
    cities: 21,
    co: CoId::Kanbei,
    funds: 10_000,
    hp: Some(100),
    power: ActivePower::Super,
    has_ammo: true,
    terrain_id: TerrainId::CRoad,
    towers: 0,
    units_id: UnitId::AntiAir,
  };
  let def = DmgCalcInput {
    cities: 21,
    co: CoId::Colin,
    funds: 10_000,
    hp: Some(100),
    power: ActivePower::Super,
    has_ammo: true,
    terrain_id: TerrainId::CRoad,
    towers: 0,
    units_id: UnitId::AntiAir,
  };
  assert_eq!(
    calc(atk, def).unwrap(),
    DmgCalcOutput {
      atk: Damage { min: 60, max: 68 },
      def_took_max: Damage { min: 9, max: 10 },
      def_took_min: Damage { min: 9, max: 10 },
    }
  );
}
