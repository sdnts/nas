use actix_web::{web, HttpResponse, Responder, Result};
use std::fs;

use crate::app_state::AppState;
use crate::error::NASError;
use crate::file::{NASFile, NASFileCategory};
use crate::utils::strip_trailing_char;

pub async fn delete(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, NASError> {
    // The NormalizePath middleware will add a trailing slash at the end of the path, so we must remove it
    let path = strip_trailing_char(path.clone());
    let nas_file = NASFile::from_relative_path_str(&path)?;

    match nas_file.category {
        NASFileCategory::Directory => {
            fs::remove_dir_all::<&NASFile>(&nas_file).map_err(|_| NASError::PathDeleteError {
                pathbuf: nas_file.into(),
            })?
        }
        _ => {
            fs::remove_file::<&NASFile>(&nas_file).map_err(|_| NASError::PathDeleteError {
                pathbuf: nas_file.into(),
            })
        }?,
    };

    Ok(HttpResponse::Ok().with_header("Content-Type", "text/html;charset=utf-8"))
}
