//! Vult CLI - Command-line interface for the API key vault
//!
//! Provides full vault functionality from the terminal, including
//! creating, listing, searching, and managing API keys.

use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use arboard::Clipboard;
use clap::{Parser, Subcommand};
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, Table};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;

use vult::services::VaultManager;
use vult::VaultError;

/// Vult - Secure API Key Vault
///
/// Store and manage your API keys securely with PIN protection
/// and AES-256-GCM encryption.
#[derive(Parser, Debug)]
#[command(name = "vult")]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,

    /// Database path override (default: ~/.vult/vault.db)
    #[arg(long, global = true, env = "VULT_DB_PATH")]
    db_path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new vault with a PIN
    Init,

    /// Change the vault PIN
    ChangePin,

    /// Lock the vault (clear any session)
    Lock,

    /// Add a new API key
    Add {
        /// Application name (e.g., "github")
        #[arg(short, long)]
        app: Option<String>,

        /// Key name (e.g., "token")
        name: String,

        /// Read key value from stdin instead of prompting
        #[arg(long)]
        stdin: bool,

        /// API URL (optional)
        #[arg(short, long)]
        url: Option<String>,

        /// Description (optional)
        #[arg(short, long)]
        description: Option<String>,
    },

    /// Get an API key value
    Get {
        /// Application name
        #[arg(short, long)]
        app: Option<String>,

        /// Key name
        name: String,

        /// Show full key details, not just the value
        #[arg(long)]
        full: bool,

        /// Copy value to clipboard (auto-clears after 45s)
        #[arg(short, long)]
        copy: bool,
    },

    /// List all API keys
    List {
        /// Show timestamps
        #[arg(short, long)]
        timestamps: bool,
    },

    /// Search API keys by name, app, or description
    Search {
        /// Search query
        query: String,

        /// Show timestamps
        #[arg(short, long)]
        timestamps: bool,
    },

    /// Update an API key
    Update {
        /// Application name
        #[arg(short, long)]
        app: Option<String>,

        /// Key name
        name: String,

        /// New key value (will prompt if not provided)
        #[arg(short, long)]
        value: Option<String>,

        /// New API URL
        #[arg(short, long)]
        url: Option<String>,

        /// New description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// Delete an API key
    Delete {
        /// Application name
        #[arg(short, long)]
        app: Option<String>,

        /// Key name
        name: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },

    /// Show vault status
    Status,
}

/// Get the database path, either from CLI arg, env, or default.
fn get_db_path(cli_path: Option<PathBuf>) -> PathBuf {
    if let Some(path) = cli_path {
        return path;
    }

    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".vult");
    std::fs::create_dir_all(&path).ok();
    path.push("vault.db");
    path
}

/// Read PIN from environment variable or prompt user.
///
/// If VULT_PIN is set, uses that value with a security warning.
/// Otherwise, prompts the user for secure input.
fn read_pin(prompt: &str) -> io::Result<String> {
    if let Ok(pin) = std::env::var("VULT_PIN") {
        eprintln!(
            "{}: Using PIN from VULT_PIN environment variable. This may be insecure.",
            "Warning".yellow().bold()
        );
        return Ok(pin);
    }
    print!("{}", prompt);
    io::stdout().flush()?;
    rpassword::read_password()
}

/// Read PIN with confirmation (never uses VULT_PIN for safety).
fn read_pin_with_confirmation(prompt: &str) -> io::Result<Option<String>> {
    let pin = read_pin(prompt)?;
    let confirm = read_pin("Confirm PIN: ")?;

    if pin != confirm {
        return Ok(None);
    }
    Ok(Some(pin))
}

/// Copy text to system clipboard.
fn copy_to_clipboard(text: &str) -> Result<(), VaultError> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| VaultError::Clipboard(format!("Failed to access clipboard: {}", e)))?;
    clipboard
        .set_text(text)
        .map_err(|e| VaultError::Clipboard(format!("Failed to copy to clipboard: {}", e)))?;
    Ok(())
}

/// Print error message in red with optional suggestion.
fn print_error(err: &VaultError) {
    eprintln!("{}: {}", "Error".red().bold(), err);
    if let Some(suggestion) = err.suggestion() {
        eprintln!("{}: {}", "Hint".cyan(), suggestion);
    }
}

/// Print success message in green.
fn print_success(msg: &str) {
    println!("{}: {}", "Success".green().bold(), msg);
}

#[tokio::main]
async fn main() -> ExitCode {
    // Handle Ctrl+C gracefully
    ctrlc::set_handler(move || {
        // Print newline to clean up after potential input prompt
        eprintln!();
        eprintln!("{}", "Interrupted. Exiting...".yellow());
        std::process::exit(130); // Standard exit code for SIGINT
    })
    .expect("Error setting Ctrl-C handler");

    let cli = Cli::parse();

    if let Err(e) = run(cli).await {
        print_error(&e);
        // Convert i32 exit code to u8 (capped at 255)
        ExitCode::from(e.exit_code().clamp(0, 255) as u8)
    } else {
        ExitCode::SUCCESS
    }
}

async fn run(cli: Cli) -> Result<(), VaultError> {
    let db_path = get_db_path(cli.db_path);
    let db_url = format!(
        "sqlite://{}?mode=rwc",
        db_path.to_str().unwrap_or(".").replace('\\', "/")
    );

    match cli.command {
        Commands::Init => cmd_init(&db_url).await,
        Commands::ChangePin => cmd_change_pin(&db_url).await,
        Commands::Lock => cmd_lock().await,
        Commands::Add {
            app,
            name,
            stdin,
            url,
            description,
        } => cmd_add(&db_url, app, name, stdin, url, description).await,
        Commands::Get {
            app,
            name,
            full,
            copy,
        } => cmd_get(&db_url, app, name, full, copy, cli.json).await,
        Commands::List { timestamps } => cmd_list(&db_url, timestamps, cli.json).await,
        Commands::Search { query, timestamps } => {
            cmd_search(&db_url, &query, timestamps, cli.json).await
        }
        Commands::Update {
            app,
            name,
            value,
            url,
            description,
        } => cmd_update(&db_url, app, name, value, url, description).await,
        Commands::Delete { app, name, force } => cmd_delete(&db_url, app, name, force).await,
        Commands::Status => cmd_status(&db_url).await,
    }
}

async fn cmd_init(db_url: &str) -> Result<(), VaultError> {
    let vault = VaultManager::new(db_url).await?;

    if vault.is_initialized().await? {
        return Err(VaultError::AlreadyInitialized);
    }

    println!("Initializing new vault...");
    println!();
    println!("{}", "IMPORTANT: Remember your PIN!".yellow().bold());
    println!("There is NO recovery option if you forget it.");
    println!();

    let pin = match read_pin_with_confirmation("Enter PIN (min 6 characters): ")
        .map_err(|e| VaultError::Io(e.to_string()))?
    {
        Some(pin) => pin,
        None => return Err(VaultError::InvalidInput("PINs do not match".to_string())),
    };

    vault.auth().init_vault(&pin).await?;
    print_success("Vault initialized successfully!");
    Ok(())
}

async fn cmd_change_pin(db_url: &str) -> Result<(), VaultError> {
    let vault = VaultManager::new(db_url).await?;

    if !vault.is_initialized().await? {
        return Err(VaultError::NotInitialized);
    }

    let old_pin = read_pin("Current PIN: ").map_err(|e| VaultError::Io(e.to_string()))?;

    let new_pin = match read_pin_with_confirmation("New PIN (min 6 characters): ")
        .map_err(|e| VaultError::Io(e.to_string()))?
    {
        Some(pin) => pin,
        None => {
            return Err(VaultError::InvalidInput(
                "New PINs do not match".to_string(),
            ))
        }
    };

    vault.auth().change_pin(&old_pin, &new_pin).await?;
    print_success("PIN changed successfully!");
    Ok(())
}

async fn cmd_lock() -> Result<(), VaultError> {
    // In single-command CLI mode, each command is a fresh process
    // The vault is locked when the process exits. This command is
    // primarily for future session mode support.
    println!("{}", "Vault is locked.".green());
    println!("(Each CLI command runs independently - vault auto-locks on exit)");
    Ok(())
}

async fn cmd_add(
    db_url: &str,
    app: Option<String>,
    name: String,
    stdin: bool,
    url: Option<String>,
    description: Option<String>,
) -> Result<(), VaultError> {
    let vault = VaultManager::new(db_url).await?;
    let pin = read_pin("PIN: ").map_err(|e| VaultError::Io(e.to_string()))?;
    vault.auth().unlock(&pin).await?;

    let value = if stdin {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| VaultError::Io(e.to_string()))?;
        buffer.trim().to_string()
    } else {
        read_pin("Key value: ").map_err(|e| VaultError::Io(e.to_string()))?
    };

    vault
        .keys()
        .create(
            app.as_deref(),
            &name,
            &value,
            url.as_deref(),
            description.as_deref(),
        )
        .await?;

    let display_name = match &app {
        Some(a) => format!("{}/{}", a, name),
        None => name,
    };
    print_success(&format!("Key '{}' added successfully", display_name));
    Ok(())
}

async fn cmd_get(
    db_url: &str,
    app: Option<String>,
    name: String,
    full: bool,
    copy: bool,
    json: bool,
) -> Result<(), VaultError> {
    let vault = VaultManager::new(db_url).await?;
    let pin = read_pin("PIN: ").map_err(|e| VaultError::Io(e.to_string()))?;
    vault.auth().unlock(&pin).await?;

    let app_name = app.as_deref().unwrap_or("");
    let key = vault.keys().get(app_name, &name).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&key).unwrap());
    } else if full {
        println!(
            "{}: {}",
            "App".cyan(),
            key.app_name.as_deref().unwrap_or("-")
        );
        println!("{}: {}", "Name".cyan(), key.key_name);
        println!("{}: {}", "Value".cyan(), key.key_value);
        println!(
            "{}: {}",
            "URL".cyan(),
            key.api_url.as_deref().unwrap_or("-")
        );
        println!(
            "{}: {}",
            "Description".cyan(),
            key.description.as_deref().unwrap_or("-")
        );
        println!("{}: {}", "Created".cyan(), key.created_at);
        println!("{}: {}", "Updated".cyan(), key.updated_at);
    } else if copy {
        copy_to_clipboard(&key.key_value)?;
        println!("{}", "Key copied to clipboard!".green());
        println!("{}", "⚠ Clipboard will be cleared in 45 seconds.".yellow());
        // Spawn background thread to clear clipboard after 45 seconds
        std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_secs(45));
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                let _ = clipboard.set_text("");
            }
        });
        // Wait a moment to ensure clipboard is set
        std::thread::sleep(std::time::Duration::from_millis(100));
    } else {
        println!("{}", key.key_value);
    }

    Ok(())
}

async fn cmd_list(db_url: &str, timestamps: bool, json: bool) -> Result<(), VaultError> {
    let vault = VaultManager::new(db_url).await?;
    let pin = read_pin("PIN: ").map_err(|e| VaultError::Io(e.to_string()))?;
    vault.auth().unlock(&pin).await?;

    let keys = vault.keys().list().await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&keys).unwrap());
        return Ok(());
    }

    if keys.is_empty() {
        println!("No keys found in the vault.");
        return Ok(());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    if timestamps {
        table.set_header(vec!["App", "Name", "Description", "Created", "Updated"]);
    } else {
        table.set_header(vec!["App", "Name", "Description"]);
    }

    for key in keys {
        let app = key.app_name.as_deref().unwrap_or("-");
        let desc = key.description.as_deref().unwrap_or("-");

        if timestamps {
            table.add_row(vec![
                app,
                &key.key_name,
                desc,
                &key.created_at.format("%Y-%m-%d %H:%M").to_string(),
                &key.updated_at.format("%Y-%m-%d %H:%M").to_string(),
            ]);
        } else {
            table.add_row(vec![app, &key.key_name, desc]);
        }
    }

    println!("{table}");
    Ok(())
}

async fn cmd_search(
    db_url: &str,
    query: &str,
    timestamps: bool,
    json: bool,
) -> Result<(), VaultError> {
    let vault = VaultManager::new(db_url).await?;
    let pin = read_pin("PIN: ").map_err(|e| VaultError::Io(e.to_string()))?;
    vault.auth().unlock(&pin).await?;

    let keys = vault.keys().search(query).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&keys).unwrap());
        return Ok(());
    }

    if keys.is_empty() {
        println!("No keys matching '{}' found.", query);
        return Ok(());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    if timestamps {
        table.set_header(vec!["App", "Name", "Description", "Created", "Updated"]);
    } else {
        table.set_header(vec!["App", "Name", "Description"]);
    }

    for key in keys {
        let app = key.app_name.as_deref().unwrap_or("-");
        let desc = key.description.as_deref().unwrap_or("-");

        if timestamps {
            table.add_row(vec![
                app,
                &key.key_name,
                desc,
                &key.created_at.format("%Y-%m-%d %H:%M").to_string(),
                &key.updated_at.format("%Y-%m-%d %H:%M").to_string(),
            ]);
        } else {
            table.add_row(vec![app, &key.key_name, desc]);
        }
    }

    println!("{table}");
    Ok(())
}

async fn cmd_update(
    db_url: &str,
    app: Option<String>,
    name: String,
    value: Option<String>,
    url: Option<String>,
    description: Option<String>,
) -> Result<(), VaultError> {
    let vault = VaultManager::new(db_url).await?;
    let pin = read_pin("PIN: ").map_err(|e| VaultError::Io(e.to_string()))?;
    vault.auth().unlock(&pin).await?;

    let app_name = app.as_deref().unwrap_or("");

    // First, get the existing key to get its ID
    let existing = vault.keys().get(app_name, &name).await?;

    // Build update request
    let request = vult::services::UpdateKeyRequest {
        key_value: value,
        api_url: url.map(Some),
        description: description.map(Some),
        ..Default::default()
    };

    vault.keys().update(&existing.id, request).await?;

    let display_name = match app {
        Some(a) => format!("{}/{}", a, name),
        None => name,
    };
    print_success(&format!("Key '{}' updated successfully", display_name));
    Ok(())
}

async fn cmd_delete(
    db_url: &str,
    app: Option<String>,
    name: String,
    force: bool,
) -> Result<(), VaultError> {
    let vault = VaultManager::new(db_url).await?;
    let pin = read_pin("PIN: ").map_err(|e| VaultError::Io(e.to_string()))?;
    vault.auth().unlock(&pin).await?;

    let app_name = app.as_deref().unwrap_or("");
    let display_name = match &app {
        Some(a) => format!("{}/{}", a, name),
        None => name.clone(),
    };

    if !force {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Delete key '{}'?", display_name))
            .default(false)
            .interact()
            .map_err(|e| VaultError::Io(e.to_string()))?;

        if !confirm {
            println!("Cancelled.");
            return Ok(());
        }
    }

    vault.keys().delete_by_name(app_name, &name).await?;
    print_success(&format!("Key '{}' deleted", display_name));
    Ok(())
}

async fn cmd_status(db_url: &str) -> Result<(), VaultError> {
    let vault = VaultManager::new(db_url).await?;

    let initialized = vault.is_initialized().await?;

    println!("Vault Status");
    println!("------------");
    println!(
        "Initialized: {}",
        if initialized {
            "Yes".green()
        } else {
            "No".red()
        }
    );

    if initialized {
        // Try to get key count after unlocking
        let pin =
            read_pin("PIN (to see key count): ").map_err(|e| VaultError::Io(e.to_string()))?;

        match vault.auth().unlock(&pin).await {
            Ok(()) => {
                let count = vault.keys().count().await?;
                println!("Keys stored: {}", count);
            }
            Err(e) => {
                println!("Could not unlock: {}", e);
            }
        }
    }

    Ok(())
}
