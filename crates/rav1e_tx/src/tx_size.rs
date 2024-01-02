use TxSize::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum TxSize {
  TX_4X4,
  TX_8X8,
  TX_16X16,
  TX_32X32,
  TX_64X64,

  TX_4X8,
  TX_8X4,
  TX_8X16,
  TX_16X8,
  TX_16X32,
  TX_32X16,
  TX_32X64,
  TX_64X32,

  TX_4X16,
  TX_16X4,
  TX_8X32,
  TX_32X8,
  TX_16X64,
  TX_64X16,
}

impl TxSize {
  /// Number of square transform sizes
  pub const TX_SIZES: usize = 5;

  /// Number of transform sizes (including non-square sizes)
  pub const TX_SIZES_ALL: usize = 14 + 5;

  #[inline]
  pub const fn width(self) -> usize {
    1 << self.width_log2()
  }

  #[inline]
  pub const fn width_log2(self) -> usize {
    match self {
      TX_4X4 | TX_4X8 | TX_4X16 => 2,
      TX_8X8 | TX_8X4 | TX_8X16 | TX_8X32 => 3,
      TX_16X16 | TX_16X8 | TX_16X32 | TX_16X4 | TX_16X64 => 4,
      TX_32X32 | TX_32X16 | TX_32X64 | TX_32X8 => 5,
      TX_64X64 | TX_64X32 | TX_64X16 => 6,
    }
  }

  #[inline]
  pub const fn width_index(self) -> usize {
    self.width_log2() - TX_4X4.width_log2()
  }

  #[inline]
  pub const fn height(self) -> usize {
    1 << self.height_log2()
  }

  #[inline]
  pub const fn height_log2(self) -> usize {
    match self {
      TX_4X4 | TX_8X4 | TX_16X4 => 2,
      TX_8X8 | TX_4X8 | TX_16X8 | TX_32X8 => 3,
      TX_16X16 | TX_8X16 | TX_32X16 | TX_4X16 | TX_64X16 => 4,
      TX_32X32 | TX_16X32 | TX_64X32 | TX_8X32 => 5,
      TX_64X64 | TX_32X64 | TX_16X64 => 6,
    }
  }

  #[inline]
  pub const fn height_index(self) -> usize {
    self.height_log2() - TX_4X4.height_log2()
  }

  // TODO: Ext trait?
  // #[inline]
  // pub const fn width_mi(self) -> usize {
  //   self.width() >> MI_SIZE_LOG2
  // }

  #[inline]
  pub const fn area(self) -> usize {
    1 << self.area_log2()
  }

  #[inline]
  pub const fn area_log2(self) -> usize {
    self.width_log2() + self.height_log2()
  }

  // TODO: Ext trait?
  // #[inline]
  // pub const fn height_mi(self) -> usize {
  //   self.height() >> MI_SIZE_LOG2
  // }

  // #[inline]
  // pub const fn block_size(self) -> BlockSize {
  //   match self {
  //     TX_4X4 => BLOCK_4X4,
  //     TX_8X8 => BLOCK_8X8,
  //     TX_16X16 => BLOCK_16X16,
  //     TX_32X32 => BLOCK_32X32,
  //     TX_64X64 => BLOCK_64X64,
  //     TX_4X8 => BLOCK_4X8,
  //     TX_8X4 => BLOCK_8X4,
  //     TX_8X16 => BLOCK_8X16,
  //     TX_16X8 => BLOCK_16X8,
  //     TX_16X32 => BLOCK_16X32,
  //     TX_32X16 => BLOCK_32X16,
  //     TX_32X64 => BLOCK_32X64,
  //     TX_64X32 => BLOCK_64X32,
  //     TX_4X16 => BLOCK_4X16,
  //     TX_16X4 => BLOCK_16X4,
  //     TX_8X32 => BLOCK_8X32,
  //     TX_32X8 => BLOCK_32X8,
  //     TX_16X64 => BLOCK_16X64,
  //     TX_64X16 => BLOCK_64X16,
  //   }
  // }

  #[inline]
  pub const fn sqr(self) -> TxSize {
    match self {
      TX_4X4 | TX_4X8 | TX_8X4 | TX_4X16 | TX_16X4 => TX_4X4,
      TX_8X8 | TX_8X16 | TX_16X8 | TX_8X32 | TX_32X8 => TX_8X8,
      TX_16X16 | TX_16X32 | TX_32X16 | TX_16X64 | TX_64X16 => TX_16X16,
      TX_32X32 | TX_32X64 | TX_64X32 => TX_32X32,
      TX_64X64 => TX_64X64,
    }
  }

  #[inline]
  pub const fn sqr_up(self) -> TxSize {
    match self {
      TX_4X4 => TX_4X4,
      TX_8X8 | TX_4X8 | TX_8X4 => TX_8X8,
      TX_16X16 | TX_8X16 | TX_16X8 | TX_4X16 | TX_16X4 => TX_16X16,
      TX_32X32 | TX_16X32 | TX_32X16 | TX_8X32 | TX_32X8 => TX_32X32,
      TX_64X64 | TX_32X64 | TX_64X32 | TX_16X64 | TX_64X16 => TX_64X64,
    }
  }

  #[inline]
  pub fn by_dims(w: usize, h: usize) -> TxSize {
    match (w, h) {
      (4, 4) => TX_4X4,
      (8, 8) => TX_8X8,
      (16, 16) => TX_16X16,
      (32, 32) => TX_32X32,
      (64, 64) => TX_64X64,
      (4, 8) => TX_4X8,
      (8, 4) => TX_8X4,
      (8, 16) => TX_8X16,
      (16, 8) => TX_16X8,
      (16, 32) => TX_16X32,
      (32, 16) => TX_32X16,
      (32, 64) => TX_32X64,
      (64, 32) => TX_64X32,
      (4, 16) => TX_4X16,
      (16, 4) => TX_16X4,
      (8, 32) => TX_8X32,
      (32, 8) => TX_32X8,
      (16, 64) => TX_16X64,
      (64, 16) => TX_64X16,
      _ => unreachable!(),
    }
  }

  #[inline]
  pub const fn is_rect(self) -> bool {
    self.width_log2() != self.height_log2()
  }
}
