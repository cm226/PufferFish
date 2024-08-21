use std::{ffi::OsStr, io::{Error, ErrorKind}, process::Command};


pub fn compile(file: &str, out: &str) -> Result<String, Error> {
    run_command("./target/debug/pufferfish", [file, "-o", out])
}

pub fn run_command<A, S>(cmd : &str, args : A) -> Result<String, Error> 
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
    return Ok(String::from(String::from_utf8(f.stdout).unwrap()));
  })
}

pub fn run_test(src: &str, expected: &str) -> Result<(), Error> {
  let in_file = format!("tests/data/{}.puff", src);
  let out_file = format!("tests/test_out/{}",src);

  compile(&in_file, &out_file)?;
  let std_out = run_command(&out_file, [""])?;

  assert_eq!(std_out, expected);
  Ok(())
}