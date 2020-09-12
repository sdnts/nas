use anyhow::*;
use async_trait::async_trait;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::marker::Send;
use std::sync::{Arc, Mutex};
use tide::sessions::SessionStore;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NASSessionMiddleware {}

// impl NASSessionMiddleware {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

// #[async_trait]
// impl<State: Send + Sync + Clone + 'static> Middleware<State> for NASSessionMiddleware {
//     async fn handle(&self, req: tide::Request<State>, next: tide::Next<'_, State>) -> tide::Result {
//         let path = req.url().path();
//         let session = req.session();

//         dbg!(session);

//         if path == "/auth" || path.starts_with("/public") {
//             // Unprotected route, carry on
//             Ok(next.run(req).await)
//         // } else if let Some(user_id) = &self.user_id {
//         //     // Protected route, but valid session
//         //     Ok(next.run(req).await)
//         } else {
//             // Protected route, and invalid session
//             Ok(tide::Redirect::new("/auth").into())
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct NASSessionStore();

unsafe impl Send for NASSessionStore {}
unsafe impl Sync for NASSessionStore {}

impl NASSessionStore {
    pub fn new() -> Self {
        Self()
    }
}

#[async_trait]
impl SessionStore for NASSessionStore {
    async fn load_session(&self, cookie_value: String) -> Result<Option<tide::sessions::Session>> {
        let db_session = crate::schema::Session::find_by_value(&cookie_value).with_context(|| {
            anyhow!("[session::SessionStore::load_session] Unable to find a matching session")
        });

        if let Ok(db_session) = db_session {
            let mut session = tide::sessions::Session::new();
            session.set_cookie_value(db_session.value);

            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    async fn store_session(&self, session: tide::sessions::Session) -> Result<Option<String>> {
        let id = session.id().to_string();
        let value = session.into_cookie_value().ok_or(anyhow!(
            "[SessionStore::store_session] Unable to compute session value"
        ))?;
        let session = crate::schema::Session::create(&id, &value).with_context(|| {
            anyhow!("[session::SessionStore::store_session] Unable to create a session")
        })?;
        Ok(Some(session.value))
    }

    async fn destroy_session(&self, session: tide::sessions::Session) -> Result<()> {
        println!("destroy_session {:?}", session);
        Ok(())
    }

    async fn clear_store(&self) -> Result<()> {
        println!("clear_store");
        Ok(())
    }
}
