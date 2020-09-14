use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StreamgenError {
    #[error("Failed to resolve path {path:?}")]
    PathCanonicalizeError { path: PathBuf },

    #[error("The path does not exist: {path:?}")]
    NonExistentPath { path: PathBuf },

    #[error("The provided path cannot be resolved as a file: {pathbuf:?}")]
    FileResolutionError { pathbuf: PathBuf },

    #[error("Unable to resolve parent directory for path {path:?}")]
    ParentDirResolutionError { path: PathBuf },

    #[error("Unable to resolve stream directory for path {path:?}")]
    StreamDirResolutionError { path: PathBuf },

    #[error("Unable to prepare directories for stream segment generation: {reason:?}")]
    PreparationError { reason: String },

    #[error("Unable to resolve hls_base_url for path {pathbuf:?}")]
    HlsBaseUrlResolutionError { pathbuf: PathBuf },

    #[error("Encountered an error during stream generation: {source:?}")]
    FfmpegError { source: std::io::Error },
}
