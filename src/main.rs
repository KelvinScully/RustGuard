mod crypto;
mod sqlite;

use rusqlite::Connection;

fn main() {
    // 1. Open DB and initialize if needed
    let conn = Connection::open("rustguard.db").unwrap();
    sqlite::init_db(&conn).unwrap();
    println!("[*] Database initialized.");

    // 2. Create our secret password string
    let secret = "MySuperSecretPassword123!";
    println!("[*] Secret to store: {}", secret);

    // 3. Hardcoded 32-byte key (use key derivation in production!)
    let key: [u8; 32] = *b"ThisIsASecretKeyForAES256!!!1234";
    println!("[*] Using key: {:?}", &key);

    // 4. Encrypt the secret password
    let (nonce, ciphertext) = crypto::encrypt_message(&key, secret);
    println!("[*] Nonce (base64): {}", nonce);
    println!("[*] Ciphertext (base64): {}", ciphertext);

    // 5. Add it to the database under a label (let's call it "test")
    sqlite::add_credential(
        &conn,
        "test",                    // label
        "testuser",                // username
        &nonce,
        &ciphertext,
        None,                      // notes
    ).unwrap();
    println!("[*] Credential stored in database as label 'test'.");

    // 6. Retrieve from DB by label
    let stored = sqlite::get_credential_by_label(&conn, "test").unwrap().expect("Not found!");
    println!("[*] Pulled from DB: label={}, username={}, nonce={}, ciphertext={}",
             stored.label, stored.username, stored.nonce, stored.ciphertext);

    // 7. Decrypt the password
    let decrypted = crypto::decrypt_message(&key, &stored.nonce, &stored.ciphertext);
    println!("[*] Decrypted password: {}", decrypted);

    // 8. Show result
    println!("\n[SUMMARY]");
    println!("Original:   {}", secret);
    println!("Decrypted:  {}", decrypted);
}
