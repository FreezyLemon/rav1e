use crate::{TxSize, TxType};

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
