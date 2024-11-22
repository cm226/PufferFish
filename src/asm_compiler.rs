use std::{ffi::OsStr, io::{Error, ErrorKind}, path::PathBuf, process::Command};
use mktemp::Temp;

fn get_asm_filename(is_debug : bool) -> Result<PathBuf, Error>{ 

  if is_debug {
    let mut temp_asm = PathBuf::new();
    temp_asm.set_file_name("output.asm");
    return Ok(temp_asm);
  }

  let mut temp_asm = Temp::new_file()?.to_path_buf();
  temp_asm.set_extension("asm");

  Ok(temp_asm)
}

pub fn compile_asm(asm : &str, generate_debug_info : bool, output: &String) -> Result<(), Error>{ 
  
  let temp_asm = get_asm_filename(generate_debug_info)?;

  let mut temp_obj_file = Temp::new_file()?.to_path_buf();
  temp_obj_file.set_extension("o");

  std::fs::write(&temp_asm, asm).expect("Failed to write tmp asm file");

  // create the output folder if needed
  let mut parent_path = std::path::PathBuf::from(output);
  parent_path.pop();
  std::fs::create_dir_all(parent_path)?;

  // use GCC to find the location of some packages
  let crtend_s = run_command("gcc",["--print-file-name=crtendS.o"])?;
  let scrt1 = run_command("gcc",["--print-file-name=Scrt1.o"])?;

  
  let mut compile_args = vec![
      "-o", temp_obj_file.to_str().unwrap(),
      "-f", "elf64",
      temp_asm.to_str().unwrap()];

  let link_args = vec![
    "-m", "elf_x86_64",
    "-o", output,
    "-l","SDL2",
    "-l","SDL2_image",
    "-l","c",
    "-l","m",
    "-dynamic-linker","/lib64/ld-linux-x86-64.so.2", // Use the 64bit loader
    temp_obj_file.to_str().unwrap(),
    "graphics_lib.o", 
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