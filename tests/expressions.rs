use std::io::Error;

mod util;

#[test]
fn expression_test() -> Result<(), Error> {
  let expected = "31.000000\n200.000000\n19.000000\n";
  util::run_test("expressions", expected)?;
  Ok(())
}