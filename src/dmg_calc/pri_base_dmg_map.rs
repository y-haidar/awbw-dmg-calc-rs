use std::{collections::BTreeMap, sync::OnceLock};

use crate::units::UnitId;

use super::BaseDmgMapKey;

pub fn pri_base_dmg_map() -> &'static BTreeMap<BaseDmgMapKey, u8> {
  static BTREE_MAP: OnceLock<BTreeMap<BaseDmgMapKey, u8>> = OnceLock::new();
  BTREE_MAP.get_or_init(|| {
    let mut m = BTreeMap::new();
    // ================================= Anti-Air
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::AntiAir,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::APC,
      },
      50,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::Artillery,
      },
      50,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::BCopter,
      },
      120,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::Bomber,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::Fighter,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::Infantry,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::MdTank,
      },
      10,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::Mech,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::MegaTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::Missile,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::Neotank,
      },
      5,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::Recon,
      },
      60,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::Rocket,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::Stealth,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::TCopter,
      },
      120,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::AntiAir,
        def: UnitId::Tank,
      },
      25,
    );
    // ================================= APC
    // ================================= Artillery
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::AntiAir,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::APC,
      },
      70,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::Artillery,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::Battleship,
      },
      40,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::BlackBoat,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::Carrier,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::Cruiser,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::Infantry,
      },
      90,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::Lander,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::MdTank,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::Mech,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::MegaTank,
      },
      15,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::Missile,
      },
      80,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::Neotank,
      },
      40,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::Recon,
      },
      80,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::Rocket,
      },
      80,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::Sub,
      },
      60,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Artillery,
        def: UnitId::Tank,
      },
      70,
    );
    // ================================= B-Copter
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::AntiAir,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::APC,
      },
      60,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Artillery,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Battleship,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::BlackBoat,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Carrier,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Cruiser,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Lander,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::MdTank,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::MegaTank,
      },
      10,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Missile,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Neotank,
      },
      20,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Recon,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Rocket,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Sub,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::BCopter,
        def: UnitId::Tank,
      },
      55,
    );
    // ================================= Battleship
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::AntiAir,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::APC,
      },
      80,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::Artillery,
      },
      80,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::Battleship,
      },
      50,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::BlackBoat,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::Carrier,
      },
      60,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::Cruiser,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::Infantry,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::Lander,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::MdTank,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::Mech,
      },
      90,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::MegaTank,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::Missile,
      },
      90,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::Neotank,
      },
      50,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::Recon,
      },
      90,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::Rocket,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::Sub,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Battleship,
        def: UnitId::Tank,
      },
      80,
    );
    // ================================= Black Boat
    // ================================= Black Bomb
    // ================================= Bomber
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::AntiAir,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::APC,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::Artillery,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::Battleship,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::BlackBoat,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::Carrier,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::Cruiser,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::Infantry,
      },
      110,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::Lander,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::MdTank,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::Mech,
      },
      110,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::MegaTank,
      },
      35,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::Missile,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::Neotank,
      },
      90,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::Recon,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::Rocket,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::Sub,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Bomber,
        def: UnitId::Tank,
      },
      105,
    );
    // ================================= Carrier
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Carrier,
        def: UnitId::BCopter,
      },
      115,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Carrier,
        def: UnitId::Bomber,
      },
      100,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Carrier,
        def: UnitId::Fighter,
      },
      100,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Carrier,
        def: UnitId::Stealth,
      },
      100,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Carrier,
        def: UnitId::TCopter,
      },
      115,
    );
    // ================================= Cruiser
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Cruiser,
        def: UnitId::BlackBoat,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Cruiser,
        def: UnitId::Carrier,
      },
      5,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Cruiser,
        def: UnitId::Sub,
      },
      90,
    );
    // ================================= Fighter
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Fighter,
        def: UnitId::BCopter,
      },
      100,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Fighter,
        def: UnitId::Bomber,
      },
      100,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Fighter,
        def: UnitId::Fighter,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Fighter,
        def: UnitId::Stealth,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Fighter,
        def: UnitId::TCopter,
      },
      100,
    );
    // ================================= Infantry
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::AntiAir,
      },
      5,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::APC,
      },
      12,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::Artillery,
      },
      15,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::BCopter,
      },
      7,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::Infantry,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::MdTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::Mech,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::MegaTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::Missile,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::Neotank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::Recon,
      },
      12,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::Rocket,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::TCopter,
      },
      30,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Infantry,
        def: UnitId::Tank,
      },
      5,
    );
    // ================================= Lander
    // ================================= Md. Tank
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::AntiAir,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::APC,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Artillery,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Battleship,
      },
      10,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::BlackBoat,
      },
      35,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Carrier,
      },
      10,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Cruiser,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Lander,
      },
      35,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::MdTank,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::MegaTank,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Missile,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Neotank,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Recon,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Rocket,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Sub,
      },
      10,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MdTank,
        def: UnitId::Tank,
      },
      85,
    );
    // ================================= Mech
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::AntiAir,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::APC,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Artillery,
      },
      70,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::MdTank,
      },
      15,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::MegaTank,
      },
      5,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Missile,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Neotank,
      },
      15,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Recon,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Rocket,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Mech,
        def: UnitId::Tank,
      },
      55,
    );
    // ================================= Mega Tank
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::AntiAir,
      },
      195,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::APC,
      },
      195,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Artillery,
      },
      195,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Battleship,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::BlackBoat,
      },
      105,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Carrier,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Cruiser,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Lander,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::MdTank,
      },
      125,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::MegaTank,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Missile,
      },
      195,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Neotank,
      },
      115,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Recon,
      },
      195,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Rocket,
      },
      195,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Sub,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::MegaTank,
        def: UnitId::Tank,
      },
      180,
    );
    // ================================= Missile
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Missile,
        def: UnitId::BCopter,
      },
      120,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Missile,
        def: UnitId::Bomber,
      },
      100,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Missile,
        def: UnitId::Fighter,
      },
      100,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Missile,
        def: UnitId::Stealth,
      },
      100,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Missile,
        def: UnitId::TCopter,
      },
      120,
    );
    // ================================= Neotank
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::AntiAir,
      },
      115,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::APC,
      },
      125,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Artillery,
      },
      115,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Battleship,
      },
      15,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::BlackBoat,
      },
      40,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Carrier,
      },
      15,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Cruiser,
      },
      50,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Lander,
      },
      40,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::MdTank,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::MegaTank,
      },
      35,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Missile,
      },
      125,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Neotank,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Recon,
      },
      125,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Rocket,
      },
      125,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Sub,
      },
      15,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Neotank,
        def: UnitId::Tank,
      },
      105,
    );
    // ================================= Piperunner
    // ================================= Recon
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::AntiAir,
      },
      4,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::APC,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::Artillery,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::BCopter,
      },
      10,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::Infantry,
      },
      70,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::MdTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::Mech,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::MegaTank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::Missile,
      },
      28,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::Neotank,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::Recon,
      },
      35,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::Rocket,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::TCopter,
      },
      35,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Recon,
        def: UnitId::Tank,
      },
      6,
    );
    // ================================= Rocket
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::AntiAir,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::APC,
      },
      80,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::Artillery,
      },
      80,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::Battleship,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::BlackBoat,
      },
      60,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::Carrier,
      },
      60,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::Cruiser,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::Infantry,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::Lander,
      },
      60,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::MdTank,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::Mech,
      },
      90,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::MegaTank,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::Missile,
      },
      90,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::Neotank,
      },
      50,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::Recon,
      },
      90,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::Rocket,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::Sub,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Rocket,
        def: UnitId::Tank,
      },
      80,
    );
    // ================================= Stealth
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::AntiAir,
      },
      50,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::APC,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Artillery,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::BCopter,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Battleship,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::BlackBoat,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Bomber,
      },
      70,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Carrier,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Cruiser,
      },
      35,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Fighter,
      },
      45,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Infantry,
      },
      90,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Lander,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::MdTank,
      },
      70,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Mech,
      },
      90,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::MegaTank,
      },
      15,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Missile,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Neotank,
      },
      60,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Recon,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Rocket,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Stealth,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Sub,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::TCopter,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Stealth,
        def: UnitId::Tank,
      },
      75,
    );
    // ================================= Sub
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Sub,
        def: UnitId::Battleship,
      },
      55,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Sub,
        def: UnitId::BlackBoat,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Sub,
        def: UnitId::Carrier,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Sub,
        def: UnitId::Cruiser,
      },
      25,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Sub,
        def: UnitId::Lander,
      },
      95,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Sub,
        def: UnitId::Sub,
      },
      55,
    );
    // ================================= T-Copter
    // ================================= Tank
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::AntiAir,
      },
      65,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::APC,
      },
      75,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Artillery,
      },
      70,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Battleship,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::BlackBoat,
      },
      10,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Carrier,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Cruiser,
      },
      5,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Lander,
      },
      10,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::MdTank,
      },
      15,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::MegaTank,
      },
      10,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Missile,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Neotank,
      },
      15,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Recon,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Rocket,
      },
      85,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Sub,
      },
      1,
    );
    m.insert(
      BaseDmgMapKey {
        atk: UnitId::Tank,
        def: UnitId::Tank,
      },
      55,
    );
    m
  })
}
