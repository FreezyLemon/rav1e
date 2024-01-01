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

#[allow(unused_imports)]
use std::env;

#[allow(unused_variables)]
fn main() {
  built::write_built_file().expect("Failed to acquire build-time information");

  let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
  let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
  // let env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

  #[cfg(feature = "asm")]
  {
    if arch == "x86_64" {
      println!("cargo:rustc-cfg={}", "nasm_x86_64");
    }
    if arch == "aarch64" {
      println!("cargo:rustc-cfg={}", "asm_neon");
    }
  }

  if os == "windows" && cfg!(feature = "decode_test") {
    panic!("Unsupported feature on this platform!");
  }

  println!("cargo:rustc-env=PROFILE={}", env::var("PROFILE").unwrap());
  if let Ok(value) = env::var("CARGO_CFG_TARGET_FEATURE") {
    println!("cargo:rustc-env=CARGO_CFG_TARGET_FEATURE={value}");
  }
  println!(
    "cargo:rustc-env=CARGO_ENCODED_RUSTFLAGS={}",
    env::var("CARGO_ENCODED_RUSTFLAGS").unwrap()
  );
}
