use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::ffi::OsString;
use std::fmt;
use std::path::PathBuf;

use crate::error::NASError;
use crate::file::{NASFile, NASFileCategory, RelativePath};
use crate::CONFIG;

#[derive(Eq, Serialize, Deserialize, Debug)]
pub struct AbsolutePath {
    pathbuf: PathBuf,
}

impl AbsolutePath {
    pub fn user_fs_root(username: &str) -> PathBuf {
        PathBuf::new().join(&CONFIG.fs_root).join(username)
    }
}

// Cast PathBuf -> AbsolutePath
impl TryFrom<PathBuf> for AbsolutePath {
    type Error = NASError;

    fn try_from(pathbuf: PathBuf) -> Result<Self, Self::Error> {
        if pathbuf.exists() {
            Ok(Self { pathbuf })
        } else {
            Err(NASError::NonExistentPath { pathbuf })
        }
    }
}

// Cast &str -> AbsolutePath
impl TryFrom<&str> for AbsolutePath {
    type Error = NASError;

    fn try_from(absolute_path_str: &str) -> Result<Self, Self::Error> {
        let pathbuf = PathBuf::new().join(absolute_path_str);
        Self::try_from(pathbuf)
    }
}

// Cast &OsString -> AbsolutePath
impl TryFrom<&OsString> for AbsolutePath {
    type Error = NASError;

    fn try_from(absolute_path_str: &OsString) -> Result<Self, Self::Error> {
        let pathbuf = PathBuf::new().join(absolute_path_str);
        Self::try_from(pathbuf)
    }
}

// `impl From<AbsolutePath> for String` and `impl From<AbsolutePath> for &str` do not exist because it is not guaranteed that paths will be valid UTF8

// Cast &RelativePath -> AbsolutePath
impl TryFrom<&RelativePath> for AbsolutePath {
    type Error = NASError;

    fn try_from(relative_path: &RelativePath) -> Result<Self, Self::Error> {
        let fs_root = &CONFIG.fs_root;
        let username = String::from(relative_path.username());

        let relative_path = OsString::from(relative_path);
        let relative_path = PathBuf::from(relative_path);

        let pathbuf = PathBuf::new()
            .join(fs_root)
            .join(username)
            .join(relative_path);

        Self::try_from(pathbuf)
    }
}

// Cast AbsolutePath -> PathBuf
impl From<AbsolutePath> for PathBuf {
    fn from(absolute_path: AbsolutePath) -> PathBuf {
        absolute_path.pathbuf
    }
}

impl NASFile for AbsolutePath {
    fn pathbuf(&self) -> &PathBuf {
        &self.pathbuf
    }
}

impl fmt::Display for AbsolutePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pathbuf.display())
    }
}

impl PartialEq for AbsolutePath {
    fn eq(&self, other: &AbsolutePath) -> bool {
        self.pathbuf == other.pathbuf
    }
}

impl Ord for AbsolutePath {
    fn cmp(&self, other: &AbsolutePath) -> Ordering {
        let name = self.name().unwrap_or_default();
        let other_name = other.name().unwrap_or_default();

        let category = self.category().unwrap_or(NASFileCategory::Unknown);
        let other_category = other.category().unwrap_or(NASFileCategory::Unknown);

        if matches!(category, NASFileCategory::Directory)
            && matches!(other_category, NASFileCategory::Directory)
        {
            // If both are directories, sort alphabetically
            name.cmp(&other_name)
        } else if matches!(category, NASFileCategory::Directory) {
            // If you are a directory, but the other isn't , you will always be above (less)
            Ordering::Less
        } else if matches!(other_category, NASFileCategory::Directory) {
            // If other is a directory, but you aren't , you will always be below (greater)
            Ordering::Greater
        } else {
            // If neither is a directory, sort alphabetically
            name.cmp(&other_name)
        }
    }
}

impl PartialOrd for AbsolutePath {
    fn partial_cmp(&self, other: &AbsolutePath) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
