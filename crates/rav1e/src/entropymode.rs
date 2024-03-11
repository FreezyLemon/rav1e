// Copyright (c) 2018-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#![allow(non_upper_case_globals)]

use crate::context::*;
use crate::partition::*;
use crate::predict::*;
use crate::util::*;

use rav1e_partitioning::*;
use rav1e_tx::TxSize;
use rav1e_tx::TX_TYPES;

pub const PALETTE_BSIZE_CTXS: usize = 7;
pub const PALETTE_Y_MODE_CONTEXTS: usize = 3;
pub const PALETTE_UV_MODE_CONTEXTS: usize = 2;
const PALETTE_COLOR_INDEX_CONTEXTS: usize = 5;
const RESTORE_SWITCHABLE_TYPES: usize = 3;
pub const TX_SIZE_CONTEXTS: usize = 3;

// from seg_common.h
const MAX_SEGMENTS: usize = 8;
const SPATIAL_PREDICTION_PROBS: usize = 3;
const SEG_TEMPORAL_PRED_CTXS: usize = 3;

// enums.h
const TX_SIZE_LUMA_MIN: usize = TxSize::TX_4X4 as usize;
const TX_SIZE_CTX_MIN: usize = TX_SIZE_LUMA_MIN + 1;
pub const MAX_TX_CATS: usize = TxSize::TX_SIZES - TX_SIZE_CTX_MIN;
pub const BIG_TX_CATS: usize = MAX_TX_CATS - 1; // All except 8x8, which has lower max depth.
pub const MAX_TX_DEPTH: usize = 2;

// LUTS ---------------------

pub static default_kf_y_mode_cdf: [[[u16; INTRA_MODES]; KF_MODE_CONTEXTS];
  KF_MODE_CONTEXTS] = cdf_3d([
  [
    [
      15588, 17027, 19338, 20218, 20682, 21110, 21825, 23244, 24189, 28165,
      29093, 30466,
    ],
    [
      12016, 18066, 19516, 20303, 20719, 21444, 21888, 23032, 24434, 28658,
      30172, 31409,
    ],
    [
      10052, 10771, 22296, 22788, 23055, 23239, 24133, 25620, 26160, 29336,
      29929, 31567,
    ],
    [
      14091, 15406, 16442, 18808, 19136, 19546, 19998, 22096, 24746, 29585,
      30958, 32462,
    ],
    [
      12122, 13265, 15603, 16501, 18609, 20033, 22391, 25583, 26437, 30261,
      31073, 32475,
    ],
  ],
  [
    [
      10023, 19585, 20848, 21440, 21832, 22760, 23089, 24023, 25381, 29014,
      30482, 31436,
    ],
    [
      5983, 24099, 24560, 24886, 25066, 25795, 25913, 26423, 27610, 29905,
      31276, 31794,
    ],
    [
      7444, 12781, 20177, 20728, 21077, 21607, 22170, 23405, 24469, 27915,
      29090, 30492,
    ],
    [
      8537, 14689, 15432, 17087, 17408, 18172, 18408, 19825, 24649, 29153,
      31096, 32210,
    ],
    [
      7543, 14231, 15496, 16195, 17905, 20717, 21984, 24516, 26001, 29675,
      30981, 31994,
    ],
  ],
  [
    [
      12613, 13591, 21383, 22004, 22312, 22577, 23401, 25055, 25729, 29538,
      30305, 32077,
    ],
    [
      9687, 13470, 18506, 19230, 19604, 20147, 20695, 22062, 23219, 27743,
      29211, 30907,
    ],
    [
      6183, 6505, 26024, 26252, 26366, 26434, 27082, 28354, 28555, 30467,
      30794, 32086,
    ],
    [
      10718, 11734, 14954, 17224, 17565, 17924, 18561, 21523, 23878, 28975,
      30287, 32252,
    ],
    [
      9194, 9858, 16501, 17263, 18424, 19171, 21563, 25961, 26561, 30072,
      30737, 32463,
    ],
  ],
  [
    [
      12602, 14399, 15488, 18381, 18778, 19315, 19724, 21419, 25060, 29696,
      30917, 32409,
    ],
    [
      8203, 13821, 14524, 17105, 17439, 18131, 18404, 19468, 25225, 29485,
      31158, 32342,
    ],
    [
      8451, 9731, 15004, 17643, 18012, 18425, 19070, 21538, 24605, 29118,
      30078, 32018,
    ],
    [
      7714, 9048, 9516, 16667, 16817, 16994, 17153, 18767, 26743, 30389,
      31536, 32528,
    ],
    [
      8843, 10280, 11496, 15317, 16652, 17943, 19108, 22718, 25769, 29953,
      30983, 32485,
    ],
  ],
  [
    [
      12578, 13671, 15979, 16834, 19075, 20913, 22989, 25449, 26219, 30214,
      31150, 32477,
    ],
    [
      9563, 13626, 15080, 15892, 17756, 20863, 22207, 24236, 25380, 29653,
      31143, 32277,
    ],
    [
      8356, 8901, 17616, 18256, 19350, 20106, 22598, 25947, 26466, 29900,
      30523, 32261,
    ],
    [
      10835, 11815, 13124, 16042, 17018, 18039, 18947, 22753, 24615, 29489,
      30883, 32482,
    ],
    [
      7618, 8288, 9859, 10509, 15386, 18657, 22903, 28776, 29180, 31355,
      31802, 32593,
    ],
  ],
]);

pub static default_angle_delta_cdf: [[u16; 2 * MAX_ANGLE_DELTA + 1];
  DIRECTIONAL_MODES] = cdf_2d([
  [2180, 5032, 7567, 22776, 26989, 30217],
  [2301, 5608, 8801, 23487, 26974, 30330],
  [3780, 11018, 13699, 19354, 23083, 31286],
  [4581, 11226, 15147, 17138, 21834, 28397],
  [1737, 10927, 14509, 19588, 22745, 28823],
  [2664, 10176, 12485, 17650, 21600, 30495],
  [2240, 11096, 15453, 20341, 22561, 28917],
  [3605, 10428, 12459, 17676, 21244, 30655],
]);

pub static default_if_y_mode_cdf: [[u16; INTRA_MODES]; BLOCK_SIZE_GROUPS] =
  cdf_2d([
    [
      22801, 23489, 24293, 24756, 25601, 26123, 26606, 27418, 27945, 29228,
      29685, 30349,
    ],
    [
      18673, 19845, 22631, 23318, 23950, 24649, 25527, 27364, 28152, 29701,
      29984, 30852,
    ],
    [
      19770, 20979, 23396, 23939, 24241, 24654, 25136, 27073, 27830, 29360,
      29730, 30659,
    ],
    [
      20155, 21301, 22838, 23178, 23261, 23533, 23703, 24804, 25352, 26575,
      27016, 28049,
    ],
  ]);

pub static default_uv_mode_cdf: [[u16; INTRA_MODES]; INTRA_MODES] = cdf_2d([
  [
    22631, 24152, 25378, 25661, 25986, 26520, 27055, 27923, 28244, 30059,
    30941, 31961,
  ],
  [
    9513, 26881, 26973, 27046, 27118, 27664, 27739, 27824, 28359, 29505,
    29800, 31796,
  ],
  [
    9845, 9915, 28663, 28704, 28757, 28780, 29198, 29822, 29854, 30764, 31777,
    32029,
  ],
  [
    13639, 13897, 14171, 25331, 25606, 25727, 25953, 27148, 28577, 30612,
    31355, 32493,
  ],
  [
    9764, 9835, 9930, 9954, 25386, 27053, 27958, 28148, 28243, 31101, 31744,
    32363,
  ],
  [
    11825, 13589, 13677, 13720, 15048, 29213, 29301, 29458, 29711, 31161,
    31441, 32550,
  ],
  [
    14175, 14399, 16608, 16821, 17718, 17775, 28551, 30200, 30245, 31837,
    32342, 32667,
  ],
  [
    12885, 13038, 14978, 15590, 15673, 15748, 16176, 29128, 29267, 30643,
    31961, 32461,
  ],
  [
    12026, 13661, 13874, 15305, 15490, 15726, 15995, 16273, 28443, 30388,
    30767, 32416,
  ],
  [
    19052, 19840, 20579, 20916, 21150, 21467, 21885, 22719, 23174, 28861,
    30379, 32175,
  ],
  [
    18627, 19649, 20974, 21219, 21492, 21816, 22199, 23119, 23527, 27053,
    31397, 32148,
  ],
  [
    17026, 19004, 19997, 20339, 20586, 21103, 21349, 21907, 22482, 25896,
    26541, 31819,
  ],
  [
    12124, 13759, 14959, 14992, 15007, 15051, 15078, 15166, 15255, 15753,
    16039, 16606,
  ],
]);

pub static default_uv_mode_cfl_cdf: [[u16; UV_INTRA_MODES]; INTRA_MODES] =
  cdf_2d([
    [
      10407, 11208, 12900, 13181, 13823, 14175, 14899, 15656, 15986, 20086,
      20995, 22455, 24212,
    ],
    [
      4532, 19780, 20057, 20215, 20428, 21071, 21199, 21451, 22099, 24228,
      24693, 27032, 29472,
    ],
    [
      5273, 5379, 20177, 20270, 20385, 20439, 20949, 21695, 21774, 23138,
      24256, 24703, 26679,
    ],
    [
      6740, 7167, 7662, 14152, 14536, 14785, 15034, 16741, 18371, 21520,
      22206, 23389, 24182,
    ],
    [
      4987, 5368, 5928, 6068, 19114, 20315, 21857, 22253, 22411, 24911, 25380,
      26027, 26376,
    ],
    [
      5370, 6889, 7247, 7393, 9498, 21114, 21402, 21753, 21981, 24780, 25386,
      26517, 27176,
    ],
    [
      4816, 4961, 7204, 7326, 8765, 8930, 20169, 20682, 20803, 23188, 23763,
      24455, 24940,
    ],
    [
      6608, 6740, 8529, 9049, 9257, 9356, 9735, 18827, 19059, 22336, 23204,
      23964, 24793,
    ],
    [
      5998, 7419, 7781, 8933, 9255, 9549, 9753, 10417, 18898, 22494, 23139,
      24764, 25989,
    ],
    [
      10660, 11298, 12550, 12957, 13322, 13624, 14040, 15004, 15534, 20714,
      21789, 23443, 24861,
    ],
    [
      10522, 11530, 12552, 12963, 13378, 13779, 14245, 15235, 15902, 20102,
      22696, 23774, 25838,
    ],
    [
      10099, 10691, 12639, 13049, 13386, 13665, 14125, 15163, 15636, 19676,
      20474, 23519, 25208,
    ],
    [
      3144, 5087, 7382, 7504, 7593, 7690, 7801, 8064, 8232, 9248, 9875, 10521,
      29048,
    ],
  ]);

pub const default_partition_w8_cdf: [[u16; 4]; PARTITION_TYPES] = cdf_2d([
  [19132, 25510, 30392],
  [13928, 19855, 28540],
  [12522, 23679, 28629],
  [9896, 18783, 25853],
]);

pub const default_partition_cdf: [[u16; EXT_PARTITION_TYPES];
  3 * PARTITION_TYPES] = cdf_2d([
  [15597, 20929, 24571, 26706, 27664, 28821, 29601, 30571, 31902],
  [7925, 11043, 16785, 22470, 23971, 25043, 26651, 28701, 29834],
  [5414, 13269, 15111, 20488, 22360, 24500, 25537, 26336, 32117],
  [2662, 6362, 8614, 20860, 23053, 24778, 26436, 27829, 31171],
  [18462, 20920, 23124, 27647, 28227, 29049, 29519, 30178, 31544],
  [7689, 9060, 12056, 24992, 25660, 26182, 26951, 28041, 29052],
  [6015, 9009, 10062, 24544, 25409, 26545, 27071, 27526, 32047],
  [1394, 2208, 2796, 28614, 29061, 29466, 29840, 30185, 31899],
  [20137, 21547, 23078, 29566, 29837, 30261, 30524, 30892, 31724],
  [6732, 7490, 9497, 27944, 28250, 28515, 28969, 29630, 30104],
  [5945, 7663, 8348, 28683, 29117, 29749, 30064, 30298, 32238],
  [870, 1212, 1487, 31198, 31394, 31574, 31743, 31881, 32332],
]);

pub const default_partition_w128_cdf: [[u16; 8]; PARTITION_TYPES] = cdf_2d([
  [27899, 28219, 28529, 32484, 32539, 32619, 32639],
  [6607, 6990, 8268, 32060, 32219, 32338, 32371],
  [5429, 6676, 7122, 32027, 32227, 32531, 32582],
  [711, 966, 1172, 32448, 32538, 32617, 32664],
]);

pub static default_intra_tx_1_cdf: [[[u16; 7]; INTRA_MODES];
  TX_SIZE_SQR_CONTEXTS] = cdf_3d([
  [
    [1535, 8035, 9461, 12751, 23467, 27825],
    [564, 3335, 9709, 10870, 18143, 28094],
    [672, 3247, 3676, 11982, 19415, 23127],
    [5279, 13885, 15487, 18044, 23527, 30252],
    [4423, 6074, 7985, 10416, 25693, 29298],
    [1486, 4241, 9460, 10662, 16456, 27694],
    [439, 2838, 3522, 6737, 18058, 23754],
    [1190, 4233, 4855, 11670, 20281, 24377],
    [1045, 4312, 8647, 10159, 18644, 29335],
    [202, 3734, 4747, 7298, 17127, 24016],
    [447, 4312, 6819, 8884, 16010, 23858],
    [277, 4369, 5255, 8905, 16465, 22271],
    [3409, 5436, 10599, 15599, 19687, 24040],
  ],
  [
    [1870, 13742, 14530, 16498, 23770, 27698],
    [326, 8796, 14632, 15079, 19272, 27486],
    [484, 7576, 7712, 14443, 19159, 22591],
    [1126, 15340, 15895, 17023, 20896, 30279],
    [655, 4854, 5249, 5913, 22099, 27138],
    [1299, 6458, 8885, 9290, 14851, 25497],
    [311, 5295, 5552, 6885, 16107, 22672],
    [883, 8059, 8270, 11258, 17289, 21549],
    [741, 7580, 9318, 10345, 16688, 29046],
    [110, 7406, 7915, 9195, 16041, 23329],
    [363, 7974, 9357, 10673, 15629, 24474],
    [153, 7647, 8112, 9936, 15307, 19996],
    [3511, 6332, 11165, 15335, 19323, 23594],
  ],
  [
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
  ],
  [
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
    [4681, 9362, 14043, 18725, 23406, 28087],
  ],
]);

pub static default_intra_tx_2_cdf: [[[u16; 5]; INTRA_MODES];
  TX_SIZE_SQR_CONTEXTS] = cdf_3d([
  [
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
  ],
  [
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
  ],
  [
    [1127, 12814, 22772, 27483],
    [145, 6761, 11980, 26667],
    [362, 5887, 11678, 16725],
    [385, 15213, 18587, 30693],
    [25, 2914, 23134, 27903],
    [60, 4470, 11749, 23991],
    [37, 3332, 14511, 21448],
    [157, 6320, 13036, 17439],
    [119, 6719, 12906, 29396],
    [47, 5537, 12576, 21499],
    [269, 6076, 11258, 23115],
    [83, 5615, 12001, 17228],
    [1968, 5556, 12023, 18547],
  ],
  [
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
    [6554, 13107, 19661, 26214],
  ],
]);

pub static default_inter_tx_1_cdf: [[u16; TX_TYPES]; TX_SIZE_SQR_CONTEXTS] =
  cdf_2d([
    [
      4458, 5560, 7695, 9709, 13330, 14789, 17537, 20266, 21504, 22848, 23934,
      25474, 27727, 28915, 30631,
    ],
    [
      1645, 2573, 4778, 5711, 7807, 8622, 10522, 15357, 17674, 20408, 22517,
      25010, 27116, 28856, 30749,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
  ]);

pub static default_inter_tx_2_cdf: [[u16; 12]; TX_SIZE_SQR_CONTEXTS] =
  cdf_2d([
    [2731, 5461, 8192, 10923, 13653, 16384, 19115, 21845, 24576, 27307, 30037],
    [2731, 5461, 8192, 10923, 13653, 16384, 19115, 21845, 24576, 27307, 30037],
    [770, 2421, 5225, 12907, 15819, 18927, 21561, 24089, 26595, 28526, 30529],
    [2731, 5461, 8192, 10923, 13653, 16384, 19115, 21845, 24576, 27307, 30037],
  ]);

pub static default_inter_tx_3_cdf: [[u16; 2]; TX_SIZE_SQR_CONTEXTS] =
  cdf_2d([[16384], [4167], [1998], [748]]);

pub static default_cfl_sign_cdf: [u16; CFL_JOINT_SIGNS] =
  cdf([1418, 2123, 13340, 18405, 26972, 28343, 32294]);

pub static default_cfl_alpha_cdf: [[u16; CFL_ALPHABET_SIZE];
  CFL_ALPHA_CONTEXTS] = cdf_2d([
  [
    7637, 20719, 31401, 32481, 32657, 32688, 32692, 32696, 32700, 32704,
    32708, 32712, 32716, 32720, 32724,
  ],
  [
    14365, 23603, 28135, 31168, 32167, 32395, 32487, 32573, 32620, 32647,
    32668, 32672, 32676, 32680, 32684,
  ],
  [
    11532, 22380, 28445, 31360, 32349, 32523, 32584, 32649, 32673, 32677,
    32681, 32685, 32689, 32693, 32697,
  ],
  [
    26990, 31402, 32282, 32571, 32692, 32696, 32700, 32704, 32708, 32712,
    32716, 32720, 32724, 32728, 32732,
  ],
  [
    17248, 26058, 28904, 30608, 31305, 31877, 32126, 32321, 32394, 32464,
    32516, 32560, 32576, 32593, 32622,
  ],
  [
    14738, 21678, 25779, 27901, 29024, 30302, 30980, 31843, 32144, 32413,
    32520, 32594, 32622, 32656, 32660,
  ],
]);

// This does not appear to be used in the rust project currently
const SWITCHABLE_FILTERS: usize = 3;
const SWITCHABLE_FILTER_CONTEXTS: usize = (SWITCHABLE_FILTERS + 1) * 4;

#[allow(unused)]
pub static default_switchable_interp_cdf: [[u16; SWITCHABLE_FILTERS];
  SWITCHABLE_FILTER_CONTEXTS] = cdf_2d([
  [31935, 32720],
  [5568, 32719],
  [422, 2938],
  [28244, 32608],
  [31206, 31953],
  [4862, 32121],
  [770, 1152],
  [20889, 25637],
  [31910, 32724],
  [4120, 32712],
  [305, 2247],
  [27403, 32636],
  [31022, 32009],
  [2963, 32093],
  [601, 943],
  [14969, 21398],
]);

pub static default_newmv_cdf: [[u16; 2]; NEWMV_MODE_CONTEXTS] = [
  cdf([24035]),
  cdf([16630]),
  cdf([15339]),
  cdf([8386]),
  cdf([12222]),
  cdf([4676]),
  [0; 2],
];

pub static default_zeromv_cdf: [[u16; 2]; GLOBALMV_MODE_CONTEXTS] =
  cdf_2d([[2175], [1054]]);

pub static default_refmv_cdf: [[u16; 2]; REFMV_MODE_CONTEXTS] =
  cdf_2d([[23974], [24188], [17848], [28622], [24312], [19923]]);

pub static default_drl_cdf: [[u16; 2]; DRL_MODE_CONTEXTS] =
  cdf_2d([[13104], [24560], [18945]]);

pub static default_compound_mode_cdf: [[u16; INTER_COMPOUND_MODES];
  INTER_MODE_CONTEXTS] = cdf_2d([
  [7760, 13823, 15808, 17641, 19156, 20666, 26891],
  [10730, 19452, 21145, 22749, 24039, 25131, 28724],
  [10664, 20221, 21588, 22906, 24295, 25387, 28436],
  [13298, 16984, 20471, 24182, 25067, 25736, 26422],
  [18904, 23325, 25242, 27432, 27898, 28258, 30758],
  [10725, 17454, 20124, 22820, 24195, 25168, 26046],
  [17125, 24273, 25814, 27492, 28214, 28704, 30592],
  [13046, 23214, 24505, 25942, 27435, 28442, 29330],
]);

#[allow(unused)]
pub static default_interintra_cdf: [[u16; 2]; BLOCK_SIZE_GROUPS] =
  cdf_2d([[16384], [26887], [27597], [30237]]);

#[allow(unused)]
pub static default_interintra_mode_cdf: [[u16;
  InterIntraMode::INTERINTRA_MODES as usize];
  BLOCK_SIZE_GROUPS] = cdf_2d([
  [8192, 16384, 24576],
  [1875, 11082, 27332],
  [2473, 9996, 26388],
  [4238, 11537, 25926],
]);

#[allow(unused)]
pub static default_wedge_interintra_cdf: [[u16; 2];
  BlockSize::BLOCK_SIZES_ALL] = cdf_2d([
  [16384],
  [16384],
  [16384],
  [20036],
  [24957],
  [26704],
  [27530],
  [29564],
  [29444],
  [26872],
  [16384],
  [16384],
  [16384],
  [16384],
  [16384],
  [16384],
  [16384],
  [16384],
  [16384],
  [16384],
  [16384],
  [16384],
]);

#[allow(unused)]
pub static default_compound_type_cdf: [[u16;
  CompoundType::COMPOUND_TYPES as usize - 1];
  BlockSize::BLOCK_SIZES_ALL] = cdf_2d([
  [16384],
  [16384],
  [16384],
  [23431],
  [13171],
  [11470],
  [9770],
  [9100],
  [8233],
  [6172],
  [16384],
  [16384],
  [16384],
  [16384],
  [16384],
  [16384],
  [16384],
  [16384],
  [11820],
  [7701],
  [16384],
  [16384],
]);

#[allow(unused)]
pub static default_wedge_idx_cdf: [[u16; 16]; BlockSize::BLOCK_SIZES_ALL] =
  cdf_2d([
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
    [
      2438, 4440, 6599, 8663, 11005, 12874, 15751, 18094, 20359, 22362, 24127,
      25702, 27752, 29450, 31171,
    ],
    [
      806, 3266, 6005, 6738, 7218, 7367, 7771, 14588, 16323, 17367, 18452,
      19422, 22839, 26127, 29629,
    ],
    [
      2779, 3738, 4683, 7213, 7775, 8017, 8655, 14357, 17939, 21332, 24520,
      27470, 29456, 30529, 31656,
    ],
    [
      1684, 3625, 5675, 7108, 9302, 11274, 14429, 17144, 19163, 20961, 22884,
      24471, 26719, 28714, 30877,
    ],
    [
      1142, 3491, 6277, 7314, 8089, 8355, 9023, 13624, 15369, 16730, 18114,
      19313, 22521, 26012, 29550,
    ],
    [
      2742, 4195, 5727, 8035, 8980, 9336, 10146, 14124, 17270, 20533, 23434,
      25972, 27944, 29570, 31416,
    ],
    [
      1727, 3948, 6101, 7796, 9841, 12344, 15766, 18944, 20638, 22038, 23963,
      25311, 26988, 28766, 31012,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
    [
      154, 987, 1925, 2051, 2088, 2111, 2151, 23033, 23703, 24284, 24985,
      25684, 27259, 28883, 30911,
    ],
    [
      1135, 1322, 1493, 2635, 2696, 2737, 2770, 21016, 22935, 25057, 27251,
      29173, 30089, 30960, 31933,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
    [
      2048, 4096, 6144, 8192, 10240, 12288, 14336, 16384, 18432, 20480, 22528,
      24576, 26624, 28672, 30720,
    ],
  ]);

#[allow(unused)]
pub static default_motion_mode_cdf: [[u16;
  MotionMode::MOTION_MODES as usize];
  BlockSize::BLOCK_SIZES_ALL] = cdf_2d([
  [10923, 21845],
  [10923, 21845],
  [10923, 21845],
  [7651, 24760],
  [4738, 24765],
  [5391, 25528],
  [19419, 26810],
  [5123, 23606],
  [11606, 24308],
  [26260, 29116],
  [20360, 28062],
  [21679, 26830],
  [29516, 30701],
  [28898, 30397],
  [30878, 31335],
  [32507, 32558],
  [10923, 21845],
  [10923, 21845],
  [28799, 31390],
  [26431, 30774],
  [28973, 31594],
  [29742, 31203],
]);

#[allow(unused)]
pub static default_obmc_cdf: [[u16; 2]; BlockSize::BLOCK_SIZES_ALL] =
  cdf_2d([
    [16384],
    [16384],
    [16384],
    [10437],
    [9371],
    [9301],
    [17432],
    [14423],
    [15142],
    [25817],
    [22823],
    [22083],
    [30128],
    [31014],
    [31560],
    [32638],
    [16384],
    [16384],
    [23664],
    [20901],
    [24008],
    [26879],
  ]);

pub static default_intra_inter_cdf: [[u16; 2]; INTRA_INTER_CONTEXTS] =
  cdf_2d([[806], [16662], [20186], [26538]]);

pub static default_comp_mode_cdf: [[u16; 2]; COMP_INTER_CONTEXTS] =
  cdf_2d([[26828], [24035], [12031], [10640], [2901]]);

pub static default_comp_ref_type_cdf: [[u16; 2]; COMP_REF_TYPE_CONTEXTS] =
  cdf_2d([[1198], [2070], [9166], [7499], [22475]]);

#[allow(unused)]
pub static default_uni_comp_ref_cdf: [[[u16; 2]; UNIDIR_COMP_REFS - 1];
  UNI_COMP_REF_CONTEXTS] = cdf_3d([
  [[5284], [3865], [3128]],
  [[23152], [14173], [15270]],
  [[31774], [25120], [26710]],
]);

pub static default_single_ref_cdf: [[[u16; 2]; SINGLE_REFS - 1];
  REF_CONTEXTS] = cdf_3d([
  [[4897], [1555], [4236], [8650], [904], [1444]],
  [[16973], [16751], [19647], [24773], [11014], [15087]],
  [[29744], [30279], [31194], [31895], [26875], [30304]],
]);

pub static default_comp_ref_cdf: [[[u16; 2]; FWD_REFS - 1]; REF_CONTEXTS] =
  cdf_3d([
    [[4946], [9468], [1503]],
    [[19891], [22441], [15160]],
    [[30731], [31059], [27544]],
  ]);

pub static default_comp_bwdref_cdf: [[[u16; 2]; BWD_REFS - 1]; REF_CONTEXTS] =
  cdf_3d([[[2235], [1423]], [[17182], [15175]], [[30606], [30489]]]);

#[allow(unused)]
pub static default_palette_y_size_cdf: [[u16;
  PaletteSize::PALETTE_SIZES as usize];
  PALETTE_BSIZE_CTXS] = cdf_2d([
  [7952, 13000, 18149, 21478, 25527, 29241],
  [7139, 11421, 16195, 19544, 23666, 28073],
  [7788, 12741, 17325, 20500, 24315, 28530],
  [8271, 14064, 18246, 21564, 25071, 28533],
  [12725, 19180, 21863, 24839, 27535, 30120],
  [9711, 14888, 16923, 21052, 25661, 27875],
  [14940, 20797, 21678, 24186, 27033, 28999],
]);

#[allow(unused)]
pub static default_palette_uv_size_cdf: [[u16;
  PaletteSize::PALETTE_SIZES as usize];
  PALETTE_BSIZE_CTXS] = cdf_2d([
  [8713, 19979, 27128, 29609, 31331, 32272],
  [5839, 15573, 23581, 26947, 29848, 31700],
  [4426, 11260, 17999, 21483, 25863, 29430],
  [3228, 9464, 14993, 18089, 22523, 27420],
  [3768, 8886, 13091, 17852, 22495, 27207],
  [2464, 8451, 12861, 21632, 25525, 28555],
  [1269, 5435, 10433, 18963, 21700, 25865],
]);

pub static default_palette_y_mode_cdfs: [[[u16; 2]; PALETTE_Y_MODE_CONTEXTS];
  PALETTE_BSIZE_CTXS] = cdf_3d([
  [[31676], [3419], [1261]],
  [[31912], [2859], [980]],
  [[31823], [3400], [781]],
  [[32030], [3561], [904]],
  [[32309], [7337], [1462]],
  [[32265], [4015], [1521]],
  [[32450], [7946], [129]],
]);

pub static default_palette_uv_mode_cdfs: [[u16; 2]; PALETTE_UV_MODE_CONTEXTS] =
  cdf_2d([[32461], [21488]]);

#[allow(unused)]
pub static default_palette_y_color_index_cdf: [[[u16;
  PaletteColor::PALETTE_COLORS as usize];
  PALETTE_COLOR_INDEX_CONTEXTS];
  PaletteSize::PALETTE_SIZES as usize] = [
  cdf_2d([[28710], [16384], [10553], [27036], [31603]]),
  cdf_2d([
    [27877, 30490],
    [11532, 25697],
    [6544, 30234],
    [23018, 28072],
    [31915, 32385],
  ]),
  cdf_2d([
    [25572, 28046, 30045],
    [9478, 21590, 27256],
    [7248, 26837, 29824],
    [19167, 24486, 28349],
    [31400, 31825, 32250],
  ]),
  cdf_2d([
    [24779, 26955, 28576, 30282],
    [8669, 20364, 24073, 28093],
    [4255, 27565, 29377, 31067],
    [19864, 23674, 26716, 29530],
    [31646, 31893, 32147, 32426],
  ]),
  cdf_2d([
    [23132, 25407, 26970, 28435, 30073],
    [7443, 17242, 20717, 24762, 27982],
    [6300, 24862, 26944, 28784, 30671],
    [18916, 22895, 25267, 27435, 29652],
    [31270, 31550, 31808, 32059, 32353],
  ]),
  cdf_2d([
    [23105, 25199, 26464, 27684, 28931, 30318],
    [6950, 15447, 18952, 22681, 25567, 28563],
    [7560, 23474, 25490, 27203, 28921, 30708],
    [18544, 22373, 24457, 26195, 28119, 30045],
    [31198, 31451, 31670, 31882, 32123, 32391],
  ]),
  cdf_2d([
    [21689, 23883, 25163, 26352, 27506, 28827, 30195],
    [6892, 15385, 17840, 21606, 24287, 26753, 29204],
    [5651, 23182, 25042, 26518, 27982, 29392, 30900],
    [19349, 22578, 24418, 25994, 27524, 29031, 30448],
    [31028, 31270, 31504, 31705, 31927, 32153, 32392],
  ]),
];

#[allow(unused)]
pub static default_palette_uv_color_index_cdf: [[[u16;
  PaletteColor::PALETTE_COLORS as usize];
  PALETTE_COLOR_INDEX_CONTEXTS];
  PaletteSize::PALETTE_SIZES as usize] = [
  cdf_2d([[29089], [16384], [8713], [29257], [31610]]),
  cdf_2d([
    [25257, 29145],
    [12287, 27293],
    [7033, 27960],
    [20145, 25405],
    [30608, 31639],
  ]),
  cdf_2d([
    [24210, 27175, 29903],
    [9888, 22386, 27214],
    [5901, 26053, 29293],
    [18318, 22152, 28333],
    [30459, 31136, 31926],
  ]),
  cdf_2d([
    [22980, 25479, 27781, 29986],
    [8413, 21408, 24859, 28874],
    [2257, 29449, 30594, 31598],
    [19189, 21202, 25915, 28620],
    [31844, 32044, 32281, 32518],
  ]),
  cdf_2d([
    [22217, 24567, 26637, 28683, 30548],
    [7307, 16406, 19636, 24632, 28424],
    [4441, 25064, 26879, 28942, 30919],
    [17210, 20528, 23319, 26750, 29582],
    [30674, 30953, 31396, 31735, 32207],
  ]),
  cdf_2d([
    [21239, 23168, 25044, 26962, 28705, 30506],
    [6545, 15012, 18004, 21817, 25503, 28701],
    [3448, 26295, 27437, 28704, 30126, 31442],
    [15889, 18323, 21704, 24698, 26976, 29690],
    [30988, 31204, 31479, 31734, 31983, 32325],
  ]),
  cdf_2d([
    [21442, 23288, 24758, 26246, 27649, 28980, 30563],
    [5863, 14933, 17552, 20668, 23683, 26411, 29273],
    [3415, 25810, 26877, 27990, 29223, 30394, 31618],
    [17965, 20084, 22232, 23974, 26274, 28402, 30390],
    [31190, 31329, 31516, 31679, 31825, 32026, 32322],
  ]),
];

pub static default_txfm_partition_cdf: [[u16; 2]; TXFM_PARTITION_CONTEXTS] =
  cdf_2d([
    [28581],
    [23846],
    [20847],
    [24315],
    [18196],
    [12133],
    [18791],
    [10887],
    [11005],
    [27179],
    [20004],
    [11281],
    [26549],
    [19308],
    [14224],
    [28015],
    [21546],
    [14400],
    [28165],
    [22401],
    [16088],
  ]);

pub static default_skip_cdfs: [[u16; 2]; SKIP_CONTEXTS] =
  cdf_2d([[31671], [16515], [4576]]);

#[allow(unused)]
pub static default_skip_mode_cdfs: [[u16; 2]; SKIP_MODE_CONTEXTS] =
  cdf_2d([[32621], [20708], [8127]]);

#[allow(unused)]
pub static default_compound_idx_cdfs: [[u16; 2]; COMP_INDEX_CONTEXTS] =
  cdf_2d([[18244], [12865], [7053], [13259], [9334], [4644]]);

#[allow(unused)]
pub static default_comp_group_idx_cdfs: [[u16; 2]; COMP_GROUP_IDX_CONTEXTS] =
  cdf_2d([[26607], [22891], [18840], [24594], [19934], [22674]]);

#[allow(unused)]
pub static default_intrabc_cdf: [u16; 2] = cdf([30531]);

#[allow(unused)]
pub static default_filter_intra_mode_cdf: [u16;
  FilterIntraMode::FILTER_INTRA_MODES as usize] =
  cdf([8949, 12776, 17211, 29558]);

pub static default_filter_intra_cdfs: [[u16; 2]; BlockSize::BLOCK_SIZES_ALL] =
  cdf_2d([
    [4621],
    [6743],
    [5893],
    [7866],
    [12551],
    [9394],
    [12408],
    [14301],
    [12756],
    [22343],
    [16384],
    [16384],
    [16384],
    [16384],
    [16384],
    [16384],
    [12770],
    [10368],
    [20229],
    [18101],
    [16384],
    [16384],
  ]);

pub static default_switchable_restore_cdf: [u16; RESTORE_SWITCHABLE_TYPES] =
  cdf([9413, 22581]);

pub static default_wiener_restore_cdf: [u16; 2] = cdf([11570]);

pub static default_sgrproj_restore_cdf: [u16; 2] = cdf([16855]);

#[allow(unused)]
pub static default_delta_q_cdf: [u16; DELTA_Q_PROBS + 1] =
  cdf([28160, 32120, 32677]);

pub static default_delta_lf_multi_cdf: [[u16; DELTA_LF_PROBS + 1];
  FRAME_LF_COUNT] = cdf_2d([
  [28160, 32120, 32677],
  [28160, 32120, 32677],
  [28160, 32120, 32677],
  [28160, 32120, 32677],
]);

pub static default_delta_lf_cdf: [u16; DELTA_LF_PROBS + 1] =
  cdf([28160, 32120, 32677]);

// FIXME(someone) need real defaults here
#[allow(unused)]
pub static default_seg_tree_cdf: [u16; MAX_SEGMENTS] =
  cdf([4096, 8192, 12288, 16384, 20480, 24576, 28672]);

#[allow(unused)]
pub static default_segment_pred_cdf: [[u16; 2]; SEG_TEMPORAL_PRED_CTXS] =
  cdf_2d([[128 * 128], [128 * 128], [128 * 128]]);

pub static default_spatial_pred_seg_tree_cdf: [[u16; MAX_SEGMENTS];
  SPATIAL_PREDICTION_PROBS] = cdf_2d([
  [5622, 7893, 16093, 18233, 27809, 28373, 32533],
  [14274, 18230, 22557, 24935, 29980, 30851, 32344],
  [27527, 28487, 28723, 28890, 32397, 32647, 32679],
]);

pub static default_tx_size_8x8_cdf: [[u16; MAX_TX_DEPTH]; TX_SIZE_CONTEXTS] =
  cdf_2d([[19968], [19968], [24320]]);

pub static default_tx_size_cdf: [[[u16; MAX_TX_DEPTH + 1]; TX_SIZE_CONTEXTS];
  BIG_TX_CATS] = cdf_3d([
  [[12272, 30172], [12272, 30172], [18677, 30848]],
  [[12986, 15180], [12986, 15180], [24302, 25602]],
  [[5782, 11475], [5782, 11475], [16803, 22759]],
]);
