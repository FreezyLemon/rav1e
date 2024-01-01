use std::env;
use std::path::Path;

const ASM_FILES: &[&str] = &[
  "asm/64/cdef.S",
  "asm/64/cdef16.S",
  "asm/64/cdef_dist.S",
  "asm/64/mc.S",
  "asm/64/mc16.S",
  "asm/64/itx.S",
  "asm/64/itx16.S",
  "asm/64/ipred.S",
  "asm/64/ipred16.S",
  "asm/64/sad.S",
  "asm/64/satd.S",
  "asm/64/sse.S",
  "asm/tables.S",
];

fn main() {
  let mut config = "
#define PRIVATE_PREFIX rav1e_
#define ARCH_AARCH64 1
#define ARCH_ARM 0
#define CONFIG_LOG 1
#define HAVE_ASM 1
"
  .to_owned();

  if env::var("CARGO_CFG_TARGET_VENDOR").unwrap() == "apple" {
    config += "#define PREFIX 1\n";
  }
  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("config.h");
  std::fs::write(&dest_path, config).expect("can write config.h");

  cc::Build::new()
    .files(ASM_FILES)
    .include(".")
    .include(&out_dir)
    .compile("rav1e-aarch64");
}
