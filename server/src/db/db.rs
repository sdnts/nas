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

        if let None = root_user {
            // Create a root user with password `nas`
            connection
                .execute(
                    "INSERT INTO Users (username, password_hash)
                    VALUES (
                        \"root\", 
                        \"62B11ABEFC9C6070DF5DFCD59A8A72EF067660957CBA0D0508E9BF38B6F3A627D28208682D6B0A9C809C48047512FE9D2E3751C43093300D5FE4E846494A54BC\"
                    )",
                    NO_PARAMS,
                )
                .map_err(|_| NASError::DBInitializationError)?;
        }

        Ok(())
    }
}
