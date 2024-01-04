#![allow(non_camel_case_types)]

mod tx_set;
mod tx_size;
mod tx_type;
mod tx_type_1d;
pub mod util;

pub use tx_set::*;
pub use tx_size::*;
pub use tx_type::*;
pub use tx_type_1d::*;

pub mod consts {
  pub static SQRT2_BITS: usize = 12;
  pub static SQRT2: i32 = 5793; // 2^12 * sqrt(2)
  pub static INV_SQRT2: i32 = 2896; // 2^12 / sqrt(2)
}

pub const TX_TYPES: usize = 16;
pub const TX_TYPES_PLUS_LL: usize = 17;

pub const RAV1E_TX_TYPES: &[TxType] = &[
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

// #[cfg(test)]
// mod test {
//   use super::TxType::*;
//   use super::TxSize::*;
//   use super::*;
//   use crate::cpu_features::CpuFeatureLevel;
//   use crate::frame::*;
//   use rand::random;
//   use std::mem::MaybeUninit;

//   fn test_roundtrip<T: Pixel>(
//     tx_size: TxSize, tx_type: TxType, tolerance: i16,
//   ) {
//     let cpu = CpuFeatureLevel::default();

//     let coeff_area: usize = tx_size.coded_tx_size().area();
//     let mut src_storage = [T::cast_from(0); 64 * 64];
//     let src = &mut src_storage[..tx_size.area()];
//     let mut dst = Plane::from_slice(
//       &[T::zero(); 64 * 64][..tx_size.area()],
//       tx_size.width(),
//     );
//     let mut res_storage = [0i16; 64 * 64];
//     let res = &mut res_storage[..tx_size.area()];
//     let mut freq_storage = [MaybeUninit::uninit(); 64 * 64];
//     let freq = &mut freq_storage[..tx_size.area()];
//     for ((r, s), d) in
//       res.iter_mut().zip(src.iter_mut()).zip(dst.data.iter_mut())
//     {
//       *s = T::cast_from(random::<u8>());
//       *d = T::cast_from(random::<u8>());
//       *r = i16::cast_from(*s) - i16::cast_from(*d);
//     }
//     forward_transform(res, freq, tx_size.width(), tx_size, tx_type, 8, cpu);
//     // SAFETY: forward_transform initialized freq
//     let freq = unsafe { slice_assume_init_mut(freq) };
//     inverse_transform_add(
//       freq,
//       &mut dst.as_region_mut(),
//       coeff_area.try_into().unwrap(),
//       tx_size,
//       tx_type,
//       8,
//       cpu,
//     );

//     for (s, d) in src.iter().zip(dst.data.iter()) {
//       assert!(i16::abs(i16::cast_from(*s) - i16::cast_from(*d)) <= tolerance);
//     }
//   }

//   #[test]
//   fn log_tx_ratios() {
//     let combinations = [
//       (TxSize::TX_4X4, 0),
//       (TxSize::TX_8X8, 0),
//       (TxSize::TX_16X16, 0),
//       (TxSize::TX_32X32, 0),
//       (TxSize::TX_64X64, 0),
//       (TxSize::TX_4X8, -1),
//       (TxSize::TX_8X4, 1),
//       (TxSize::TX_8X16, -1),
//       (TxSize::TX_16X8, 1),
//       (TxSize::TX_16X32, -1),
//       (TxSize::TX_32X16, 1),
//       (TxSize::TX_32X64, -1),
//       (TxSize::TX_64X32, 1),
//       (TxSize::TX_4X16, -2),
//       (TxSize::TX_16X4, 2),
//       (TxSize::TX_8X32, -2),
//       (TxSize::TX_32X8, 2),
//       (TxSize::TX_16X64, -2),
//       (TxSize::TX_64X16, 2),
//     ];

//     for &(tx_size, expected) in combinations.iter() {
//       println!(
//         "Testing combination {:?}, {:?}",
//         tx_size.width(),
//         tx_size.height()
//       );
//       assert!(
//         get_rect_tx_log_ratio(tx_size.width(), tx_size.height()) == expected
//       );
//     }
//   }

//   fn roundtrips<T: Pixel>() {
//     let combinations = [
//       (TX_4X4, WHT_WHT, 0),
//       (TX_4X4, DCT_DCT, 0),
//       (TX_4X4, ADST_DCT, 0),
//       (TX_4X4, DCT_ADST, 0),
//       (TX_4X4, ADST_ADST, 0),
//       (TX_4X4, FLIPADST_DCT, 0),
//       (TX_4X4, DCT_FLIPADST, 0),
//       (TX_4X4, IDTX, 0),
//       (TX_4X4, V_DCT, 0),
//       (TX_4X4, H_DCT, 0),
//       (TX_4X4, V_ADST, 0),
//       (TX_4X4, H_ADST, 0),
//       (TX_8X8, DCT_DCT, 1),
//       (TX_8X8, ADST_DCT, 1),
//       (TX_8X8, DCT_ADST, 1),
//       (TX_8X8, ADST_ADST, 1),
//       (TX_8X8, FLIPADST_DCT, 1),
//       (TX_8X8, DCT_FLIPADST, 1),
//       (TX_8X8, IDTX, 0),
//       (TX_8X8, V_DCT, 0),
//       (TX_8X8, H_DCT, 0),
//       (TX_8X8, V_ADST, 0),
//       (TX_8X8, H_ADST, 1),
//       (TX_16X16, DCT_DCT, 1),
//       (TX_16X16, ADST_DCT, 1),
//       (TX_16X16, DCT_ADST, 1),
//       (TX_16X16, ADST_ADST, 1),
//       (TX_16X16, FLIPADST_DCT, 1),
//       (TX_16X16, DCT_FLIPADST, 1),
//       (TX_16X16, IDTX, 0),
//       (TX_16X16, V_DCT, 1),
//       (TX_16X16, H_DCT, 1),
//       // 32x transforms only use DCT_DCT and IDTX
//       (TX_32X32, DCT_DCT, 2),
//       (TX_32X32, IDTX, 0),
//       // 64x transforms only use DCT_DCT and IDTX
//       //(TX_64X64, DCT_DCT, 0),
//       (TX_4X8, DCT_DCT, 1),
//       (TX_8X4, DCT_DCT, 1),
//       (TX_4X16, DCT_DCT, 1),
//       (TX_16X4, DCT_DCT, 1),
//       (TX_8X16, DCT_DCT, 1),
//       (TX_16X8, DCT_DCT, 1),
//       (TX_8X32, DCT_DCT, 2),
//       (TX_32X8, DCT_DCT, 2),
//       (TX_16X32, DCT_DCT, 2),
//       (TX_32X16, DCT_DCT, 2),
//     ];
//     for &(tx_size, tx_type, tolerance) in combinations.iter() {
//       println!("Testing combination {:?}, {:?}", tx_size, tx_type);
//       test_roundtrip::<T>(tx_size, tx_type, tolerance);
//     }
//   }

//   #[test]
//   fn roundtrips_u8() {
//     roundtrips::<u8>();
//   }

//   #[test]
//   fn roundtrips_u16() {
//     roundtrips::<u16>();
//   }
// }
