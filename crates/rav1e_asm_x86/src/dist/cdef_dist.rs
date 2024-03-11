use std::arch::x86_64::*;

pub type CdefDistKernelFn = unsafe extern fn(
  src: *const u8,
  src_stride: isize,
  dst: *const u8,
  dst_stride: isize,
  ret_ptr: *mut u32,
);

pub type CdefDistKernelHBDFn = unsafe fn(
  src: *const u16,
  src_stride: isize,
  dst: *const u16,
  dst_stride: isize,
) -> (u32, u32, u32);

/// Store functions in a 8x8 grid. Most will be empty.
pub const CDEF_DIST_KERNEL_FNS_LENGTH: usize = 8 * 8;

pub const fn kernel_fn_index(w: usize, h: usize) -> usize {
  ((w - 1) << 3) | (h - 1)
}

pub const CDEF_DIST_KERNEL_FNS_SSE2: [Option<CdefDistKernelFn>;
  CDEF_DIST_KERNEL_FNS_LENGTH] = {
  let mut out: [Option<CdefDistKernelFn>; CDEF_DIST_KERNEL_FNS_LENGTH] =
    [None; CDEF_DIST_KERNEL_FNS_LENGTH];

  out[kernel_fn_index(4, 4)] = Some(rav1e_cdef_dist_kernel_4x4_sse2);
  out[kernel_fn_index(4, 8)] = Some(rav1e_cdef_dist_kernel_4x8_sse2);
  out[kernel_fn_index(8, 4)] = Some(rav1e_cdef_dist_kernel_8x4_sse2);
  out[kernel_fn_index(8, 8)] = Some(rav1e_cdef_dist_kernel_8x8_sse2);

  out
};

extern {
  fn rav1e_cdef_dist_kernel_4x4_sse2(
    src: *const u8, src_stride: isize, dst: *const u8, dst_stride: isize,
    ret_ptr: *mut u32,
  );
  fn rav1e_cdef_dist_kernel_4x8_sse2(
    src: *const u8, src_stride: isize, dst: *const u8, dst_stride: isize,
    ret_ptr: *mut u32,
  );
  fn rav1e_cdef_dist_kernel_8x4_sse2(
    src: *const u8, src_stride: isize, dst: *const u8, dst_stride: isize,
    ret_ptr: *mut u32,
  );
  fn rav1e_cdef_dist_kernel_8x8_sse2(
    src: *const u8, src_stride: isize, dst: *const u8, dst_stride: isize,
    ret_ptr: *mut u32,
  );
}

pub const CDEF_DIST_KERNEL_HBD_FNS_AVX2: [Option<CdefDistKernelHBDFn>;
  CDEF_DIST_KERNEL_FNS_LENGTH] = {
  let mut out: [Option<CdefDistKernelHBDFn>; CDEF_DIST_KERNEL_FNS_LENGTH] =
    [None; CDEF_DIST_KERNEL_FNS_LENGTH];

  out[kernel_fn_index(8, 8)] = Some(rav1e_cdef_dist_kernel_8x8_hbd_avx2);

  out
};

#[target_feature(enable = "avx2")]
#[inline]
unsafe fn mm256_sum_i32(ymm: __m256i) -> i32 {
  // We split the vector in half and then add the two halves and sum.
  let m1 = _mm256_extracti128_si256(ymm, 1);
  let m2 = _mm256_castsi256_si128(ymm);
  let m2 = _mm_add_epi32(m2, m1);
  let m1 = _mm_shuffle_epi32(m2, 0b11_10_11_10);
  let m2 = _mm_add_epi32(m2, m1);
  let m1 = _mm_shuffle_epi32(m2, 0b01_01_01_01);
  let m2 = _mm_add_epi32(m2, m1);
  _mm_cvtsi128_si32(m2)
}

#[target_feature(enable = "avx2")]
#[inline]
unsafe fn rav1e_cdef_dist_kernel_8x8_hbd_avx2(
  src: *const u16, src_stride: isize, dst: *const u16, dst_stride: isize,
) -> (u32, u32, u32) {
  let src = src as *const u8;
  let dst = dst as *const u8;

  unsafe fn sum16(src: *const u8, src_stride: isize) -> u32 {
    let h = 8;
    let res = (0..h)
      .map(|row| _mm_load_si128(src.offset(row * src_stride) as *const _))
      .reduce(|a, b| _mm_add_epi16(a, b))
      .unwrap();

    let m32 = _mm256_cvtepi16_epi32(res);
    mm256_sum_i32(m32) as u32
  }
  unsafe fn mpadd32(
    src: *const u8, src_stride: isize, dst: *const u8, dst_stride: isize,
  ) -> u32 {
    let h = 8;
    let res = (0..h / 2)
      .map(|row| {
        let s = _mm256_loadu2_m128i(
          src.offset(2 * row * src_stride) as *const _,
          src.offset((2 * row + 1) * src_stride) as *const _,
        );

        let d = _mm256_loadu2_m128i(
          dst.offset(2 * row * dst_stride) as *const _,
          dst.offset((2 * row + 1) * dst_stride) as *const _,
        );

        _mm256_madd_epi16(s, d)
      })
      .reduce(|a, b| _mm256_add_epi32(a, b))
      .unwrap();
    mm256_sum_i32(res) as u32
  }

  let sum_s = sum16(src, src_stride);
  let sum_d = sum16(dst, dst_stride);
  let sum_s2 = mpadd32(src, src_stride, src, src_stride);
  let sum_d2 = mpadd32(dst, dst_stride, dst, dst_stride);
  let sum_sd = mpadd32(src, src_stride, dst, dst_stride);

  // To get the distortion, compute sum of squared error and apply a weight
  // based on the variance of the two planes.
  let sse = sum_d2 + sum_s2 - 2 * sum_sd;

  // Convert to 64-bits to avoid overflow when squaring
  let sum_s = sum_s as u64;
  let sum_d = sum_d as u64;

  let svar = (sum_s2 as u64 - (sum_s * sum_s + 32) / 64) as u32;
  let dvar = (sum_d2 as u64 - (sum_d * sum_d + 32) / 64) as u32;

  (svar, dvar, sse)
}
