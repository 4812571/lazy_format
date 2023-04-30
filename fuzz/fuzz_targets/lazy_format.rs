#![no_main]

use libfuzzer_sys::fuzz_target;
use lazy_format::lazy_format;

fuzz_target!(|data: &[u8]| {
    let string = String::from_utf8_lossy(data);
    let _ = lazy_format!("{}", string).to_string();
});