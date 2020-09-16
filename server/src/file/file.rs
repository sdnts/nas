use std::ffi::OsString;
use std::path::PathBuf;

use crate::error::NASError;
use crate::file::NASFileCategory;

pub trait NASFile {
    fn pathbuf(&self) -> &PathBuf;

    fn name(&self) -> Result<OsString, NASError> {
        let pathbuf = self.pathbuf();

        let filename = pathbuf
            .file_name()
            .ok_or(NASError::NASFileNameResolutionError {
                pathbuf: pathbuf.to_owned(),
            })?;

        Ok(filename.to_os_string())
    }

    fn parent_name(&self) -> Result<OsString, NASError> {
        let pathbuf = self.pathbuf();

        let parent_path = pathbuf
            .parent()
            .ok_or(NASError::NASFileNameResolutionError {
                pathbuf: pathbuf.to_owned(),
            })?;
        let parent_name = parent_path
            .file_name()
            .ok_or(NASError::NASFileNameResolutionError {
                pathbuf: pathbuf.to_owned(),
            })?;

        Ok(parent_name.to_os_string())
    }

    fn extension(&self) -> Result<OsString, NASError> {
        let pathbuf = self.pathbuf();

        if pathbuf.is_dir() {
            Ok(OsString::from(""))
        } else {
            let extension =
                pathbuf
                    .extension()
                    .ok_or(NASError::NASFileExtensionResolutionError {
                        pathbuf: pathbuf.to_owned(),
                    })?;

            Ok(extension.to_os_string())
        }
    }

    fn size_bytes(&self) -> Result<u64, NASError> {
        let pathbuf = self.pathbuf();

        if pathbuf.is_dir() {
            Ok(0)
        } else {
            let size = pathbuf.metadata().map_err(|_| NASError::FileSizeError {
                pathbuf: pathbuf.to_owned(),
            })?;
            let size = size.len();

            Ok(size)
        }
    }

    fn category(&self) -> Result<NASFileCategory, NASError> {
        let pathbuf = self.pathbuf();
        let extension = self.extension()?;

        if pathbuf.is_dir() {
            Ok(NASFileCategory::Directory)
        } else {
            Ok(NASFileCategory::Unknown)
            // match OsString::from(extension) {
            //     OsString::from("mp3") => Ok(NASFileCategory::Audio),

            //     "avi" => Ok(NASFileCategory::Video),
            //     "mkv" => Ok(NASFileCategory::Video),
            //     "mp4" => Ok(NASFileCategory::Video),

            //     "m3u8" => Ok(NASFileCategory::StreamPlaylist),
            //     "ts" => Ok(NASFileCategory::StreamSegment),

            //     "pdf" => Ok(NASFileCategory::Document),
            //     "txt" => Ok(NASFileCategory::Document),

            //     "png" => Ok(NASFileCategory::Image),
            //     "jpg" => Ok(NASFileCategory::Image),
            //     "jpeg" => Ok(NASFileCategory::Image),
            //     "webp" => Ok(NASFileCategory::Image),

            //     _ => Ok(NASFileCategory::Unknown),
            // }
        }
    }
}
