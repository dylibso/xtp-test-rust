# xtp-test

A Rust test framework for [xtp](https://getxtp.com) /
[Extism](https://extism.org) plugins.

## Example

```rust
use extism_pdk::*;
use xtp_test;

// You _must_ export a single `test` function for the runner to execute.
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
    // using the macro version here will also capture filename and line number
    xtp_test::assert_eq!("count_vowels of 'some input'", res.count, 4);

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
    let example = xtp_test::call("example", example_input)?;
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
xtp plugin test ./plugin-*.wasm --with test.wasm --mock-host host.wasm
#               ^^^^^^^^^^^^^^^        ^^^^^^^^^             ^^^^^^^^^
#               your plugin(s)         test to run           optional mock host functions
```

**Note:** The optional mock host functions must be implemented as Extism
plugins, whose exported functions match the host function signature imported by
the plugins being tested.

## Need Help?

Please reach out via the
[`#xtp` channel on Discord](https://discord.com/channels/1011124058408112148/1220464672784908358).
