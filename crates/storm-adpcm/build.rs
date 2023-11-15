use bindgen::Builder;
use cmake::Config;
use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
  println!("cargo:rerun-if-changed=build.rs");

  // ===========================================================================
  // 1. Setup Env
  // ===========================================================================

  let root: &str = env!("CARGO_MANIFEST_DIR");
  let root: &Path = Path::new(root);

  let dest: String = env::var("OUT_DIR").unwrap();
  let dest: &Path = Path::new(&dest);

  // ===========================================================================
  // 2. Compile
  // ===========================================================================

  let output: PathBuf = Config::new("lib").build();
  let outdir: PathBuf = output.join("lib");

  // ===========================================================================
  // 3. Link
  // ===========================================================================

  println!("cargo:rustc-link-search=native={}", outdir.display());
  println!("cargo:rustc-link-lib=adpcm");

  #[cfg(target_os = "macos")]
  {
    println!("cargo:rustc-link-lib=c++");
  }

  #[cfg(target_os = "linux")]
  {
    println!("cargo:rustc-link-lib=stdc++");
  }

  // ===========================================================================
  // 4. Generate Bindings
  // ===========================================================================

  let header: PathBuf = root.join("wrapper.hpp");
  let header: &str = header.to_str().unwrap();

  let import: PathBuf = root.join("lib");
  let import: String = format!("-I{}", import.display());

  Builder::default()
    .allowlist_function("CompressADPCM")
    .allowlist_function("DecompressADPCM")
    .allowlist_function("DecompressADPCM_SC1B")
    .allowlist_var("INITIAL_ADPCM_STEP_INDEX")
    .allowlist_var("MAX_ADPCM_CHANNEL_COUNT")
    .clang_arg(import)
    .fit_macro_constants(true)
    .generate_comments(false)
    .generate_cstr(true)
    .header(header)
    .layout_tests(true)
    .merge_extern_blocks(true)
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .use_core()
    .generate()
    .unwrap()
    .write_to_file(dest.join("bindings.rs"))
    .unwrap();
}
