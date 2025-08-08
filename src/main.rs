mod crypto;
mod cli;
mod sqlite;

use clap::Parser;
use serde::{Serialize, Deserialize};
use rusqlite::Connection;

#[derive(Serialize, Deserialize)]
struct Credential {
    user: String,
    password: Option<String>,
    notes: Option<String>,
}

fn main() {
    // 1. Open DB and initialize
    let conn = Connection::open("rustguard.db").expect("Failed to open DB");
    sqlite::init_db(&conn).expect("Failed to init DB");

    // 2. Hardcoded 32-byte key (TODO: derive from master password in real app)
    let key: [u8; 32] = *b"ThisIsASecretKeyForAES256!!!1234";

    // 3. Parse CLI arguments
    let cli = cli::Cli::parse();

    // 4. Handles commands
    match &cli.command {
        // Add a new credential
        cli::Commands::Add { label, user, password, notes } => {
            let password_str = password.clone().unwrap_or_default();
            let (nonce, ciphertext) = crypto::encrypt_message(&key, &password_str);

            sqlite::add_credential(
                &conn,
                label,
                user,
                &nonce,
                &ciphertext,
                notes.as_deref(),
            ).expect("Failed to add credential");

            println!("Added: label={}, user={}, password=<encrypted>, notes={:?}", label, user, notes);
        }
        // Get a credential by label
        cli::Commands::Get { label } => {
            match sqlite::get_credential_by_label(&conn, label) {
                Ok(Some(cred)) => {
                    let decrypted = crypto::decrypt_message(&key, &cred.nonce, &cred.ciphertext);
                    println!("Label: {}, User: {}, Password: {}, Notes: {:?}", cred.label, cred.username, decrypted, cred.notes);
                }
                Ok(None) => println!("Credential not found for label: {}", label),
                Err(e) => println!("DB error: {}", e),
            }
        }
        // Delete a credential by label
        cli::Commands::Delete { label, .. } => {
            match sqlite::delete_credential_by_label(&conn, label) {
                Ok(true) => println!("Deleted credential with label: {}", label),
                Ok(false) => println!("No credential found for label: {}", label),
                Err(e) => println!("DB error: {}", e),
            }
        }
        // List all credentials
        cli::Commands::List => {
            match sqlite::list_credentials(&conn) {
                Ok(creds) => {
                    println!("Stored credentials:");
                    for cred in creds {
                        let decrypted = crypto::decrypt_message(&key, &cred.nonce, &cred.ciphertext);
                        println!("Label: {}, User: {}, Password: {}, Notes: {:?}", cred.label, cred.username, decrypted, cred.notes);
                    }
                }
                Err(e) => println!("DB error: {}", e),
            }
        }
        // Generate a random password
       cli::Commands::Generate { length, symbols } => {
    use rand::{Rng};

    let mut rng = rand::thread_rng();
    let mut charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();

    if *symbols {
        charset.extend("!@#$%^&*()-_=+[]{};:,.<>?/\\|".chars());
    }

    let password: String = (0..*length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx]
        })
        .collect();

    println!("Generated password: {}", password);
},
        // Export credentials to JSON file
cli::Commands::Export { path } => {
    match sqlite::list_credentials(&conn) {
        Ok(creds) => {
            let json = serde_json::to_string_pretty(&creds).expect("Failed to serialize credentials");
            std::fs::write(path, json).expect("Failed to write export file");
            println!("Exported {} credentials to {}", creds.len(), path);
        }
        Err(e) => println!("DB error: {}", e),
    }
},
        // Import credentials from JSON file
cli::Commands::Import { path } => {
    let data = std::fs::read_to_string(path)
        .expect("Failed to read import file");

    // Explicitly tell serde_json to read a Vec of CredentialRecord
    let creds: Vec<sqlite::CredentialRecord> =
        serde_json::from_str(&data)
        .expect("Failed to parse JSON");

    for cred in &creds {
        sqlite::add_credential(
            &conn,
            &cred.label,
            &cred.username,
            &cred.nonce,
            &cred.ciphertext,
            cred.notes.as_deref(),
        ).expect("Failed to import credential");
    }

    println!("Imported {} credentials from {}", creds.len(), path);
}

}
}
