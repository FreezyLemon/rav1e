macro_rules! decl_sad_plane_fn {
  ($($f:ident),+) => {
    extern {
      $(
        pub fn $f(
          src: *const u8, dst: *const u8, stride: libc::size_t,
          width: libc::size_t, rows: libc::size_t
        ) -> u64;
      )*
    }
  };
}

decl_sad_plane_fn!(rav1e_sad_plane_8bpc_sse2, rav1e_sad_plane_8bpc_avx2);
