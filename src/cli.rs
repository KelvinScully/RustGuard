use clap::{Parser, Subcommand};

/// RustGuard: Local, offline password manager
#[derive(Debug, Parser)]
#[command(name = "rustguard", about = "Manage your passwords securely from the CLI")]
pub struct Cli {
    /// Path to the encrypted store file
    #[arg(short, long, default_value = "store.db")]
    pub store: String,

    /// Master password (if omitted, will prompt securely)
    #[arg(short, long)]
    pub master: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a new credential
    Add {
        /// Label for the credential (e.g., github)
        label: String,
        /// Username or email
        user: String,
        /// Password (if omitted, CLI will prompt)
        #[arg(short, long)]
        password: Option<String>,
        /// Notes or URL
        #[arg(short, long)]
        notes: Option<String>,
    },

    /// Retrieve and decrypt a credential by label
    Get {
        /// Label to fetch
        label: String,
    },

    /// Delete a credential by label
    Delete {
        /// Label to remove
        label: String,
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// List all stored labels
    List,

    /// Generate a random password
    Generate {
        /// Desired length (default: 16)
        #[arg(short, long, default_value_t = 16)]
        length: usize,
        /// Include symbols
        #[arg(short, long)]
        symbols: bool,
    },

    /// Export the store to a file (encrypted)
    Export {
        /// Output path
        path: String,
    },

    /// Import a store from a file (encrypted)
    Import {
        /// Source path
        path: String,
    },
}