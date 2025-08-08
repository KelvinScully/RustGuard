use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialRecord {
    pub label: String,
    pub username: String,
    pub nonce: String,
    pub ciphertext: String,
    pub notes: Option<String>,
}


// Delete a credential by label
pub fn delete_credential_by_label(conn: &Connection, label: &str) -> Result<bool> {
    let affected = conn.execute(
        "DELETE FROM credentials WHERE label = ?1",
        params![label],
    )?;
    Ok(affected > 0)
}

// List all credentials
pub fn list_credentials(conn: &Connection) -> rusqlite::Result<Vec<CredentialRecord>> {
    let mut stmt = conn.prepare("SELECT label, username, nonce, ciphertext, notes FROM credentials")?;
    let rows = stmt.query_map([], |row| {
        Ok(CredentialRecord {
            label: row.get(0)?,
            username: row.get(1)?,
            nonce: row.get(2)?,
            ciphertext: row.get(3)?,
            notes: row.get(4)?,
        })
    })?;

    let mut creds = Vec::new();
    for row in rows {
        creds.push(row?);
    }
    Ok(creds)
}

use rusqlite::{params, Connection, Result};

#[allow(dead_code)]
pub struct Credential {
    pub id: i32,
    pub label: String,
    pub username: String,
    pub nonce: String,      // base64-encoded nonce
    pub ciphertext: String, // base64-encoded encrypted data
    pub notes: Option<String>,
}
pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS credentials (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            label       TEXT NOT NULL,
            username    TEXT NOT NULL,
            nonce       TEXT NOT NULL,
            ciphertext  TEXT NOT NULL,
            notes       TEXT
        )",
        [],
    )?;
    Ok(())
}

pub fn add_credential(
    conn: &Connection,
    label: &str,
    username: &str,
    nonce: &str,
    ciphertext: &str,
    notes: Option<&str>,
) -> Result<()> {
    conn.execute(
        "INSERT INTO credentials (label, username, nonce, ciphertext, notes)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![label, username, nonce, ciphertext, notes],
    )?;
    Ok(())
}

pub fn get_credential_by_label(
    conn: &Connection,
    label: &str,
) -> Result<Option<Credential>> {
    let mut stmt = conn.prepare(
        "SELECT id, label, username, nonce, ciphertext, notes FROM credentials WHERE label = ?1 LIMIT 1"
    )?;

    let mut rows = stmt.query(params![label])?;
    if let Some(row) = rows.next()? {
        Ok(Some(Credential {
            id: row.get(0)?,
            label: row.get(1)?,
            username: row.get(2)?,
            nonce: row.get(3)?,
            ciphertext: row.get(4)?,
            notes: row.get(5)?,
        }))
    } else {
        Ok(None)
    }
}