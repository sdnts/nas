use serde::Serialize;
use std::cmp::Ordering;
use std::convert::{AsRef, Into};
use std::path::{Path, PathBuf};

use crate::error::NASError;

#[derive(Debug, Serialize, Eq, Ord)]
pub struct NASFile {
    pub name: String,
    pub relative_path_str: String,
    pub absolute_path_str: String,
    pub category: NASFileCategory,
    pub extension: String,
    pub size_bytes: u64,
}

impl NASFile {
    pub fn from_pathbuf(pathbuf: PathBuf) -> Result<Self, NASError> {
        if !pathbuf.starts_with(&crate::CONFIG.fs_root) {
            return Err(NASError::PathAccessDenied { pathbuf: pathbuf });
        }

        let absolute_path_str = pathbuf.to_str().ok_or(NASError::InvalidPathBuf {
            pathbuf: pathbuf.to_owned(),
        })?;
        let absolute_path_str = absolute_path_str.to_string();

        let relative_path_str = absolute_path_str
            .strip_prefix(&crate::CONFIG.fs_root)
            .ok_or(NASError::PathAccessDenied {
                pathbuf: pathbuf.to_owned(),
            })?;
        let relative_path_str = relative_path_str.to_string();

        let name = NASFile::file_name(&pathbuf)?;
        let category = NASFile::category(&pathbuf);
        let extension = NASFile::extension(&pathbuf)?;
        let size_bytes = NASFile::size_bytes(&pathbuf)?;

        Ok(Self {
            name,
            absolute_path_str,
            relative_path_str,
            category,
            extension,
            size_bytes,
        })
    }

    pub fn from_relative_path_str(path: &str) -> Result<Self, NASError> {
        let relative_path_str = path.to_string();
        let pathbuf = Path::new(&crate::CONFIG.fs_root).join(&relative_path_str);

        Self::from_pathbuf(pathbuf)
    }
}

impl NASFile {
    fn file_name(pathbuf: &PathBuf) -> Result<String, NASError> {
        let file_name = pathbuf.file_name().ok_or(NASError::FileNameError {
            pathbuf: pathbuf.to_owned(),
        })?;
        let file_name = file_name.to_str().ok_or(NASError::OsStrConversionError {
            osstring: file_name.to_os_string(),
        })?;
        Ok(file_name.to_string())
    }

    fn category(pathbuf: &PathBuf) -> NASFileCategory {
        let is_dir = pathbuf.is_dir();
        let extension = pathbuf.extension();

        if is_dir {
            NASFileCategory::Directory
        } else if let Some(e) = extension {
            if let Some(e) = e.to_str() {
                match e {
                    "mp3" => NASFileCategory::Audio,

                    "avi" => NASFileCategory::Video,
                    "mkv" => NASFileCategory::Video,
                    "mp4" => NASFileCategory::Video,

                    "m3u8" => NASFileCategory::StreamPlaylist,
                    "ts" => NASFileCategory::StreamSegment,

                    "pdf" => NASFileCategory::Document,
                    "txt" => NASFileCategory::Document,

                    "png" => NASFileCategory::Image,
                    "jpg" => NASFileCategory::Image,
                    "jpeg" => NASFileCategory::Image,
                    "webp" => NASFileCategory::Image,

                    _ => NASFileCategory::Unknown,
                }
            } else {
                NASFileCategory::Unknown
            }
        } else {
            NASFileCategory::Unknown
        }
    }

    fn extension(pathbuf: &PathBuf) -> Result<String, NASError> {
        if pathbuf.is_dir() {
            return Ok("".to_string());
        }

        let extension = pathbuf.extension().ok_or(NASError::FileExtensionError {
            pathbuf: pathbuf.to_owned(),
        })?;
        let extension = extension.to_str().ok_or(NASError::OsStrConversionError {
            osstring: extension.to_os_string(),
        })?;
        Ok(extension.to_string())
    }

    fn size_bytes(pathbuf: &PathBuf) -> Result<u64, NASError> {
        if pathbuf.is_dir() {
            return Ok(0);
        }

        let size = pathbuf.metadata().map_err(|_| NASError::FileSizeError {
            pathbuf: pathbuf.to_owned(),
        })?;
        let size = size.len();

        Ok(size)
    }

    pub fn relative_to_absolute_str(path: &str) -> Result<String, NASError> {
        let pathbuf = Path::new(&crate::CONFIG.fs_root).join(path);
        let path_str = pathbuf.to_str().ok_or(NASError::InvalidPathBuf {
            pathbuf: pathbuf.to_owned(),
        })?;

        Ok(path_str.to_string())
    }
}

impl Into<PathBuf> for NASFile {
    fn into(self) -> PathBuf {
        PathBuf::new().join(self.absolute_path_str)
    }
}

impl AsRef<Path> for NASFile {
    fn as_ref(&self) -> &Path {
        Path::new(&self.absolute_path_str)
    }
}

impl PartialEq for NASFile {
    fn eq(&self, other: &NASFile) -> bool {
        let pathbuf = &self.absolute_path_str;
        let other_pathbuf = &other.absolute_path_str;

        pathbuf == other_pathbuf
    }
}

impl PartialOrd for NASFile {
    fn partial_cmp(&self, other: &NASFile) -> Option<Ordering> {
        if matches!(self.category, NASFileCategory::Directory)
            && matches!(other.category, NASFileCategory::Directory)
        {
            self.name
                .to_lowercase()
                .partial_cmp(&other.name.to_lowercase())
        } else if matches!(self.category, NASFileCategory::Directory) {
            Some(Ordering::Less)
        } else if matches!(other.category, NASFileCategory::Directory) {
            Some(Ordering::Greater)
        } else {
            self.name
                .to_lowercase()
                .partial_cmp(&other.name.to_lowercase())
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Eq, Ord)]
pub enum NASFileCategory {
    Directory,
    Audio,
    Video,
    StreamPlaylist,
    StreamSegment,
    Document,
    Image,
    Unknown,
}

impl PartialOrd for NASFileCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if matches!(self, Self::Directory) && matches!(other, Self::Directory) {
            Some(Ordering::Less)
        } else if matches!(self, Self::Directory) && matches!(other, Self::Directory) {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}
