# Contributing to Vult

Thank you for your interest in contributing to Vult! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Node.js 18+ and npm (for UI development)
- System dependencies (see [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/cyberelf/vult.git
cd vult

# Install Tauri CLI
cargo install tauri-cli --version "^2.0.0"

# Install UI dependencies
cd ui-sveltekit
npm install
cd ..

# Run in development mode
cargo tauri dev
```

## Project Structure

```
vult/
├── src/                    # Rust backend
│   ├── lib.rs             # Library entry point
│   ├── main.rs            # GUI entry point  
│   ├── services/          # Service layer (VaultManager, AuthService, etc.)
│   ├── gui/               # GUI-specific code (commands, auth_manager)
│   ├── bin/               # Binary entry points
│   │   ├── vult.rs        # CLI binary
│   │   └── vult-gui.rs    # GUI binary
│   └── ...                # Core modules (crypto, database, etc.)
├── ui-sveltekit/          # Frontend (SvelteKit + Tailwind)
├── tests/                 # Integration tests
└── openspec/              # Spec-driven development artifacts
```

## Development Workflow

### Building

```bash
# Build library only
cargo build --lib

# Build CLI
cargo build --bin vult --features cli

# Build GUI
cargo build --bin vult-gui --features gui

# Build everything
cargo build --features "cli gui"

# Release builds
cargo build --release --features "cli gui"
```

### Testing

```bash
# Run all library tests
cargo test --lib

# Run specific test module
cargo test --lib database
cargo test --lib crypto

# Run integration tests
cargo test --test integration_test

# Run with output visible
cargo test -- --nocapture

# Run property-based tests (slower)
cargo test crypto::proptests
```

### Code Quality

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy --lib --features gui -- -D warnings

# Run security audit
cargo audit

# Run cargo-deny (if installed)
cargo deny check
```

## Code Conventions

### Rust Style

- Follow standard Rust formatting (`cargo fmt`)
- Use `Result<T>` type aliases for error handling
- Prefer `thiserror` for custom error types
- Use `zeroize` for sensitive data (PINs, keys)
- Add rustdoc comments to all public items

### Error Handling

```rust
// Use VaultError for all error types
use crate::VaultError;

fn my_function() -> Result<(), VaultError> {
    // Use ? operator for error propagation
    some_operation()?;
    Ok(())
}
```

### Security Guidelines

**Do:**
- ✅ Use per-key encryption
- ✅ Clear clipboard after timeout
- ✅ Validate all inputs
- ✅ Test encryption/decryption roundtrips
- ✅ Use `zeroize` for sensitive data

**Don't:**
- ❌ Log PINs or decrypted keys
- ❌ Store decrypted keys longer than needed
- ❌ Write sensitive data to disk unencrypted
- ❌ Skip validation for "convenience"

## Making Changes

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation changes
- `refactor/description` - Code refactoring

### Commit Messages

Follow conventional commits:

```
type(scope): description

feat(cli): add --stay-unlocked flag for session mode
fix(crypto): handle empty salt in key derivation
docs(readme): update installation instructions
refactor(services): extract common validation logic
```

### Pull Request Process

1. Create a feature branch from `main`
2. Make your changes with tests
3. Run `cargo fmt` and `cargo clippy`
4. Run `cargo test --lib`
5. Update documentation if needed
6. Submit a pull request

## Testing Guidelines

### Unit Tests

- Add tests for all new functions
- Test both success and error cases
- Use descriptive test names

```rust
#[test]
fn test_unlock_with_correct_pin_succeeds() {
    // Test implementation
}

#[test]
fn test_unlock_with_wrong_pin_returns_error() {
    // Test implementation
}
```

### Integration Tests

- Test complete workflows
- Use in-memory databases for speed
- Clean up test artifacts

### Property-Based Tests

For critical security functions, use `proptest`:

```rust
proptest! {
    #[test]
    fn prop_encrypt_decrypt_roundtrip(data: Vec<u8>) {
        // Property: decrypt(encrypt(data)) == data
    }
}
```

## Service Layer Pattern

Vult uses a service layer pattern for clean separation:

```
VaultManager (orchestrator)
├── AuthService (authentication)
├── KeyService (key CRUD)
├── CryptoService (encryption)
└── VaultDb (storage)
```

### Adding New Functionality

1. Add method to appropriate service
2. Update VaultManager if needed
3. Add adapter in GUI commands.rs
4. Add CLI command if applicable
5. Add tests

## Questions?

- Open an issue for questions or bugs
- Check existing issues before creating new ones
- See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for design details

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
