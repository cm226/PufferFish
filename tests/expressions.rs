use std::io::Error;

mod util;

#[test]
fn expression_test() -> Result<(), Error> {
  let expected = "3\01\0\n\02\00\00\0\n\01\09\0\n\0";
  util::run_test("expressions", expected)?;
  Ok(())
}