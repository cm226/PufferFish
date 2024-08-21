use std::io::Error;

mod util;

#[test]
fn function_test() -> Result<(), Error>{
    let expected = "2\00\00\0\n\01\00\00\0\n\09\0\n\0";
    util::run_test("fn_stack", expected)?;
    Ok(())
}