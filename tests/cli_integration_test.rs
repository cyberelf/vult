//! CLI Integration Tests
//!
//! These tests verify the CLI binary works correctly end-to-end.
//! Tests use a temporary database for isolation.

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

/// Get a command configured for the test binary with a temp database
fn vult_cmd(temp_dir: &TempDir) -> Command {
    let db_path = temp_dir.path().join("test-vault.db");
    let mut cmd = cargo_bin_cmd!("vult");
    cmd.env("VULT_DB_PATH", db_path);
    cmd
}

/// Initialize a test vault
fn init_vault(temp_dir: &TempDir, pin: &str) {
    vult_cmd(temp_dir)
        .arg("init")
        .env("VULT_PIN", pin)
        .assert()
        .success();
}

#[test]
fn test_cli_version() {
    cargo_bin_cmd!("vult")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("vult"));
}

#[test]
fn test_cli_help() {
    cargo_bin_cmd!("vult")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Secure API key vault"));
}

#[test]
fn test_init_creates_vault() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    vult_cmd(&temp_dir)
        .arg("init")
        .env("VULT_PIN", "123456")
        .assert()
        .success()
        .stdout(predicate::str::contains("Vault initialized"));
}

#[test]
fn test_init_fails_with_short_pin() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    vult_cmd(&temp_dir)
        .arg("init")
        .env("VULT_PIN", "123")
        .assert()
        .failure()
        .stderr(predicate::str::contains("at least 6 characters"));
}

#[test]
fn test_init_fails_if_already_initialized() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    vult_cmd(&temp_dir)
        .arg("init")
        .env("VULT_PIN", "654321")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already initialized"));
}

#[test]
fn test_status_shows_initialized() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    vult_cmd(&temp_dir)
        .arg("status")
        .env("VULT_PIN", "123456")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized: Yes"));
}

#[test]
fn test_add_and_get_key() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    // Add a key (use --stdin for non-interactive testing)
    vult_cmd(&temp_dir)
        .args(["add", "test-key", "-a", "github", "--stdin"])
        .env("VULT_PIN", "123456")
        .write_stdin("secret-value")
        .assert()
        .success()
        .stdout(predicate::str::contains("added successfully"));

    // Get the key
    vult_cmd(&temp_dir)
        .args(["get", "test-key", "-a", "github"])
        .env("VULT_PIN", "123456")
        .assert()
        .success()
        .stdout(predicate::str::contains("secret-value"));
}

#[test]
fn test_add_key_with_stdin() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    vult_cmd(&temp_dir)
        .args(["add", "stdin-key", "--stdin"])
        .env("VULT_PIN", "123456")
        .write_stdin("stdin-secret-value")
        .assert()
        .success()
        .stdout(predicate::str::contains("added successfully"));

    vult_cmd(&temp_dir)
        .args(["get", "stdin-key"])
        .env("VULT_PIN", "123456")
        .assert()
        .success()
        .stdout(predicate::str::contains("stdin-secret-value"));
}

#[test]
fn test_list_empty() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    vult_cmd(&temp_dir)
        .arg("list")
        .env("VULT_PIN", "123456")
        .assert()
        .success()
        .stdout(predicate::str::contains("No keys found"));
}

#[test]
fn test_list_shows_keys() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    // Add keys
    vult_cmd(&temp_dir)
        .args(["add", "key1", "-a", "app1", "--stdin"])
        .env("VULT_PIN", "123456")
        .write_stdin("value1")
        .assert()
        .success();

    vult_cmd(&temp_dir)
        .args(["add", "key2", "-a", "app2", "--stdin"])
        .env("VULT_PIN", "123456")
        .write_stdin("value2")
        .assert()
        .success();

    // List should show both
    vult_cmd(&temp_dir)
        .arg("list")
        .env("VULT_PIN", "123456")
        .assert()
        .success()
        .stdout(predicate::str::contains("key1"))
        .stdout(predicate::str::contains("key2"));
}

#[test]
fn test_list_json_output() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    vult_cmd(&temp_dir)
        .args(["add", "json-key", "-a", "test", "--stdin"])
        .env("VULT_PIN", "123456")
        .write_stdin("json-value")
        .assert()
        .success();

    vult_cmd(&temp_dir)
        .args(["list", "--json"])
        .env("VULT_PIN", "123456")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"key_name\": \"json-key\""));
}

#[test]
fn test_search_finds_matching_keys() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    // Add keys
    vult_cmd(&temp_dir)
        .args(["add", "github-token", "-a", "github", "--stdin"])
        .env("VULT_PIN", "123456")
        .write_stdin("gh-token")
        .assert()
        .success();

    vult_cmd(&temp_dir)
        .args(["add", "gitlab-token", "-a", "gitlab", "--stdin"])
        .env("VULT_PIN", "123456")
        .write_stdin("gl-token")
        .assert()
        .success();

    // Search should find github
    vult_cmd(&temp_dir)
        .args(["search", "github"])
        .env("VULT_PIN", "123456")
        .assert()
        .success()
        .stdout(predicate::str::contains("github-token"))
        .stdout(predicate::str::contains("github"));
}

#[test]
fn test_search_no_results() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    vult_cmd(&temp_dir)
        .args(["search", "nonexistent"])
        .env("VULT_PIN", "123456")
        .assert()
        .success()
        .stdout(predicate::str::contains("No keys matching"));
}

#[test]
fn test_update_key_value() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    // Add key
    vult_cmd(&temp_dir)
        .args(["add", "update-key", "--stdin"])
        .env("VULT_PIN", "123456")
        .write_stdin("original-value")
        .assert()
        .success();

    // Update value
    vult_cmd(&temp_dir)
        .args(["update", "update-key", "--value", "new-value"])
        .env("VULT_PIN", "123456")
        .assert()
        .success()
        .stdout(predicate::str::contains("updated successfully"));

    // Verify new value
    vult_cmd(&temp_dir)
        .args(["get", "update-key"])
        .env("VULT_PIN", "123456")
        .assert()
        .success()
        .stdout(predicate::str::contains("new-value"));
}

#[test]
fn test_delete_key() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    // Add key
    vult_cmd(&temp_dir)
        .args(["add", "delete-me", "--stdin"])
        .env("VULT_PIN", "123456")
        .write_stdin("to-be-deleted")
        .assert()
        .success();

    // Delete with --force
    vult_cmd(&temp_dir)
        .args(["delete", "delete-me", "--force"])
        .env("VULT_PIN", "123456")
        .assert()
        .success()
        .stdout(predicate::str::contains("deleted"));

    // Verify it's gone
    vult_cmd(&temp_dir)
        .args(["get", "delete-me"])
        .env("VULT_PIN", "123456")
        .assert()
        .failure();
}

#[test]
fn test_wrong_pin_fails() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    vult_cmd(&temp_dir)
        .arg("list")
        .env("VULT_PIN", "wrong-pin")
        .assert()
        .failure();
}

#[test]
fn test_lock_command() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    vult_cmd(&temp_dir)
        .arg("lock")
        .assert()
        .success()
        .stdout(predicate::str::contains("locked"));
}

#[test]
fn test_duplicate_key_fails() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    // Add key
    vult_cmd(&temp_dir)
        .args(["add", "dup-key", "--stdin"])
        .env("VULT_PIN", "123456")
        .write_stdin("value1")
        .assert()
        .success();

    // Try to add same key again
    vult_cmd(&temp_dir)
        .args(["add", "dup-key", "--stdin"])
        .env("VULT_PIN", "123456")
        .write_stdin("value2")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn test_get_nonexistent_key_fails() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    vult_cmd(&temp_dir)
        .args(["get", "nonexistent-key"])
        .env("VULT_PIN", "123456")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_change_pin_success() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    // Add a key with the original PIN
    vult_cmd(&temp_dir)
        .args(["add", "test-key", "-a", "app", "--stdin"])
        .env("VULT_PIN", "123456")
        .write_stdin("secret-value")
        .assert()
        .success();

    // Change PIN from 123456 to 654321
    vult_cmd(&temp_dir)
        .arg("change-pin")
        .env("VULT_OLD_PIN", "123456")
        .env("VULT_NEW_PIN", "654321")
        .assert()
        .success()
        .stdout(predicate::str::contains("PIN changed successfully"));

    // Verify old PIN no longer works
    vult_cmd(&temp_dir)
        .args(["get", "test-key", "-a", "app"])
        .env("VULT_PIN", "123456")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid PIN"));

    // Verify new PIN works and can access the key
    vult_cmd(&temp_dir)
        .args(["get", "test-key", "-a", "app"])
        .env("VULT_PIN", "654321")
        .assert()
        .success()
        .stdout(predicate::str::contains("secret-value"));
}

#[test]
fn test_change_pin_wrong_old_pin() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    // Try to change PIN with wrong old PIN
    vult_cmd(&temp_dir)
        .arg("change-pin")
        .env("VULT_OLD_PIN", "wrong-pin")
        .env("VULT_NEW_PIN", "654321")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid PIN"));
}

#[test]
fn test_change_pin_short_new_pin() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    init_vault(&temp_dir, "123456");

    // Try to change to a short PIN
    vult_cmd(&temp_dir)
        .arg("change-pin")
        .env("VULT_OLD_PIN", "123456")
        .env("VULT_NEW_PIN", "123")
        .assert()
        .failure()
        .stderr(predicate::str::contains("at least 6 characters"));
}
