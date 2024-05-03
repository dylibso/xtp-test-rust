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
    // call a function from some Extism plugin (you'll link these up in the CLI command to run the test),
    // passing in some data and getting back a string (`callString` is a helper for string output)
    let Json(res): Json<Count> = xtp_test::call("count_vowels", "some input")?;
    // assert the count of the vowels is correct, giving the test case a name (which will be shown in the CLI output)
    xtp_test::assert_eq("count_vowels of 'some input'", res.count, 4);

    let time_ns = xtp_test::time_ns("count_vowels", "o".repeat(1024 * 10))?;
    const TARGET_NS: u64 = 6e5 as u64;
    xtp_test::assert(
        "timing count_vowels nanos (10KB input)",
       time_ns < TARGET_NS,
        format!("{} > {}", time_ns, TARGET_NS),
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
