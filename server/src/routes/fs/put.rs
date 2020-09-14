use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Responder, Result};
use std::fs;
use std::path::PathBuf;

use crate::app_state::AppState;
use crate::error::NASError;
use crate::file::NASFile;
use crate::templates::AuthPageParams;
use crate::utils::strip_trailing_char;
use crate::CONFIG;

pub async fn put(
    identity: Identity,
    app_state: web::Data<AppState>,
    path: web::Path<String>,
    name: String,
) -> Result<impl Responder> {
    let templates = &app_state.templates;
    let identity = identity.identity();

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
    let path = strip_trailing_char(path.clone());

    let nas_file = NASFile::from_relative_path_str(&path, &username)?;
    let renamed_file = NASFile::from_relative_path_str(&path, &username)?;

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
        }
        .into())
    } else {
        fs::rename(&nas_file, &renamed_pathbuf).map_err(|_| NASError::PathRenameError {
            pathbuf: renamed_pathbuf,
        })?;

        Ok(HttpResponse::Ok().finish())
    }
}
