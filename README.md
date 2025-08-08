## ReadMe ##

## Encryption Module - RustGuard

This module provides secure AES-256-GCM encryption and decryption for storing sensitive information such as passwords. It uses a unique nonce per encryption and encodes the output in Base64 for safe storage and transfer.

### Features:
- AES-256-GCM encryption using `aes-gcm`, `aead`, and `aes` crates.
- Random nonce generation using `rand` crate.
- Base64 encoding/decoding using the modern `Engine::encode` API.
- Written in a modular way (`crypto.rs`) for easy integration with database and CLI components.

### Example Usage (from `main.rs`):

```rust
let key: [u8; 32] = *b"ThisIsASecretKeyForAES256!!!1234"; // Must be 32 bytes
let message = "RustGuard is secure!";

let (nonce, encrypted) = crypto::encrypt_message(&key, message);
let decrypted = crypto::decrypt_message(&key, &nonce, &encrypted);

println!("Original: {}", message);
println!("Encrypted: {}", encrypted);
println!("Decrypted: {}", decrypted);

