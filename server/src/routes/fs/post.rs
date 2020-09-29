use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Responder, Result};
use std::convert::TryFrom;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::thread;

use crate::app_state::AppState;
use crate::error::NASError;
use crate::file::{AbsolutePath, NASFile, NASFileCategory};
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

    if identity.is_none() {
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
                    .map_err(|e| NASError::TemplateRenderError {
                        template: "auth".to_string(),
                        error: e.to_string(),
                    })?,
            ));
    }

    let username = identity.unwrap();

    // The NormalizePath middleware will add a trailing slash at the end of the path, so we must remove it
    let relative_path_str = strip_trailing_char(&path);
    // Only get a pathbuf, not an AbsolutePath, because we know this path doesn't exist yet (We'll create it)
    let pathbuf = AbsolutePath::user_fs_root(&username).join(relative_path_str);

    if body.is_empty() {
        // Create Dir at path
        fs::create_dir_all(&pathbuf).map_err(|_| NASError::PathCreateError { pathbuf })?;
    } else {
        // Create file at path
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&pathbuf)
            .map_err(|_| NASError::PathCreateError {
                pathbuf: pathbuf.to_owned(),
            })?;

        io::copy(&mut &body.to_vec()[..], &mut file).map_err(|_| NASError::PathCreateError {
            pathbuf: pathbuf.to_owned(),
        })?;

        let absolute_path = AbsolutePath::try_from(pathbuf)?;
        let category = absolute_path.category()?;

        // If this is a video file, start generating stream segments
        if let NASFileCategory::Video = category {
            thread::spawn(move || {
                let pathbuf: PathBuf = absolute_path.into();
                streamgen::generate_stream_segments_for_path(
                    &pathbuf,
                    &AbsolutePath::user_fs_root(&username),
                )
            });
        };
    }

    Ok(HttpResponse::Ok().finish())
}
