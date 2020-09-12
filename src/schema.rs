use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::db::NASDB;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u8,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn login(username: &str, password_hash: &str) -> Result<User> {
        let db = NASDB::new()?;
        let db = db.connection();

        let mut db_query = db.prepare(
            "SELECT id, username, password_hash FROM Users
                WHERE username = ?1 AND password_hash = ?2",
        )?;
        let mut users = db_query.query_map(&[username, password_hash], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
            })
        })?;

        let user = users
            .next()
            .ok_or(anyhow!("[schema::User::login] Invalid credentials"))?;
        let user = user?;

        Ok(user)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: u8,
    pub value: String,
}

impl Session {
    pub fn create(id: &str, value: &str) -> Result<Session> {
        let db = NASDB::new()?;
        let db = db.connection();

        let mut db_query = db
            .prepare(
                "INSERT INTO Sessions (id,value)
                VALUES (?1,?2)",
            )
            .with_context(|| {
                anyhow!("[schema::Session::create] Failure during DB query preparation")
            })?;

        let mut sessions = db_query
            .query_map(&[id, value], |row| {
                Ok(Session {
                    id: row.get(0)?,
                    value: value.to_string(),
                })
            })
            .with_context(|| {
                anyhow!("[schema::Session::create] Failure during DB Query execution")
            })?;

        let session = sessions.next().ok_or(anyhow!(
            "[schema::Session::create] Failure during session creation"
        ))?;
        let session = session.with_context(|| {
            anyhow!("[schema::Session::create] Failure during session creation")
        })?;

        Ok(session)
    }

    pub fn find_by_value(value: &str) -> Result<Session> {
        let db = NASDB::new()?;
        let db = db.connection();

        let mut db_query = db
            .prepare(
                "SELECT id, value FROM Sessions
                WHERE value = ?1",
            )
            .with_context(|| {
                anyhow!("[schema::Session::find_by_value Failure during DB query preparation")
            })?;

        let mut sessions = db_query
            .query_map(&[value], |row| {
                Ok(Session {
                    id: row.get(0)?,
                    value: row.get(1)?,
                })
            })
            .with_context(|| {
                anyhow!("[schema::Session::find_by_value] Failure during DB query execution")
            })?;

        let session = sessions.next().ok_or(anyhow!(
            "[schema::Session::find_by_value] Unable to find session"
        ))?;
        let session = session
            .with_context(|| anyhow!("[schema::Session::find_by_value] Unable to find session"))?;

        Ok(session)
    }
}
