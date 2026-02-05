# Vult CLI User Guide

A comprehensive guide to using the Vult command-line interface for managing your API keys securely.

## Table of Contents

- [Getting Started](#getting-started)
- [Authentication](#authentication)
- [Key Management](#key-management)
- [Advanced Usage](#advanced-usage)
- [Security Best Practices](#security-best-practices)
- [Troubleshooting](#troubleshooting)

## Getting Started

### Installation

**Linux:**
```bash
# Download the latest release binary
wget https://github.com/yourusername/vult/releases/latest/download/vult-linux-x64
chmod +x vult-linux-x64
sudo mv vult-linux-x64 /usr/local/bin/vult
```

**Windows:**
```powershell
# Download the latest release
Invoke-WebRequest -Uri "https://github.com/yourusername/vult/releases/latest/download/vult-windows-x64.exe" -OutFile "vult.exe"
# Add to PATH or move to a directory in PATH
```

### First-Time Setup

Initialize your vault with a secure PIN:

```bash
vult init
```

You'll be prompted to:
1. Enter a new PIN (minimum 6 characters)
2. Confirm the PIN

> **Important**: There is no PIN recovery. If you forget your PIN, your data is permanently inaccessible by design.

### Basic Workflow

```bash
# 1. Add an API key
vult add github token
# Enter your PIN when prompted
# Enter the API key value when prompted

# 2. Retrieve a key
vult get github token
# Enter your PIN
# Key value is printed to stdout

# 3. List all keys
vult list
# Shows table of all stored keys (no values)

# 4. Delete a key
vult delete github token
# Confirm deletion when prompted
```

## Authentication

### PIN Requirements

- Minimum 6 characters
- Maximum 64 characters
- Any printable characters allowed
- Choose a strong, memorable PIN

### Entering Your PIN

By default, each command prompts for your PIN:

```bash
vult get github token
PIN: ********
```

### Scripting with Environment Variable

For automation, you can set the PIN via environment variable:

```bash
export VULT_PIN="your-secret-pin"
vult get github token
```

> ⚠️ **Security Warning**: Using `VULT_PIN` exposes your PIN in:
> - Shell history
> - Process list (`ps aux`)
> - Environment variable dumps
> 
> Only use this for automated scripts in secure environments.

### Changing Your PIN

```bash
vult change-pin
# Enter current PIN
# Enter new PIN
# Confirm new PIN
```

Note: Existing keys remain accessible with the new PIN.

## Key Management

### Adding Keys

**Interactive mode (recommended):**
```bash
vult add github personal-token
# Prompts for key value securely (hidden input)
```

**With description:**
```bash
vult add github personal-token --description "Personal access token for CI/CD"
```

**From stdin (for piping):**
```bash
echo "ghp_xxxx..." | vult add github token --stdin
# Or from a file
cat secret.txt | vult add github token --stdin
```

**With expiration:**
```bash
vult add github token --expires-at "2025-12-31"
```

### Retrieving Keys

**Get raw value (for scripts):**
```bash
vult get github token
# Outputs just the key value, no formatting
```

**Get with full metadata:**
```bash
vult get github token --full
# Shows: App name, Key name, Value, Description, Created, Updated
```

**Copy to clipboard:**
```bash
vult get github token --copy
# Key copied to clipboard, auto-clears in 45 seconds
```

### Listing Keys

**Table format (default):**
```bash
vult list
```
Output:
```
┌─────────────┬────────────────────┬──────────────────────────────────────┐
│ App Name    │ Key Name           │ ID                                   │
├─────────────┼────────────────────┼──────────────────────────────────────┤
│ github      │ personal-token     │ 550e8400-e29b-41d4-a716-446655440000 │
│ aws         │ access-key         │ 660f9500-f3ab-51e5-b827-557766550111 │
└─────────────┴────────────────────┴──────────────────────────────────────┘
```

**With timestamps:**
```bash
vult list --timestamps
```

**JSON format:**
```bash
vult list --json
```
Output:
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "app_name": "github",
    "key_name": "personal-token",
    "description": "Personal access token",
    "created_at": "2025-01-15T10:30:00Z",
    "updated_at": "2025-01-15T10:30:00Z"
  }
]
```

### Searching Keys

```bash
vult search github
# Searches app_name, key_name, and description
```

**Case-insensitive partial matching:**
```bash
vult search TOKEN
# Matches: github/token, aws/api-token, etc.
```

### Updating Keys

**Interactive mode:**
```bash
vult update github token
# Prompts for new value
```

**Update just the value:**
```bash
vult update github token --value "new-key-value"
```

**Update metadata only:**
```bash
vult update github token --description "Updated description"
```

### Deleting Keys

**With confirmation:**
```bash
vult delete github token
# Are you sure? [y/N]
```

**Skip confirmation (for scripts):**
```bash
vult delete github token --force
```

## Advanced Usage

### Database Location

Default location: `~/.vult/vault.db`

Override with environment variable:
```bash
export VULT_DB_PATH="/custom/path/vault.db"
vult list
```

### Exit Codes

Use exit codes for script control flow:

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Authentication failed |
| 2 | Key not found |
| 3 | Vault not initialized |
| 4 | Key already exists |
| 5 | Invalid input |
| 6 | Cryptographic error |
| 7 | Database error |
| 8 | I/O error |
| 9 | Invalid state |
| 10 | Clipboard error |

**Script example:**
```bash
if vult get github token > /dev/null 2>&1; then
    echo "Key exists"
else
    echo "Key not found or error"
fi
```

### JSON Output for Scripting

Combine with `jq` for powerful querying:

```bash
# Get all GitHub keys
vult list --json | jq '.[] | select(.app_name == "github")'

# Count keys by app
vult list --json | jq 'group_by(.app_name) | map({app: .[0].app_name, count: length})'

# Get key names only
vult list --json | jq -r '.[].key_name'
```

### Piping Secrets

Use for secure secret injection:

```bash
# Set as environment variable
export API_KEY=$(vult get github token)

# Pass to Docker
vult get github token | docker secret create github_token -

# Use with curl
curl -H "Authorization: Bearer $(vult get github token)" https://api.example.com
```

## Security Best Practices

### DO ✅

1. **Use a strong PIN** - At least 8 characters, mix of types
2. **Use --copy flag** - Avoid key values in terminal history
3. **Clear clipboard manually** - Don't wait for auto-clear for sensitive operations
4. **Use separate vaults** - Consider different vaults for different environments
5. **Keep vault file backed up** - But store backups securely!

### DON'T ❌

1. **Don't share your PIN** - No recovery mechanism exists
2. **Don't use VULT_PIN in scripts** checked into version control
3. **Don't pipe key values to files** - Use `--copy` instead
4. **Don't leave terminal with keys visible** - Clear with `clear` command
5. **Don't use short PINs** - 6 is minimum, 12+ recommended

### Securing the Vault File

On Linux, ensure proper permissions:
```bash
chmod 600 ~/.vult/vault.db
chmod 700 ~/.vult
```

## Troubleshooting

### "Vault not initialized"

Run `vult init` to set up your vault with a PIN.

### "Invalid PIN"

Double-check your PIN. Remember:
- PINs are case-sensitive
- No way to reset if forgotten

### "Key not found"

Check exact app_name and key_name:
```bash
vult list  # See all keys
vult search partial-name  # Search for partial matches
```

### "Database error"

The vault file may be corrupted or inaccessible:
```bash
# Check file exists and permissions
ls -la ~/.vult/vault.db

# Check not locked by another process
lsof ~/.vult/vault.db
```

### Clipboard Not Working

On Linux, ensure you have a clipboard manager running:
```bash
# X11
xclip -version

# Wayland
wl-copy --version
```

## Getting Help

```bash
# General help
vult --help

# Command-specific help
vult add --help
vult get --help
```

## Version Information

```bash
vult --version
```

---

For more information, see the [README](../README.md) or open an issue on GitHub.
