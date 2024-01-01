pub mod cdef_dist;
pub mod sse;

macro_rules! declare_asm_dist_fn {
  ($(($name: ident, $T: ident)),+) => (
    $(
      extern { pub fn $name (
        src: *const $T, src_stride: isize, dst: *const $T, dst_stride: isize
      ) -> u32; }
    )+
  )
}

macro_rules! declare_asm_satd_hbd_fn {
  ($($name: ident),+) => (
    $(
      extern { pub fn $name (
        src: *const u16, src_stride: isize, dst: *const u16, dst_stride: isize, bdmax: u32
      ) -> u32; }
    )+
  )
}

declare_asm_dist_fn![
  // SSSE3
  (rav1e_sad_4x4_hbd_ssse3, u16),
  (rav1e_sad_16x16_hbd_ssse3, u16),
  (rav1e_satd_8x8_ssse3, u8),
  // SSE2
  (rav1e_sad4x4_sse2, u8),
  (rav1e_sad4x8_sse2, u8),
  (rav1e_sad4x16_sse2, u8),
  (rav1e_sad8x4_sse2, u8),
  (rav1e_sad8x8_sse2, u8),
  (rav1e_sad8x16_sse2, u8),
  (rav1e_sad8x32_sse2, u8),
  (rav1e_sad16x4_sse2, u8),
  (rav1e_sad16x8_sse2, u8),
  (rav1e_sad16x16_sse2, u8),
  (rav1e_sad16x32_sse2, u8),
  (rav1e_sad16x64_sse2, u8),
  (rav1e_sad32x8_sse2, u8),
  (rav1e_sad32x16_sse2, u8),
  (rav1e_sad32x32_sse2, u8),
  (rav1e_sad32x64_sse2, u8),
  (rav1e_sad64x16_sse2, u8),
  (rav1e_sad64x32_sse2, u8),
  (rav1e_sad64x64_sse2, u8),
  (rav1e_sad64x128_sse2, u8),
  (rav1e_sad128x64_sse2, u8),
  (rav1e_sad128x128_sse2, u8),
  // SSE4
  (rav1e_satd_4x4_sse4, u8),
  // AVX
  (rav1e_sad32x8_avx2, u8),
  (rav1e_sad32x16_avx2, u8),
  (rav1e_sad32x32_avx2, u8),
  (rav1e_sad32x64_avx2, u8),
  (rav1e_sad64x16_avx2, u8),
  (rav1e_sad64x32_avx2, u8),
  (rav1e_sad64x64_avx2, u8),
  (rav1e_sad64x128_avx2, u8),
  (rav1e_sad128x64_avx2, u8),
  (rav1e_sad128x128_avx2, u8),
  (rav1e_satd_4x4_avx2, u8),
  (rav1e_satd_8x8_avx2, u8),
  (rav1e_satd_16x16_avx2, u8),
  (rav1e_satd_32x32_avx2, u8),
  (rav1e_satd_64x64_avx2, u8),
  (rav1e_satd_128x128_avx2, u8),
  (rav1e_satd_4x8_avx2, u8),
  (rav1e_satd_8x4_avx2, u8),
  (rav1e_satd_8x16_avx2, u8),
  (rav1e_satd_16x8_avx2, u8),
  (rav1e_satd_16x32_avx2, u8),
  (rav1e_satd_32x16_avx2, u8),
  (rav1e_satd_32x64_avx2, u8),
  (rav1e_satd_64x32_avx2, u8),
  (rav1e_satd_64x128_avx2, u8),
  (rav1e_satd_128x64_avx2, u8),
  (rav1e_satd_4x16_avx2, u8),
  (rav1e_satd_16x4_avx2, u8),
  (rav1e_satd_8x32_avx2, u8),
  (rav1e_satd_32x8_avx2, u8),
  (rav1e_satd_16x64_avx2, u8),
  (rav1e_satd_64x16_avx2, u8)
];

declare_asm_satd_hbd_fn![
  rav1e_satd_4x4_hbd_avx2,
  rav1e_satd_8x4_hbd_avx2,
  rav1e_satd_4x8_hbd_avx2,
  rav1e_satd_8x8_hbd_avx2,
  rav1e_satd_16x8_hbd_avx2,
  rav1e_satd_16x16_hbd_avx2,
  rav1e_satd_32x32_hbd_avx2,
  rav1e_satd_64x64_hbd_avx2,
  rav1e_satd_128x128_hbd_avx2,
  rav1e_satd_16x32_hbd_avx2,
  rav1e_satd_16x64_hbd_avx2,
  rav1e_satd_32x16_hbd_avx2,
  rav1e_satd_32x64_hbd_avx2,
  rav1e_satd_64x16_hbd_avx2,
  rav1e_satd_64x32_hbd_avx2,
  rav1e_satd_64x128_hbd_avx2,
  rav1e_satd_128x64_hbd_avx2,
  rav1e_satd_32x8_hbd_avx2,
  rav1e_satd_8x16_hbd_avx2,
  rav1e_satd_8x32_hbd_avx2,
  rav1e_satd_16x4_hbd_avx2,
  rav1e_satd_4x16_hbd_avx2
];
