use rav1e_tx::{TxSize, TxType, TX_TYPES_PLUS_LL};

// FIXME: Create a asm_shared crate for things like this?
// Note: Input coeffs are mutable since the assembly uses them as a scratchpad
pub type InvTxfmFunc =
  unsafe extern fn(*mut u8, libc::ptrdiff_t, *mut i16, i32);

pub type InvTxfmHBDFunc =
  unsafe extern fn(*mut u16, libc::ptrdiff_t, *mut i16, i32, i32);

const fn merge_hbd_fns(
  a: [[Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL],
  b: [[Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL],
) -> [[Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL] {
  let mut out = b;
  let mut tx_size = 0;
  loop {
    let mut tx_type = 0;
    loop {
      if a[tx_size][tx_type].is_some() {
        out[tx_size][tx_type] = a[tx_size][tx_type];
      }
      tx_type += 1;
      if tx_type == TX_TYPES_PLUS_LL {
        break;
      }
    }
    tx_size += 1;
    if tx_size == TxSize::TX_SIZES_ALL {
      break;
    }
  }
  out
}

pub const INV_TXFM_HBD_FNS_10_SSE2: [[Option<InvTxfmHBDFunc>;
  TX_TYPES_PLUS_LL];
  TxSize::TX_SIZES_ALL] = INV_TXFM_HBD_FNS_16_SSE2;

pub const INV_TXFM_HBD_FNS_12_SSE2: [[Option<InvTxfmHBDFunc>;
  TX_TYPES_PLUS_LL];
  TxSize::TX_SIZES_ALL] = INV_TXFM_HBD_FNS_16_SSE2;

pub const INV_TXFM_HBD_FNS_10_AVX2: [[Option<InvTxfmHBDFunc>;
  TX_TYPES_PLUS_LL];
  TxSize::TX_SIZES_ALL] =
  merge_hbd_fns(INV_TXFM_HBD_FNS_10__AVX2, INV_TXFM_HBD_FNS_16_AVX2);

pub const INV_TXFM_HBD_FNS_12_AVX2: [[Option<InvTxfmHBDFunc>;
  TX_TYPES_PLUS_LL];
  TxSize::TX_SIZES_ALL] =
  merge_hbd_fns(INV_TXFM_HBD_FNS_12__AVX2, INV_TXFM_HBD_FNS_16_AVX2);

macro_rules! decl_itx_fns {
  // Takes a 2d list of tx types for W and H
  ([$([$(($ENUM:expr, $TYPE1:ident, $TYPE2:ident)),*]),*], $W:expr, $H:expr,
   $OPT_LOWER:ident, $OPT_UPPER:ident) => {
    paste::item! {
      // For each tx type, declare an function for the current WxH
      $(
        $(
          extern {
            // Note: type1 and type2 are flipped
            pub fn [<rav1e_inv_txfm_add_ $TYPE2 _$TYPE1 _$W x $H _8bpc_$OPT_LOWER>](
              dst: *mut u8, dst_stride: libc::ptrdiff_t, coeff: *mut i16,
              eob: i32
            );
          }
        )*
      )*

      // Create a lookup table for the tx types declared above
      pub const [<INV_TXFM_FNS_$W _$H _$OPT_UPPER>]: [Option<InvTxfmFunc>; TX_TYPES_PLUS_LL] = {
        let mut out: [Option<InvTxfmFunc>; TX_TYPES_PLUS_LL] = [None; TX_TYPES_PLUS_LL];
        $(
          $(
            out[$ENUM as usize] = Some([<rav1e_inv_txfm_add_$TYPE2 _$TYPE1 _$W x $H _8bpc_$OPT_LOWER>]);
          )*
        )*
        out
      };
    }
  };
}

macro_rules! create_wxh_tables {
  // Create a lookup table for each cpu feature
  ([$([$(($W:expr, $H:expr)),*]),*], $OPT_LOWER:ident, $OPT_UPPER:ident) => {
    paste::item! {
      pub const [<INV_TXFM_FNS_$OPT_UPPER>]: [[Option<InvTxfmFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL] = {
        let mut out: [[Option<InvTxfmFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL] =
          [[None; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL];
        // For each dimension, add an entry to the table
        $(
          $(
            out[TxSize::[<TX_ $W X $H>] as usize] = [<INV_TXFM_FNS_$W _$H _$OPT_UPPER>];
          )*
        )*
        out
      };
    }
  };

  // Loop through cpu features
  ($DIMS:tt, [$(($OPT_LOWER:ident, $OPT_UPPER:ident)),+]) => {
    $(
      create_wxh_tables!($DIMS, $OPT_LOWER, $OPT_UPPER);
    )*
  };
}

macro_rules! impl_itx_fns {
  ($TYPES:tt, $W:expr, $H:expr, [$(($OPT_LOWER:ident, $OPT_UPPER:ident)),+]) => {
    $(
      decl_itx_fns!($TYPES, $W, $H, $OPT_LOWER, $OPT_UPPER);
    )*
  };

  // Loop over a list of dimensions
  ($TYPES_VALID:tt, [$(($W:expr, $H:expr)),*], $OPT:tt) => {
    $(
      impl_itx_fns!($TYPES_VALID, $W, $H, $OPT);
    )*
  };

  ($TYPES64:tt, $DIMS64:tt, $TYPES32:tt, $DIMS32:tt, $TYPES16:tt, $DIMS16:tt,
   $TYPES84:tt, $DIMS84:tt, $TYPES4:tt, $DIMS4:tt, $OPT:tt) => {
    // Make 2d list of tx types for each set of dimensions. Each set of
    //   dimensions uses a superset of the previous set of tx types.
    impl_itx_fns!([$TYPES64], $DIMS64, $OPT);
    impl_itx_fns!([$TYPES64, $TYPES32], $DIMS32, $OPT);
    impl_itx_fns!([$TYPES64, $TYPES32, $TYPES16], $DIMS16, $OPT);
    impl_itx_fns!(
      [$TYPES64, $TYPES32, $TYPES16, $TYPES84], $DIMS84, $OPT
    );
    impl_itx_fns!(
      [$TYPES64, $TYPES32, $TYPES16, $TYPES84, $TYPES4], $DIMS4, $OPT
    );

    // Pool all of the dimensions together to create a table for each cpu
    // feature level.
    create_wxh_tables!(
      [$DIMS64, $DIMS32, $DIMS16, $DIMS84, $DIMS4], $OPT
    );
  };
}

impl_itx_fns!(
  // 64x
  [(TxType::DCT_DCT, dct, dct)],
  [(64, 64), (64, 32), (32, 64), (16, 64), (64, 16)],
  // 32x
  [(TxType::IDTX, identity, identity)],
  [(32, 32), (32, 16), (16, 32), (32, 8), (8, 32)],
  // 16x16
  [
    (TxType::DCT_ADST, dct, adst),
    (TxType::ADST_DCT, adst, dct),
    (TxType::DCT_FLIPADST, dct, flipadst),
    (TxType::FLIPADST_DCT, flipadst, dct),
    (TxType::V_DCT, dct, identity),
    (TxType::H_DCT, identity, dct),
    (TxType::ADST_ADST, adst, adst),
    (TxType::ADST_FLIPADST, adst, flipadst),
    (TxType::FLIPADST_ADST, flipadst, adst),
    (TxType::FLIPADST_FLIPADST, flipadst, flipadst)
  ],
  [(16, 16)],
  // 8x, 4x and 16x (minus 16x16 and 4x4)
  [
    (TxType::V_ADST, adst, identity),
    (TxType::H_ADST, identity, adst),
    (TxType::V_FLIPADST, flipadst, identity),
    (TxType::H_FLIPADST, identity, flipadst)
  ],
  [(16, 8), (8, 16), (16, 4), (4, 16), (8, 8), (8, 4), (4, 8)],
  // 4x4
  [(TxType::WHT_WHT, wht, wht)],
  [(4, 4)],
  [(avx2, AVX2)]
);

impl_itx_fns!(
  // 64x
  [(TxType::DCT_DCT, dct, dct)],
  [(64, 64), (64, 32), (32, 64), (16, 64), (64, 16)],
  // 32x
  [(TxType::IDTX, identity, identity)],
  [(32, 32), (32, 16), (16, 32), (32, 8), (8, 32)],
  // 16x16
  [
    (TxType::DCT_ADST, dct, adst),
    (TxType::ADST_DCT, adst, dct),
    (TxType::DCT_FLIPADST, dct, flipadst),
    (TxType::FLIPADST_DCT, flipadst, dct),
    (TxType::V_DCT, dct, identity),
    (TxType::H_DCT, identity, dct),
    (TxType::ADST_ADST, adst, adst),
    (TxType::ADST_FLIPADST, adst, flipadst),
    (TxType::FLIPADST_ADST, flipadst, adst),
    (TxType::FLIPADST_FLIPADST, flipadst, flipadst)
  ],
  [(16, 16)],
  // 8x, 4x and 16x (minus 16x16)
  [
    (TxType::V_ADST, adst, identity),
    (TxType::H_ADST, identity, adst),
    (TxType::V_FLIPADST, flipadst, identity),
    (TxType::H_FLIPADST, identity, flipadst)
  ],
  [(16, 8), (8, 16), (16, 4), (4, 16), (8, 8), (8, 4), (4, 8), (4, 4)],
  // 4x4
  [],
  [],
  [(avx512icl, AVX512ICL), (ssse3, SSSE3)]
);

impl_itx_fns!(
  // 64x
  [],
  [],
  // 32x
  [],
  [],
  // 16x16
  [],
  [],
  // 8x, 4x and 16x (minus 16x16 and 4x4)
  [],
  [],
  // 4x4
  [(TxType::WHT_WHT, wht, wht)],
  [(4, 4)],
  [(sse2, SSE2)]
);

macro_rules! decl_itx_hbd_fns {
  // Takes a 2d list of tx types for W and H
  ([$([$(($ENUM:expr, $TYPE1:ident, $TYPE2:ident)),*]),*], $W:expr, $H:expr, $BPC:expr,
   $OPT_LOWER:ident, $OPT_UPPER:ident) => {
    paste::item! {
      // For each tx type, declare an function for the current WxH
      $(
        $(
          extern {
            // Note: type1 and type2 are flipped
            pub fn [<rav1e_inv_txfm_add_ $TYPE2 _$TYPE1 _$W x $H _ $BPC bpc_$OPT_LOWER>](
              dst: *mut u16, dst_stride: libc::ptrdiff_t, coeff: *mut i16,
              eob: i32, bitdepth_max: i32,
            );
          }
        )*
      )*

      // Create a lookup table for the tx types declared above
      pub const [<INV_TXFM_HBD_FNS_$W _$H _$BPC _$OPT_UPPER>]: [Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL] = {
        #[allow(unused_mut)]
        let mut out: [Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL] = [None; TX_TYPES_PLUS_LL];
        $(
          $(
            out[$ENUM as usize] = Some([<rav1e_inv_txfm_add_$TYPE2 _$TYPE1 _$W x $H _ $BPC bpc_$OPT_LOWER>]);
          )*
        )*
        out
      };
    }
  };
}

macro_rules! create_wxh_hbd_tables {
  // Create a lookup table for each cpu feature
  ([$([$(($W:expr, $H:expr)),*]),*], $EXT:ident, $BPC:expr, $OPT_LOWER:ident, $OPT_UPPER:ident) => {
    paste::item! {
      pub const [<INV_TXFM_HBD_FNS $EXT _$OPT_UPPER>]: [[Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL] = {
        let mut out: [[Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL] =
          [[None; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL];
        // For each dimension, add an entry to the table
        $(
          $(
            out[TxSize::[<TX_ $W X $H>] as usize] = [<INV_TXFM_HBD_FNS_$W _$H _$BPC _$OPT_UPPER>];
          )*
        )*
        out
      };
    }
  };

  // Loop through cpu features
  ($DIMS:tt, $EXT:ident, [$(($BPC:expr, $OPT_LOWER:ident, $OPT_UPPER:ident)),+]) => {
    $(
      create_wxh_hbd_tables!($DIMS, $EXT, $BPC, $OPT_LOWER, $OPT_UPPER);
    )*
  };
}

macro_rules! impl_itx_hbd_fns {

  ($TYPES:tt, $W:expr, $H:expr, [$(($BPC:expr, $OPT_LOWER:ident, $OPT_UPPER:ident)),+]) => {
    $(
      decl_itx_hbd_fns!($TYPES, $W, $H, $BPC, $OPT_LOWER, $OPT_UPPER);
    )*
  };

  // Loop over a list of dimensions
  ($TYPES_VALID:tt, [$(($W:expr, $H:expr)),*], $OPT:tt) => {
    $(
      impl_itx_hbd_fns!($TYPES_VALID, $W, $H, $OPT);
    )*
  };

  ($TYPES64:tt, $DIMS64:tt, $TYPES32:tt, $DIMS32:tt, $TYPES16:tt, $DIMS16:tt,
   $TYPES84:tt, $DIMS84:tt, $TYPES4:tt, $DIMS4:tt, $EXT:ident, $OPT:tt) => {
    // Make 2d list of tx types for each set of dimensions. Each set of
    //   dimensions uses a superset of the previous set of tx types.
    impl_itx_hbd_fns!([$TYPES64], $DIMS64, $OPT);
    impl_itx_hbd_fns!([$TYPES64, $TYPES32], $DIMS32, $OPT);
    impl_itx_hbd_fns!([$TYPES64, $TYPES32, $TYPES16], $DIMS16, $OPT);
    impl_itx_hbd_fns!(
      [$TYPES64, $TYPES32, $TYPES16, $TYPES84], $DIMS84, $OPT
    );
    impl_itx_hbd_fns!(
      [$TYPES64, $TYPES32, $TYPES16, $TYPES84, $TYPES4], $DIMS4, $OPT
    );

    // Pool all of the dimensions together to create a table for each cpu
    // feature level.
    create_wxh_hbd_tables!(
      [$DIMS64, $DIMS32, $DIMS16, $DIMS84, $DIMS4], $EXT, $OPT
    );
  };
}

impl_itx_hbd_fns!(
  // 64x
  [(TxType::DCT_DCT, dct, dct)],
  [(64, 64), (64, 32), (32, 64), (64, 16), (16, 64)],
  // 32x
  [(TxType::IDTX, identity, identity)],
  [(32, 32), (32, 16), (16, 32), (32, 8), (8, 32)],
  // 16x16
  [
    (TxType::DCT_ADST, dct, adst),
    (TxType::ADST_DCT, adst, dct),
    (TxType::DCT_FLIPADST, dct, flipadst),
    (TxType::FLIPADST_DCT, flipadst, dct),
    (TxType::V_DCT, dct, identity),
    (TxType::H_DCT, identity, dct),
    (TxType::ADST_ADST, adst, adst),
    (TxType::ADST_FLIPADST, adst, flipadst),
    (TxType::FLIPADST_ADST, flipadst, adst),
    (TxType::FLIPADST_FLIPADST, flipadst, flipadst)
  ],
  [(16, 16)],
  // 8x, 4x and 16x (minus 16x16)
  [
    (TxType::V_ADST, adst, identity),
    (TxType::H_ADST, identity, adst),
    (TxType::V_FLIPADST, flipadst, identity),
    (TxType::H_FLIPADST, identity, flipadst)
  ],
  [(16, 8), (8, 16), (8, 8)],
  // 4x4
  [],
  [],
  _10,
  [(10, avx512icl, AVX512ICL)]
);

impl_itx_hbd_fns!(
  // 64x
  [(TxType::DCT_DCT, dct, dct)],
  [(64, 64), (64, 32), (32, 64), (16, 64), (64, 16)],
  // 32x
  [(TxType::IDTX, identity, identity)],
  [(32, 32), (32, 16), (16, 32), (32, 8), (8, 32)],
  // 16x16
  [
    (TxType::DCT_ADST, dct, adst),
    (TxType::ADST_DCT, adst, dct),
    (TxType::DCT_FLIPADST, dct, flipadst),
    (TxType::FLIPADST_DCT, flipadst, dct),
    (TxType::V_DCT, dct, identity),
    (TxType::H_DCT, identity, dct),
    (TxType::ADST_ADST, adst, adst),
    (TxType::ADST_FLIPADST, adst, flipadst),
    (TxType::FLIPADST_ADST, flipadst, adst),
    (TxType::FLIPADST_FLIPADST, flipadst, flipadst)
  ],
  [(16, 16)],
  // 8x, 4x and 16x (minus 16x16)
  [
    (TxType::V_ADST, adst, identity),
    (TxType::H_ADST, identity, adst),
    (TxType::V_FLIPADST, flipadst, identity),
    (TxType::H_FLIPADST, identity, flipadst)
  ],
  [(16, 8), (8, 16), (16, 4), (4, 16), (8, 8), (8, 4), (4, 8), (4, 4)],
  // 4x4
  [],
  [],
  _10_,
  [(10, avx2, AVX2)]
);

impl_itx_hbd_fns!(
  // 64x
  [(TxType::DCT_DCT, dct, dct)],
  [(64, 64), (64, 32), (32, 64), (16, 64), (64, 16)],
  // 32x
  [(TxType::IDTX, identity, identity)],
  [(32, 32), (32, 16), (16, 32), (32, 8), (8, 32)],
  // 16x16
  [
    (TxType::DCT_ADST, dct, adst),
    (TxType::ADST_DCT, adst, dct),
    (TxType::DCT_FLIPADST, dct, flipadst),
    (TxType::FLIPADST_DCT, flipadst, dct),
    (TxType::V_DCT, dct, identity),
    (TxType::H_DCT, identity, dct),
    (TxType::ADST_ADST, adst, adst),
    (TxType::ADST_FLIPADST, adst, flipadst),
    (TxType::FLIPADST_ADST, flipadst, adst),
    (TxType::FLIPADST_FLIPADST, flipadst, flipadst)
  ],
  [(16, 16)],
  // 8x, 4x and 16x (minus 16x16)
  [
    (TxType::V_ADST, adst, identity),
    (TxType::H_ADST, identity, adst),
    (TxType::V_FLIPADST, flipadst, identity),
    (TxType::H_FLIPADST, identity, flipadst)
  ],
  [(16, 8), (8, 16), (16, 4), (4, 16), (8, 8), (8, 4), (4, 8), (4, 4)],
  // 4x4
  [],
  [],
  _10,
  [(16, sse4, SSE4_1)]
);

impl_itx_hbd_fns!(
  // 64x
  [],
  [],
  // 32x
  [],
  [],
  // 16x16
  [],
  [],
  // 8x, 4x and 16x (minus 16x16 and 4x4)
  [],
  [],
  // 4x4
  [(TxType::WHT_WHT, wht, wht)],
  [(4, 4)],
  _16,
  [(16, sse2, SSE2), (16, avx2, AVX2)]
);

impl_itx_hbd_fns!(
  // 32x (DCT and IDTX swapped due to incomplete DCT implementation)
  [(TxType::IDTX, identity, identity)],
  [(32, 32), (32, 16), (16, 32)],
  [(TxType::DCT_DCT, dct, dct)],
  [(32, 8), (8, 32)],
  // 16x16
  [
    (TxType::DCT_ADST, dct, adst),
    (TxType::ADST_DCT, adst, dct),
    (TxType::DCT_FLIPADST, dct, flipadst),
    (TxType::FLIPADST_DCT, flipadst, dct),
    (TxType::V_DCT, dct, identity),
    (TxType::H_DCT, identity, dct),
    (TxType::ADST_ADST, adst, adst),
    (TxType::ADST_FLIPADST, adst, flipadst),
    (TxType::FLIPADST_ADST, flipadst, adst),
    (TxType::FLIPADST_FLIPADST, flipadst, flipadst)
  ],
  [(16, 16)],
  // 8x, 4x and 16x (minus 16x16)
  [
    (TxType::V_ADST, adst, identity),
    (TxType::H_ADST, identity, adst),
    (TxType::V_FLIPADST, flipadst, identity),
    (TxType::H_FLIPADST, identity, flipadst)
  ],
  [(16, 8), (8, 16), (16, 4), (4, 16), (8, 8), (8, 4), (4, 8), (4, 4)],
  // 4x4
  [],
  [],
  _12_,
  [(12, avx2, AVX2)]
);
