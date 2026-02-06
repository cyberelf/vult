#![no_main]

use libfuzzer_sys::fuzz_target;
use vult::crypto::{encrypt, decrypt, VaultKey, EncryptedData};

fuzz_target!(|data: &[u8]| {
    // Fuzz encryption/decryption with arbitrary key values and plaintexts
    if data.len() < 32 {
        return; // Need at least 32 bytes for a key
    }
    
    // Use first 32 bytes as key, rest as plaintext
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&data[0..32]);
    let key = VaultKey::from_bytes(key_bytes);
    let plaintext = &data[32..];
    
    // Test encryption - should never panic
    if let Ok(encrypted) = encrypt(plaintext, &key) {
        // Test decryption roundtrip - should never panic
        if let Ok(decrypted) = decrypt(&encrypted, &key) {
            // Verify roundtrip correctness
            assert_eq!(plaintext, decrypted.as_slice(), "Encryption/decryption roundtrip failed");
        }
    }
    
    // Test decryption with invalid data - should fail gracefully, not panic
    let invalid_encrypted = EncryptedData {
        ciphertext: data.to_vec(),
        nonce: vec![0u8; 12], // Valid nonce length
    };
    let _ = decrypt(&invalid_encrypted, &key);
});
