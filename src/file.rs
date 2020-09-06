use anyhow::Result;
use serde::Serialize;
use std::convert::{AsRef, Into};
use std::path::{Path, PathBuf};

use crate::error::NASError;

const ROOT: &str = "/home/ozark/nas_root";

#[derive(Debug, Serialize)]
pub struct NASFile {
    pub name: String,
    pub absolute_path_str: String,
    pub relative_path_str: String,
    pub category: NASFileCategory,
    pub extension: String,
    pub size_bytes: u64,
}

impl NASFile {
    pub fn from_pathbuf(pathbuf: PathBuf) -> Result<Self, NASError> {
        let absolute_path_str = pathbuf.to_str().ok_or(NASError::UnsupportedPathError)?;
        let absolute_path_str = absolute_path_str.to_string();

        if !absolute_path_str.starts_with(ROOT) {
            return Err(NASError::InvalidPathError(absolute_path_str));
        }

        let relative_path_str = absolute_path_str
            .strip_prefix(&ROOT)
            .ok_or(NASError::InvalidPathError(absolute_path_str.to_string()))?;
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
        let relative_path_str = percent_encoding::percent_decode_str(&path)
            .decode_utf8()
            .map_err(|_| NASError::UnsupportedPathError)?;
        let relative_path_str = relative_path_str.to_string();

        let pathbuf = Path::new(ROOT).join(&relative_path_str);

        Self::from_pathbuf(pathbuf)
    }
}

impl NASFile {
    fn file_name(pathbuf: &PathBuf) -> Result<String, NASError> {
        let file_name = pathbuf.file_name().ok_or(NASError::UnsupportedPathError)?;
        let file_name = file_name.to_str().ok_or(NASError::UnsupportedPathError)?;
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

        let extension = pathbuf.extension().ok_or(NASError::UnsupportedPathError)?;
        let extension = extension.to_str().ok_or(NASError::UnsupportedPathError)?;
        Ok(extension.to_string())
    }

    fn size_bytes(pathbuf: &PathBuf) -> Result<u64, NASError> {
        if pathbuf.is_dir() {
            return Ok(0);
        }

        let size = pathbuf
            .metadata()
            .map_err(|_| NASError::UnsupportedPathError)?;
        let size = size.len();

        Ok(size)
    }
}

impl Into<PathBuf> for NASFile {
    fn into(self) -> PathBuf {
        PathBuf::new().join(self.relative_path_str)
    }
}

impl AsRef<Path> for NASFile {
    fn as_ref(&self) -> &Path {
        Path::new(&self.absolute_path_str)
    }
}

#[derive(Debug, Serialize)]
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
