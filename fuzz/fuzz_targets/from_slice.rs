#![no_main]

use libfuzzer_sys::fuzz_target;
use serde_json_1400::{from_slice, Value};

fuzz_target!(|data: &[u8]| {
    _ = from_slice::<Value>(data);
});
