use serde::Serialize;
use std::fs;
use std::path::PathBuf;

use crate::db::NASDB;
use crate::error::NASError;

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize)]
pub enum NASTheme {
    Light,
    Dark,
}

#[derive(Debug, Serialize)]
pub struct NASConfig {
    pub fs_root: PathBuf,
    pub cookie_secret: String,
    pub theme: NASTheme,
}

impl NASConfig {
    pub fn init() -> Result<(), NASError> {
        // Initialize the database
        NASDB::init().map_err(|_| NASError::DBInitializationError)?;

        // Create a directory for the root if it does not already exist
        let root_user_dir = crate::CONFIG.fs_root.join("root");
        if !root_user_dir.exists() {
            fs::create_dir(&root_user_dir)?;
        }

        Ok(())
    }
}

impl Default for NASConfig {
    fn default() -> Self {
        Self {
            fs_root: PathBuf::new().join("/home/ozark/nas_root"),
            cookie_secret: "012345678901234567890123456789012".to_string(),
            theme: NASTheme::Dark,
        }
    }
}
