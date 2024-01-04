#![allow(non_camel_case_types)]

// TODO handle serde stuff

use std::fmt;

use rav1e_tx::TxSize;
use thiserror::Error;

use BlockSize::*;
use TxSize::*;

use crate::{PartitionType, MI_SIZE_LOG2, IMPORTANCE_BLOCK_TO_BLOCK_SHIFT, BLOCK_TO_PLANE_SHIFT};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BlockSize {
  BLOCK_4X4,
  BLOCK_4X8,
  BLOCK_8X4,
  BLOCK_8X8,
  BLOCK_8X16,
  BLOCK_16X8,
  BLOCK_16X16,
  BLOCK_16X32,
  BLOCK_32X16,
  BLOCK_32X32,
  BLOCK_32X64,
  BLOCK_64X32,
  BLOCK_64X64,
  BLOCK_64X128,
  BLOCK_128X64,
  BLOCK_128X128,
  BLOCK_4X16,
  BLOCK_16X4,
  BLOCK_8X32,
  BLOCK_32X8,
  BLOCK_16X64,
  BLOCK_64X16,
}

#[derive(Debug, Error, Copy, Clone, Eq, PartialEq)]
pub struct InvalidBlockSize;

impl fmt::Display for InvalidBlockSize {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("invalid block size")
  }
}

impl PartialOrd for BlockSize {
  #[inline(always)]
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    use std::cmp::Ordering::{Equal, Greater, Less};
    match (
      self.width().cmp(&other.width()),
      self.height().cmp(&other.height()),
    ) {
      (Greater, Less) | (Less, Greater) => None,
      (Equal, Equal) => Some(Equal),
      (Greater, _) | (_, Greater) => Some(Greater),
      (Less, _) | (_, Less) => Some(Less),
    }
  }
}

#[cfg(test)]
impl Default for BlockSize {
  fn default() -> Self {
    BlockSize::BLOCK_64X64
  }
}

pub static MAX_TXSIZE_RECT_LOOKUP: [TxSize; BlockSize::BLOCK_SIZES_ALL] = [
  TX_4X4,   // 4x4
  TX_4X8,   // 4x8
  TX_8X4,   // 8x4
  TX_8X8,   // 8x8
  TX_8X16,  // 8x16
  TX_16X8,  // 16x8
  TX_16X16, // 16x16
  TX_16X32, // 16x32
  TX_32X16, // 32x16
  TX_32X32, // 32x32
  TX_32X64, // 32x64
  TX_64X32, // 64x32
  TX_64X64, // 64x64
  TX_64X64, // 64x128
  TX_64X64, // 128x64
  TX_64X64, // 128x128
  TX_4X16,  // 4x16
  TX_16X4,  // 16x4
  TX_8X32,  // 8x32
  TX_32X8,  // 32x8
  TX_16X64, // 16x64
  TX_64X16, // 64x16
];

impl BlockSize {
  pub const BLOCK_SIZES_ALL: usize = 22;
  pub const BLOCK_SIZES: usize = BlockSize::BLOCK_SIZES_ALL - 6; // BLOCK_SIZES_ALL minus 4:1 non-squares, six of them

  #[inline]
  /// # Errors
  ///
  /// - Returns `InvalidBlockSize` if the given `w` and `h` do not produce
  ///   a valid block size.
  pub fn from_width_and_height_opt(
    w: usize, h: usize,
  ) -> Result<BlockSize, InvalidBlockSize> {
    match (w, h) {
      (4, 4) => Ok(BLOCK_4X4),
      (4, 8) => Ok(BLOCK_4X8),
      (4, 16) => Ok(BLOCK_4X16),
      (8, 4) => Ok(BLOCK_8X4),
      (8, 8) => Ok(BLOCK_8X8),
      (8, 16) => Ok(BLOCK_8X16),
      (8, 32) => Ok(BLOCK_8X32),
      (16, 4) => Ok(BLOCK_16X4),
      (16, 8) => Ok(BLOCK_16X8),
      (16, 16) => Ok(BLOCK_16X16),
      (16, 32) => Ok(BLOCK_16X32),
      (16, 64) => Ok(BLOCK_16X64),
      (32, 8) => Ok(BLOCK_32X8),
      (32, 16) => Ok(BLOCK_32X16),
      (32, 32) => Ok(BLOCK_32X32),
      (32, 64) => Ok(BLOCK_32X64),
      (64, 16) => Ok(BLOCK_64X16),
      (64, 32) => Ok(BLOCK_64X32),
      (64, 64) => Ok(BLOCK_64X64),
      (64, 128) => Ok(BLOCK_64X128),
      (128, 64) => Ok(BLOCK_128X64),
      (128, 128) => Ok(BLOCK_128X128),
      _ => Err(InvalidBlockSize),
    }
  }

  /// # Panics
  ///
  /// - If the given `w` and `h` do not produce a valid block size.
  pub fn from_width_and_height(w: usize, h: usize) -> BlockSize {
    Self::from_width_and_height_opt(w, h).unwrap()
  }

  #[inline]
  pub fn cfl_allowed(self) -> bool {
    // TODO: fix me when enabling EXT_PARTITION_TYPES
    self <= BlockSize::BLOCK_32X32
  }

  #[inline]
  pub const fn width(self) -> usize {
    1 << self.width_log2()
  }

  /// width * height
  #[inline]
  pub const fn area(self) -> usize {
    self.width() * self.height()
  }

  #[inline]
  pub const fn width_log2(self) -> usize {
    match self {
      BLOCK_4X4 | BLOCK_4X8 | BLOCK_4X16 => 2,
      BLOCK_8X4 | BLOCK_8X8 | BLOCK_8X16 | BLOCK_8X32 => 3,
      BLOCK_16X4 | BLOCK_16X8 | BLOCK_16X16 | BLOCK_16X32 | BLOCK_16X64 => 4,
      BLOCK_32X8 | BLOCK_32X16 | BLOCK_32X32 | BLOCK_32X64 => 5,
      BLOCK_64X16 | BLOCK_64X32 | BLOCK_64X64 | BLOCK_64X128 => 6,
      BLOCK_128X64 | BLOCK_128X128 => 7,
    }
  }

  #[inline]
  pub const fn width_mi_log2(self) -> usize {
    self.width_log2() - 2
  }

  #[inline]
  pub const fn width_mi(self) -> usize {
    self.width() >> MI_SIZE_LOG2
  }

  #[inline]
  pub fn width_imp_b(self) -> usize {
    (self.width() >> (IMPORTANCE_BLOCK_TO_BLOCK_SHIFT + BLOCK_TO_PLANE_SHIFT))
      .max(1)
  }

  #[inline]
  pub const fn height(self) -> usize {
    1 << self.height_log2()
  }

  #[inline]
  pub const fn height_log2(self) -> usize {
    match self {
      BLOCK_4X4 | BLOCK_8X4 | BLOCK_16X4 => 2,
      BLOCK_4X8 | BLOCK_8X8 | BLOCK_16X8 | BLOCK_32X8 => 3,
      BLOCK_4X16 | BLOCK_8X16 | BLOCK_16X16 | BLOCK_32X16 | BLOCK_64X16 => 4,
      BLOCK_8X32 | BLOCK_16X32 | BLOCK_32X32 | BLOCK_64X32 => 5,
      BLOCK_16X64 | BLOCK_32X64 | BLOCK_64X64 | BLOCK_128X64 => 6,
      BLOCK_64X128 | BLOCK_128X128 => 7,
    }
  }

  #[inline]
  pub const fn height_mi_log2(self) -> usize {
    self.height_log2() - 2
  }

  #[inline]
  pub const fn height_mi(self) -> usize {
    self.height() >> MI_SIZE_LOG2
  }

  #[inline]
  pub fn height_imp_b(self) -> usize {
    (self.height() >> (IMPORTANCE_BLOCK_TO_BLOCK_SHIFT + BLOCK_TO_PLANE_SHIFT))
      .max(1)
  }

  #[inline]
  pub const fn tx_size(self) -> TxSize {
    match self {
      BLOCK_4X4 => TX_4X4,
      BLOCK_4X8 => TX_4X8,
      BLOCK_8X4 => TX_8X4,
      BLOCK_8X8 => TX_8X8,
      BLOCK_8X16 => TX_8X16,
      BLOCK_16X8 => TX_16X8,
      BLOCK_16X16 => TX_16X16,
      BLOCK_16X32 => TX_16X32,
      BLOCK_32X16 => TX_32X16,
      BLOCK_32X32 => TX_32X32,
      BLOCK_32X64 => TX_32X64,
      BLOCK_64X32 => TX_64X32,
      BLOCK_4X16 => TX_4X16,
      BLOCK_16X4 => TX_16X4,
      BLOCK_8X32 => TX_8X32,
      BLOCK_32X8 => TX_32X8,
      BLOCK_16X64 => TX_16X64,
      BLOCK_64X16 => TX_64X16,
      _ => TX_64X64,
    }
  }

  /// Source: `Subsampled_Size` (AV1 specification section 5.11.38)
  ///
  /// # Errors
  ///
  /// - Returns `InvalidBlockSize` if the given block size cannot
  ///   be subsampled in the requested way.
  #[inline]
  pub const fn subsampled_size(
    self, xdec: usize, ydec: usize,
  ) -> Result<BlockSize, InvalidBlockSize> {
    Ok(match (xdec, ydec) {
      (0, 0) /* 4:4:4 */ => self,
      (1, 0) /* 4:2:2 */ => match self {
        BLOCK_4X4 | BLOCK_8X4 => BLOCK_4X4,
        BLOCK_8X8 => BLOCK_4X8,
        BLOCK_16X4 => BLOCK_8X4,
        BLOCK_16X8 => BLOCK_8X8,
        BLOCK_16X16 => BLOCK_8X16,
        BLOCK_32X8 => BLOCK_16X8,
        BLOCK_32X16 => BLOCK_16X16,
        BLOCK_32X32 => BLOCK_16X32,
        BLOCK_64X16 => BLOCK_32X16,
        BLOCK_64X32 => BLOCK_32X32,
        BLOCK_64X64 => BLOCK_32X64,
        BLOCK_128X64 => BLOCK_64X64,
        BLOCK_128X128 => BLOCK_64X128,
        _ => return Err(InvalidBlockSize),
      },
      (1, 1) /* 4:2:0 */ => match self {
        BLOCK_4X4 | BLOCK_4X8 | BLOCK_8X4 | BLOCK_8X8 => BLOCK_4X4,
        BLOCK_4X16 | BLOCK_8X16 => BLOCK_4X8,
        BLOCK_8X32 => BLOCK_4X16,
        BLOCK_16X4 | BLOCK_16X8 => BLOCK_8X4,
        BLOCK_16X16 => BLOCK_8X8,
        BLOCK_16X32 => BLOCK_8X16,
        BLOCK_16X64 => BLOCK_8X32,
        BLOCK_32X8 => BLOCK_16X4,
        BLOCK_32X16 => BLOCK_16X8,
        BLOCK_32X32 => BLOCK_16X16,
        BLOCK_32X64 => BLOCK_16X32,
        BLOCK_64X16 => BLOCK_32X8,
        BLOCK_64X32 => BLOCK_32X16,
        BLOCK_64X64 => BLOCK_32X32,
        BLOCK_64X128 => BLOCK_32X64,
        BLOCK_128X64 => BLOCK_64X32,
        BLOCK_128X128 => BLOCK_64X64,
      },
      _ => return Err(InvalidBlockSize),
    })
  }

  /// # Panics
  ///
  /// Will panic if the subsampling is not possible
  #[inline]
  pub fn largest_chroma_tx_size(self, xdec: usize, ydec: usize) -> TxSize {
    let plane_bsize = self
      .subsampled_size(xdec, ydec)
      .expect("invalid block size for this subsampling mode");

    let chroma_tx_size = MAX_TXSIZE_RECT_LOOKUP[plane_bsize as usize];

    chroma_tx_size.coded_tx_size()
  }

  #[inline]
  pub const fn is_sqr(self) -> bool {
    self.width_log2() == self.height_log2()
  }

  #[inline]
  pub const fn is_sub8x8(self, xdec: usize, ydec: usize) -> bool {
    xdec != 0 && self.width_log2() == 2 || ydec != 0 && self.height_log2() == 2
  }

  #[inline]
  pub const fn sub8x8_offset(
    self, xdec: usize, ydec: usize,
  ) -> (isize, isize) {
    let offset_x = if xdec != 0 && self.width_log2() == 2 { -1 } else { 0 };
    let offset_y = if ydec != 0 && self.height_log2() == 2 { -1 } else { 0 };

    (offset_x, offset_y)
  }

  /// # Errors
  ///
  /// - Returns `InvalidBlockSize` if the block size cannot be split
  ///   in the requested way.
  pub const fn subsize(
    self, partition: PartitionType,
  ) -> Result<BlockSize, InvalidBlockSize> {
    use PartitionType::*;

    Ok(match partition {
      PARTITION_NONE => self,
      PARTITION_SPLIT => match self {
        BLOCK_8X8 => BLOCK_4X4,
        BLOCK_16X16 => BLOCK_8X8,
        BLOCK_32X32 => BLOCK_16X16,
        BLOCK_64X64 => BLOCK_32X32,
        BLOCK_128X128 => BLOCK_64X64,
        _ => return Err(InvalidBlockSize),
      },
      PARTITION_HORZ | PARTITION_HORZ_A | PARTITION_HORZ_B => match self {
        BLOCK_8X8 => BLOCK_8X4,
        BLOCK_16X16 => BLOCK_16X8,
        BLOCK_32X32 => BLOCK_32X16,
        BLOCK_64X64 => BLOCK_64X32,
        BLOCK_128X128 => BLOCK_128X64,
        _ => return Err(InvalidBlockSize),
      },
      PARTITION_VERT | PARTITION_VERT_A | PARTITION_VERT_B => match self {
        BLOCK_8X8 => BLOCK_4X8,
        BLOCK_16X16 => BLOCK_8X16,
        BLOCK_32X32 => BLOCK_16X32,
        BLOCK_64X64 => BLOCK_32X64,
        BLOCK_128X128 => BLOCK_64X128,
        _ => return Err(InvalidBlockSize),
      },
      PARTITION_HORZ_4 => match self {
        BLOCK_16X16 => BLOCK_16X4,
        BLOCK_32X32 => BLOCK_32X8,
        BLOCK_64X64 => BLOCK_64X16,
        _ => return Err(InvalidBlockSize),
      },
      PARTITION_VERT_4 => match self {
        BLOCK_16X16 => BLOCK_4X16,
        BLOCK_32X32 => BLOCK_8X32,
        BLOCK_64X64 => BLOCK_16X64,
        _ => return Err(InvalidBlockSize),
      },
      _ => return Err(InvalidBlockSize),
    })
  }

  pub const fn is_rect_tx_allowed(self) -> bool {
    !matches!(
      self,
      BLOCK_4X4
        | BLOCK_8X8
        | BLOCK_16X16
        | BLOCK_32X32
        | BLOCK_64X64
        | BLOCK_64X128
        | BLOCK_128X64
        | BLOCK_128X128
    )
  }
}

impl fmt::Display for BlockSize {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(
      f,
      "{}",
      match self {
        BlockSize::BLOCK_4X4 => "4x4",
        BlockSize::BLOCK_4X8 => "4x8",
        BlockSize::BLOCK_8X4 => "8x4",
        BlockSize::BLOCK_8X8 => "8x8",
        BlockSize::BLOCK_8X16 => "8x16",
        BlockSize::BLOCK_16X8 => "16x8",
        BlockSize::BLOCK_16X16 => "16x16",
        BlockSize::BLOCK_16X32 => "16x32",
        BlockSize::BLOCK_32X16 => "32x16",
        BlockSize::BLOCK_32X32 => "32x32",
        BlockSize::BLOCK_32X64 => "32x64",
        BlockSize::BLOCK_64X32 => "64x32",
        BlockSize::BLOCK_64X64 => "64x64",
        BlockSize::BLOCK_64X128 => "64x128",
        BlockSize::BLOCK_128X64 => "128x64",
        BlockSize::BLOCK_128X128 => "128x128",
        BlockSize::BLOCK_4X16 => "4x16",
        BlockSize::BLOCK_16X4 => "16x4",
        BlockSize::BLOCK_8X32 => "8x32",
        BlockSize::BLOCK_32X8 => "32x8",
        BlockSize::BLOCK_16X64 => "16x64",
        BlockSize::BLOCK_64X16 => "64x16",
      }
    )
  }
}
