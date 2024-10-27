use std::io::Error;

mod util;

#[test]
fn vars_test() -> Result<(), Error>{
    let expected = "111.000000\n222.000000\n";
    util::run_test("vars", expected)?;

    Ok(())
}
