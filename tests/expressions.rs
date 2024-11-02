use std::io::Error;

mod util;

#[test]
fn expression_test() -> Result<(), Error> {
  let expected = "31.00\n200.00\n19.00\n";
  util::run_test("expressions", expected)?;
  Ok(())
}