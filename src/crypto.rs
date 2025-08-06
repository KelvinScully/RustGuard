use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-256-GCM cipher
use aes_gcm::aead::{Aead, KeyInit};   // Encryption traits
use rand::RngCore;                    // For secure random number generation
use base64::{engine::general_purpose, Engine as _}; // Base64 encoding (modern API)

/// Encrypts a plaintext message using AES-256-GCM
///
/// # Arguments
/// * `key_bytes` - A 32-byte AES key
/// * `plaintext` - The message to encrypt
///
/// # Returns
/// A tuple containing:
/// * `nonce_base64` - A base64-encoded nonce
/// * `ciphertext_base64` - A base64-encoded encrypted message
pub fn encrypt_message(key_bytes: &[u8; 32], plaintext: &str) -> (String, String) {
    // Generate a random 12-byte nonce
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    // Create AES-GCM cipher with the key
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt the plaintext
    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
        .expect("Encryption failed!");

    // Return base64-encoded nonce and ciphertext
    (
        general_purpose::STANDARD.encode(nonce_bytes),
        general_purpose::STANDARD.encode(ciphertext),
    )
}

/// Decrypts a base64-encoded ciphertext using AES-256-GCM
///
/// # Arguments
/// * `key_bytes` - A 32-byte AES key
/// * `base64_nonce` - The base64-encoded nonce
/// * `base64_ciphertext` - The base64-encoded ciphertext
///
/// # Returns
/// A `String` containing the decrypted plaintext
pub fn decrypt_message(key_bytes: &[u8; 32], base64_nonce: &str, base64_ciphertext: &str) -> String {
    // Decode base64-encoded nonce and ciphertext
    let nonce_bytes = general_purpose::STANDARD.decode(base64_nonce)
        .expect("Failed to decode nonce");
    let ciphertext = general_purpose::STANDARD.decode(base64_ciphertext)
        .expect("Failed to decode ciphertext");

    // Create AES-GCM cipher with the key
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Decrypt the ciphertext
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .expect("Decryption failed!");

    // Convert decrypted bytes to UTF-8 string
    String::from_utf8(plaintext).expect("Invalid UTF-8")
}
