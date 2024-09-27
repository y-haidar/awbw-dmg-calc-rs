use std::{collections::BTreeMap, sync::OnceLock};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum TerrainId {
  // https://raw.githubusercontent.com/DeamonHunter/AWBW-Replay-Player/9c3a314d716beae222c8ef068bafeaabdf49df1a/AWBWApp.Resources/Json/Tiles.json
  Plain = 1,
  Mountain = 2,
  Wood = 3,
  HRiver = 4,
  VRiver = 5,
  CRiver = 6,
  ESRiver = 7,
  SWRiver = 8,
  WNRiver = 9,
  NERiver = 10,
  ESWRiver = 11,
  SWNRiver = 12,
  WNERiver = 13,
  NESRiver = 14,
  HRoad = 15,
  VRoad = 16,
  CRoad = 17,
  ESRoad = 18,
  SWRoad = 19,
  WNRoad = 20,
  NERoad = 21,
  ESWRoad = 22,
  SWNRoad = 23,
  WNERoad = 24,
  NESRoad = 25,
  HBridge = 26,
  VBridge = 27,
  Sea = 28,
  Shoal_N = 29,
  Shoal_S = 30,
  Shoal_E = 31,
  Shoal_W = 32,
  Reef = 33,
  VPipe = 101,
  HPipe = 102,
  NEPipe = 103,
  ESPipe = 104,
  SWPipe = 105,
  WNPipe = 106,
  NPipeEnd = 107,
  EPipeEnd = 108,
  SPipeEnd = 109,
  WPipeEnd = 110,
  MissileSiloEmpty = 112,
  HPipeRubble = 115,
  VPipeRubble = 116,
  TeleportTile = 195,
  Sea_N_E_S_W = 1000,
  Sea_E_S_W = 1001,
  Sea_N_S_W = 1002,
  Sea_N_E_W = 1003,
  Sea_N_E_S = 1004,
  Sea_N_S = 1005,
  Sea_E_W = 1006,
  Sea_N_E_SW = 1007,
  Sea_N_E = 1008,
  Sea_E_S_NW = 1009,
  Sea_E_S = 1010,
  Sea_S_W_NE = 1011,
  Sea_S_W = 1012,
  Sea_N_W_SE = 1013,
  Sea_N_W = 1014,
  Sea_N_SE_SW = 1015,
  Sea_N_SE = 1016,
  Sea_N_SW = 1017,
  Sea_N = 1018,
  Sea_E_NW_SW = 1019,
  Sea_E_NW = 1020,
  Sea_E_SW = 1021,
  Sea_E = 1022,
  Sea_S_NW_NE = 1023,
  Sea_S_NW = 1024,
  Sea_S_NE = 1025,
  Sea_S = 1026,
  Sea_W_NE_SE = 1027,
  Sea_W_NE = 1028,
  Sea_W_SE = 1029,
  Sea_W = 1030,
  Sea_NW_NE_SE_SW = 1031,
  Sea_NE_SE_SW = 1032,
  Sea_NW_SE_SW = 1033,
  Sea_NW_NE_SW = 1034,
  Sea_NW_NE_SE = 1035,
  Sea_SE_SW = 1036,
  Sea_NW_SW = 1037,
  Sea_NW_NE = 1038,
  Sea_NE_SE = 1039,
  Sea_NW_SE = 1040,
  Sea_NE_SW = 1041,
  Sea_NW = 1042,
  Sea_NE = 1043,
  Sea_SE = 1044,
  Sea_SW = 1045,
  Shoal_C = 1046,
  Shoal_AN = 1047,
  Shoal_AE = 1048,
  Shoal_AS = 1049,
  Shoal_AW = 1050,
  Shoal_N_E = 1051,
  Shoal_N_AE = 1052,
  Shoal_AN_E = 1053,
  Shoal_AN_AE = 1054,
  Shoal_N_S = 1055,
  Shoal_N_AS = 1056,
  Shoal_AN_S = 1057,
  Shoal_AN_AS = 1058,
  Shoal_N_W = 1059,
  Shoal_N_AW = 1060,
  Shoal_AN_W = 1061,
  Shoal_AN_AW = 1062,
  Shoal_E_S = 1063,
  Shoal_E_AS = 1064,
  Shoal_AE_S = 1065,
  Shoal_AE_AS = 1066,
  Shoal_E_W = 1067,
  Shoal_E_AW = 1068,
  Shoal_AE_W = 1069,
  Shoal_AE_AW = 1070,
  Shoal_S_W = 1071,
  Shoal_S_AW = 1072,
  Shoal_AS_W = 1073,
  Shoal_AS_AW = 1074,
  Shoal_N_E_S = 1075,
  Shoal_N_E_AS = 1076,
  Shoal_N_AE_S = 1077,
  Shoal_N_AE_AS = 1078,
  Shoal_AN_E_S = 1079,
  Shoal_AN_E_AS = 1080,
  Shoal_AN_AE_S = 1081,
  Shoal_AN_AE_AS = 1082,
  Shoal_N_S_W = 1083,
  Shoal_N_S_AW = 1084,
  Shoal_N_AS_W = 1085,
  Shoal_N_AS_AW = 1086,
  Shoal_AN_S_W = 1087,
  Shoal_AN_S_AW = 1088,
  Shoal_AN_AS_W = 1089,
  Shoal_AN_AS_AW = 1090,
  Shoal_N_E_W = 1091,
  Shoal_N_E_AW = 1092,
  Shoal_N_AE_W = 1093,
  Shoal_N_AE_AW = 1094,
  Shoal_AN_E_W = 1095,
  Shoal_AN_E_AW = 1096,
  Shoal_AN_AE_W = 1097,
  Shoal_AN_AE_AW = 1098,
  Shoal_E_S_W = 1099,
  Shoal_E_S_AW = 1100,
  Shoal_E_AS_W = 1101,
  Shoal_E_AS_AW = 1102,
  Shoal_AE_S_W = 1103,
  Shoal_AE_S_AW = 1104,
  Shoal_AE_AS_W = 1105,
  Shoal_AE_AS_AW = 1106,
  Shoal_N_E_S_W = 1107,
  Shoal_N_E_S_AW = 1108,
  Shoal_N_E_AS_W = 1109,
  Shoal_N_E_AS_AW = 1110,
  Shoal_N_AE_S_W = 1111,
  Shoal_N_AE_S_AW = 1112,
  Shoal_N_AE_AS_W = 1113,
  Shoal_N_AE_AS_AW = 1114,
  Shoal_AN_E_S_W = 1115,
  Shoal_AN_E_S_AW = 1116,
  Shoal_AN_E_AS_W = 1117,
  Shoal_AN_E_AS_AW = 1118,
  Shoal_AN_AE_S_W = 1119,
  Shoal_AN_AE_S_AW = 1120,
  Shoal_AN_AE_AS_W = 1121,
  Shoal_AN_AE_AS_AW = 1122,
  // https://raw.githubusercontent.com/DeamonHunter/AWBW-Replay-Player/9c3a314d716beae222c8ef068bafeaabdf49df1a/AWBWApp.Resources/Json/Buildings.json
  NeutralCity = 34,
  NeutralBase = 35,
  NeutralAirport = 36,
  NeutralPort = 37,
  NeutralComTower = 133,
  NeutralLab = 145,
  MissileSilo = 111,
  HorizontalPipeSeam = 113,
  VerticalPipeSeam = 114,
  OrangeStarAirport = 40,
  OrangeStarBase = 39,
  OrangeStarCity = 38,
  OrangeStarHQ = 42,
  OrangeStarPort = 41,
  OrangeStarComTower = 134,
  OrangeStarLab = 146,
  BlueMoonAirport = 45,
  BlueMoonBase = 44,
  BlueMoonCity = 43,
  BlueMoonHQ = 47,
  BlueMoonPort = 46,
  BlueMoonComTower = 129,
  BlueMoonLab = 140,
  GreenEarthAirport = 50,
  GreenEarthBase = 49,
  GreenEarthCity = 48,
  GreenEarthHQ = 52,
  GreenEarthPort = 51,
  GreenEarthComTower = 131,
  GreenEarthLab = 142,
  YellowCometAirport = 55,
  YellowCometBase = 54,
  YellowCometCity = 53,
  YellowCometHQ = 57,
  YellowCometPort = 56,
  YellowCometComTower = 136,
  YellowCometLab = 148,
  RedFireAirport = 83,
  RedFireBase = 82,
  RedFireCity = 81,
  RedFireHQ = 85,
  RedFirePort = 84,
  RedFireComTower = 135,
  RedFireLab = 147,
  GreySkyAirport = 88,
  GreySkyBase = 87,
  GreySkyCity = 86,
  GreySkyHQ = 90,
  GreySkyPort = 89,
  GreySkyComTower = 137,
  GreySkyLab = 143,
  BlackHoleAirport = 93,
  BlackHoleBase = 92,
  BlackHoleCity = 91,
  BlackHoleHQ = 95,
  BlackHolePort = 94,
  BlackHoleComTower = 128,
  BlackHoleLab = 139,
  BrownDesertAirport = 98,
  BrownDesertBase = 97,
  BrownDesertCity = 96,
  BrownDesertHQ = 100,
  BrownDesertPort = 99,
  BrownDesertComTower = 130,
  BrownDesertLab = 141,
  AmberBlazeAirport = 117,
  AmberBlazeBase = 118,
  AmberBlazeCity = 119,
  AmberBlazeHQ = 120,
  AmberBlazePort = 121,
  AmberBlazeComTower = 127,
  AmberBlazeLab = 138,
  JadeSunAirport = 122,
  JadeSunBase = 123,
  JadeSunCity = 124,
  JadeSunHQ = 125,
  JadeSunPort = 126,
  JadeSunComTower = 132,
  JadeSunLab = 144,
  CobaltIceAirport = 149,
  CobaltIceBase = 150,
  CobaltIceCity = 151,
  CobaltIceHQ = 153,
  CobaltIcePort = 155,
  CobaltIceComTower = 152,
  CobaltIceLab = 154,
  PinkCosmosAirport = 156,
  PinkCosmosBase = 157,
  PinkCosmosCity = 158,
  PinkCosmosHQ = 160,
  PinkCosmosPort = 162,
  PinkCosmosComTower = 159,
  PinkCosmosLab = 161,
  TealGalaxyAirport = 163,
  TealGalaxyBase = 164,
  TealGalaxyCity = 165,
  TealGalaxyHQ = 167,
  TealGalaxyPort = 169,
  TealGalaxyComTower = 166,
  TealGalaxyLab = 168,
  PurpleLightningAirport = 170,
  PurpleLightningBase = 171,
  PurpleLightningCity = 172,
  PurpleLightningHQ = 174,
  PurpleLightningPort = 176,
  PurpleLightningComTower = 173,
  PurpleLightningLab = 175,
  AcidRainAirport = 181,
  AcidRainBase = 182,
  AcidRainCity = 183,
  AcidRainHQ = 185,
  AcidRainPort = 187,
  AcidRainComTower = 184,
  AcidRainLab = 186,
  WhiteNovaAirport = 188,
  WhiteNovaBase = 189,
  WhiteNovaCity = 190,
  WhiteNovaHQ = 192,
  WhiteNovaPort = 194,
  WhiteNovaComTower = 191,
  WhiteNovaLab = 193,
  AzureAsteroidAirport = 196,
  AzureAsteroidBase = 197,
  AzureAsteroidCity = 198,
  AzureAsteroidHQ = 200,
  AzureAsteroidPort = 202,
  AzureAsteroidComTower = 199,
  AzureAsteroidLab = 201,
  NoirEclipseAirport = 203,
  NoirEclipseBase = 204,
  NoirEclipseCity = 205,
  NoirEclipseHQ = 207,
  NoirEclipsePort = 209,
  NoirEclipseComTower = 206,
  NoirEclipseLab = 208,
}

pub const ROAD_TILES_ID: [TerrainId; 11] = [
  TerrainId::HRoad,
  TerrainId::VRoad,
  TerrainId::CRoad,
  TerrainId::ESRoad,
  TerrainId::SWRoad,
  TerrainId::WNRoad,
  TerrainId::NERoad,
  TerrainId::ESWRoad,
  TerrainId::SWNRoad,
  TerrainId::WNERoad,
  TerrainId::NESRoad,
];

pub fn terrain_stars_map() -> &'static BTreeMap<TerrainId, u8> {
  static BTREE_MAP: OnceLock<BTreeMap<TerrainId, u8>> = OnceLock::new();
  BTREE_MAP.get_or_init(|| {
    let mut m = BTreeMap::new();
    m.insert(TerrainId::Plain, 1);
    m.insert(TerrainId::Mountain, 4);
    m.insert(TerrainId::Wood, 2);
    m.insert(TerrainId::HRiver, 0);
    m.insert(TerrainId::VRiver, 0);
    m.insert(TerrainId::CRiver, 0);
    m.insert(TerrainId::ESRiver, 0);
    m.insert(TerrainId::SWRiver, 0);
    m.insert(TerrainId::WNRiver, 0);
    m.insert(TerrainId::NERiver, 0);
    m.insert(TerrainId::ESWRiver, 0);
    m.insert(TerrainId::SWNRiver, 0);
    m.insert(TerrainId::WNERiver, 0);
    m.insert(TerrainId::NESRiver, 0);
    m.insert(TerrainId::HRoad, 0);
    m.insert(TerrainId::VRoad, 0);
    m.insert(TerrainId::CRoad, 0);
    m.insert(TerrainId::ESRoad, 0);
    m.insert(TerrainId::SWRoad, 0);
    m.insert(TerrainId::WNRoad, 0);
    m.insert(TerrainId::NERoad, 0);
    m.insert(TerrainId::ESWRoad, 0);
    m.insert(TerrainId::SWNRoad, 0);
    m.insert(TerrainId::WNERoad, 0);
    m.insert(TerrainId::NESRoad, 0);
    m.insert(TerrainId::HBridge, 0);
    m.insert(TerrainId::VBridge, 0);
    m.insert(TerrainId::Sea, 0);
    m.insert(TerrainId::Shoal_N, 0);
    m.insert(TerrainId::Shoal_S, 0);
    m.insert(TerrainId::Shoal_E, 0);
    m.insert(TerrainId::Shoal_W, 0);
    m.insert(TerrainId::Reef, 1);
    m.insert(TerrainId::VPipe, 0);
    m.insert(TerrainId::HPipe, 0);
    m.insert(TerrainId::NEPipe, 0);
    m.insert(TerrainId::ESPipe, 0);
    m.insert(TerrainId::SWPipe, 0);
    m.insert(TerrainId::WNPipe, 0);
    m.insert(TerrainId::NPipeEnd, 0);
    m.insert(TerrainId::EPipeEnd, 0);
    m.insert(TerrainId::SPipeEnd, 0);
    m.insert(TerrainId::WPipeEnd, 0);
    m.insert(TerrainId::MissileSiloEmpty, 3);
    m.insert(TerrainId::HPipeRubble, 1);
    m.insert(TerrainId::VPipeRubble, 1);
    m.insert(TerrainId::TeleportTile, 1);
    m.insert(TerrainId::Sea_N_E_S_W, 0);
    m.insert(TerrainId::Sea_E_S_W, 0);
    m.insert(TerrainId::Sea_N_S_W, 0);
    m.insert(TerrainId::Sea_N_E_W, 0);
    m.insert(TerrainId::Sea_N_E_S, 0);
    m.insert(TerrainId::Sea_N_S, 0);
    m.insert(TerrainId::Sea_E_W, 0);
    m.insert(TerrainId::Sea_N_E_SW, 0);
    m.insert(TerrainId::Sea_N_E, 0);
    m.insert(TerrainId::Sea_E_S_NW, 0);
    m.insert(TerrainId::Sea_E_S, 0);
    m.insert(TerrainId::Sea_S_W_NE, 0);
    m.insert(TerrainId::Sea_S_W, 0);
    m.insert(TerrainId::Sea_N_W_SE, 0);
    m.insert(TerrainId::Sea_N_W, 0);
    m.insert(TerrainId::Sea_N_SE_SW, 0);
    m.insert(TerrainId::Sea_N_SE, 0);
    m.insert(TerrainId::Sea_N_SW, 0);
    m.insert(TerrainId::Sea_N, 0);
    m.insert(TerrainId::Sea_E_NW_SW, 0);
    m.insert(TerrainId::Sea_E_NW, 0);
    m.insert(TerrainId::Sea_E_SW, 0);
    m.insert(TerrainId::Sea_E, 0);
    m.insert(TerrainId::Sea_S_NW_NE, 0);
    m.insert(TerrainId::Sea_S_NW, 0);
    m.insert(TerrainId::Sea_S_NE, 0);
    m.insert(TerrainId::Sea_S, 0);
    m.insert(TerrainId::Sea_W_NE_SE, 0);
    m.insert(TerrainId::Sea_W_NE, 0);
    m.insert(TerrainId::Sea_W_SE, 0);
    m.insert(TerrainId::Sea_W, 0);
    m.insert(TerrainId::Sea_NW_NE_SE_SW, 0);
    m.insert(TerrainId::Sea_NE_SE_SW, 0);
    m.insert(TerrainId::Sea_NW_SE_SW, 0);
    m.insert(TerrainId::Sea_NW_NE_SW, 0);
    m.insert(TerrainId::Sea_NW_NE_SE, 0);
    m.insert(TerrainId::Sea_SE_SW, 0);
    m.insert(TerrainId::Sea_NW_SW, 0);
    m.insert(TerrainId::Sea_NW_NE, 0);
    m.insert(TerrainId::Sea_NE_SE, 0);
    m.insert(TerrainId::Sea_NW_SE, 0);
    m.insert(TerrainId::Sea_NE_SW, 0);
    m.insert(TerrainId::Sea_NW, 0);
    m.insert(TerrainId::Sea_NE, 0);
    m.insert(TerrainId::Sea_SE, 0);
    m.insert(TerrainId::Sea_SW, 0);
    m.insert(TerrainId::Shoal_C, 0);
    m.insert(TerrainId::Shoal_AN, 0);
    m.insert(TerrainId::Shoal_AE, 0);
    m.insert(TerrainId::Shoal_AS, 0);
    m.insert(TerrainId::Shoal_AW, 0);
    m.insert(TerrainId::Shoal_N_E, 0);
    m.insert(TerrainId::Shoal_N_AE, 0);
    m.insert(TerrainId::Shoal_AN_E, 0);
    m.insert(TerrainId::Shoal_AN_AE, 0);
    m.insert(TerrainId::Shoal_N_S, 0);
    m.insert(TerrainId::Shoal_N_AS, 0);
    m.insert(TerrainId::Shoal_AN_S, 0);
    m.insert(TerrainId::Shoal_AN_AS, 0);
    m.insert(TerrainId::Shoal_N_W, 0);
    m.insert(TerrainId::Shoal_N_AW, 0);
    m.insert(TerrainId::Shoal_AN_W, 0);
    m.insert(TerrainId::Shoal_AN_AW, 0);
    m.insert(TerrainId::Shoal_E_S, 0);
    m.insert(TerrainId::Shoal_E_AS, 0);
    m.insert(TerrainId::Shoal_AE_S, 0);
    m.insert(TerrainId::Shoal_AE_AS, 0);
    m.insert(TerrainId::Shoal_E_W, 0);
    m.insert(TerrainId::Shoal_E_AW, 0);
    m.insert(TerrainId::Shoal_AE_W, 0);
    m.insert(TerrainId::Shoal_AE_AW, 0);
    m.insert(TerrainId::Shoal_S_W, 0);
    m.insert(TerrainId::Shoal_S_AW, 0);
    m.insert(TerrainId::Shoal_AS_W, 0);
    m.insert(TerrainId::Shoal_AS_AW, 0);
    m.insert(TerrainId::Shoal_N_E_S, 0);
    m.insert(TerrainId::Shoal_N_E_AS, 0);
    m.insert(TerrainId::Shoal_N_AE_S, 0);
    m.insert(TerrainId::Shoal_N_AE_AS, 0);
    m.insert(TerrainId::Shoal_AN_E_S, 0);
    m.insert(TerrainId::Shoal_AN_E_AS, 0);
    m.insert(TerrainId::Shoal_AN_AE_S, 0);
    m.insert(TerrainId::Shoal_AN_AE_AS, 0);
    m.insert(TerrainId::Shoal_N_S_W, 0);
    m.insert(TerrainId::Shoal_N_S_AW, 0);
    m.insert(TerrainId::Shoal_N_AS_W, 0);
    m.insert(TerrainId::Shoal_N_AS_AW, 0);
    m.insert(TerrainId::Shoal_AN_S_W, 0);
    m.insert(TerrainId::Shoal_AN_S_AW, 0);
    m.insert(TerrainId::Shoal_AN_AS_W, 0);
    m.insert(TerrainId::Shoal_AN_AS_AW, 0);
    m.insert(TerrainId::Shoal_N_E_W, 0);
    m.insert(TerrainId::Shoal_N_E_AW, 0);
    m.insert(TerrainId::Shoal_N_AE_W, 0);
    m.insert(TerrainId::Shoal_N_AE_AW, 0);
    m.insert(TerrainId::Shoal_AN_E_W, 0);
    m.insert(TerrainId::Shoal_AN_E_AW, 0);
    m.insert(TerrainId::Shoal_AN_AE_W, 0);
    m.insert(TerrainId::Shoal_AN_AE_AW, 0);
    m.insert(TerrainId::Shoal_E_S_W, 0);
    m.insert(TerrainId::Shoal_E_S_AW, 0);
    m.insert(TerrainId::Shoal_E_AS_W, 0);
    m.insert(TerrainId::Shoal_E_AS_AW, 0);
    m.insert(TerrainId::Shoal_AE_S_W, 0);
    m.insert(TerrainId::Shoal_AE_S_AW, 0);
    m.insert(TerrainId::Shoal_AE_AS_W, 0);
    m.insert(TerrainId::Shoal_AE_AS_AW, 0);
    m.insert(TerrainId::Shoal_N_E_S_W, 0);
    m.insert(TerrainId::Shoal_N_E_S_AW, 0);
    m.insert(TerrainId::Shoal_N_E_AS_W, 0);
    m.insert(TerrainId::Shoal_N_E_AS_AW, 0);
    m.insert(TerrainId::Shoal_N_AE_S_W, 0);
    m.insert(TerrainId::Shoal_N_AE_S_AW, 0);
    m.insert(TerrainId::Shoal_N_AE_AS_W, 0);
    m.insert(TerrainId::Shoal_N_AE_AS_AW, 0);
    m.insert(TerrainId::Shoal_AN_E_S_W, 0);
    m.insert(TerrainId::Shoal_AN_E_S_AW, 0);
    m.insert(TerrainId::Shoal_AN_E_AS_W, 0);
    m.insert(TerrainId::Shoal_AN_E_AS_AW, 0);
    m.insert(TerrainId::Shoal_AN_AE_S_W, 0);
    m.insert(TerrainId::Shoal_AN_AE_S_AW, 0);
    m.insert(TerrainId::Shoal_AN_AE_AS_W, 0);
    m.insert(TerrainId::Shoal_AN_AE_AS_AW, 0);
    // Buildings
    m.insert(TerrainId::NeutralCity, 3);
    m.insert(TerrainId::NeutralBase, 3);
    m.insert(TerrainId::NeutralAirport, 3);
    m.insert(TerrainId::NeutralPort, 3);
    m.insert(TerrainId::NeutralComTower, 3);
    m.insert(TerrainId::NeutralLab, 3);
    m.insert(TerrainId::MissileSilo, 3);
    m.insert(TerrainId::HorizontalPipeSeam, 0);
    m.insert(TerrainId::VerticalPipeSeam, 0);
    m.insert(TerrainId::OrangeStarAirport, 3);
    m.insert(TerrainId::OrangeStarBase, 3);
    m.insert(TerrainId::OrangeStarCity, 3);
    m.insert(TerrainId::OrangeStarHQ, 4);
    m.insert(TerrainId::OrangeStarPort, 3);
    m.insert(TerrainId::OrangeStarComTower, 3);
    m.insert(TerrainId::OrangeStarLab, 3);
    m.insert(TerrainId::BlueMoonAirport, 3);
    m.insert(TerrainId::BlueMoonBase, 3);
    m.insert(TerrainId::BlueMoonCity, 3);
    m.insert(TerrainId::BlueMoonHQ, 4);
    m.insert(TerrainId::BlueMoonPort, 3);
    m.insert(TerrainId::BlueMoonComTower, 3);
    m.insert(TerrainId::BlueMoonLab, 3);
    m.insert(TerrainId::GreenEarthAirport, 3);
    m.insert(TerrainId::GreenEarthBase, 3);
    m.insert(TerrainId::GreenEarthCity, 3);
    m.insert(TerrainId::GreenEarthHQ, 4);
    m.insert(TerrainId::GreenEarthPort, 3);
    m.insert(TerrainId::GreenEarthComTower, 3);
    m.insert(TerrainId::GreenEarthLab, 3);
    m.insert(TerrainId::YellowCometAirport, 3);
    m.insert(TerrainId::YellowCometBase, 3);
    m.insert(TerrainId::YellowCometCity, 3);
    m.insert(TerrainId::YellowCometHQ, 4);
    m.insert(TerrainId::YellowCometPort, 3);
    m.insert(TerrainId::YellowCometComTower, 3);
    m.insert(TerrainId::YellowCometLab, 3);
    m.insert(TerrainId::RedFireAirport, 3);
    m.insert(TerrainId::RedFireBase, 3);
    m.insert(TerrainId::RedFireCity, 3);
    m.insert(TerrainId::RedFireHQ, 4);
    m.insert(TerrainId::RedFirePort, 3);
    m.insert(TerrainId::RedFireComTower, 3);
    m.insert(TerrainId::RedFireLab, 3);
    m.insert(TerrainId::GreySkyAirport, 3);
    m.insert(TerrainId::GreySkyBase, 3);
    m.insert(TerrainId::GreySkyCity, 3);
    m.insert(TerrainId::GreySkyHQ, 4);
    m.insert(TerrainId::GreySkyPort, 3);
    m.insert(TerrainId::GreySkyComTower, 3);
    m.insert(TerrainId::GreySkyLab, 3);
    m.insert(TerrainId::BlackHoleAirport, 3);
    m.insert(TerrainId::BlackHoleBase, 3);
    m.insert(TerrainId::BlackHoleCity, 3);
    m.insert(TerrainId::BlackHoleHQ, 4);
    m.insert(TerrainId::BlackHolePort, 3);
    m.insert(TerrainId::BlackHoleComTower, 3);
    m.insert(TerrainId::BlackHoleLab, 3);
    m.insert(TerrainId::BrownDesertAirport, 3);
    m.insert(TerrainId::BrownDesertBase, 3);
    m.insert(TerrainId::BrownDesertCity, 3);
    m.insert(TerrainId::BrownDesertHQ, 4);
    m.insert(TerrainId::BrownDesertPort, 3);
    m.insert(TerrainId::BrownDesertComTower, 3);
    m.insert(TerrainId::BrownDesertLab, 3);
    m.insert(TerrainId::AmberBlazeAirport, 3);
    m.insert(TerrainId::AmberBlazeBase, 3);
    m.insert(TerrainId::AmberBlazeCity, 3);
    m.insert(TerrainId::AmberBlazeHQ, 4);
    m.insert(TerrainId::AmberBlazePort, 3);
    m.insert(TerrainId::AmberBlazeComTower, 3);
    m.insert(TerrainId::AmberBlazeLab, 3);
    m.insert(TerrainId::JadeSunAirport, 3);
    m.insert(TerrainId::JadeSunBase, 3);
    m.insert(TerrainId::JadeSunCity, 3);
    m.insert(TerrainId::JadeSunHQ, 4);
    m.insert(TerrainId::JadeSunPort, 3);
    m.insert(TerrainId::JadeSunComTower, 3);
    m.insert(TerrainId::JadeSunLab, 3);
    m.insert(TerrainId::CobaltIceAirport, 3);
    m.insert(TerrainId::CobaltIceBase, 3);
    m.insert(TerrainId::CobaltIceCity, 3);
    m.insert(TerrainId::CobaltIceHQ, 4);
    m.insert(TerrainId::CobaltIcePort, 3);
    m.insert(TerrainId::CobaltIceComTower, 3);
    m.insert(TerrainId::CobaltIceLab, 3);
    m.insert(TerrainId::PinkCosmosAirport, 3);
    m.insert(TerrainId::PinkCosmosBase, 3);
    m.insert(TerrainId::PinkCosmosCity, 3);
    m.insert(TerrainId::PinkCosmosHQ, 4);
    m.insert(TerrainId::PinkCosmosPort, 3);
    m.insert(TerrainId::PinkCosmosComTower, 3);
    m.insert(TerrainId::PinkCosmosLab, 3);
    m.insert(TerrainId::TealGalaxyAirport, 3);
    m.insert(TerrainId::TealGalaxyBase, 3);
    m.insert(TerrainId::TealGalaxyCity, 3);
    m.insert(TerrainId::TealGalaxyHQ, 4);
    m.insert(TerrainId::TealGalaxyPort, 3);
    m.insert(TerrainId::TealGalaxyComTower, 3);
    m.insert(TerrainId::TealGalaxyLab, 3);
    m.insert(TerrainId::PurpleLightningAirport, 3);
    m.insert(TerrainId::PurpleLightningBase, 3);
    m.insert(TerrainId::PurpleLightningCity, 3);
    m.insert(TerrainId::PurpleLightningHQ, 4);
    m.insert(TerrainId::PurpleLightningPort, 3);
    m.insert(TerrainId::PurpleLightningComTower, 3);
    m.insert(TerrainId::PurpleLightningLab, 3);
    m.insert(TerrainId::AcidRainAirport, 3);
    m.insert(TerrainId::AcidRainBase, 3);
    m.insert(TerrainId::AcidRainCity, 3);
    m.insert(TerrainId::AcidRainHQ, 4);
    m.insert(TerrainId::AcidRainPort, 3);
    m.insert(TerrainId::AcidRainComTower, 3);
    m.insert(TerrainId::AcidRainLab, 3);
    m.insert(TerrainId::WhiteNovaAirport, 3);
    // m.insert(TerrainId::WhiteNovaAirport, w);
    m.insert(TerrainId::WhiteNovaBase, 3);
    m.insert(TerrainId::WhiteNovaCity, 3);
    m.insert(TerrainId::WhiteNovaHQ, 4);
    m.insert(TerrainId::WhiteNovaPort, 3);
    m.insert(TerrainId::WhiteNovaComTower, 3);
    m.insert(TerrainId::WhiteNovaLab, 3);
    m.insert(TerrainId::AzureAsteroidAirport, 3);
    // m.insert(TerrainId::AzureAsteroidAirport, a);
    m.insert(TerrainId::AzureAsteroidBase, 3);
    m.insert(TerrainId::AzureAsteroidCity, 3);
    m.insert(TerrainId::AzureAsteroidHQ, 4);
    m.insert(TerrainId::AzureAsteroidPort, 3);
    m.insert(TerrainId::AzureAsteroidComTower, 3);
    m.insert(TerrainId::AzureAsteroidLab, 3);
    m.insert(TerrainId::NoirEclipseAirport, 3);
    // m.insert(TerrainId::NoirEclipseAirport, n);
    m.insert(TerrainId::NoirEclipseBase, 3);
    m.insert(TerrainId::NoirEclipseCity, 3);
    m.insert(TerrainId::NoirEclipseHQ, 4);
    m.insert(TerrainId::NoirEclipsePort, 3);
    m.insert(TerrainId::NoirEclipseComTower, 3);
    m.insert(TerrainId::NoirEclipseLab, 3);
    m
  })
}
