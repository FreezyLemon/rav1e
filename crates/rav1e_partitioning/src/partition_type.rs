#![allow(non_camel_case_types)]

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
pub enum PartitionType {
  PARTITION_NONE,
  PARTITION_HORZ,
  PARTITION_VERT,
  PARTITION_SPLIT,
  PARTITION_HORZ_A, // HORZ split and the top partition is split again
  PARTITION_HORZ_B, // HORZ split and the bottom partition is split again
  PARTITION_VERT_A, // VERT split and the left partition is split again
  PARTITION_VERT_B, // VERT split and the right partition is split again
  PARTITION_HORZ_4, // 4:1 horizontal partition
  PARTITION_VERT_4, // 4:1 vertical partition
  PARTITION_INVALID,
}
