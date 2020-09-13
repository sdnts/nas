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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: u8,
    pub value: String,
}

impl Session {
    pub fn create(id: &str, value: &str) -> Result<Session, NASError> {
        let db = NASDB::new()?;
        let db = db.connection();

        let mut db_query = db
            .prepare(
                "INSERT INTO Sessions (id,value)
                VALUES (?1,?2)",
            )
            .map_err(|_| NASError::SessionCreateError)?;

        let mut sessions = db_query
            .query_map(&[id, value], |row| {
                Ok(Session {
                    id: row.get(0)?,
                    value: value.to_string(),
                })
            })
            .map_err(|_| NASError::SessionCreateError)?;

        let session = sessions.next().ok_or(NASError::SessionCreateError)?;
        let session = session.map_err(|_| NASError::SessionCreateError)?;

        Ok(session)
    }

    pub fn find_by_value(value: &str) -> Result<Session, NASError> {
        let db = NASDB::new()?;
        let db = db.connection();

        let mut db_query = db
            .prepare(
                "SELECT id, value FROM Sessions
                WHERE value = ?1",
            )
            .map_err(|_| NASError::SessionReadError)?;

        let mut sessions = db_query
            .query_map(&[value], |row| {
                Ok(Session {
                    id: row.get(0)?,
                    value: row.get(1)?,
                })
            })
            .map_err(|_| NASError::SessionReadError)?;

        let session = sessions.next().ok_or(NASError::SessionReadError)?;
        let session = session.map_err(|_| NASError::SessionReadError)?;

        Ok(session)
    }
}
