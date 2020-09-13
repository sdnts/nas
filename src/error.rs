use std::ffi::OsString;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NASError {
    #[error("The PathBuf {pathbuf:?} is invalid")]
    InvalidPathBuf { pathbuf: PathBuf },

    #[error("You do not have permissions to access {pathbuf:?}")]
    PathAccessDenied { pathbuf: PathBuf },

    #[error("Unable to get file name for {pathbuf:?}")]
    FileNameError { pathbuf: PathBuf },

    #[error("Unable to compute extension for {pathbuf:?}")]
    FileExtensionError { pathbuf: PathBuf },

    #[error("Unable to compute size for {pathbuf:?}")]
    FileSizeError { pathbuf: PathBuf },

    #[error("Cannot convert {osstring:?} to valid str")]
    OsStrConversionError { osstring: OsString },

    #[error("The path {path:?} does not exist")]
    NonExistentPath { path: String },

    #[error("Failed to render {template:?} template")]
    TemplateRenderError { template: &'static str },

    #[error("Unable to read file or directory at path {path:?}")]
    PathReadError { path: String },

    #[error("Unable to calculate breadcrumbs for {pathbuf:?}")]
    BreadcrumbError { pathbuf: PathBuf },

    #[error("Unable to create file / directory at path{pathbuf:?}")]
    PathCreateError { pathbuf: PathBuf },

    #[error("Encountered an anyhow error")]
    AnyhowError,
}

impl From<anyhow::Error> for NASError {
    fn from(e: anyhow::Error) -> Self {
        NASError::AnyhowError
    }
}

impl actix_web::error::ResponseError for NASError {}

pub enum AuthError {}

pub enum StreamError {}
