use actix_web::{web, HttpResponse, Responder, Result};
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;

use crate::error::NASError;
use crate::file::NASFile;
use crate::utils::strip_trailing_char;

pub async fn post(path: web::Path<String>, body: web::Bytes) -> Result<impl Responder> {
    // The NormalizePath middleware will add a trailing slash at the end of the path, so we must remove it
    let path = strip_trailing_char(path.clone());
    let path = NASFile::relative_to_absolute_str(&path)?;
    let path = Path::new(&path);

    if body.is_empty() {
        // Create Dir at path
        fs::create_dir_all(path).map_err(|_| NASError::PathCreateError {
            pathbuf: path.into(),
        })?;
    } else {
        // Create file at path
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .map_err(|_| NASError::PathCreateError {
                pathbuf: path.into(),
            })?;

        io::copy(&mut &body.to_vec()[..], &mut file).map_err(|_| NASError::PathCreateError {
            pathbuf: path.into(),
        })?;
    }

    Ok(HttpResponse::Ok())
}
