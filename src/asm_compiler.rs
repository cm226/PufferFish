use std::{ffi::OsStr, io::{Error, ErrorKind}, process::Command};

pub fn compile_asm(asm : &str) -> Result<(), Error>{ 
  std::fs::write("output.asm", asm).expect("Failed to write tmp asm file");

  // Compile
  run_command("nasm",["-f", "elf", "output.asm"] )?;
  // link
  run_command("ld",["-m", "elf_i386", "-s", "-o", "output", "output.o"])?;
  
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