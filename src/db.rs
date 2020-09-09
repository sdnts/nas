use anyhow::*;
use rusqlite::{params, Connection};
use serde::Serialize;
use sha2::{Digest, Sha512};
use std::path::Path;
use std::str;

#[derive(Clone, Debug, Serialize)]
pub struct User {
    id: u8,
    username: String,
    password_hash: String,
}

pub struct NASDB {
    connection: Connection,
}

impl NASDB {
    pub fn new() -> Result<Self> {
        let db_file = Path::new(&crate::CONFIG.fs_root).join("db.sqlite");
        let connection = Connection::open(db_file)?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS User (
                  id              INTEGER PRIMARY KEY AUTOINCREMENT,
                  username        TEXT NOT NULL,
                  password_hash   TEXT NOT NULL
                  )",
            params![],
        )?;

        Ok(Self { connection })
    }
}

impl NASDB {
    pub fn login(&self, username: &str, password: &str) -> Result<User> {
        let mut hasher = Sha512::new();
        hasher.update(password);
        let password_hash = hasher.finalize();
        let password_hash = password_hash.as_slice().to_vec();
        let password_hash = hex::encode(&password_hash);

        let mut stmt = self.connection.prepare(
            "SELECT id,username,password_hash FROM User
                WHERE username = ?1 AND password_hash = ?2",
        )?;

        let users: Vec<Result<User, rusqlite::Error>> = stmt
            .query_map(&[username, &password_hash], |row| {
                Ok(User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    password_hash: row.get(2)?,
                })
            })?
            .collect();

        let user = users[0]
            .as_ref()
            .map_err(|_| anyhow!("[db::login] Invalid credentials"))?
            .to_owned();

        Ok(user)
    }
}
