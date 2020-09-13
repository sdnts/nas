use actix_web::{web, HttpResponse, Responder, Result};
use std::fs;
use std::path::PathBuf;

use crate::error::NASError;
use crate::file::NASFile;
use crate::utils::strip_trailing_char;

pub async fn put(path: web::Path<String>, name: String) -> Result<impl Responder, NASError> {
    // The NormalizePath middleware will add a trailing slash at the end of the path, so we must remove it
    let path = strip_trailing_char(path.clone());

    let nas_file = NASFile::from_relative_path_str(&path)?;
    let renamed_file = NASFile::from_relative_path_str(&path)?;

    let renamed_pathbuf: PathBuf = renamed_file.into();
    let renamed_pathbuf = renamed_pathbuf
        .parent()
        .ok_or(NASError::InvalidPathBuf {
            pathbuf: renamed_pathbuf.to_owned(),
        })?
        .join(&name);

    if renamed_pathbuf.exists() {
        // Behaviour differs with platform, so exit early
        Err(NASError::PathExistsError {
            pathbuf: renamed_pathbuf.to_owned(),
        })
    } else {
        fs::rename(&nas_file, &renamed_pathbuf).map_err(|_| NASError::PathRenameError {
            pathbuf: renamed_pathbuf,
        })?;

        Ok(HttpResponse::Ok())
    }
}
