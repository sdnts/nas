use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Responder, Result};
use std::fs;

use crate::app_state::AppState;
use crate::error::NASError;
use crate::file::{NASFile, NASFileCategory};
use crate::templates::AuthPageParams;
use crate::utils::strip_trailing_char;

pub async fn delete(
    identity: Identity,
    app_state: web::Data<AppState>,
    path: web::Path<String>,
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

    Ok(HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "text/html;charset=utf-8")
        .finish())
}
