// Copyright (c) 2022-2023, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::activity::apply_ssim_boost;
use crate::cpu_features::CpuFeatureLevel;
use crate::dist::*;
use crate::tiling::PlaneRegion;
use crate::util::Pixel;
use crate::util::PixelType;

use rav1e_asm_x86::dist::cdef_dist::*;

/// # Panics
///
/// - If in `check_asm` mode, panics on mismatch between native and ASM results.
#[allow(clippy::let_and_return)]
pub fn cdef_dist_kernel<T: Pixel>(
  src: &PlaneRegion<'_, T>, dst: &PlaneRegion<'_, T>, w: usize, h: usize,
  bit_depth: usize, cpu: CpuFeatureLevel,
) -> u32 {
  debug_assert!(src.plane_cfg.xdec == 0);
  debug_assert!(src.plane_cfg.ydec == 0);
  debug_assert!(dst.plane_cfg.xdec == 0);
  debug_assert!(dst.plane_cfg.ydec == 0);

  // Limit kernel to 8x8
  debug_assert!(w <= 8);
  debug_assert!(h <= 8);

  let call_rust =
    || -> u32 { rust::cdef_dist_kernel(dst, src, w, h, bit_depth, cpu) };
  #[cfg(feature = "check_asm")]
  let ref_dist = call_rust();

  let (svar, dvar, sse) = match T::type_enum() {
    PixelType::U8 => {
      if let Some(func) =
        CDEF_DIST_KERNEL_FNS[cpu.as_index()][kernel_fn_index(w, h)]
      {
        let mut ret_buf = [0u32; 3];
        // SAFETY: Calls Assembly code.
        unsafe {
          func(
            src.data_ptr() as *const _,
            T::to_asm_stride(src.plane_cfg.stride),
            dst.data_ptr() as *const _,
            T::to_asm_stride(dst.plane_cfg.stride),
            ret_buf.as_mut_ptr(),
          )
        }

        (ret_buf[0], ret_buf[1], ret_buf[2])
      } else {
        return call_rust();
      }
    }
    PixelType::U16 => {
      if let Some(func) =
        CDEF_DIST_KERNEL_HBD_FNS[cpu.as_index()][kernel_fn_index(w, h)]
      {
        // SAFETY: Calls Assembly code.
        unsafe {
          func(
            src.data_ptr() as *const _,
            T::to_asm_stride(src.plane_cfg.stride),
            dst.data_ptr() as *const _,
            T::to_asm_stride(dst.plane_cfg.stride),
          )
        }
      } else {
        return call_rust();
      }
    }
  };

  let dist = apply_ssim_boost(sse, svar, dvar, bit_depth);
  #[cfg(feature = "check_asm")]
  assert_eq!(
    dist, ref_dist,
    "CDEF Distortion {}x{}: Assembly doesn't match reference code.",
    w, h
  );

  dist
}

cpu_function_lookup_table!(
  CDEF_DIST_KERNEL_FNS:
    [[Option<CdefDistKernelFn>; CDEF_DIST_KERNEL_FNS_LENGTH]],
  default: [None; CDEF_DIST_KERNEL_FNS_LENGTH],
  [SSE2]
);

cpu_function_lookup_table!(
  CDEF_DIST_KERNEL_HBD_FNS:
    [[Option<CdefDistKernelHBDFn>; CDEF_DIST_KERNEL_FNS_LENGTH]],
  default: [None; CDEF_DIST_KERNEL_FNS_LENGTH],
  [AVX2]
);
