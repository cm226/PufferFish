use std::{ffi::OsStr, io::{Error, ErrorKind, Write}, process::Command};

pub fn compile(file: &str, out: &str) -> Result<String, Error> {
    run_command("./target/debug/pufferfish", ["-o", out], file)
}

pub fn run_command<A, S>(cmd : &str, args : A, piped : &str) -> Result<String, Error> 
where A : IntoIterator<Item = S>,
      S: AsRef<OsStr>,
{ 
  let mut child = Command::new(cmd)
  .args(args)
  .stdin(std::process::Stdio::piped())
  .stdout(std::process::Stdio::piped())
  .spawn()
  .expect("Failed to run compiler");

  let mut stdin = child.stdin.take().expect("Failed to open stdin");
  stdin.write_all(piped.as_bytes())?;
  drop(stdin);
  
  child.wait_with_output()
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

pub fn run_test(src: &str) -> Result<TestOutput, Error> { 
  
  let out_file = format!("./tests/test_out/{}",src);

  compile(src, &out_file)?;
  let cmd_out = run_command(&out_file, [""], "")?;
  Ok(TestOutput::new(cmd_out))
}

pub struct TestOutput {
  cur_val: Box<dyn Iterator<Item = String>>
}

impl TestOutput {

    pub fn new(raw_out: String) -> Self {
        let cal = raw_out.split('\n').map(|f|f.to_owned()).collect::<Vec<_>>();
        TestOutput {
            cur_val : Box::new(cal.into_iter())
        }
    }

    pub fn assert_next(&mut self, expected: &str) {
      assert_eq!(self.cur_val.next().unwrap(), expected)
    }
}
