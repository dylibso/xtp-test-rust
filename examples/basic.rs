use extism_pdk::*;
use xtp_test;

#[plugin_fn]
pub fn test() -> FnResult<()> {
    let example_input = "example";
    xtp_test::group("checks example function and timing", || {
        let example = xtp_test::call_string("example", example_input)?;
        xtp_test::assert_ne("example not null", &example, "");
        xtp_test::assert_eq("example output", example, "Hello, world!");
        xtp_test::assert(
            "example runtime",
            xtp_test::time_sec("example", vec![])? < 0.5,
        );
        xtp_test::assert(
            "runs in some time",
            xtp_test::time_ns("example", "example")? < 1000000,
        );
        Ok(())
    })?;
    xtp_test::group("small basic group", || {
        let example = xtp_test::call_string("example", example_input)?;
        xtp_test::assert_ne("example not null", &example, "");

        Ok(())
    })?;
    let example = xtp_test::call_string("example", example_input)?;
    xtp_test::assert_ne("example not null", &example, "");
    xtp_test::assert_eq("example output", example, "Hello, world!");
    xtp_test::assert_eq(
        "example runtime",
        xtp_test::time_sec("example", vec![])?.floor(),
        0.,
    );
    Ok(())
}
