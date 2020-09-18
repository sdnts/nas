use serde::Serialize;
use std::ffi::OsString;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum NASError {
    #[error("Unable to resolve FS Root for user {username:?} from {fs_root:?}")]
    FSRootResolutionError { fs_root: String, username: String },

    #[error("Unable to initialize NAS DB")]
    DBInitializationError,

    #[error("Unable to initialize NAS AppState: {error:?}")]
    AppStateInitializationError { error: String },

    #[error("Invalid credentials for {username:?}")]
    UserValidationError { username: String },

    #[error("Unable to read User from DB")]
    UserReadError,

    #[error("Unable to resolve relative path to {pathbuf:?}")]
    RelativePathResolutionError { pathbuf: PathBuf },

    #[error("Unable to resolve parent path for {pathbuf:?}")]
    ParentPathResolutionError { pathbuf: PathBuf },

    #[error("Unable to resolve file name for path {pathbuf:?}")]
    NASFileNameResolutionError { pathbuf: PathBuf },

    #[error("Unable to resolve file extension for path {pathbuf:?}")]
    NASFileExtensionResolutionError { pathbuf: PathBuf },

    #[error("You do not have permissions to access {pathbuf:?}")]
    PathAccessDenied { pathbuf: PathBuf },

    #[error("Unable to get file name for {pathbuf:?}")]
    FileNameError { pathbuf: PathBuf },

    #[error("Unable to compute extension for {pathbuf:?}")]
    FileExtensionError { pathbuf: PathBuf },

    #[error("Unable to compute size for {pathbuf:?}")]
    FileSizeError { pathbuf: PathBuf },

    #[error("Cannot convert {osstring:?} to UTF8 str")]
    OsStrConversionError { osstring: OsString },

    #[error("The path {pathbuf:?} does not exist")]
    NonExistentPath { pathbuf: PathBuf },

    #[error("Failed to render {template:?} template: {error:?}")]
    TemplateRenderError { template: String, error: String },

    #[error("Unable to read file or directory at path {pathbuf:?}")]
    PathReadError { pathbuf: PathBuf },

    #[error("Unable to calculate breadcrumbs for {pathbuf:?}")]
    BreadcrumbError { pathbuf: PathBuf },

    #[error("Unable to create file / directory at path {pathbuf:?}")]
    PathCreateError { pathbuf: PathBuf },

    #[error("Unable to create file / directory at path {pathbuf:?} because a file already exists at this path")]
    PathExistsError { pathbuf: PathBuf },

    #[error("Unable to rename file / directory at path {from:?} to {to:?}")]
    PathRenameError { from: PathBuf, to: PathBuf },

    #[error("Unable to delete file / directory at path {pathbuf:?}")]
    PathDeleteError { pathbuf: PathBuf },

    #[error("Unable to resolve Trash path {pathbuf:?}")]
    TrashPathResolutionError { pathbuf: PathBuf },

    #[error("IOError: {error:?}")]
    IOError { error: String },
}

impl actix_web::error::ResponseError for NASError {}

// impl From<io::Error> for NASError {
//     fn from(source: io::Error) -> Self {
//         Self::IOError { source }
//     }
// }
