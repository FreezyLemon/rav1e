use crate::TX_TYPES_PLUS_LL;

#[derive(Debug, Clone, Copy)]
pub enum TxType1D {
  DCT,
  ADST,
  FLIPADST,
  IDTX,
  WHT,
}

pub const VTX_TAB: [TxType1D; TX_TYPES_PLUS_LL] = [
  TxType1D::DCT,
  TxType1D::ADST,
  TxType1D::DCT,
  TxType1D::ADST,
  TxType1D::FLIPADST,
  TxType1D::DCT,
  TxType1D::FLIPADST,
  TxType1D::ADST,
  TxType1D::FLIPADST,
  TxType1D::IDTX,
  TxType1D::DCT,
  TxType1D::IDTX,
  TxType1D::ADST,
  TxType1D::IDTX,
  TxType1D::FLIPADST,
  TxType1D::IDTX,
  TxType1D::WHT,
];

pub const HTX_TAB: [TxType1D; TX_TYPES_PLUS_LL] = [
  TxType1D::DCT,
  TxType1D::DCT,
  TxType1D::ADST,
  TxType1D::ADST,
  TxType1D::DCT,
  TxType1D::FLIPADST,
  TxType1D::FLIPADST,
  TxType1D::FLIPADST,
  TxType1D::ADST,
  TxType1D::IDTX,
  TxType1D::IDTX,
  TxType1D::DCT,
  TxType1D::IDTX,
  TxType1D::ADST,
  TxType1D::IDTX,
  TxType1D::FLIPADST,
  TxType1D::WHT,
];
