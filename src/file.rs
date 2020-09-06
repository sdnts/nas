use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::error::NASError;

pub use crate::file_type::NASFileType;

const ROOT: &str = "/home/ozark/nas_root";

/// Represents a file in the NAS
///
/// Absolute paths are paths to the file in the host OS
/// Relative paths are paths to the file in the NAS
#[derive(Debug)]
pub(crate) struct NASFile {
    pub path: PathBuf,
    pub file_type: NASFileType,
}

impl NASFile {
    pub fn from_relative_path_str(path: &str) -> Result<Self, NASError> {
        let relative_path = percent_encoding::percent_decode_str(&path).decode_utf8();
        if let Err(_) = relative_path {
            return Err(NASError::InvalidPathError(path.to_string()));
        }

        let relative_path = relative_path.unwrap();
        let absolute_path = Path::new(ROOT).join(&*relative_path);

        if !absolute_path.exists() {
            return Err(NASError::InvalidPathError(path.to_string()));
        }

        let file_type = NASFile::file_type(&absolute_path);

        Ok(Self {
            path: absolute_path,
            file_type,
        })
    }

    pub fn from_absolute_path_str(path: &str) -> Result<Self, NASError> {
        let absolute_path_str = percent_encoding::percent_decode_str(path).decode_utf8();
        if let Err(_) = absolute_path_str {
            return Err(NASError::InvalidPathError(path.to_string()));
        }

        let absolute_path_str = absolute_path_str.unwrap();
        if !absolute_path_str.starts_with(ROOT) {
            return Err(NASError::InvalidPathError(path.to_string()));
        }

        let absolute_path_str = absolute_path_str.to_string();
        let absolute_path = Path::new(&absolute_path_str);
        let absolute_pathbuf = absolute_path.to_path_buf();
        let file_type = NASFile::file_type(&absolute_pathbuf);

        Ok(Self {
            path: absolute_pathbuf,
            file_type,
        })
    }
}

impl NASFile {
    pub fn to_relative_path_str(&self) -> Result<&str> {
        let path = self
            .path
            .strip_prefix(&ROOT)
            .map_err(|_| NASError::UnknownError("Could not strip FS prefix".to_string()))?;

        let path = path.to_str().ok_or(NASError::UnsupportedPathError)?;

        Ok(path)
    }

    pub fn to_absolute_path_str(&self) -> Result<&str> {
        let path = self.path.to_str().ok_or(NASError::UnsupportedPathError)?;

        Ok(path)
    }
}

// Static Methods
impl NASFile {
    fn file_type(pathbuf: &PathBuf) -> NASFileType {
        let is_dir = pathbuf.is_dir();
        let extension = pathbuf.extension();

        if is_dir {
            NASFileType::Directory
        } else if let Some(e) = extension {
            if let Some(e) = e.to_str() {
                match e {
                    "mp3" => NASFileType::Audio,

                    "avi" => NASFileType::Video,
                    "mkv" => NASFileType::Video,
                    "mp4" => NASFileType::Video,

                    "m3u8" => NASFileType::StreamPlaylist,
                    "ts" => NASFileType::StreamSegment,

                    "pdf" => NASFileType::Document,

                    "png" => NASFileType::Image,
                    "jpg" => NASFileType::Image,
                    "jpeg" => NASFileType::Image,
                    "webp" => NASFileType::Image,

                    _ => NASFileType::Unknown,
                }
            } else {
                NASFileType::Unknown
            }
        } else {
            NASFileType::Unknown
        }
    }
}
