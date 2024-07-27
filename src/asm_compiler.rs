use std::{ffi::OsStr, io::{Error, ErrorKind}, process::Command};

pub fn compile_asm(asm : &str, generate_debug_info : bool) -> Result<(), Error>{ 
  std::fs::write("output.asm", asm).expect("Failed to write tmp asm file");

  let mut compile_args = vec!["-f", "elf",  "output.asm"];
  let link_args = vec!["-m", "elf_i386", "-o", "output", "output.o"];

  if generate_debug_info { 
    compile_args.extend(vec!["-g", "-F", "dwarf"]);
  }

  // Compile with debug symbols
  run_command("nasm",compile_args)?;
  // link include debug symbols
  run_command("ld",link_args)?;
  
  Ok(())
}

fn run_command<A, S>(cmd : &str, args : A) -> Result<(), Error> 
where A : IntoIterator<Item = S>,
      S: AsRef<OsStr>,
{ 
  Command::new(cmd)
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

  return Ok(());

}