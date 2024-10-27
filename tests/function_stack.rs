use std::io::Error;

mod util;

#[test]
fn function_test() -> Result<(), Error>{
    let expected = "200.000000\n100.000000\n9.000000\n";
    util::run_test("fn_stack", expected)?;
    Ok(())
}