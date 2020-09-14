use serde::{Deserialize, Serialize};

use crate::db::NASDB;
use crate::error::NASError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u8,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn login(username: &str, password_hash: &str) -> Result<User, NASError> {
        let db = NASDB::new()?;
        let db = db.connection();

        let mut db_query = db
            .prepare(
                "SELECT id, username, password_hash FROM Users
                WHERE username = ?1 AND password_hash = ?2",
            )
            .map_err(|_| NASError::UserReadError)?;
        let mut users = db_query
            .query_map(&[username, password_hash], |row| {
                Ok(User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    password_hash: row.get(2)?,
                })
            })
            .map_err(|_| NASError::UserReadError)?;

        let user = users.next().ok_or(NASError::UserValidationError {
            username: username.to_string(),
        })?;
        let user = user.map_err(|_| NASError::UserReadError)?;

        Ok(user)
    }
}
