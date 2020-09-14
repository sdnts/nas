use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Responder, Result};
use std::fs;
use std::io;
use std::path::Path;

use crate::app_state::AppState;
use crate::error::NASError;
use crate::file::NASFile;
use crate::templates::AuthPageParams;
use crate::utils::strip_trailing_char;
use crate::CONFIG;

pub async fn post(
    identity: Identity,
    app_state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Bytes,
) -> Result<impl Responder> {
    let templates = &app_state.templates;

    if let None = identity.identity() {
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
        let mut file = fs::OpenOptions::new()
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

    Ok(HttpResponse::Ok().finish())
}
