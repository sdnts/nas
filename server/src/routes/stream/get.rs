use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Responder, Result};
use flate2::{write::GzEncoder, Compression};
use std::fs;
use std::io::prelude::*;

use crate::app_state::AppState;
use crate::error::NASError;
use crate::file::{NASFile, NASFileCategory};
use crate::templates::AuthPageParams;
use crate::utils::strip_trailing_char;
use crate::CONFIG;

pub async fn get(
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

    let response = {
        let mut file = fs::File::open(&nas_file).map_err(|_| NASError::PathReadError {
            path: nas_file.relative_path_str.to_string(),
        })?;
        let mut file_bytes = Vec::new();
        file.read_to_end(&mut file_bytes)
            .map_err(|_| NASError::PathReadError {
                path: nas_file.relative_path_str.to_string(),
            })?;

        // Gzip it
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&file_bytes)?;
        let file_bytes = encoder.finish()?;

        // And send it back
        HttpResponse::Ok()
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(http::header::CONTENT_ENCODING, "gzip")
            .header(
                http::header::ACCESS_CONTROL_EXPOSE_HEADERS,
                "Content-Length",
            )
            .header(http::header::ACCESS_CONTROL_ALLOW_HEADERS, "Range")
            .header(http::header::CONTENT_TYPE, {
                match nas_file.category {
                    NASFileCategory::StreamPlaylist => "application/vnd.apple.mpegurl",
                    NASFileCategory::StreamSegment => "application/octet-stream",
                    _ => "",
                }
            })
            .body(file_bytes)
    };

    Ok(response)
}
