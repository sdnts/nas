use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Responder, Result};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::thread;

use crate::app_state::AppState;
use crate::error::NASError;
use crate::file::{NASFile, NASFileCategory};
use crate::templates::AuthPageParams;
use crate::utils::strip_trailing_char;
use crate::CONFIG;

pub async fn post(
    identity: Identity,
    app_state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Bytes,
) -> Result<impl Responder> {
    let identity = identity.identity();
    let templates = &app_state.templates;

    if let None = identity {
        return Ok(HttpResponse::Unauthorized()
            .header(http::header::CONTENT_TYPE, "text/html;charset=utf-8")
            .body(
                templates
                    .render(
                        "auth",
                        &AuthPageParams {
                            theme: CONFIG.theme.clone(),
                            logged_in: false,
                            message: Some("Protected resource, please log in".to_string()),
                            redirect_url: None,
                        },
                    )
                    .map_err(|_| NASError::TemplateRenderError { template: "auth" })?,
            ));
    }

    let username = identity.unwrap();

    // The NormalizePath middleware will add a trailing slash at the end of the path, so we must remove it
    let relative_path_str = strip_trailing_char(path.clone());
    let path = NASFile::relative_to_absolute_str(&relative_path_str, &username)?;
    let path = PathBuf::new().join(&path);
    let user_fs_root = NASFile::user_fs_root(&username)?;

    if body.is_empty() {
        // Create Dir at path
        fs::create_dir_all(path).map_err(|_| NASError::PathCreateError {
            path: relative_path_str,
        })?;
    } else {
        // Create file at path
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .map_err(|_| NASError::PathCreateError {
                path: relative_path_str.to_owned(),
            })?;

        io::copy(&mut &body.to_vec()[..], &mut file).map_err(|_| NASError::PathCreateError {
            path: relative_path_str.to_owned(),
        })?;

        // If this is a video file, start generating stream segments
        let file = NASFile::from_relative_path_str(&relative_path_str, &username)?;
        match file.category {
            NASFileCategory::Video => {
                thread::spawn(move || {
                    streamgen::generate_stream_segments_for_path(&path, &user_fs_root)
                });
            }
            _ => {}
        };
    }

    Ok(HttpResponse::Ok().finish())
}
