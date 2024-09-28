use super::*;

#[test]
fn test_lash_aa_vs_aa_city() {
  let atk = DmgCalcInput {
    cities: 21,
    co: CoId::Lash,
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
    co: CoId::Adder,
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
      atk: Damage { min: 58, max: 67 },
      def_took_max: Damage { min: 12, max: 15 },
      def_took_min: Damage { min: 15, max: 18 }
    }
  );
  assert_eq!(
    calc(def, atk).unwrap(),
    DmgCalcOutput {
      atk: Damage { min: 31, max: 37 },
      def_took_max: Damage { min: 41, max: 47 },
      def_took_min: Damage { min: 41, max: 47 }
    }
  );
}
#[test]
fn test_lash_aa_vs_aa_city_scop() {
  let atk = DmgCalcInput {
    cities: 21,
    co: CoId::Lash,
    funds: 10_000,
    hp: Some(100),
    power: ActivePower::Super,
    has_ammo: true,
    terrain_id: TerrainId::GreySkyCity,
    towers: 0,
    units_id: UnitId::AntiAir,
  };
  let def = DmgCalcInput {
    cities: 21,
    co: CoId::Adder,
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
      atk: Damage { min: 76, max: 85 },
      def_took_max: Damage { min: 2, max: 3 },
      def_took_min: Damage { min: 4, max: 4 }
    }
  );
  assert_eq!(
    calc(def, atk).unwrap(),
    DmgCalcOutput {
      atk: Damage { min: 13, max: 16 },
      def_took_max: Damage { min: 68, max: 77 },
      def_took_min: Damage { min: 68, max: 77 }
    }
  );
}
