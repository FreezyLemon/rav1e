use std::mem::MaybeUninit;

macro_rules! decl_angular_ipred_fn {
  ($($f:ident),+) => {
    extern {
      $(
        pub fn $f(
          dst: *mut u8, stride: libc::ptrdiff_t, topleft: *const u8,
          width: libc::c_int, height: libc::c_int, angle: libc::c_int,
        );
      )*
    }
  };
}

decl_angular_ipred_fn! {
  rav1e_ipred_h_8bpc_ssse3,
  rav1e_ipred_h_8bpc_avx2,
  rav1e_ipred_h_8bpc_avx512icl,
  rav1e_ipred_v_8bpc_ssse3,
  rav1e_ipred_v_8bpc_avx2,
  rav1e_ipred_v_8bpc_avx512icl,
  rav1e_ipred_dc_8bpc_ssse3,
  rav1e_ipred_dc_8bpc_avx2,
  rav1e_ipred_dc_8bpc_avx512icl,
  rav1e_ipred_dc_left_8bpc_ssse3,
  rav1e_ipred_dc_left_8bpc_avx2,
  rav1e_ipred_dc_left_8bpc_avx512icl,
  rav1e_ipred_dc_128_8bpc_ssse3,
  rav1e_ipred_dc_128_8bpc_avx2,
  rav1e_ipred_dc_128_8bpc_avx512icl,
  rav1e_ipred_dc_top_8bpc_ssse3,
  rav1e_ipred_dc_top_8bpc_avx2,
  rav1e_ipred_dc_top_8bpc_avx512icl,
  rav1e_ipred_smooth_v_8bpc_ssse3,
  rav1e_ipred_smooth_v_8bpc_avx2,
  rav1e_ipred_smooth_v_8bpc_avx512icl,
  rav1e_ipred_smooth_h_8bpc_ssse3,
  rav1e_ipred_smooth_h_8bpc_avx2,
  rav1e_ipred_smooth_h_8bpc_avx512icl,
  rav1e_ipred_smooth_8bpc_ssse3,
  rav1e_ipred_smooth_8bpc_avx2,
  rav1e_ipred_smooth_8bpc_avx512icl,
  rav1e_ipred_z1_8bpc_ssse3,
  rav1e_ipred_z1_8bpc_avx2,
  rav1e_ipred_z3_8bpc_ssse3,
  rav1e_ipred_z3_8bpc_avx2,
  rav1e_ipred_paeth_8bpc_ssse3,
  rav1e_ipred_paeth_8bpc_avx2,
  rav1e_ipred_paeth_8bpc_avx512icl
}

macro_rules! decl_angular_ipred_hbd_fn {
  ($($f:ident),+) => {
    extern {
      $(
        pub fn $f(
          dst: *mut u16, stride: libc::ptrdiff_t, topleft: *const u16,
          width: libc::c_int, height: libc::c_int, angle: libc::c_int,
          max_width: libc::c_int, max_height: libc::c_int,
          bit_depth_max: libc::c_int,
        );
      )*
    }
  };
}

decl_angular_ipred_hbd_fn! {
  rav1e_ipred_h_16bpc_ssse3,
  rav1e_ipred_h_16bpc_avx2,
  rav1e_ipred_v_16bpc_ssse3,
  rav1e_ipred_v_16bpc_avx2,
  rav1e_ipred_dc_16bpc_ssse3,
  rav1e_ipred_dc_16bpc_avx2,
  rav1e_ipred_dc_left_16bpc_ssse3,
  rav1e_ipred_dc_left_16bpc_avx2,
  rav1e_ipred_dc_128_16bpc_ssse3,
  rav1e_ipred_dc_128_16bpc_avx2,
  rav1e_ipred_dc_top_16bpc_ssse3,
  rav1e_ipred_dc_top_16bpc_avx2,
  rav1e_ipred_smooth_v_16bpc_ssse3,
  rav1e_ipred_smooth_v_16bpc_avx2,
  rav1e_ipred_smooth_v_16bpc_avx512icl,
  rav1e_ipred_smooth_h_16bpc_ssse3,
  rav1e_ipred_smooth_h_16bpc_avx2,
  rav1e_ipred_smooth_h_16bpc_avx512icl,
  rav1e_ipred_smooth_16bpc_ssse3,
  rav1e_ipred_smooth_16bpc_avx2,
  rav1e_ipred_smooth_16bpc_avx512icl,
  rav1e_ipred_z1_16bpc_ssse3,
  rav1e_ipred_z1_16bpc_avx2,
  rav1e_ipred_z2_16bpc_ssse3,
  rav1e_ipred_z3_16bpc_ssse3,
  rav1e_ipred_z3_16bpc_avx2,
  rav1e_ipred_paeth_16bpc_ssse3,
  rav1e_ipred_paeth_16bpc_avx2,
  rav1e_ipred_paeth_16bpc_avx512icl
}

// For z2 prediction, we need to provide extra parameters, dx and dy, which indicate
// the distance between the predicted block's top-left pixel and the frame's edge.
// It is required for the intra edge filtering process.
extern {
  pub fn rav1e_ipred_z2_8bpc_ssse3(
    dst: *mut u8, stride: libc::ptrdiff_t, topleft: *const u8,
    width: libc::c_int, height: libc::c_int, angle: libc::c_int,
    dx: libc::c_int, dy: libc::c_int,
  );

  pub fn rav1e_ipred_z2_8bpc_avx2(
    dst: *mut u8, stride: libc::ptrdiff_t, topleft: *const u8,
    width: libc::c_int, height: libc::c_int, angle: libc::c_int,
    dx: libc::c_int, dy: libc::c_int,
  );

  pub fn rav1e_ipred_z2_16bpc_avx2(
    dst: *mut u16, stride: libc::ptrdiff_t, topleft: *const u16,
    width: libc::c_int, height: libc::c_int, angle: libc::c_int,
    dx: libc::c_int, dy: libc::c_int, bit_depth_max: libc::c_int,
  );
}

macro_rules! decl_cfl_ac_fn {
  ($($f:ident),+) => {
    extern {
      $(
        pub fn $f(
          ac: *mut MaybeUninit<i16>, src: *const u8, stride: libc::ptrdiff_t,
          w_pad: libc::c_int, h_pad: libc::c_int,
          width: libc::c_int, height: libc::c_int,
        );
      )*
    }
  };
}

decl_cfl_ac_fn! {
  rav1e_ipred_cfl_ac_420_8bpc_avx2,
  rav1e_ipred_cfl_ac_420_8bpc_ssse3,
  rav1e_ipred_cfl_ac_422_8bpc_avx2,
  rav1e_ipred_cfl_ac_422_8bpc_ssse3,
  rav1e_ipred_cfl_ac_444_8bpc_avx2,
  rav1e_ipred_cfl_ac_444_8bpc_ssse3
}

macro_rules! decl_cfl_ac_hbd_fn {
  ($($f:ident),+) => {
    extern {
      $(
        pub fn $f(
          ac: *mut MaybeUninit<i16>, src: *const u16, stride: libc::ptrdiff_t,
          w_pad: libc::c_int, h_pad: libc::c_int,
          width: libc::c_int, height: libc::c_int,
        );
      )*
    }
  };
}

decl_cfl_ac_hbd_fn! {
  rav1e_ipred_cfl_ac_420_16bpc_ssse3,
  rav1e_ipred_cfl_ac_420_16bpc_avx2,
  rav1e_ipred_cfl_ac_422_16bpc_ssse3,
  rav1e_ipred_cfl_ac_422_16bpc_avx2,
  rav1e_ipred_cfl_ac_444_16bpc_ssse3,
  rav1e_ipred_cfl_ac_444_16bpc_avx2
}

macro_rules! decl_cfl_pred_fn {
  ($($f:ident),+) => {
    extern {
      $(
        pub fn $f(
          dst: *mut u8, stride: libc::ptrdiff_t, topleft: *const u8,
          width: libc::c_int, height: libc::c_int, ac: *const i16,
          alpha: libc::c_int,
        );
      )*
    }
  };
}

decl_cfl_pred_fn! {
  rav1e_ipred_cfl_8bpc_ssse3,
  rav1e_ipred_cfl_8bpc_avx2,
  rav1e_ipred_cfl_left_8bpc_ssse3,
  rav1e_ipred_cfl_left_8bpc_avx2,
  rav1e_ipred_cfl_top_8bpc_ssse3,
  rav1e_ipred_cfl_top_8bpc_avx2,
  rav1e_ipred_cfl_128_8bpc_ssse3,
  rav1e_ipred_cfl_128_8bpc_avx2
}

macro_rules! decl_cfl_pred_hbd_fn {
  ($($f:ident),+) => {
    extern {
      $(
        pub fn $f(
          dst: *mut u16, stride: libc::ptrdiff_t, topleft: *const u16,
          width: libc::c_int, height: libc::c_int, ac: *const i16,
          alpha: libc::c_int, bit_depth_max: libc::c_int,
        );
      )*
    }
  };
}

decl_cfl_pred_hbd_fn! {
  rav1e_ipred_cfl_16bpc_ssse3,
  rav1e_ipred_cfl_16bpc_avx2,
  rav1e_ipred_cfl_128_16bpc_ssse3,
  rav1e_ipred_cfl_128_16bpc_avx2,
  rav1e_ipred_cfl_left_16bpc_ssse3,
  rav1e_ipred_cfl_left_16bpc_avx2,
  rav1e_ipred_cfl_top_16bpc_ssse3,
  rav1e_ipred_cfl_top_16bpc_avx2
}