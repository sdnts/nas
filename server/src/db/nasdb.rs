use rusqlite::{Connection, NO_PARAMS};
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

        // Create a `Users` table
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS Users (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                username        TEXT NOT NULL,
                password_hash   TEXT NOT NULL
            )",
                NO_PARAMS,
            )
            .map_err(|_| NASError::DBInitializationError)?;

        // Check if a `root` user exists, if not, create one
        let mut db_query = connection
            .prepare("SELECT id FROM Users WHERE username = \"root\"")
            .map_err(|_| NASError::DBInitializationError)?;
        let mut users = db_query
            .query(NO_PARAMS)
            .map_err(|_| NASError::DBInitializationError)?;
        let root_user = users.next().map_err(|_| NASError::DBInitializationError)?;

        if root_user.is_none() {
            // Create a root user with password `nas`
            connection
                .execute(
                    "INSERT INTO Users (username, password_hash)
                    VALUES (
                        \"root\", 
                        \"62b11abefc9c6070df5dfcd59a8a72ef067660957cba0d0508e9bf38b6f3a627d28208682d6b0a9c809c48047512fe9d2e3751c43093300d5fe4e846494a54bc\"
                    )",
                    NO_PARAMS,
                )
                .map_err(|_| NASError::DBInitializationError)?;
        }

        Ok(())
    }
}
