#![no_main]

use libfuzzer_sys::fuzz_target;
use vult::core::validate_pin;

fuzz_target!(|data: &[u8]| {
    // Fuzz PIN validation with arbitrary byte sequences
    if let Ok(pin_str) = std::str::from_utf8(data) {
        // Try validating the PIN - should never panic
        let _ = validate_pin(pin_str);
    }
});
