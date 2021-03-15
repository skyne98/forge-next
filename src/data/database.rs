use std::path::PathBuf;

use anyhow::{Context, Result};
use rusqlite::Connection;

pub struct Database {
    path: PathBuf,
    connection: Connection,
}

impl Database {
    pub fn open() -> Result<Self> {
        let user_dirs =
            directories::UserDirs::new().context("user directories are not available")?;
        let documents_dir = user_dirs
            .document_dir()
            .context("cannot find a user documents directory")?;
        let forge_dir = documents_dir.join("forge");
        let database_file = forge_dir.join("data.db");

        let connection = Connection::open(&database_file)
            .context(format!("failed to open the database at {}", database_file))?;

        Ok(Database {
            path: database_file,
            connection,
        })
    }
}
