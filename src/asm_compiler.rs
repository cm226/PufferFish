use std::process::Command;

pub fn compile_asm(asm : &str) { 
  std::fs::write("output.asm", asm).expect("Failed to write tmp asm file");

  // Compile
  let _ = Command::new("nasm")
      .args(["-f", "elf", "output.asm"])
      .output()
      .expect("failed to compile program asm!!!");

  // link
  let _ = Command::new("ld")
  .args(["-m", "elf_i386", "-s", "-o", "output", "output.o"])
  .output()
  .expect("failed to link program");
}