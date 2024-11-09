use std::io::Error;

mod util;

#[test]
fn function_test() -> Result<(), Error>{
    let input = "
        fn test(f){
            print(f);
        };

        test(1.0);
        test(20.0);";

    let mut result = util::run_test(input)?;
    result.assert_next("1.00");
    result.assert_next("20.00");
    Ok(())
}

#[test]
fn function_stack_test() -> Result<(), Error>{
    let input = "    
        var n = 9.0;

        fn test(f){
            var n = 100.0;
            var m = 200.0;
            print(m);
            print(n);
        };

        test(1.0);
        print(n);
    ";
    let mut result = util::run_test(input)?;
    result.assert_next("200.00");
    result.assert_next("100.00");
    result.assert_next("9.00");
    Ok(())
}


#[test]
fn function_multi_arg() -> Result<(), Error>{
    let input = "
        fn test(f1, f2) { 
            print(f1);
            print(f2);
        };
        test(10.0, 20.0);
    ";
    let mut result = util::run_test(input)?;
    result.assert_next("10.00");
    result.assert_next("20.00");
    Ok(())
}

#[test]
fn extrnal_fn_calling() -> Result<(), Error> {
    let mut result = util::run_test("print(sin(0.0));")?;
    result.assert_next("0.00");
    Ok(())
}