use std::fmt;

#[derive(Debug)]
pub enum NASError {
    FileNotFoundError(String),
    InvalidPathError(String),
    UnsupportedPathError,

    UnknownError(String),
}

impl fmt::Display for NASError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            NASError::FileNotFoundError(file) => write!(f, "File was not found: {}", file)?,
            NASError::InvalidPathError(path) => {
                write!(f, "File path is not valid unicode {:?}", path)?
            }
            NASError::UnsupportedPathError => write!(f, "File path is not valid unicode")?,

            NASError::UnknownError(msg) => write!(f, "An unknown error occurred: {}", msg)?,
        };

        Ok(())
    }
}

impl std::error::Error for NASError {}
