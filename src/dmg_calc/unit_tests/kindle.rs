use super::*;

#[test]
fn test_kindle_aa_vs_aa_road() {
  let atk = DmgCalcInput {
    cities: 21,
    co: CoId::Kindle,
    funds: 10_000,
    hp: Some(100),
    power: ActivePower::None,
    has_ammo: true,
    terrain_id: TerrainId::CRoad,
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
      atk: Damage { min: 45, max: 54 },
      def_took_max: Damage { min: 22, max: 27 },
      def_took_min: Damage { min: 27, max: 32 }
    }
  );
  assert_eq!(
    calc(def, atk).unwrap(),
    DmgCalcOutput {
      atk: Damage { min: 45, max: 54 },
      def_took_max: Damage { min: 22, max: 27 },
      def_took_min: Damage { min: 27, max: 32 }
    }
  );
}
#[test]
fn test_kindle_aa_vs_aa_city() {
  let atk = DmgCalcInput {
    cities: 21,
    co: CoId::Kindle,
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
      atk: Damage { min: 63, max: 72 },
      def_took_max: Damage { min: 9, max: 11 },
      def_took_min: Damage { min: 12, max: 15 }
    }
  );
  assert_eq!(
    calc(def, atk).unwrap(),
    DmgCalcOutput {
      atk: Damage { min: 31, max: 37 },
      def_took_max: Damage { min: 44, max: 50 },
      def_took_min: Damage { min: 44, max: 50 }
    }
  );
}
#[test]
fn test_kindle_aa_vs_aa_city_cop() {
  let atk = DmgCalcInput {
    cities: 21,
    co: CoId::Kindle,
    funds: 10_000,
    hp: Some(100),
    power: ActivePower::Normal,
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
      atk: Damage { min: 85, max: 94 },
      def_took_max: Damage { min: 2, max: 3 },
      def_took_min: Damage { min: 5, max: 6 }
    }
  );
  assert_eq!(
    calc(def, atk).unwrap(),
    DmgCalcOutput {
      atk: Damage { min: 27, max: 32 },
      def_took_max: Damage { min: 59, max: 66 },
      def_took_min: Damage { min: 68, max: 75 }
    }
  );
}
#[test]
fn test_kindle_aa_vs_aa_city_scop() {
  let atk = DmgCalcInput {
    cities: 21,
    co: CoId::Kindle,
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
      atk: Damage { min: 108, max: 117 },
      def_took_max: Damage { min: 0, max: 0 },
      def_took_min: Damage { min: 0, max: 0 }
    }
  );
  assert_eq!(
    calc(def, atk).unwrap(),
    DmgCalcOutput {
      atk: Damage { min: 27, max: 32 },
      def_took_max: Damage { min: 75, max: 81 },
      def_took_min: Damage { min: 86, max: 93 }
    }
  );
}
