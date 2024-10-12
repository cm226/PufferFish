use std::process::Command;
use std::env;

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    let lib_file = "src/Graphics/graphics_lib.c";
    println!("cargo::rerun-if-changed={}",lib_file);
    
    // I dont think this is the right place for this, for now it will do TODO
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    Command::new("gcc").args([
      "-m64", "-Wall", 
      "-I", "/usr/include/SDL2",
      "-c",
      lib_file,
      "-o"])
      .arg(&format!("{}/graphics_lib.o", out_dir))
      .status().unwrap();     
  }