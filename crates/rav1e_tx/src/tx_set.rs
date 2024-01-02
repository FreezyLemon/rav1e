#[derive(Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum TxSet {
  // DCT only
  TX_SET_DCTONLY,
  // DCT + Identity only
  TX_SET_INTER_3, // TX_SET_DCT_IDTX
  // Discrete Trig transforms w/o flip (4) + Identity (1)
  TX_SET_INTRA_2, // TX_SET_DTT4_IDTX
  // Discrete Trig transforms w/o flip (4) + Identity (1) + 1D Hor/vert DCT (2)
  TX_SET_INTRA_1, // TX_SET_DTT4_IDTX_1DDCT
  // Discrete Trig transforms w/ flip (9) + Identity (1) + 1D Hor/Ver DCT (2)
  TX_SET_INTER_2, // TX_SET_DTT9_IDTX_1DDCT
  // Discrete Trig transforms w/ flip (9) + Identity (1) + 1D Hor/Ver (6)
  TX_SET_INTER_1, // TX_SET_ALL16
}
