use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StreamgenError {
    #[error("Failed to resolve path {path:?}")]
    PathCanonicalizeError { path: PathBuf },

    #[error("The path does not exist: {path:?}")]
    NonExistentPath { path: PathBuf },

    #[error("Unable to resolve parent directory for path {path:?}")]
    ParentDirResolutionError { path: PathBuf },

    #[error("Unable to resolve relative parent directory for path {path:?}")]
    RelativeParentDirResolutionError { path: PathBuf, fs_root: PathBuf },

    #[error("The provided path cannot be resolved as a file: {pathbuf:?}")]
    FileResolutionError { pathbuf: PathBuf },

    #[error("Unable to prepare directories for stream segment generation: {reason:?}")]
    PreparationError { reason: String },

    #[error("Encountered an error during stream generation: {source:?}")]
    FfmpegError { source: std::io::Error },
}
