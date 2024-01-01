macro_rules! declare_asm_sse_fn {
  ($($name: ident),+) => (
    $(
      extern { fn $name (
        src: *const u8, src_stride: isize, dst: *const u8, dst_stride: isize, scale: *const u32, scale_stride: isize
      ) -> u64; }
    )+
  )
}

macro_rules! declare_asm_hbd_sse_fn {
  ($($name: ident),+) => (
    $(
      extern { fn $name (
        src: *const u16, src_stride: isize, dst: *const u16, dst_stride: isize, scale: *const u32, scale_stride: isize
      ) -> u64; }
    )+
  )
}

declare_asm_sse_fn![
  // SSSE3
  rav1e_weighted_sse_4x4_ssse3,
  rav1e_weighted_sse_4x8_ssse3,
  rav1e_weighted_sse_4x16_ssse3,
  rav1e_weighted_sse_8x4_ssse3,
  rav1e_weighted_sse_8x8_ssse3,
  rav1e_weighted_sse_8x16_ssse3,
  rav1e_weighted_sse_8x32_ssse3,
  // AVX2
  rav1e_weighted_sse_16x4_avx2,
  rav1e_weighted_sse_16x8_avx2,
  rav1e_weighted_sse_16x16_avx2,
  rav1e_weighted_sse_16x32_avx2,
  rav1e_weighted_sse_16x64_avx2,
  rav1e_weighted_sse_32x8_avx2,
  rav1e_weighted_sse_32x16_avx2,
  rav1e_weighted_sse_32x32_avx2,
  rav1e_weighted_sse_32x64_avx2,
  rav1e_weighted_sse_64x16_avx2,
  rav1e_weighted_sse_64x32_avx2,
  rav1e_weighted_sse_64x64_avx2,
  rav1e_weighted_sse_64x128_avx2,
  rav1e_weighted_sse_128x64_avx2,
  rav1e_weighted_sse_128x128_avx2
];

declare_asm_hbd_sse_fn![
  // SSE2
  rav1e_weighted_sse_4x4_hbd_sse2
];
