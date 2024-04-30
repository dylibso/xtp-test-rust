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
    let example_input = "example";
    xtp_test::group("checks example function and timing", || {
        let example: String = xtp_test::call("example", example_input)?;
        xtp_test::assert_ne("example not null", &example, "");
        xtp_test::assert_eq("example output", example, "Hello, world!");
        xtp_test::assert!(
            "example runtime",
            xtp_test::time_sec("example", vec![])? < 0.5
        );
        xtp_test::assert!(
            "runs in some time",
            xtp_test::time_ns("example", "example")? < 10000
        );
        Ok(())
    })?;

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
