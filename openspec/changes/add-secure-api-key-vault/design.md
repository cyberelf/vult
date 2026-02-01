# Design: Secure API Key Vault

## Context

**Problem**: Users need a secure way to store and manage API keys on their desktop machines with strong access controls.

**Constraints**:
- Must work cross-platform (Windows, macOS, Linux)
- Single-user design simplifies implementation
- Security is critical - encryption at rest, authenticated access
- Desktop app (not web/service-based)

**Stakeholders**: End users managing multiple API keys for various services

## Goals / Non-Goals

### Goals
- Secure encrypted storage of API keys
- Device-gated access via biometric or PIN
- Simple, intuitive user interface
- Cross-platform compatibility
- Key properties: app name, key name, API URL, description

### Non-Goals
- Multi-user support on same device
- Cloud sync or backup (future consideration)
- Sharing keys between devices
- Password generation
- Browser extension integration (future)
- API key usage monitoring/auditing

## Decisions

### Decision 1: Desktop Framework - Tauri

**Choice**: Use Tauri for cross-platform desktop application

**Rationale**:
- Rust backend matches project language
- Smaller bundle size vs Electron
- Native OS integration for biometrics
- Strong security track record
- Active community and good documentation

**Alternatives Considered**:
- **Electron**: Rejected due to large bundle size, Node.js/JS stack
- **Native (Win/Mac/Linux separately)**: Rejected due to 3x code maintenance
- **Flutter Desktop**: Rejected due to Dart language mismatch

### Decision 2: Encryption Strategy - Argon2 + AES-256-GCM

**Choice**: Derive encryption key from PIN using Argon2id, encrypt with AES-256-GCM

**Rationale**:
- Argon2id is memory-hard, resistant to GPU/ASIC attacks
- AES-256-GCM provides authenticated encryption (prevents tampering)
- Well-studied, battle-tested algorithms
- Rust has excellent crate support (rust-argon2, aes-gcm)

**For biometric auth**: Use platform secure enclave (Windows Hello Credential Manager, macOS Keychain, Linux Secret Service) to store a randomly generated vault key, biometric unlocks access to that key.

**Alternatives Considered**:
- **Age encryption**: Rejected due to less familiar key management pattern
- **ChaCha20-Poly1305**: Valid alternative, but AES has wider platform acceleration

### Decision 3: Data Storage - SQLite with SQLx

**Choice**: SQLite database with SQLx for async database operations

**Rationale**:
- Embedded, no external server needed
- Single file simplifies backup/migration
- SQLx provides compile-time query verification
- Efficient for expected data size (hundreds to thousands of keys)

**Schema**:
```sql
CREATE TABLE api_keys (
    id TEXT PRIMARY KEY,
    app_name TEXT NOT NULL,
    key_name TEXT NOT NULL,
    api_url TEXT,
    description TEXT,
    encrypted_key_value BLOB NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    UNIQUE(app_name, key_name)
);
```

**Alternatives Considered**:
- **JSON file**: Rejected due to need to decrypt/entire file for reads
- **Redis/Postgres**: Rejected due to requiring external service

### Decision 4: Biometric Integration

**Choice**: Platform-specific APIs via abstraction layer

**Implementation**:
- Windows: Windows Hello via `windows` crate
- macOS: LocalAuthentication via objc
- Linux: libsecret (Secret Service API) or polkit

**Fallback**: PIN-based authentication always available

**Alternatives Considered**:
- **WebAuthn**: Rejected as designed for web, adds complexity
- **Third-party biometric SDK**: Rejected to avoid vendor lock-in

### Decision 5: Application State

**Choice**: Single locked/unlocked state, no background decryption

**Behavior**:
- App launches in locked state
- Authentication required to unlock
- Auto-lock after inactivity (configurable, default 5 minutes)
- Keys decrypted only in memory when unlocked
- Encryption keys cleared from memory on lock

**Security Rationale**: Minimizes exposure window for decrypted keys

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      UI Layer (Tauri Frontend)              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │   Auth   │  │  Key     │  │  Create  │  │  Edit    │   │
│  │  Screen  │  │   List   │  │   Form   │  │   Form   │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer (Rust)                 │
│  ┌──────────────────┐  ┌────────────────────────────────┐  │
│  │   Auth Manager   │  │      API Key Service           │  │
│  │  - PIN validation │  │  - Create, Read, Update, Delete│  │
│  │  - Biometric     │  │  - Search                      │  │
│  │  - Session mgmt  │  │  - Copy to clipboard           │  │
│  └──────────────────┘  └────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                      Infrastructure Layer                   │
│  ┌──────────────────┐  ┌────────────────────────────────┐  │
│  │  Crypto Module   │  │      Storage Layer              │  │
│  │  - Argon2id KDF  │  │  - SQLite + SQLx               │  │
│  │  - AES-256-GCM   │  │  - Migrations                  │  │
│  │  - Key wrapping  │  │  - Transactions                │  │
│  └──────────────────┘  └────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │           Platform Integration                        │  │
│  │  - Windows Hello / macOS Keychain / Linux Secret Svc │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Security Considerations

### Threat Model

**Protected Against**:
- Physical theft of device (locked vault)
- Malware scanning memory (keys encrypted at rest, minimized in-memory exposure)
- OS compromise (mitigated by platform secure enclave for biometric)

**Not Protected Against** (acknowledged limitations):
- Malware running as user with screen recording/keylogging
- Memory dump while app is unlocked
- Coercion/forced PIN disclosure

### Security Practices

1. **Key Hygiene**:
   - Zero sensitive memory after use
   - Prefer `Vec<u8>` over `String` for keys
   - Use `zeroize` crate for secure clearing

2. **Encryption at Rest**:
   - Every key individually encrypted
   - Random IV/nonce per encryption
   - Authenticated encryption (GCM) prevents tampering

3. **Authentication**:
   - Rate limiting on PIN attempts (exponential backoff)
   - Biometric requires device setup (PIN fallback if not available)
   - Session timeout minimizes exposure

4. **Data in Transit**:
   - IPC between frontend/backend uses Tauri's secure IPC
   - No network transmission (local-only app)

## Risks / Trade-offs

| Risk | Impact | Mitigation |
|------|--------|------------|
| Weak PIN chosen | Medium | Enforce minimum PIN length, entropy requirements |
| Biometric spoofing | Medium | Platform-level protections; PIN fallback for high-security |
| Memory exposure | Medium | Minimize time keys are decrypted; use zeroize |
| Database corruption | Low | Regular backups; SQLite durability |
| Lost PIN/biometric | High | Recovery codes stored securely during setup (future enhancement) |
| Supply chain attack | High | Pin dependencies, regular audits |

## Migration Plan

**Initial Setup** (No migration needed):
- New installations run first-time setup wizard
- User chooses PIN, sets up biometric if available
- New vault database created

**Future Migration** (When upgrading):
- On app upgrade, detect database version
- Run migrations in transaction
- Backup before migration
- Rollback on failure

## Open Questions

1. **Recovery Mechanism**: If user forgets PIN, how do they recover?
   - *Proposal*: Initial version = data loss (document clearly)
   - *Future*: Recovery key generated during setup

2. **Cloud Backup**: Should users be able to backup encrypted vault?
   - *Deferred*: Out of scope for MVP

3. **Import/Export**: Support importing from plaintext CSV?
   - *Deferred*: Security consideration, defer to later

4. **Clipboard Timeout**: How long should copied keys remain?
   - *Proposal*: 30 seconds default, configurable
