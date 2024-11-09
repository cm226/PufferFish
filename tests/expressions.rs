use std::io::Error;

mod util;

#[test]
fn expression_test() -> Result<(), Error> {
  let input = "
    var i = 10.0;
    var j = 20.0;
    print(i + j + 1.0);
    print(i*j);
    print(j-1.0);
    print(1.0/2.0);
  ";
 
  let mut result = util::run_test(input)?;
  result.assert_next( "31.00");
  result.assert_next("200.00");
  result.assert_next("19.00");
  result.assert_next("0.50");
  Ok(())
}

// TODO, I want this to be possible, currently not supported
//#[test]
//fn fn_return_expressions() -> Result<(), Error> {
  //let mut result = util::run_test("
  //print(sin(0.0) + 1.0);
  //")?;
  //result.assert_next("1.00");
  //Ok(())
//}