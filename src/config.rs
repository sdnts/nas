use serde::Serialize;

use crate::db::User;

#[derive(Debug, Serialize)]
pub enum NASTheme {
    Light,
    Dark,
    Auto,
}

#[derive(Debug, Serialize)]
pub struct NASConfig {
    pub fs_root: String,
    pub cookie_secret: String,
    pub hostname: String,
    pub theme: NASTheme,
    pub user: Option<User>,
}
