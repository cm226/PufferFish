use std::io::Error;

mod util;

#[test]
fn expression_test() -> Result<(), Error> {
  let expected = "3\01\0\n\0";
  util::run_test("expressions", expected)?;
  Ok(())
}