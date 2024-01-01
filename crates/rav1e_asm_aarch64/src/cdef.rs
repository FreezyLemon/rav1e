extern {
  pub fn rav1e_cdef_filter4_8bpc_neon(
    dst: *mut u8, dst_stride: isize, tmp: *const u16, pri_strength: i32,
    sec_strength: i32, dir: i32, damping: i32, h: i32, edges: isize,
  );

  pub fn rav1e_cdef_padding4_8bpc_neon(
    tmp: *mut u16, src: *const u8, src_stride: isize, left: *const [u8; 2],
    top: *const u8, bottom: *const u8, h: i32, edges: isize,
  );

  pub fn rav1e_cdef_filter8_8bpc_neon(
    dst: *mut u8, dst_stride: isize, tmp: *const u16, pri_strength: i32,
    sec_strength: i32, dir: i32, damping: i32, h: i32, edges: isize,
  );

  pub fn rav1e_cdef_padding8_8bpc_neon(
    tmp: *mut u16, src: *const u8, src_stride: isize, left: *const [u8; 2],
    top: *const u8, bottom: *const u8, h: i32, edges: isize,
  );

  pub fn rav1e_cdef_filter4_16bpc_neon(
    dst: *mut u16, dst_stride: isize, tmp: *const u16, pri_strength: i32,
    sec_strength: i32, dir: i32, damping: i32, h: i32, edges: isize, bd: i32,
  );

  pub fn rav1e_cdef_padding4_16bpc_neon(
    tmp: *mut u16, src: *const u16, src_stride: isize, left: *const [u16; 2],
    top: *const u16, bottom: *const u16, h: i32, edges: isize,
  );

  pub fn rav1e_cdef_filter8_16bpc_neon(
    dst: *mut u16, dst_stride: isize, tmp: *const u16, pri_strength: i32,
    sec_strength: i32, dir: i32, damping: i32, h: i32, edges: isize, bd: i32,
  );

  pub fn rav1e_cdef_padding8_16bpc_neon(
    tmp: *mut u16, src: *const u16, src_stride: isize, left: *const [u16; 2],
    top: *const u16, bottom: *const u16, h: i32, edges: isize,
  );
}

extern {
  pub fn rav1e_cdef_find_dir_8bpc_neon(
    tmp: *const u8, tmp_stride: isize, var: *mut u32,
  ) -> i32;
}

extern {
  pub fn rav1e_cdef_find_dir_16bpc_neon(
    tmp: *const u16, tmp_stride: isize, var: *mut u32, max_bitdepth: i32,
  ) -> i32;
}
