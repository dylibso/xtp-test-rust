use anyhow::Result;
use extism_pdk::{Memory, ToMemory};

mod harness {
    #[link(wasm_import_module = "xtp:test/harness")]
    extern "C" {
        pub fn call(name: u64, input: u64) -> u64;
        pub fn time(name: u64, input: u64) -> u64;
        pub fn assert(name: u64, value: u64);
        pub fn reset();
        pub fn group(name: u64);
    }
}

/// Call a function from the Extism plugin being tested, passing input and returning its output Memory.
pub fn call_memory(func_name: impl AsRef<str>, input: impl ToMemory) -> Result<Memory> {
    let func_name = func_name.as_ref();
    let func_mem = Memory::from_bytes(func_name)?;
    let input_mem = input.to_memory()?;
    let output_ptr = unsafe { harness::call(func_mem.offset(), input_mem.offset()) };
    func_mem.free();
    input_mem.free();

    let output = match Memory::find(output_ptr) {
        None => anyhow::bail!("Error in call to {func_name}: invalid output offset"),
        Some(x) => x,
    };
    Ok(output)
}

/// Call a function from the Extism plugin being tested, passing input and returning its output.
pub fn call<T: extism_pdk::FromBytesOwned>(
    func_name: impl AsRef<str>,
    input: impl ToMemory,
) -> Result<T> {
    let output_mem = call_memory(func_name, input)?;
    let output = output_mem.to();
    output_mem.free();
    output
}

/// Call a function from the Extism plugin being tested, passing input and returning the time in nanoseconds spent in the fuction.
pub fn time_ns(func_name: impl AsRef<str>, input: impl ToMemory) -> Result<u64> {
    let func_name = func_name.as_ref();
    let func_mem = Memory::from_bytes(func_name)?;
    let input_mem = input.to_memory()?;
    let ns = unsafe { harness::time(func_mem.offset(), input_mem.offset()) };
    func_mem.free();
    input_mem.free();

    Ok(ns)
}

/// Call a function from the Extism plugin being tested, passing input and returning the time in seconds spent in the fuction.
pub fn time_sec(func_name: impl AsRef<str>, input: impl ToMemory) -> Result<f64> {
    time_ns(func_name, input).map(|x| x as f64 / 1e9)
}

/// Assert that the `outcome` is true, naming the assertion with `msg`, which will be used as a label in the CLI runner.
pub fn assert(msg: impl AsRef<str>, outcome: bool) {
    let msg_mem = Memory::from_bytes(msg.as_ref()).expect("assert message Extism memory");
    unsafe {
        harness::assert(msg_mem.offset(), outcome as u64);
    }
    msg_mem.free();
}

/// Assert that `x` and `y` are equal, naming the assertion with `msg`, which will be used as a label in the CLI runner.
pub fn assert_eq<U, T: PartialEq<U>>(msg: impl AsRef<str>, x: T, y: U) {
    assert(msg, x == y);
}

/// Assert that `x` and `y` are not equal, naming the assertion with `msg`, which will be used as a label in the CLI runner.
pub fn assert_ne<U, T: PartialEq<U>>(msg: impl AsRef<str>, x: T, y: U) {
    assert(msg, x != y);
}

// Create a new test group. NOTE: these cannot be nested and starting a new group will end the last one.
fn start_group(name: impl AsRef<str>) {
    let name_mem = Memory::from_bytes(name.as_ref()).expect("assert message Extism memory");
    unsafe {
        harness::group(name_mem.offset());
    }
    name_mem.free();
}

/// Reset the loaded plugin, clearing all state.
pub fn reset() {
    unsafe {
        harness::reset();
    }
}

/// Run a test group, resetting the plugin before and after the group is run.
/// ```rust
/// use extism_pdk::*;
///
/// #[plugin_fn]
/// pub fn test() -> FnResult<()> {
///   xtp_test::group("group name", || {
///       xtp_test::assert("test name", true);
///   })?;
///   Ok(())
/// }
/// ```
pub fn group(name: impl AsRef<str>, f: impl FnOnce() -> Result<()>) -> Result<()> {
    reset();
    start_group(name);
    let res = f();
    reset();
    res
}
