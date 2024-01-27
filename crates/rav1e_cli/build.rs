use std::env;

fn main() {
  built::write_built_file().expect("Failed to acquire build-time information");

  // Forward env variables that are build.rs-exclusive to the "normal" compilation
  if let Ok(value) = env::var("CARGO_CFG_TARGET_FEATURE") {
    println!("cargo:rustc-env=CARGO_CFG_TARGET_FEATURE={value}");
  }
  println!(
    "cargo:rustc-env=CARGO_ENCODED_RUSTFLAGS={}",
    env::var("CARGO_ENCODED_RUSTFLAGS").unwrap()
  );
}
