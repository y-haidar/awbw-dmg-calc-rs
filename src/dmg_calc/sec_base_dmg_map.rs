use std::{collections::BTreeMap, sync::OnceLock};

use crate::units::UnitId;

use super::BaseDmgMapKey;

pub fn sec_base_dmg_map() -> &'static BTreeMap<BaseDmgMapKey, u8> {
  static BTREE_MAP: OnceLock<BTreeMap<BaseDmgMapKey, u8>> = OnceLock::new();
  BTREE_MAP.get_or_init(|| {
    let mut m = BTreeMap::new();
    // ================================= Anti-Air
    // ================================= APC
    // ================================= Artillery
    // ================================= B-Copter
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::AntiAir,
      },
      6,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::APC,
      },
      20,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Artillery,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::BCopter,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Infantry,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::MdTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Mech,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::MegaTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Missile,
      },
      35,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Neotank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Recon,
      },
      30,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Rocket,
      },
      35,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::TCopter,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Tank,
      },
      6,
    );
    // ================================= Battleship
    // ================================= Black Boat
    // ================================= Black Bomb
    // ================================= Bomber
    // ================================= Carrier
    // ================================= Cruiser
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Cruiser,
        def: UnitId::BCopter,
      },
      115,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Cruiser,
        def: UnitId::Bomber,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Cruiser,
        def: UnitId::Fighter,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Cruiser,
        def: UnitId::Stealth,
      },
      100,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Cruiser,
        def: UnitId::TCopter,
      },
      115,
    );
    // ================================= Fighter
    // ================================= Infantry
    // ================================= Lander
    // ================================= Md. Tank
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::AntiAir,
      },
      7,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::APC,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Artillery,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::BCopter,
      },
      12,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Infantry,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::MdTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Mech,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::MegaTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Missile,
      },
      35,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Neotank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Recon,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Rocket,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::TCopter,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Tank,
      },
      8,
    );
    // ================================= Mech
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::AntiAir,
      },
      6,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::APC,
      },
      20,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Artillery,
      },
      32,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::BCopter,
      },
      9,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Infantry,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::MdTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Mech,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::MegaTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Missile,
      },
      35,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Neotank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Recon,
      },
      18,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Rocket,
      },
      35,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::TCopter,
      },
      35,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Tank,
      },
      6,
    );
    // ================================= Mega Tank
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::AntiAir,
      },
      17,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::APC,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Artillery,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::BCopter,
      },
      22,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Infantry,
      },
      135,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::MdTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Mech,
      },
      125,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::MegaTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Missile,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Neotank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Recon,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Rocket,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::TCopter,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Tank,
      },
      10,
    );
    // ================================= Missile
    // ================================= Neotank
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::AntiAir,
      },
      17,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::APC,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Artillery,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::BCopter,
      },
      22,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Infantry,
      },
      125,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::MdTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Mech,
      },
      115,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::MegaTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Missile,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Neotank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Recon,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Rocket,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::TCopter,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Tank,
      },
      10,
    );
    // ================================= Piperunner
    // ================================= Recon
    // ================================= Rocket
    // ================================= Stealth
    // ================================= Sub
    // ================================= T-Copter
    // ================================= Tank
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::AntiAir,
      },
      5,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::APC,
      },
      54,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Artillery,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::BCopter,
      },
      10,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Infantry,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::MdTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Mech,
      },
      70,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::MegaTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Missile,
      },
      30,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Neotank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Recon,
      },
      40,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Rocket,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::TCopter,
      },
      40,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Tank,
      },
      6,
    );

    m
  })
}
