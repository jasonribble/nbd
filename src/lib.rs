// `clippy.toml` relaxes unwrap/expect/panic in tests, but `panic_in_result_fn`
// (from the denied `nursery` group) has no such toggle. Allow it for test
// builds only — production `Result` fns stay forbidden from panicking.
#![cfg_attr(test, allow(clippy::panic_in_result_fn))]

pub mod db;
pub mod models;

pub mod utils;

#[cfg(test)]
pub mod test_helpers;
