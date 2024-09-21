use std::process::Command;


// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=src/Graphics/graphics_lib");
    
    let out_dir = ".";

    Command::new("gcc").args([
      "-m64", "-Wall", 
      "-I", "/usr/include/SDL2",
      "-c",
      "src/Graphics/graphics_lib.c",
      "-o"])
      .arg(&format!("{}/graphics_lib.o", out_dir))
      .status().unwrap();     
}