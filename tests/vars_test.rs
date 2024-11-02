use std::io::Error;

mod util;

#[test]
fn vars_test() -> Result<(), Error>{
    let expected = "111.00\n222.00\n";
    util::run_test("vars", expected)?;

    Ok(())
}
