use std::fmt;

#[derive(Debug)]
pub(crate) enum NASError {
    // FileNotFoundError(String),
    DirectoryNotFoundError(String),
    InvalidPathError(String),
    InvalidExtensionError(String),

    InvalidStreamPlaylist(String),
    MissingStreamPlaylist,

    UnknownError(String),
}

impl fmt::Display for NASError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            // NASError::FileNotFoundError(file) => write!(f, "File was not found: {}", file)?,
            NASError::DirectoryNotFoundError(dir) => write!(f, "Directory was not found: {}", dir)?,
            NASError::InvalidPathError(path) => write!(f, "Invalid path: {}", path)?,
            NASError::InvalidExtensionError(path) => {
                write!(f, "Invalid extension for path: {}", path)?
            }

            NASError::InvalidStreamPlaylist(playlist) => {
                write!(f, "Invalid stream playlist: {}", playlist)?
            }
            NASError::MissingStreamPlaylist => write!(f, "Missing stream playlist in session")?,

            NASError::UnknownError(msg) => write!(f, "An unknown error occurred: {}", msg)?,
        };

        Ok(())
    }
}

impl std::error::Error for NASError {}
