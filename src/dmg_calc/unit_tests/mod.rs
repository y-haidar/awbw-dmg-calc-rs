use super::*;

mod kanbei;

#[test]
fn test_adder_vs_adder_aa_vs_inf() {
  let atk = DmgCalcInput {
    cities: 21,
    co: CoId::Adder,
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
    units_id: UnitId::Infantry,
  };
  insta::assert_debug_snapshot!(calc(atk, def), @r###"
    Ok(
        DmgCalcOutput {
            atk: Damage {
                min: 105,
                max: 114,
            },
            def_took_max: Damage {
                min: 0,
                max: 0,
            },
            def_took_min: Damage {
                min: 0,
                max: 0,
            },
        },
    )
    "###);
}
#[test]
fn test_adder_vs_adder_aa_vs_inf_cop() {
  let atk = DmgCalcInput {
    cities: 21,
    co: CoId::Adder,
    funds: 10_000,
    hp: Some(100),
    power: ActivePower::Normal,
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
    units_id: UnitId::Infantry,
  };
  insta::assert_debug_snapshot!(calc(atk, def), @r###"
    Ok(
        DmgCalcOutput {
            atk: Damage {
                min: 115,
                max: 124,
            },
            def_took_max: Damage {
                min: 0,
                max: 0,
            },
            def_took_min: Damage {
                min: 0,
                max: 0,
            },
        },
    )
    "###);
}

#[test]
fn test_colin_vs_adder_aa_vs_inf() {
  let atk = DmgCalcInput {
    cities: 21,
    co: CoId::Colin,
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
    units_id: UnitId::Infantry,
  };
  insta::assert_debug_snapshot!(calc(atk, def), @r###"
    Ok(
        DmgCalcOutput {
            atk: Damage {
                min: 94,
                max: 103,
            },
            def_took_max: Damage {
                min: 0,
                max: 0,
            },
            def_took_min: Damage {
                min: 0,
                max: 1,
            },
        },
    )
    "###);
}
#[test]
fn test_colin_vs_adder_aa_vs_inf_scop() {
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
    co: CoId::Adder,
    funds: 0,
    hp: Some(100),
    power: ActivePower::None,
    has_ammo: true,
    terrain_id: TerrainId::CRoad,
    towers: 0,
    units_id: UnitId::Infantry,
  };
  insta::assert_debug_snapshot!(calc(atk, def), @r###"
    Ok(
        DmgCalcOutput {
            atk: Damage {
                min: 136,
                max: 145,
            },
            def_took_max: Damage {
                min: 0,
                max: 0,
            },
            def_took_min: Damage {
                min: 0,
                max: 0,
            },
        },
    )
    "###);
}
#[test]
fn test_colin_vs_adder_aa_vs_aa() {
  let atk = DmgCalcInput {
    cities: 21,
    co: CoId::Colin,
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
  insta::assert_debug_snapshot!(calc(atk, def), @r###"
    Ok(
        DmgCalcOutput {
            atk: Damage {
                min: 40,
                max: 49,
            },
            def_took_max: Damage {
                min: 27,
                max: 32,
            },
            def_took_min: Damage {
                min: 27,
                max: 32,
            },
        },
    )
    "###);
}
#[test]
fn test_colin_vs_adder_aa_vs_aa_scop() {
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
    co: CoId::Adder,
    funds: 0,
    hp: Some(100),
    power: ActivePower::None,
    has_ammo: true,
    terrain_id: TerrainId::CRoad,
    towers: 0,
    units_id: UnitId::AntiAir,
  };
  insta::assert_debug_snapshot!(calc(atk, def), @r###"
    Ok(
        DmgCalcOutput {
            atk: Damage {
                min: 58,
                max: 67,
            },
            def_took_max: Damage {
                min: 16,
                max: 19,
            },
            def_took_min: Damage {
                min: 20,
                max: 24,
            },
        },
    )
    "###);
}
