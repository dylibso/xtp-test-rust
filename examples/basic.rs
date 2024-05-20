use extism_pdk::*;
use xtp_test;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Count {
    count: usize,
    total: usize,
    vowels: String,
}

#[plugin_fn]
pub fn test() -> FnResult<()> {
    // test mock_input, which enables the test harness to inject data
    let Json(output): Json<Count> =
        xtp_test::call("count_vowels", xtp_test::mock_input::<Vec<u8>>()?)?;
    xtp_test::assert_ne!("count_vowels of mock_input isn't empty", output.count, 0);

    xtp_test::assert_gt("gt test", 100, 1);
    xtp_test::assert_lt("lt test", std::f64::MIN, std::f64::MAX);
    xtp_test::assert_lte("gte test", 'Z', 'a');
    xtp_test::assert_lte("lte test", 1 / 10, 1 / 10);

    // call a function from some Extism plugin (you'll link these up in the CLI command to run the test),
    // passing in some data and getting back a string (`callString` is a helper for string output)
    let Json(res): Json<Count> = xtp_test::call("count_vowels", "some input")?;
    // assert the count of the vowels is correct, giving the test case a name (which will be shown in the CLI output)
    // using the macro version here will also capture filename and line number
    xtp_test::assert_eq!("count_vowels of 'some input'", res.count, 4);

    let time_ns = xtp_test::time_ns("count_vowels", "o".repeat(1024 * 10))?;
    xtp_test::assert_lt!(
        "timing count_vowels nanos (10KB input)",
        time_ns,
        5e7 as u64
    );

    // create a group of tests, which will be run together and reset after the group is complete
    xtp_test::group("count_vowels maintains state", || {
        let mut accum_total = 0;
        let expected_final_total = 12;
        for i in 0..3 {
            let Json(res): Json<Count> = xtp_test::call("count_vowels", "this is a test")?;
            accum_total += res.count;
            xtp_test::assert_eq("total count increased", accum_total, 4 * (i + 1));
        }

        xtp_test::assert_eq(
            "expected total at and of test",
            accum_total,
            expected_final_total,
        );
        Ok(())
    })?;

    // internal check that `reset` is done after the group above has executed
    let Json(reset_output): Json<Count> = xtp_test::call("count_vowels", "this is a test")?;
    xtp_test::assert_eq("reset plugin has vars cleared", reset_output.total, 4);

    Ok(())
}
