use std::ops::Index;

use crate::{TxSize, TxType1D};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum TxType {
  DCT_DCT = 0,   // DCT  in both horizontal and vertical
  ADST_DCT = 1,  // ADST in vertical, DCT in horizontal
  DCT_ADST = 2,  // DCT  in vertical, ADST in horizontal
  ADST_ADST = 3, // ADST in both directions
  FLIPADST_DCT = 4,
  DCT_FLIPADST = 5,
  FLIPADST_FLIPADST = 6,
  ADST_FLIPADST = 7,
  FLIPADST_ADST = 8,
  IDTX = 9,
  V_DCT = 10,
  H_DCT = 11,
  V_ADST = 12,
  H_ADST = 13,
  V_FLIPADST = 14,
  H_FLIPADST = 15,
  WHT_WHT = 16,
}

impl TxType {
  /// Compute transform type for inter chroma.
  ///
  /// <https://aomediacodec.github.io/av1-spec/#compute-transform-type-function>
  #[inline]
  pub fn uv_inter(self, uv_tx_size: TxSize) -> Self {
    use TxType::*;
    if uv_tx_size.sqr_up() == TxSize::TX_32X32 {
      match self {
        IDTX => IDTX,
        _ => DCT_DCT,
      }
    } else if uv_tx_size.sqr() == TxSize::TX_16X16 {
      match self {
        V_ADST | H_ADST | V_FLIPADST | H_FLIPADST => DCT_DCT,
        _ => self,
      }
    } else {
      self
    }
  }

  pub fn into_1d_types(self) -> (TxType1D, TxType1D) {
    match self {
      TxType::DCT_DCT => (TxType1D::DCT, TxType1D::DCT),
      TxType::ADST_DCT => (TxType1D::ADST, TxType1D::DCT),
      TxType::DCT_ADST => (TxType1D::DCT, TxType1D::ADST),
      TxType::ADST_ADST => (TxType1D::ADST, TxType1D::ADST),
      TxType::FLIPADST_DCT => (TxType1D::FLIPADST, TxType1D::DCT),
      TxType::DCT_FLIPADST => (TxType1D::DCT, TxType1D::FLIPADST),
      TxType::FLIPADST_FLIPADST => (TxType1D::FLIPADST, TxType1D::FLIPADST),
      TxType::ADST_FLIPADST => (TxType1D::ADST, TxType1D::FLIPADST),
      TxType::FLIPADST_ADST => (TxType1D::FLIPADST, TxType1D::ADST),
      TxType::IDTX => (TxType1D::IDTX, TxType1D::IDTX),
      TxType::V_DCT => (TxType1D::DCT, TxType1D::IDTX),
      TxType::H_DCT => (TxType1D::IDTX, TxType1D::DCT),
      TxType::V_ADST => (TxType1D::ADST, TxType1D::IDTX),
      TxType::H_ADST => (TxType1D::IDTX, TxType1D::ADST),
      TxType::V_FLIPADST => (TxType1D::FLIPADST, TxType1D::IDTX),
      TxType::H_FLIPADST => (TxType1D::IDTX, TxType1D::FLIPADST),
      TxType::WHT_WHT => (TxType1D::WHT, TxType1D::WHT),
    }
  }
}

pub const TX_TYPES: usize = 16;
pub const TX_TYPES_PLUS_LL: usize = 17;

impl<T> Index<TxType> for [T; TX_TYPES] {
  type Output = T;
  #[inline]
  fn index(&self, tx_type: TxType) -> &Self::Output {
    // SAFETY: Wraps WHT_WHT to DCT_DCT
    unsafe { self.get_unchecked((tx_type as usize) & 15) }
  }
}

impl<T> Index<TxType> for [T; TX_TYPES_PLUS_LL] {
  type Output = T;
  #[inline]
  fn index(&self, tx_type: TxType) -> &Self::Output {
    // SAFETY: values of TxType are < TX_TYPES_PLUS_LL
    unsafe { self.get_unchecked(tx_type as usize) }
  }
}
