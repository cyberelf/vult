# Vult

<div align="center">

**Secure API Key Vault**

A cross-platform desktop application for securely storing and managing API keys with client-side encryption.

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-1.80%2B-orange)
![Tauri](https://img.shields.io/badge/tauri-2.1%2B-blueviolet)

</div>

## Features

- 🔐 **Secure Storage**: AES-256-GCM encryption with Argon2id key derivation
- 🔑 **Per-Key Encryption**: Each API key encrypted with unique derived key
- 🔓 **Windows Hello**: Optional biometric unlock on Windows 10/11 (feature-gated)
- ⏱️ **Auto-Lock**: Automatically locks after 5 minutes of inactivity
- 📋 **Clipboard Integration**: Copy keys with auto-clear after 45 seconds
- 🔍 **Search**: Quickly find keys by app name, key name, or description
- 📊 **Table View**: Clean, table-based UI with inline editing
- 💾 **Database Migration**: Automatic schema migrations with backups
- 🖥️ **Cross-Platform**: Windows, macOS, and Linux support
- 📱 **Responsive UI**: Adapts smoothly to any window size from 320px to 4K+

## Responsive Design

Vult features a fully responsive, autoscaling UI that adapts to any window size.

### Viewport Support
- **Small (320px - 767px)**: Compact layout with stacked components
- **Medium (768px - 1023px)**: Tablet/compact desktop layout
- **Large (1024px+)**: Full desktop layout with expanded spacing

### Responsive Features
- **Fluid Typography**: Text scales smoothly using CSS `clamp()` - no discrete jumps
- **Autoscaling Components**: Buttons, forms, and modals adapt to viewport size
- **Responsive Tables**: Transforms to stacked card layout on small viewports
- **Touch-Friendly**: All interactive elements meet 44x44px touch target minimum
- **Keyboard Accessible**: Visible focus rings on all interactive elements

### Breakpoint Strategy
| Breakpoint | Width | Layout |
|------------|-------|--------|
| Small | 320px - 767px | Single column, stacked cards |
| Medium | 768px - 1023px | Two-column forms, optimized spacing |
| Large | 1024px+ | Multi-column, expanded spacing |

### Container Widths
- **Setup/Unlock Screens**: `clamp(320px, 80vw, 600px)`
- **Vault Screen**: `clamp(400px, 90vw, 1200px)`
- **Modals**: `min(90vw, 500px)` on small, `500px` fixed on large

## Security

### Encryption
- **PIN-based Authentication**: Master key derived from your PIN using Argon2id
  - Memory: 64 MiB
  - Iterations: 3
  - Parallelism: 4
  - Output: 256-bit key
- **Per-Key Encryption**: Each API key encrypted with a unique derived key
  - Key derivation from: master key + app name + key name + per-key salt
  - Compromise of one key doesn't affect others
- **Algorithm**: AES-256-GCM (authenticated encryption)
- **Secure Memory**: Uses `zeroize` to securely clear sensitive data

### Database
- **Location**: `~/.vult/vault.db`
- **Schema Versioning**: Tracks and migrates database schema automatically
- **Backup Protection**: Creates backups before migrations
- **Version Guard**: Blocks opening databases with newer schema versions

### Windows Hello (Optional)
- **Platform**: Windows 10 (version 1903+) or Windows 11
- **Requirements**: Compatible biometric device (fingerprint, face, iris)
- **Feature Flag**: Build with `--features windows-biometric` to enable
- **Privacy**: Biometric data never leaves Windows security subsystem
- **Fallback**: PIN authentication always available as backup
- **User Control**: Can be enabled/disabled in settings

## Installation

### Prerequisites
- Rust 1.80 or later
- Node.js 18+ and npm (for UI development)
- System dependencies:
  - **Windows**: WebView2 Runtime (usually pre-installed)
  - **macOS**: Xcode Command Line Tools
  - **Linux**: See [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/vult.git
cd vult

# Install dependencies
cargo install tauri-cli --version "^2.0.0"

# Run in development mode
cargo tauri dev

# Build for release
cargo tauri build

# Build with Windows Hello support (Windows only)
cargo tauri build --features windows-biometric
```

## Usage

Vult provides two interfaces: a GUI (desktop app) and a CLI (command-line).

### GUI (Desktop Application)

#### First Time Setup
1. Launch Vult
2. Create a PIN (minimum 6 characters)
3. **Important**: Remember your PIN - there is no recovery mechanism!

#### Adding API Keys
1. Click "+ Add Key"
2. Fill in the required fields:
   - **Key Name**: Required (e.g., "GitHub Personal Token")
   - **App Name**: Optional (e.g., "GitHub")
   - **API URL**: Optional (e.g., "https://api.github.com")
   - **Description**: Optional
   - **API Key Value**: Required
3. Click "Save"

#### Managing Keys
- **View**: Click the eye icon to show/hide the key value
- **Copy**: Click the copy icon to copy the key to clipboard
- **Edit**: Click the edit icon to modify key details
- **Delete**: Click the delete icon to remove a key
- **Search**: Use the search bar to filter keys

#### Locking the Vault
- Click the "Lock" button to manually lock the vault
- Auto-locks after 5 minutes of inactivity

### CLI (Command-Line Interface)

The CLI provides full vault functionality from the terminal.

#### Installation

```bash
# Build the CLI
cargo build --release --features cli --bin vult

# Linux/macOS - Install to local bin
cp target/release/vult ~/.local/bin/

# Windows - Copy to a directory in PATH
# PowerShell:
copy target\release\vult.exe $env:LOCALAPPDATA\Programs\
# Or add target\release to your PATH

# Verify installation
vult --version
```

#### Building Both Binaries

```bash
# Build everything (library + CLI + GUI)
cargo build --release --features "cli gui"

# GUI binary is at: target/release/vult-gui
# CLI binary is at: target/release/vult
```

#### Platform-Specific Notes

**Linux:**
- Session files stored in `/tmp/vult-session-<uid>`
- Database at `~/.vult/vault.db`

**Windows:**
- Session files stored in `%TEMP%\vult-session`
- Database at `%USERPROFILE%\.vult\vault.db`

#### Commands

```bash
# Initialize a new vault
vult init

# Add a key
vult add github-token -a github
vult add my-api-key --stdin < secret.txt

# Get a key value
vult get github-token -a github
vult get github-token -a github --copy  # Copy to clipboard

# List all keys
vult list
vult list --json
vult list --timestamps

# Search keys
vult search github

# Update a key
vult update github-token -a github --value "new-value"
vult update github-token -a github --description "Updated token"

# Delete a key
vult delete github-token -a github
vult delete github-token -a github --force  # Skip confirmation

# Change PIN
vult change-pin

# Show vault status
vult status
```

#### Global Options

```bash
--json             # Output in JSON format
--db-path PATH     # Use custom database path
--stay-unlocked    # Keep vault unlocked for 5 minutes (session mode)
```

#### Session Mode (--stay-unlocked)

By default, each CLI command requires PIN entry. Use `--stay-unlocked` to create
a session that persists for 5 minutes:

```bash
# First command creates session
vult get mykey --stay-unlocked

# Subsequent commands use existing session (no PIN needed)
vult list
vult get another-key

# Lock vault to clear session
vult lock
```

**Security notes:**
- Session tokens stored in temp directory with restricted permissions (0600)
- Sessions auto-expire after 5 minutes
- Ctrl+C automatically clears the session
- Use `vult lock` to explicitly end a session

#### Environment Variables

```bash
VULT_DB_PATH     # Custom database path (default: ~/.vult/vault.db)
VULT_PIN         # PIN for non-interactive use (CAUTION: insecure)
```

**Warning:** Using `VULT_PIN` is insecure as it may appear in process listings.
Use only in controlled environments (CI/CD, containers).

#### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Authentication error |
| 2 | Not found |
| 3 | Not initialized |
| 4 | Duplicate key |
| 5 | Invalid input |
| 6 | Encryption error |
| 7 | Database error |
| 8 | I/O error |

#### Shell Completions

Generate shell completion scripts for your shell:

```bash
# Bash
vult completions bash > ~/.local/share/bash-completion/completions/vult

# Zsh
vult completions zsh > ~/.zfunc/_vult

# Fish
vult completions fish > ~/.config/fish/completions/vult.fish

# PowerShell
vult completions powershell > _vult.ps1
```

## Development

### Project Structure
```
vult/
├── src/                    # Rust backend
│   ├── main.rs            # GUI entry point
│   ├── lib.rs             # Library entry point
│   ├── core/              # Core types, constants, validation
│   ├── services/          # Business logic (VaultManager, AuthService, etc.)
│   ├── gui/               # GUI-specific (AuthManager with Tauri events)
│   ├── commands.rs        # Tauri command handlers
│   ├── crypto.rs          # Cryptographic operations
│   ├── database.rs        # Database operations & migrations
│   ├── clipboard.rs       # Clipboard management
│   └── bin/               # CLI and GUI binaries
├── ui-sveltekit/          # Frontend UI (SvelteKit + TypeScript)
│   ├── src/
│   │   ├── routes/       # SvelteKit routes (+layout.svelte, +page.svelte)
│   │   ├── lib/
│   │   │   ├── components/  # Svelte components
│   │   │   │   ├── auth/    # Authentication screens
│   │   │   │   ├── vault/   # Vault management screens
│   │   │   │   ├── modals/  # Modal components
│   │   │   │   └── ui/      # shadcn-svelte components
│   │   │   ├── stores/      # Svelte stores (vault, ui, clipboard)
│   │   │   ├── services/    # Tauri API wrappers, activity tracking
│   │   │   ├── types/       # TypeScript type definitions
│   │   │   └── css/         # Tailwind CSS v4 with @theme
│   │   └── tests/           # Vitest tests
│   ├── vite.config.ts
│   └── svelte.config.js
└── capabilities/          # Tauri capabilities
```

### Frontend Architecture

The frontend uses **SvelteKit** (Svelte 5) with:

- **Svelte 5 Runes**: `$state()`, `$props()`, `$derived()` for reactivity
- **TypeScript**: Strict mode for type safety
- **Svelte Stores**: For global state management
- **Tailwind CSS v4**: With custom `@theme` configuration
- **shadcn-svelte**: Copy-paste component library
- **Vitest**: Testing framework with jsdom environment

### Running Tests

```bash
# Run Rust tests
cargo test

# Run specific module tests
cargo test --lib database
cargo test --lib crypto
cargo test --lib auth

# Run frontend tests (from ui-sveltekit directory)
cd ui-sveltekit
npm test              # Run all tests
npm run test:watch    # Watch mode
```

### Building the Frontend

```bash
# Build SvelteKit frontend (from ui-sveltekit directory)
cd ui-sveltekit
npm run build

# Build Tauri app with SvelteKit frontend
cargo tauri build
```

### Database Migrations
Database migrations are handled automatically on startup. The migration system:
1. Checks current schema version
2. Blocks if database is newer than the application
3. Creates a backup before migration
4. Runs the appropriate migration
5. Cleans up orphaned tables

To add a new migration:
1. Increment `SCHEMA_VERSION` in `src/database.rs`
2. Add a migration case in `run_migration()`
3. Document the changes in `CHANGELOG.md`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup
```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Security Considerations

### Important Notes
- **No PIN Recovery**: If you forget your PIN, your data is permanently inaccessible. This is by design for maximum security.
- **Backup Your Database**: Regularly backup `~/.vult/vault.db` to a secure location.
- **Strong PIN**: Use a strong, unique PIN that you won't forget.
- **System Security**: Ensure your system is secure and free from malware. A compromised system can intercept your PIN or decrypted keys.

### Threat Model
Vult protects against:
- ✅ Database theft (encrypted at rest)
- ✅ Unauthorized access (PIN required)
- ✅ Key isolation (per-key encryption)
- ✅ Clipboard snooping (auto-clear)

Vult does NOT protect against:
- ❌ Compromised system (keyloggers, screen capture)
- ❌ Memory dumps while unlocked
- ❌ Physical access while unlocked

## Acknowledgments

Built with:
- [Tauri](https://tauri.app/) - Cross-platform desktop framework
- [SvelteKit](https://kit.svelte.dev/) - Web framework
- [Svelte 5](https://svelte.dev/) - Reactive UI library
- [Tailwind CSS v4](https://tailwindcss.com/) - Utility-first CSS framework
- [shadcn-svelte](https://www.shadcn-svelte.com/) - Component library
- [SQLite](https://www.sqlite.org/) - Embedded database
- [Argon2](https://argon2-cffi.readthedocs.io/) - Password hashing
- [AES-GCM](https://github.com/RustCrypto/AEADs) - Authenticated encryption

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes in each version.
