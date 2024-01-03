// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::cpu_features::CpuFeatureLevel;
use crate::tiling::PlaneRegionMut;
use crate::transform::inverse::*;
use crate::transform::*;
use crate::{Pixel, PixelType};

use crate::asm::shared::transform::inverse::{InvTxfmFunc, InvTxfmHBDFunc, *};

use rav1e_asm_x86::transform::inverse::*;

pub fn inverse_transform_add<T: Pixel>(
  input: &[T::Coeff], output: &mut PlaneRegionMut<'_, T>, eob: u16,
  tx_size: TxSize, tx_type: TxType, bd: usize, cpu: CpuFeatureLevel,
) {
  match T::type_enum() {
    PixelType::U8 => {
      if let Some(func) = INV_TXFM_FNS[cpu.as_index()][tx_size][tx_type] {
        return call_inverse_func(
          func,
          input,
          output,
          eob,
          tx_size.width(),
          tx_size.height(),
          bd,
        );
      }
    }
    PixelType::U16 if bd == 10 => {
      if let Some(func) = INV_TXFM_HBD_FNS_10[cpu.as_index()][tx_size][tx_type]
      {
        return call_inverse_hbd_func(
          func,
          input,
          output,
          eob,
          tx_size.width(),
          tx_size.height(),
          bd,
        );
      }
    }
    PixelType::U16 => {
      if let Some(func) = INV_TXFM_HBD_FNS_12[cpu.as_index()][tx_size][tx_type]
      {
        return call_inverse_hbd_func(
          func,
          input,
          output,
          eob,
          tx_size.width(),
          tx_size.height(),
          bd,
        );
      }
    }
  };

  rust::inverse_transform_add(input, output, eob, tx_size, tx_type, bd, cpu);
}

cpu_function_lookup_table!(
  INV_TXFM_FNS: [[[Option<InvTxfmFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL]],
  default: [[None; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL],
  [SSE2, SSSE3, AVX2, AVX512ICL]
);

cpu_function_lookup_table!(
  INV_TXFM_HBD_FNS_10: [[[Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL]],
  default: [[None; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL],
  [SSE2, SSE4_1, AVX2, AVX512ICL]
);

cpu_function_lookup_table!(
  INV_TXFM_HBD_FNS_12: [[[Option<InvTxfmHBDFunc>; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL]],
  default: [[None; TX_TYPES_PLUS_LL]; TxSize::TX_SIZES_ALL],
  [SSE2, AVX2]
);
