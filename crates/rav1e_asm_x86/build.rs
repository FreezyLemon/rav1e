// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#![allow(clippy::print_literal)]
#![allow(clippy::unused_io_amount)]

use std::env;
use std::path::Path;

const ASM_FILES: &[&str] = &[
  "asm/cdef_avx2.asm",
  "asm/cdef_avx512.asm",
  "asm/cdef_dist.asm",
  "asm/cdef_rav1e.asm",
  "asm/cdef_sse.asm",
  "asm/cdef16_avx2.asm",
  "asm/cdef16_avx512.asm",
  "asm/cdef16_sse.asm",
  "asm/ipred_avx2.asm",
  "asm/ipred_avx512.asm",
  "asm/ipred_sse.asm",
  "asm/ipred16_avx2.asm",
  "asm/ipred16_avx512.asm",
  "asm/ipred16_sse.asm",
  "asm/itx_avx2.asm",
  "asm/itx_avx512.asm",
  "asm/itx_sse.asm",
  "asm/itx16_avx2.asm",
  "asm/itx16_avx512.asm",
  "asm/itx16_sse.asm",
  "asm/looprestoration_avx2.asm",
  "asm/looprestoration_avx512.asm",
  "asm/looprestoration_sse.asm",
  "asm/looprestoration16_avx2.asm",
  "asm/looprestoration16_avx512.asm",
  "asm/looprestoration16_sse.asm",
  "asm/mc_avx2.asm",
  "asm/mc_avx512.asm",
  "asm/mc_sse.asm",
  "asm/mc16_avx2.asm",
  "asm/mc16_avx512.asm",
  "asm/mc16_sse.asm",
  "asm/me.asm",
  "asm/sad_avx.asm",
  "asm/sad_plane.asm",
  "asm/sad_sse2.asm",
  "asm/satd.asm",
  "asm/satd16_avx2.asm",
  "asm/sse.asm",
  "asm/tables.asm",
];

fn build_nasm_files() {
  let mut config = "
%pragma preproc sane_empty_expansion true
%define private_prefix rav1e
%define ARCH_X86_32 0
%define ARCH_X86_64 1
%define PIC 1
%define STACK_ALIGNMENT 16
%define HAVE_AVX512ICL 1
"
  .to_owned();

  if env::var("CARGO_CFG_TARGET_VENDOR").unwrap() == "apple" {
    config += "%define PREFIX 1\n";
  }

  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("config.asm");
  std::fs::write(&dest_path, config).expect("can write config.asm");

  let obj = nasm_rs::Build::new()
    .min_version(2, 15, 0)
    .include(&out_dir)
    .include("ext")
    .files(ASM_FILES)
    .compile_objects()
    .unwrap_or_else(|e| {
      panic!("NASM build failed. Make sure you have nasm installed or disable the \"asm\" feature.\n\
              You can get NASM from https://nasm.us or your system's package manager.\n\
              \n\
              error: {e}");
    });

  // cc is better at finding the correct archiver
  let mut cc = cc::Build::new();
  for o in obj {
    cc.object(o);
  }
  cc.compile("rav1easm");

  // Strip local symbols from the asm library since they
  // confuse the debugger.
  if let Some(strip) = strip_command() {
    let _ = std::process::Command::new(strip)
      .arg("-x")
      .arg(Path::new(&out_dir).join("librav1easm.a"))
      .status();
  }
}

fn strip_command() -> Option<String> {
  let target = env::var("TARGET").expect("TARGET");
  // follows Cargo's naming convention for the linker setting
  let normalized_target = target.replace('-', "_").to_uppercase();
  let explicit_strip =
    env::var(format!("CARGO_TARGET_{normalized_target}_STRIP"))
      .ok()
      .or_else(|| env::var("STRIP").ok());
  if explicit_strip.is_some() {
    return explicit_strip;
  }

  // strip command is target-specific, e.g. macOS's strip breaks MUSL's archives
  let host = env::var("HOST").expect("HOST");
  if host != target {
    return None;
  }

  Some("strip".into())
}

fn main() {
  build_nasm_files();

  println!("cargo:rustc-env=PROFILE={}", env::var("PROFILE").unwrap());
  if let Ok(value) = env::var("CARGO_CFG_TARGET_FEATURE") {
    println!("cargo:rustc-env=CARGO_CFG_TARGET_FEATURE={value}");
  }
  println!(
    "cargo:rustc-env=CARGO_ENCODED_RUSTFLAGS={}",
    env::var("CARGO_ENCODED_RUSTFLAGS").unwrap()
  );
}
