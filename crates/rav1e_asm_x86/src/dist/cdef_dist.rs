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