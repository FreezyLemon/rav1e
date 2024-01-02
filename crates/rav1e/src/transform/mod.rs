// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[macro_use]
pub mod forward_shared;

pub use rav1e_tx::*;

pub use self::forward::forward_transform;
pub use self::inverse::inverse_transform_add;

use crate::context::MI_SIZE_LOG2;
use crate::partition::{BlockSize, BlockSize::*};
use crate::util::*;

use TxSize::*;

pub mod forward;
pub mod inverse;

pub static RAV1E_TX_TYPES: &[TxType] = &[
  TxType::DCT_DCT,
  TxType::ADST_DCT,
  TxType::DCT_ADST,
  TxType::ADST_ADST,
  // TODO: Add a speed setting for FLIPADST
  // TxType::FLIPADST_DCT,
  // TxType::DCT_FLIPADST,
  // TxType::FLIPADST_FLIPADST,
  // TxType::ADST_FLIPADST,
  // TxType::FLIPADST_ADST,
  TxType::IDTX,
  TxType::V_DCT,
  TxType::H_DCT,
  //TxType::V_FLIPADST,
  //TxType::H_FLIPADST,
];

pub mod consts {
  pub static SQRT2_BITS: usize = 12;
  pub static SQRT2: i32 = 5793; // 2^12 * sqrt(2)
  pub static INV_SQRT2: i32 = 2896; // 2^12 / sqrt(2)
}

pub trait TxSizeExt {
  fn block_size(self) -> BlockSize;

  fn width_mi(self) -> usize;
  fn height_mi(self) -> usize;
}

impl TxSizeExt for TxSize {
  #[inline]
  fn block_size(self) -> BlockSize {
    match self {
      TX_4X4 => BLOCK_4X4,
      TX_8X8 => BLOCK_8X8,
      TX_16X16 => BLOCK_16X16,
      TX_32X32 => BLOCK_32X32,
      TX_64X64 => BLOCK_64X64,
      TX_4X8 => BLOCK_4X8,
      TX_8X4 => BLOCK_8X4,
      TX_8X16 => BLOCK_8X16,
      TX_16X8 => BLOCK_16X8,
      TX_16X32 => BLOCK_16X32,
      TX_32X16 => BLOCK_32X16,
      TX_32X64 => BLOCK_32X64,
      TX_64X32 => BLOCK_64X32,
      TX_4X16 => BLOCK_4X16,
      TX_16X4 => BLOCK_16X4,
      TX_8X32 => BLOCK_8X32,
      TX_32X8 => BLOCK_32X8,
      TX_16X64 => BLOCK_16X64,
      TX_64X16 => BLOCK_64X16,
    }
  }

  #[inline]
  fn width_mi(self) -> usize {
    self.width() >> MI_SIZE_LOG2
  }

  #[inline]
  fn height_mi(self) -> usize {
    self.height() >> MI_SIZE_LOG2
  }
}

pub const TX_TYPES: usize = 16;
pub const TX_TYPES_PLUS_LL: usize = 17;

/// Utility function that returns the log of the ratio of the col and row sizes.
#[inline]
pub fn get_rect_tx_log_ratio(col: usize, row: usize) -> i8 {
  debug_assert!(col > 0 && row > 0);
  ILog::ilog(col) as i8 - ILog::ilog(row) as i8
}

// performs half a butterfly
#[inline]
const fn half_btf(w0: i32, in0: i32, w1: i32, in1: i32, bit: usize) -> i32 {
  // Ensure defined behaviour for when w0*in0 + w1*in1 is negative and
  //   overflows, but w0*in0 + w1*in1 + rounding isn't.
  let result = (w0 * in0).wrapping_add(w1 * in1);
  // Implement a version of round_shift with wrapping
  if bit == 0 {
    result
  } else {
    result.wrapping_add(1 << (bit - 1)) >> bit
  }
}

// clamps value to a signed integer type of bit bits
#[inline]
fn clamp_value(value: i32, bit: usize) -> i32 {
  let max_value: i32 = ((1i64 << (bit - 1)) - 1) as i32;
  let min_value: i32 = (-(1i64 << (bit - 1))) as i32;
  clamp(value, min_value, max_value)
}

pub fn av1_round_shift_array(arr: &mut [i32], size: usize, bit: i8) {
  if bit == 0 {
    return;
  }
  if bit > 0 {
    let bit = bit as usize;
    arr.iter_mut().take(size).for_each(|i| {
      *i = round_shift(*i, bit);
    })
  } else {
    arr.iter_mut().take(size).for_each(|i| {
      *i <<= -bit;
    })
  }
}

#[inline]
pub const fn valid_av1_transform(tx_size: TxSize, tx_type: TxType) -> bool {
  let size_sq = tx_size.sqr_up();
  use TxSize::*;
  use TxType::*;
  match (size_sq, tx_type) {
    (TX_64X64, DCT_DCT) => true,
    (TX_64X64, _) => false,
    (TX_32X32, DCT_DCT) => true,
    (TX_32X32, IDTX) => true,
    (TX_32X32, _) => false,
    (_, _) => true,
  }
}

#[cfg(any(test, feature = "bench"))]
pub fn get_valid_txfm_types(tx_size: TxSize) -> &'static [TxType] {
  let size_sq = tx_size.sqr_up();
  use TxType::*;
  if size_sq == TxSize::TX_64X64 {
    &[DCT_DCT]
  } else if size_sq == TxSize::TX_32X32 {
    &[DCT_DCT, IDTX]
  } else if size_sq == TxSize::TX_4X4 {
    &[
      DCT_DCT,
      ADST_DCT,
      DCT_ADST,
      ADST_ADST,
      FLIPADST_DCT,
      DCT_FLIPADST,
      FLIPADST_FLIPADST,
      ADST_FLIPADST,
      FLIPADST_ADST,
      IDTX,
      V_DCT,
      H_DCT,
      V_ADST,
      H_ADST,
      V_FLIPADST,
      H_FLIPADST,
      WHT_WHT,
    ]
  } else {
    &[
      DCT_DCT,
      ADST_DCT,
      DCT_ADST,
      ADST_ADST,
      FLIPADST_DCT,
      DCT_FLIPADST,
      FLIPADST_FLIPADST,
      ADST_FLIPADST,
      FLIPADST_ADST,
      IDTX,
      V_DCT,
      H_DCT,
      V_ADST,
      H_ADST,
      V_FLIPADST,
      H_FLIPADST,
    ]
  }
}

#[cfg(test)]
mod test {
  use super::TxType::*;
  use super::*;
  use crate::context::av1_get_coded_tx_size;
  use crate::cpu_features::CpuFeatureLevel;
  use crate::frame::*;
  use rand::random;
  use std::mem::MaybeUninit;

  fn test_roundtrip<T: Pixel>(
    tx_size: TxSize, tx_type: TxType, tolerance: i16,
  ) {
    let cpu = CpuFeatureLevel::default();

    let coeff_area: usize = av1_get_coded_tx_size(tx_size).area();
    let mut src_storage = [T::cast_from(0); 64 * 64];
    let src = &mut src_storage[..tx_size.area()];
    let mut dst = Plane::from_slice(
      &[T::zero(); 64 * 64][..tx_size.area()],
      tx_size.width(),
    );
    let mut res_storage = [0i16; 64 * 64];
    let res = &mut res_storage[..tx_size.area()];
    let mut freq_storage = [MaybeUninit::uninit(); 64 * 64];
    let freq = &mut freq_storage[..tx_size.area()];
    for ((r, s), d) in
      res.iter_mut().zip(src.iter_mut()).zip(dst.data.iter_mut())
    {
      *s = T::cast_from(random::<u8>());
      *d = T::cast_from(random::<u8>());
      *r = i16::cast_from(*s) - i16::cast_from(*d);
    }
    forward_transform(res, freq, tx_size.width(), tx_size, tx_type, 8, cpu);
    // SAFETY: forward_transform initialized freq
    let freq = unsafe { slice_assume_init_mut(freq) };
    inverse_transform_add(
      freq,
      &mut dst.as_region_mut(),
      coeff_area.try_into().unwrap(),
      tx_size,
      tx_type,
      8,
      cpu,
    );

    for (s, d) in src.iter().zip(dst.data.iter()) {
      assert!(i16::abs(i16::cast_from(*s) - i16::cast_from(*d)) <= tolerance);
    }
  }

  #[test]
  fn log_tx_ratios() {
    let combinations = [
      (TxSize::TX_4X4, 0),
      (TxSize::TX_8X8, 0),
      (TxSize::TX_16X16, 0),
      (TxSize::TX_32X32, 0),
      (TxSize::TX_64X64, 0),
      (TxSize::TX_4X8, -1),
      (TxSize::TX_8X4, 1),
      (TxSize::TX_8X16, -1),
      (TxSize::TX_16X8, 1),
      (TxSize::TX_16X32, -1),
      (TxSize::TX_32X16, 1),
      (TxSize::TX_32X64, -1),
      (TxSize::TX_64X32, 1),
      (TxSize::TX_4X16, -2),
      (TxSize::TX_16X4, 2),
      (TxSize::TX_8X32, -2),
      (TxSize::TX_32X8, 2),
      (TxSize::TX_16X64, -2),
      (TxSize::TX_64X16, 2),
    ];

    for &(tx_size, expected) in combinations.iter() {
      println!(
        "Testing combination {:?}, {:?}",
        tx_size.width(),
        tx_size.height()
      );
      assert!(
        get_rect_tx_log_ratio(tx_size.width(), tx_size.height()) == expected
      );
    }
  }

  fn roundtrips<T: Pixel>() {
    let combinations = [
      (TX_4X4, WHT_WHT, 0),
      (TX_4X4, DCT_DCT, 0),
      (TX_4X4, ADST_DCT, 0),
      (TX_4X4, DCT_ADST, 0),
      (TX_4X4, ADST_ADST, 0),
      (TX_4X4, FLIPADST_DCT, 0),
      (TX_4X4, DCT_FLIPADST, 0),
      (TX_4X4, IDTX, 0),
      (TX_4X4, V_DCT, 0),
      (TX_4X4, H_DCT, 0),
      (TX_4X4, V_ADST, 0),
      (TX_4X4, H_ADST, 0),
      (TX_8X8, DCT_DCT, 1),
      (TX_8X8, ADST_DCT, 1),
      (TX_8X8, DCT_ADST, 1),
      (TX_8X8, ADST_ADST, 1),
      (TX_8X8, FLIPADST_DCT, 1),
      (TX_8X8, DCT_FLIPADST, 1),
      (TX_8X8, IDTX, 0),
      (TX_8X8, V_DCT, 0),
      (TX_8X8, H_DCT, 0),
      (TX_8X8, V_ADST, 0),
      (TX_8X8, H_ADST, 1),
      (TX_16X16, DCT_DCT, 1),
      (TX_16X16, ADST_DCT, 1),
      (TX_16X16, DCT_ADST, 1),
      (TX_16X16, ADST_ADST, 1),
      (TX_16X16, FLIPADST_DCT, 1),
      (TX_16X16, DCT_FLIPADST, 1),
      (TX_16X16, IDTX, 0),
      (TX_16X16, V_DCT, 1),
      (TX_16X16, H_DCT, 1),
      // 32x transforms only use DCT_DCT and IDTX
      (TX_32X32, DCT_DCT, 2),
      (TX_32X32, IDTX, 0),
      // 64x transforms only use DCT_DCT and IDTX
      //(TX_64X64, DCT_DCT, 0),
      (TX_4X8, DCT_DCT, 1),
      (TX_8X4, DCT_DCT, 1),
      (TX_4X16, DCT_DCT, 1),
      (TX_16X4, DCT_DCT, 1),
      (TX_8X16, DCT_DCT, 1),
      (TX_16X8, DCT_DCT, 1),
      (TX_8X32, DCT_DCT, 2),
      (TX_32X8, DCT_DCT, 2),
      (TX_16X32, DCT_DCT, 2),
      (TX_32X16, DCT_DCT, 2),
    ];
    for &(tx_size, tx_type, tolerance) in combinations.iter() {
      println!("Testing combination {:?}, {:?}", tx_size, tx_type);
      test_roundtrip::<T>(tx_size, tx_type, tolerance);
    }
  }

  #[test]
  fn roundtrips_u8() {
    roundtrips::<u8>();
  }

  #[test]
  fn roundtrips_u16() {
    roundtrips::<u16>();
  }
}
