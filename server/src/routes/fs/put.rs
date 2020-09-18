use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Responder, Result};
use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

use crate::app_state::AppState;
use crate::error::NASError;
use crate::file::{AbsolutePath, RelativePath};
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
                    .map_err(|e| NASError::TemplateRenderError {
                        template: "auth".to_string(),
                        error: e.to_string(),
                    })?,
            ));
    }

    let username = identity.unwrap();

    // The NormalizePath middleware will add a trailing slash at the end of the path, so we must remove it
    let relative_path_str = strip_trailing_char(&path);
    let relative_path = RelativePath::new(&relative_path_str, &username);
    let absolute_path = AbsolutePath::try_from(&relative_path)?;

    let pathbuf: PathBuf = absolute_path.into();
    let renamed_pathbuf = pathbuf
        .parent()
        .ok_or(NASError::NonExistentPath {
            pathbuf: pathbuf.to_owned(),
        })?
        .join(&name);

    if renamed_pathbuf.exists() {
        // Rename behaviour differs with platform, so exit early
        Err(NASError::PathExistsError {
            pathbuf: renamed_pathbuf.to_owned(),
        }
        .into())
    } else {
        fs::rename(&pathbuf, &renamed_pathbuf).map_err(|_| NASError::PathRenameError {
            from: pathbuf,
            to: renamed_pathbuf,
        })?;

        Ok(HttpResponse::Ok().finish())
    }
}
