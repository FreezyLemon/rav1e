extern {
  fn rav1e_cdef_filter_4x4_avx2(
    dst: *mut u8, dst_stride: isize, tmp: *const u16, tmp_stride: isize,
    pri_strength: i32, sec_strength: i32, dir: i32, damping: i32,
  );

  fn rav1e_cdef_filter_4x8_avx2(
    dst: *mut u8, dst_stride: isize, tmp: *const u16, tmp_stride: isize,
    pri_strength: i32, sec_strength: i32, dir: i32, damping: i32,
  );

  fn rav1e_cdef_filter_8x8_avx2(
    dst: *mut u8, dst_stride: isize, tmp: *const u16, tmp_stride: isize,
    pri_strength: i32, sec_strength: i32, dir: i32, damping: i32,
  );
}

extern {
  fn rav1e_cdef_dir_8bpc_ssse3(
    tmp: *const u8, tmp_stride: isize, var: *mut u32,
  ) -> i32;

  fn rav1e_cdef_dir_8bpc_avx2(
    tmp: *const u8, tmp_stride: isize, var: *mut u32,
  ) -> i32;

  fn rav1e_cdef_dir_16bpc_ssse3(
    tmp: *const u16, tmp_stride: isize, var: *mut u32, bitdepth_max: i32,
  ) -> i32;

  fn rav1e_cdef_dir_16bpc_sse4(
    tmp: *const u16, tmp_stride: isize, var: *mut u32, bitdepth_max: i32,
  ) -> i32;

  fn rav1e_cdef_dir_16bpc_avx2(
    tmp: *const u16, tmp_stride: isize, var: *mut u32, bitdepth_max: i32,
  ) -> i32;
}
