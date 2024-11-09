use std::io::Error;

mod util;

#[test]
fn vars_test() -> Result<(), Error>{
    let input = "
        var n1 = 111.0;
        var n2 = 222.0;

        print(n1);
        print(n2);
    ";
    let mut result = util::run_test(input)?;
    result.assert_next("111.00");
    result.assert_next("222.00");

    Ok(())
}
