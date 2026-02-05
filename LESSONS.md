# Lessons Learned - Vult Development

This file documents important bugs, mistakes, and lessons learned during development to prevent similar issues in the future.

## 2026-02-04: Integer Overflow Bug in Auto-Lock Feature

### The Bug
Auto-lock functionality was completely broken due to an integer overflow bug in the activity counter.

**Location**: `src/gui/auth_manager.rs`, `start_activity_counter()` method

**Problematic Code**:
```rust
if s.is_unlocked && s.last_activity_secs < u64::MAX as i64 {
    s.last_activity_secs += 1;
}
```

**The Issue**:
- `u64::MAX` is `18446744073709551615`
- When cast to `i64`, it overflows and wraps to `-1`
- The condition `0 < -1` is always `false`
- Counter never incremented, auto-lock never triggered

**The Fix**:
```rust
if s.is_unlocked && s.last_activity_secs < i64::MAX {
    s.last_activity_secs += 1;
}
```

### Root Cause Analysis

1. **Type Confusion**: Mixed use of `i64` for counter but checked against `u64::MAX`
2. **Insufficient Testing**: No integration tests for auto-lock timing
3. **Silent Failure**: Condition silently failed without error or warning
4. **Missing Runtime Validation**: No assertions or debug checks during development

### Lessons Learned

1. **Always use matching types**: If a variable is `i64`, compare against `i64::MAX`, not `u64::MAX`
2. **Test time-based features**: Features involving timers and counters need specific tests with accelerated time
3. **Add debug logging during development**: Would have caught this immediately
4. **Use type-safe constants**: Consider using const generics or associated constants with proper types
5. **Integer casting is dangerous**: Be extremely careful with `as` casts, especially with MAX/MIN values

### Prevention Strategies

1. ✅ Add comprehensive unit tests for counter increment logic
2. ✅ Add integration tests for auto-lock with short timeouts
3. ✅ Use clippy lints to catch suspicious casts
4. ✅ Document type choices and constraints in code comments
5. ✅ Add assertions in debug builds for critical invariants

### Related Code Patterns to Avoid

```rust
// ❌ BAD - Cross-type MAX comparisons
if value < u64::MAX as i64 { }  // Will overflow!
if value < u32::MAX as i16 { }  // Will overflow!

// ✅ GOOD - Use matching types
if value < i64::MAX { }
if value < i16::MAX { }

// ✅ GOOD - Use TryFrom for safe conversion
if let Ok(max) = i64::try_from(u64::MAX) {
    if value < max { }
}
```

### Test Coverage Added

- Unit test: `test_activity_counter_increment`
- Unit test: `test_auto_lock_timeout_exact`
- Integration test: `test_auto_lock_with_frontend_event`
- Integration test: `test_activity_reset_prevents_lock`

---

## Template for Future Entries

### [Date]: [Brief Title]

**The Bug**: One-line description

**Location**: File and function

**Problematic Code**: 
```rust
// bad code
```

**The Fix**:
```rust
// good code
```

**Root Cause**: Why it happened

**Lessons Learned**: What we learned

**Prevention**: How to prevent it

**Tests Added**: List of new tests

---
