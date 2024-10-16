use std::{ffi::OsStr, io::{Error, ErrorKind}, process::Command};

pub fn compile_asm(asm : &str, generate_debug_info : bool, output: &String) -> Result<(), Error>{ 
  std::fs::write("output.asm", asm).expect("Failed to write tmp asm file");

  // use GCC to find the location of some packages
  let crtend_s = run_command("gcc",["--print-file-name=crtendS.o"])?;
  let scrt1 = run_command("gcc",["--print-file-name=Scrt1.o"])?;

  let mut compile_args = vec!["-f", "elf64",  "output.asm"];
  let link_args = vec![
    "-m", "elf_x86_64",
    "-o", output,
    "-l","SDL2",
    "-l","c",
    "-dynamic-linker","/lib64/ld-linux-x86-64.so.2", // Use the 64bit loader
    "output.o", "graphics_lib.o", 
    scrt1.trim(), crtend_s.trim() // Implementation of _start
  ];  

  if generate_debug_info {
    println!("Debugging enabled");
    compile_args.extend(vec!["-g", "-F", "dwarf"]);
  }
  // Compile with debug symbols
  run_command("nasm",compile_args)?;
  // link include debug symbols
  run_command("ld",link_args)?;
  
  Ok(())
}

fn run_command<A, S>(cmd : &str, args : A) -> Result<String, Error> 
where A : IntoIterator<Item = S>,
      S: AsRef<OsStr>,
{ 
  let output = Command::new(cmd)
  .args(args)
  .output()
  .and_then(|f| {
    if !f.status.success() {
      return Err(
        Error::new(
          ErrorKind::Other, 
          String::from(String::from_utf8_lossy(&f.stderr).to_string().trim()))
      )
    }
    return Ok(f);
  })?;

  return Ok(String::from_utf8( output.stdout).expect("Failed to read STDOUT"));

}