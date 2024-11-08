use std::io::Error;

mod util;

#[test]
fn function_test() -> Result<(), Error>{
    let expected = "1.00\n20.00\n";
    util::run_test("functions", expected)?;
    Ok(())
}

#[test]
fn function_stack_test() -> Result<(), Error>{
    let expected = "200.00\n100.00\n9.00\n";
    util::run_test("fn_stack", expected)?;
    Ok(())
}


#[test]
fn function_multi_arg() -> Result<(), Error>{
    let expected = "10.00\n20.00\n";
    util::run_test("fn_multi_arg", expected)?;
    Ok(())
}