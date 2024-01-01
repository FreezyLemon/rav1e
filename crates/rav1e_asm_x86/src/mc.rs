macro_rules! decl_mc_fns {
  ($($func_name:ident),+) => {
    paste::item! {
      extern {
        $(
          pub fn [<$func_name _ssse3>](
            dst: *mut u8, dst_stride: isize, src: *const u8, src_stride: isize,
            w: i32, h: i32, mx: i32, my: i32
          );

          pub fn [<$func_name _avx2>](
            dst: *mut u8, dst_stride: isize, src: *const u8, src_stride: isize,
            w: i32, h: i32, mx: i32, my: i32
          );

          pub fn [<$func_name _avx512icl>](
            dst: *mut u8, dst_stride: isize, src: *const u8, src_stride: isize,
            w: i32, h: i32, mx: i32, my: i32
          );
        )*
      }
    }
  }
}

decl_mc_fns!(
  rav1e_put_8tap_regular_8bpc,
  rav1e_put_8tap_regular_smooth_8bpc,
  rav1e_put_8tap_regular_sharp_8bpc,
  rav1e_put_8tap_smooth_regular_8bpc,
  rav1e_put_8tap_smooth_8bpc,
  rav1e_put_8tap_smooth_sharp_8bpc,
  rav1e_put_8tap_sharp_regular_8bpc,
  rav1e_put_8tap_sharp_smooth_8bpc,
  rav1e_put_8tap_sharp_8bpc,
  rav1e_put_bilin_8bpc
);

macro_rules! decl_mc_hbd_fns {
  ($($func_name:ident),+) => {
    paste::item! {
      extern {
        $(
          pub fn [<$func_name _ssse3>](
            dst: *mut u16, dst_stride: isize, src: *const u16, src_stride: isize,
            w: i32, h: i32, mx: i32, my: i32, bitdepth_max: i32,
          );

          pub fn [<$func_name _avx2>](
            dst: *mut u16, dst_stride: isize, src: *const u16, src_stride: isize,
            w: i32, h: i32, mx: i32, my: i32, bitdepth_max: i32,
          );
        )*
      }
    }
  }
}

decl_mc_hbd_fns!(
  rav1e_put_8tap_regular_16bpc,
  rav1e_put_8tap_regular_smooth_16bpc,
  rav1e_put_8tap_regular_sharp_16bpc,
  rav1e_put_8tap_smooth_regular_16bpc,
  rav1e_put_8tap_smooth_16bpc,
  rav1e_put_8tap_smooth_sharp_16bpc,
  rav1e_put_8tap_sharp_regular_16bpc,
  rav1e_put_8tap_sharp_smooth_16bpc,
  rav1e_put_8tap_sharp_16bpc,
  rav1e_put_bilin_16bpc
);

macro_rules! decl_mct_fns {
  ($($func_name:ident),+) => {
    paste::item! {
      extern {
        $(
          pub fn [<$func_name _sse2>](
            tmp: *mut i16, src: *const u8, src_stride: libc::ptrdiff_t, w: i32,
            h: i32, mx: i32, my: i32
          );

          pub fn [<$func_name _ssse3>](
            tmp: *mut i16, src: *const u8, src_stride: libc::ptrdiff_t, w: i32,
            h: i32, mx: i32, my: i32
          );

          pub fn [<$func_name _avx2>](
            tmp: *mut i16, src: *const u8, src_stride: libc::ptrdiff_t, w: i32,
            h: i32, mx: i32, my: i32
          );

          pub fn [<$func_name _avx512icl>](
            tmp: *mut i16, src: *const u8, src_stride: libc::ptrdiff_t, w: i32,
            h: i32, mx: i32, my: i32
          );
        )*
      }
    }
  }
}

decl_mct_fns!(
  rav1e_prep_8tap_regular_8bpc,
  rav1e_prep_8tap_regular_smooth_8bpc,
  rav1e_prep_8tap_regular_sharp_8bpc,
  rav1e_prep_8tap_smooth_regular_8bpc,
  rav1e_prep_8tap_smooth_8bpc,
  rav1e_prep_8tap_smooth_sharp_8bpc,
  rav1e_prep_8tap_sharp_regular_8bpc,
  rav1e_prep_8tap_sharp_smooth_8bpc,
  rav1e_prep_8tap_sharp_8bpc,
  rav1e_prep_bilin_8bpc
);

macro_rules! decl_mct_hbd_fns {
  ($($func_name:ident),+) => {
    paste::item! {
      extern {
        $(
          pub fn [<$func_name _ssse3>](
            tmp: *mut i16, src: *const u16, src_stride: libc::ptrdiff_t, w: i32,
            h: i32, mx: i32, my: i32, bitdepth_max: i32,
          );

          pub fn [<$func_name _avx2>](
            tmp: *mut i16, src: *const u16, src_stride: libc::ptrdiff_t, w: i32,
            h: i32, mx: i32, my: i32, bitdepth_max: i32,
          );
        )*
      }
    }
  }
}

decl_mct_hbd_fns!(
  rav1e_prep_8tap_regular_16bpc,
  rav1e_prep_8tap_regular_smooth_16bpc,
  rav1e_prep_8tap_regular_sharp_16bpc,
  rav1e_prep_8tap_smooth_regular_16bpc,
  rav1e_prep_8tap_smooth_16bpc,
  rav1e_prep_8tap_smooth_sharp_16bpc,
  rav1e_prep_8tap_sharp_regular_16bpc,
  rav1e_prep_8tap_sharp_smooth_16bpc,
  rav1e_prep_8tap_sharp_16bpc,
  rav1e_prep_bilin_16bpc
);

extern {
  pub fn rav1e_avg_8bpc_ssse3(
    dst: *mut u8, dst_stride: libc::ptrdiff_t, tmp1: *const i16,
    tmp2: *const i16, w: i32, h: i32,
  );

  pub fn rav1e_avg_8bpc_avx2(
    dst: *mut u8, dst_stride: libc::ptrdiff_t, tmp1: *const i16,
    tmp2: *const i16, w: i32, h: i32,
  );

  pub fn rav1e_avg_8bpc_avx512icl(
    dst: *mut u8, dst_stride: libc::ptrdiff_t, tmp1: *const i16,
    tmp2: *const i16, w: i32, h: i32,
  );

  pub fn rav1e_avg_16bpc_ssse3(
    dst: *mut u16, dst_stride: libc::ptrdiff_t, tmp1: *const i16,
    tmp2: *const i16, w: i32, h: i32, bitdepth_max: i32,
  );

  pub fn rav1e_avg_16bpc_avx2(
    dst: *mut u16, dst_stride: libc::ptrdiff_t, tmp1: *const i16,
    tmp2: *const i16, w: i32, h: i32, bitdepth_max: i32,
  );
}
