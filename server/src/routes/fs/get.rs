use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Responder, Result};
use serde_json::{json, Value};
use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

use crate::app_state::AppState;
use crate::error::NASError;
use crate::file::{AbsolutePath, Breadcrumbs, NASFile, NASFileCategory, RelativePath};
use crate::templates::{AuthPageParams, BadRequestPageParams, FSPageParams, StreamPageParams};
use crate::utils::strip_trailing_char;
use crate::CONFIG;

pub async fn get(
    identity: Identity,
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let identity = identity.identity();
    let templates = &app_state.templates;

    if identity.is_none() {
        println!();
        return Ok(HttpResponse::Unauthorized()
            .header(http::header::CONTENT_TYPE, "text/html;charset=utf-8")
            .body(
                templates
                    .render(
                        "auth",
                        &AuthPageParams {
                            theme: CONFIG.theme.clone(),
                            message: None,
                            logged_in: false,
                            redirect_url: Some(format!("/fs/{}", path.clone())),
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

    let response_body = {
        match &absolute_path.category()? {
            NASFileCategory::Directory => {
                // For directories, render the file list page
                let breadcrumbs = Breadcrumbs::from(&relative_path);

                let pathbuf: PathBuf = absolute_path.into();

                let parent_pathbuf: PathBuf = pathbuf
                    .parent()
                    .ok_or(NASError::ParentPathResolutionError {
                        pathbuf: pathbuf.to_owned(),
                    })?
                    .into();
                let parent_href = format!("/fs/{}", parent_pathbuf.display());

                let contents = fs::read_dir(&pathbuf).map_err(|_| NASError::PathReadError {
                    pathbuf: pathbuf.to_owned(),
                })?;
                let mut files = contents
                    .map(|f| -> Result<AbsolutePath, NASError> {
                        let file = f
                            .map_err(|e| NASError::IOError {
                                error: e.to_string(),
                            })?
                            .path();
                        let file = AbsolutePath::try_from(file)?;
                        Ok(file)
                    })
                    .collect::<Result<Vec<AbsolutePath>, NASError>>()?;
                files.sort();

                let files = files
                    .iter()
                    .map(|file| -> Result<Value, NASError> {
                        // Must convert OsString to String (potentially losing data) to be able to display in a browser
                        let name = file.name()?;
                        let name = name.to_str().ok_or(NASError::OsStrConversionError {
                            osstring: name.to_owned(),
                        })?;

                        let category = file.category()?;

                        // Must convert OsString to String (potentially losing data) to be able to display in a browser
                        let extension = file.extension()?;
                        let extension =
                            extension.to_str().ok_or(NASError::OsStrConversionError {
                                osstring: extension.to_owned(),
                            })?;

                        let size_bytes = file.size_bytes()?;

                        Ok(json!({
                                "name": name,
                                "category": category,
                                "extension":extension,
                                "size_bytes": size_bytes,
                            }
                        ))
                    })
                    .collect::<Result<Vec<Value>, NASError>>()?;

                templates
                    .render(
                        "fs",
                        &FSPageParams {
                            theme: CONFIG.theme.clone(),
                            username,
                            breadcrumbs,
                            parent_href,
                            files,
                        },
                    )
                    .map_err(|e| NASError::TemplateRenderError {
                        template: "fs".to_string(),
                        error: e.to_string(),
                    })?
            }
            NASFileCategory::StreamPlaylist => {
                let filename = absolute_path.name()?;
                let filename = filename.to_str().ok_or(NASError::OsStrConversionError {
                    osstring: filename.to_owned(),
                })?;
                templates
                    .render(
                        "stream",
                        &StreamPageParams {
                            theme: CONFIG.theme.clone(),
                            src: format!("/stream/{}", path),
                            filename: filename.to_string(),
                        },
                    )
                    .map_err(|e| NASError::TemplateRenderError {
                        template: "stream".to_string(),
                        error: e.to_string(),
                    })?
            }
            _ => templates
                .render(
                    "400",
                    &BadRequestPageParams {
                        theme: CONFIG.theme.clone(),
                        title: "/fs".to_string(),
                        username,
                    },
                )
                .map_err(|e| NASError::TemplateRenderError {
                    template: "400".to_string(),
                    error: e.to_string(),
                })?,
        }
    };

    Ok(HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "text/html;charset=utf-8")
        .body(response_body))
}
