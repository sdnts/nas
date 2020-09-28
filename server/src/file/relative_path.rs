use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::error::NASError;
use crate::file::AbsolutePath;
use crate::CONFIG;

/// `RelativePath` exists only to be able to co-relate a fs/* path to a real file on the file system. It does not ACTUALLY point to the file
/// It is not expected to be able to perform any real operations on the file itself via this struct.
/// Convert to `AbsolutePath` to be able to do those things.
#[derive(Eq, Serialize, Deserialize, Debug)]
pub struct RelativePath {
    pathbuf: PathBuf,
    username: String,
}

impl RelativePath {
    pub fn new(relative_path_str: &str, username: &str) -> Self {
        let pathbuf = PathBuf::new().join(relative_path_str);

        Self {
            pathbuf,
            username: String::from(username),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}

impl TryFrom<AbsolutePath> for RelativePath {
    type Error = NASError;

    fn try_from(absolute_path: AbsolutePath) -> Result<Self, Self::Error> {
        let fs_root = &CONFIG.fs_root;

        let absolute_path: PathBuf = absolute_path.into();
        let relative_path = absolute_path.strip_prefix(fs_root).map_err(|_| {
            NASError::RelativePathResolutionError {
                pathbuf: absolute_path.to_owned(),
            }
        })?;

        let mut path_components = relative_path.components();
        let username = path_components
            .next()
            .ok_or(NASError::RelativePathResolutionError {
                pathbuf: absolute_path.to_owned(),
            })?;
        let username =
            username
                .as_os_str()
                .to_str()
                .ok_or(NASError::RelativePathResolutionError {
                    pathbuf: absolute_path.to_owned(),
                })?;
        let relative_path = path_components.as_path().to_owned();

        Ok(Self {
            pathbuf: relative_path,
            username: username.to_string(),
        })
    }
}

impl From<&RelativePath> for OsString {
    fn from(path: &RelativePath) -> OsString {
        path.pathbuf.as_os_str().to_os_string()
    }
}

// `impl From<RelativePath> for PathBuf` does not exist because we never want to give out a PathBuf for a RelativePath
// This is because that PathBuf can never point to a real file on the file system.
// Convert to `AbsolutePath` to get a PathBuf reference and do any real operations on the file (with AbsolutePath's guarantees)
//
// Instead, we provide `impl From<RelativePath> for OsString` as above

// `impl NASFile` does not exist because we never want people to be able to get file information from a relative path
// (because the actual file cannot be resolved from a relative path)

// `impl From<RelativePath> for String` and `impl From<RelativePath> for &str` do not exist because it is not guaranteed that paths will be valid UTF8

impl PartialEq for RelativePath {
    fn eq(&self, other: &RelativePath) -> bool {
        // If the relative pathbuf & the username are the same, the absolute paths will also be the same
        self.pathbuf == other.pathbuf && self.username == other.username
    }
}
