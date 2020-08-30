use anyhow::Result;
use std::path;

use crate::error::NASError;

const ROOT: &str = "/home/ozark/nas";

pub struct NASPath {
    path: std::path::PathBuf,
}

impl NASPath {
    pub fn from_pathbuf(path: path::PathBuf) -> Self {
        Self { path }
    }

    pub fn from_relative_path_str(path: &str) -> Result<Self> {
        let relative_path = percent_encoding::percent_decode_str(path).decode_utf8()?;

        Ok(Self {
            path: path::Path::new(ROOT).join(&*relative_path),
        })
    }

    pub fn to_pathbuf(&self) -> &path::PathBuf {
        &self.path
    }

    pub fn to_relative_path_str(&self) -> Result<&str> {
        let path = self
            .path
            .strip_prefix(&ROOT)
            .map_err(|_| NASError::UnknownError("Could not strip FS prefix".to_string()))?;

        let path = path.to_str().ok_or(NASError::InvalidPathError(
            "File path is not valid unicode".to_string(),
        ))?;

        Ok(path)
    }

    pub fn to_absolute_path_str(&self) -> Result<&str> {
        let path = self.path.to_str().ok_or(NASError::InvalidPathError(
            "File path is not valid unicode".to_string(),
        ))?;

        Ok(path)
    }
}
