# Fuzzing Guide

## Overview

Vult uses cargo-fuzz with libFuzzer for security testing. Two fuzz targets test critical vult functionality:

1. **fuzz_pin_validation** - PIN validation with arbitrary byte sequences
2. **fuzz_crypto** - Encryption/decryption roundtrips with arbitrary data

## Setup

### Install Nightly Rust

Fuzzing requires nightly Rust for sanitizer instrumentation:

```bash
rustup toolchain install nightly
```

### Install cargo-fuzz

```bash
cargo install cargo-fuzz
```

## Running Fuzz Tests

### Quick Test (10 seconds)

```bash
cd /path/to/vult
cargo +nightly fuzz run fuzz_pin_validation -- -max_total_time=10
```

### Extended Test (24 hours)

For thorough security testing, run for extended periods:

```bash
cargo +nightly fuzz run fuzz_pin_validation -- -max_total_time=86400
```

### Run All Targets

```bash
# PIN validation
cargo +nightly fuzz run fuzz_pin_validation -- -max_total_time=3600

# Crypto operations
cargo +nightly fuzz run fuzz_crypto -- -max_total_time=3600
```

## Understanding Results

### Successful Run

```
Done 8119409 runs in 11 second(s)
```

No crashes found - this is good!

### Crash Found

If a crash is found:

1. Crash artifacts saved to `fuzz/artifacts/<target>/`
2. Minimized test case savedto corpus
3. Review crash with: `cargo +nightly fuzz run <target> fuzz/artifacts/<target>/<crash_file>`

## Adding Crashes to Test Suite

When a crash is found:

1. Review the crash artifact
2. Add regression test to `tests/` directory
3. Fix the underlying issue
4. Re-run fuzzer to verify fix

Example:

```rust
#[test]
fn test_fuzzer_crash_001() {
    // Crash artifact: [0xFF, 0xFF, ...]
    let crash_input = vec![0xFF, 0xFF];
    // Should not panic
    let result = validate_pin(&crash_input);
    assert!(result.is_err());
}
```

## Corpus

Fuzzer builds a corpus of interesting inputs in:

```
fuzz/corpus/<target>/
```

This corpus is reused across runs for faster coverage. Commit interesting corpus entries to the repository for regression testing.

## CI Integration

To integrate fuzzing into CI (short runs on each PR):

```yaml
# .github/workflows/fuzz.yml
name: Fuzz Testing

on: [pull_request]

jobs:
  fuzz:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
          override: true
      - run: cargo install cargo-fuzz
      - name: Fuzz PIN validation
        run: cargo +nightly fuzz run fuzz_pin_validation -- -max_total_time=60
      - name: Fuzz crypto
        run: cargo +nightly fuzz run fuzz_crypto -- -max_total_time=60
```

## Performance

Expected throughput (depends on hardware):

- `fuzz_pin_validation`: ~738k exec/s (simple input validation)
- `fuzz_crypto`: ~120k exec/s (complex crypto operations)

## Coverage

Fuzz targets achieve:

- **PIN validation**: 40 basic blocks, 75 features
- **Crypto**: 361 basic blocks, 621 features

## Best Practices

1. **Run locally before CI**: Catch issues early with local fuzzing
2. **24h+ runs monthly**: Schedule deep fuzzing sessions
3. **Commit interesting corpus**: Build regression test suite
4. **Test with ASAN**: Address Sanitizer catches memory errors
5. **Monitor coverage**: Track coverage growth over time

## Troubleshooting

### "error: the option `Z` is only accepted on the nightly compiler"

Run with nightly: `cargo +nightly fuzz ...`

### "Out of memory"

Reduce max_len or workers:

```bash
cargo +nightly fuzz run target -- -max_len=512 -workers=2
```

### Build errors

Clean and rebuild:

```bash
cargo +nightly fuzz clean
cargo +nightly fuzz build
```

## Security Reporting

If fuzzing discovers a security issue:

1. **Do not** publicize the issue
2. Review [SECURITY.md](../SECURITY.md) for reporting guidelines
3. Create a minimal test case
4. Report privately to maintainers

## References

- [cargo-fuzz documentation](https://rust-fuzz.github.io/book/cargo-fuzz.html)
- [libFuzzer options](https://llvm.org/docs/LibFuzzer.html#options)
- [Rust Fuzz Book](https://rust-fuzz.github.io/book/)
