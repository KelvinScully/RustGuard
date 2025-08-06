
mod crypto;

fn main() {
    let key: [u8; 32] = *b"ThisIsASecretKeyForAES256!!!1234"; // Must be exactly 32 bytes

    let message: &str = "RustGuard is secure!";
    println!("Original: {}", message);

    // Encrypt the message
    let (nonce, encrypted) = crypto::encrypt_message(&key, message);
    println!("Encrypted (base64): {}", encrypted);
    println!("Nonce (base64): {}", nonce);

    // Decrypt the message
    let decrypted = crypto::decrypt_message(&key, &nonce, &encrypted);
    println!("Decrypted: {}", decrypted);
}
