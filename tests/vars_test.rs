use std::io::Error;

mod util;

#[test]
fn vars_test() -> Result<(), Error>{
    // Why the null chars?
    // Comes from the print impl, can only push 16 bit onto the stack
    // To remove the nulls need to improve print impl so that it computes high and low 
    // bits for 16 bit register before pushing onto the stack. TODO
    let expected = "1\01\01\0\n\02\02\02\0\n\0";
    util::run_test("vars", expected)?;

    Ok(())
}
