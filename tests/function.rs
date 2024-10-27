use std::io::Error;

mod util;

#[test]
fn function_test() -> Result<(), Error>{
    let expected = "1.000000\n20.000000\n";
    util::run_test("functions", expected)?;
    Ok(())
}