use crate::*;
use v_frame::plane::{PlaneConfig, PlaneOffset};

pub const MAX_PLANES: usize = 3;

pub const BLOCK_SIZE_GROUPS: usize = 4;
pub const MAX_ANGLE_DELTA: usize = 3;
pub const DIRECTIONAL_MODES: usize = 8;
pub const KF_MODE_CONTEXTS: usize = 5;

pub const INTRA_INTER_CONTEXTS: usize = 4;
pub const INTER_MODE_CONTEXTS: usize = 8;
pub const DRL_MODE_CONTEXTS: usize = 3;
pub const COMP_INTER_CONTEXTS: usize = 5;
pub const COMP_REF_TYPE_CONTEXTS: usize = 5;
pub const UNI_COMP_REF_CONTEXTS: usize = 3;

pub const PLANE_TYPES: usize = 2;
pub const REF_TYPES: usize = 2;

pub const COMP_INDEX_CONTEXTS: usize = 6;
pub const COMP_GROUP_IDX_CONTEXTS: usize = 6;

/// Absolute offset in blocks, where a block is defined
/// to be an `N*N` square where `N == (1 << BLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BlockOffset {
  pub x: usize,
  pub y: usize,
}

/// Absolute offset in blocks inside a plane, where a block is defined
/// to be an `N*N` square where `N == (1 << BLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaneBlockOffset(pub BlockOffset);

/// Absolute offset in blocks inside a tile, where a block is defined
/// to be an `N*N` square where `N == (1 << BLOCK_TO_PLANE_SHIFT)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TileBlockOffset(pub BlockOffset);

impl BlockOffset {
  /// Offset of the superblock in which this block is located.
  #[inline]
  const fn sb_offset(self) -> SuperBlockOffset {
    SuperBlockOffset {
      x: self.x >> SUPERBLOCK_TO_BLOCK_SHIFT,
      y: self.y >> SUPERBLOCK_TO_BLOCK_SHIFT,
    }
  }

  /// Offset of the top-left pixel of this block.
  #[inline]
  const fn plane_offset(self, plane: &PlaneConfig) -> PlaneOffset {
    PlaneOffset {
      x: (self.x >> plane.xdec << BLOCK_TO_PLANE_SHIFT) as isize,
      y: (self.y >> plane.ydec << BLOCK_TO_PLANE_SHIFT) as isize,
    }
  }

  /// Convert to plane offset without decimation.
  #[inline]
  const fn to_luma_plane_offset(self) -> PlaneOffset {
    PlaneOffset {
      x: (self.x as isize) << BLOCK_TO_PLANE_SHIFT,
      y: (self.y as isize) << BLOCK_TO_PLANE_SHIFT,
    }
  }

  #[inline]
  const fn y_in_sb(self) -> usize {
    self.y % MIB_SIZE
  }

  #[inline]
  fn with_offset(self, col_offset: isize, row_offset: isize) -> BlockOffset {
    let x = self.x as isize + col_offset;
    let y = self.y as isize + row_offset;
    debug_assert!(x >= 0);
    debug_assert!(y >= 0);

    BlockOffset { x: x as usize, y: y as usize }
  }
}

impl PlaneBlockOffset {
  /// Offset of the superblock in which this block is located.
  #[inline]
  pub const fn sb_offset(self) -> PlaneSuperBlockOffset {
    PlaneSuperBlockOffset(self.0.sb_offset())
  }

  /// Offset of the top-left pixel of this block.
  #[inline]
  pub const fn plane_offset(self, plane: &PlaneConfig) -> PlaneOffset {
    self.0.plane_offset(plane)
  }

  /// Convert to plane offset without decimation.
  #[inline]
  pub const fn to_luma_plane_offset(self) -> PlaneOffset {
    self.0.to_luma_plane_offset()
  }

  #[inline]
  pub const fn y_in_sb(self) -> usize {
    self.0.y_in_sb()
  }

  #[inline]
  pub fn with_offset(
    self, col_offset: isize, row_offset: isize,
  ) -> PlaneBlockOffset {
    Self(self.0.with_offset(col_offset, row_offset))
  }
}

impl TileBlockOffset {
  /// Offset of the superblock in which this block is located.
  #[inline]
  pub const fn sb_offset(self) -> TileSuperBlockOffset {
    TileSuperBlockOffset(self.0.sb_offset())
  }

  /// Offset of the top-left pixel of this block.
  #[inline]
  pub const fn plane_offset(self, plane: &PlaneConfig) -> PlaneOffset {
    self.0.plane_offset(plane)
  }

  /// Convert to plane offset without decimation.
  #[inline]
  pub const fn to_luma_plane_offset(self) -> PlaneOffset {
    self.0.to_luma_plane_offset()
  }

  #[inline]
  pub const fn y_in_sb(self) -> usize {
    self.0.y_in_sb()
  }

  #[inline]
  pub fn with_offset(
    self, col_offset: isize, row_offset: isize,
  ) -> TileBlockOffset {
    Self(self.0.with_offset(col_offset, row_offset))
  }
}
