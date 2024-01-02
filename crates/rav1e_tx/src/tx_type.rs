use std::ops::Index;

use crate::TxSize;

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
