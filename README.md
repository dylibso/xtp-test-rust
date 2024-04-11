# xtp-test

A Rust test framework for [xtp](https://getxtp.com) /
[Extism](https://extism.org) plugins.

## Example

```rust
use extism_pdk::*;
use xtp_test;

// You _must_ export a single `test` function for the runner to execute.
#[plugin_fn]
pub fn test() -> FnResult<()> {
    let example_input = "example";

    // call a function from the Extism plugin being tested
    let example = xtp_test::call_string("example", example_input)?;
    // assert various things about the behavior and performance of the function call
    xtp_test::assert_ne("example not null", &example, "");
    xtp_test::assert_eq("example output", example, "Hello, world!");
    xtp_test::assert_eq(
        "example runtime",
        xtp_test::time_sec("example", vec![])?.floor(),
        0.,
    );

    // create a group of tests, which will all share an instance of a plugin. 
    // after the group completes, the plugin instance is reset, so its state 
    // doesn't live outside of the group.
    xtp_test::group("checks example function and timing", || {
        let example = xtp_test::call_string("example", example_input)?;
        xtp_test::assert_ne("example not null", &example, "");
        xtp_test::assert_eq("example output", example, "Hello, world!");
        // check that the function runs in under a 0.5s 
        xtp_test::assert(
            "example run time check",
            xtp_test::time_sec("example", vec![])? < 0.5,
        );
        // check that the function runs within 10000 ns. 
        xtp_test::assert(
            "runs fast",
            xtp_test::time_ns("example", "example")? < 10000,
        );
        Ok(())
    })?;

    // create any number of groups, and they will be displayed together 
    // in the `xtp` CLI output after the tests are run.
    xtp_test::group("small basic group", || {
        let example = xtp_test::call_string("example", example_input)?;
        xtp_test::assert_ne("example not null", &example, "");
        Ok(())
    })?;

    Ok(())
}
```

## API Docs

Please see the [**`docs.rs`**](https://docs.rs/xtp-test) documentation page for
full details.

## Usage

**1. Create a Rust project using the XTP Test crate**

```sh
cargo new --lib rust-xtp-test
cd rust-xtp-test
# ensure you have `crate-type = ["cdylib"]` in your `[lib]` section of Cargo.toml
cargo add xtp-test extism-pdk
```

**2. Write your test in Rust**

```rust
use extism_pdk::*;
use xtp_test;

// You _must_ export a single `test` function for the runner to execute.
#[plugin_fn]
pub fn test() -> FnResult<()> {
    // call a function from the Extism plugin being tested
    let example = xtp_test::call_string("example", example_input)?;
    // assert various things about the behavior and performance of the function call
    xtp_test::assert_ne("example not null", &example, "");
    // ...
    Ok(())
}
```

**3. Compile your test to .wasm:**

Ensure you have the `wasm32-unknown-unknown` and/or `wasm32-wasi` targets
installed via `rustup`, and run:

```sh
cargo build --target wasm32-unknown-unknown --release
```

**4. Run the test against your plugin:** Once you have your test code as a
`.wasm` module, you can run the test against your plugin using the `xtp` CLI:

### Install `xtp`

```sh
curl https://static.dylibso.com/cli/install.sh | sudo sh
```

### Run the test suite

```sh
xtp plugin test ./plugin-*.wasm --with test.wasm --host host.wasm
#               ^^^^^^^^^^^^^^^        ^^^^^^^^^        ^^^^^^^^^
#               your plugin(s)         test to run      optional mock host functions
```

**Note:** The optional mock host functions must be implemented as Extism
plugins, whose exported functions match the host function signature imported by
the plugins being tested.

## Need Help?

Please reach out via the
[`#xtp` channel on Discord](https://discord.com/channels/1011124058408112148/1220464672784908358).
