use rusqlite::{params, Connection};
use std::path::Path;

use crate::error::NASError;

#[derive(Debug)]
pub struct NASDB(pub Connection);

impl NASDB {
    pub fn new() -> Result<Self, NASError> {
        let db_file = Path::new(&crate::CONFIG.fs_root).join("db.sqlite");
        let connection = Connection::open(db_file).map_err(|_| NASError::DBInitializationError)?;
        Ok(Self(connection))
    }

    pub fn connection(&self) -> &Connection {
        &self.0
    }
}

impl NASDB {
    pub fn init() -> Result<(), NASError> {
        let db = NASDB::new()?;
        let connection = db.0;

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS Users (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                username        TEXT NOT NULL,
                password_hash   TEXT NOT NULL
            )",
                params![],
            )
            .map_err(|_| NASError::DBInitializationError)?;

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS Sessions (
                id TEXT PRIMARY KEY,
                value TEXT UNIQUE
            )",
                params![],
            )
            .map_err(|_| NASError::DBInitializationError)?;

        Ok(())
    }
}
