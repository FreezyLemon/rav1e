extern {
  fn rav1e_avg_8bpc_ssse3(
    dst: *mut u8, dst_stride: libc::ptrdiff_t, tmp1: *const i16,
    tmp2: *const i16, w: i32, h: i32,
  );

  fn rav1e_avg_8bpc_avx2(
    dst: *mut u8, dst_stride: libc::ptrdiff_t, tmp1: *const i16,
    tmp2: *const i16, w: i32, h: i32,
  );

  fn rav1e_avg_8bpc_avx512icl(
    dst: *mut u8, dst_stride: libc::ptrdiff_t, tmp1: *const i16,
    tmp2: *const i16, w: i32, h: i32,
  );

  fn rav1e_avg_16bpc_ssse3(
    dst: *mut u16, dst_stride: libc::ptrdiff_t, tmp1: *const i16,
    tmp2: *const i16, w: i32, h: i32, bitdepth_max: i32,
  );

  fn rav1e_avg_16bpc_avx2(
    dst: *mut u16, dst_stride: libc::ptrdiff_t, tmp1: *const i16,
    tmp2: *const i16, w: i32, h: i32, bitdepth_max: i32,
  );
}
