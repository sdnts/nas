use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Responder, Result};
use std::fs;
use std::path::PathBuf;

use crate::app_state::AppState;
use crate::error::NASError;
use crate::file::{NASFile, NASFileCategory};
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

    if let None = identity {
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
                    .map_err(|_| NASError::TemplateRenderError { template: "auth" })?,
            ));
    }

    let username = identity.unwrap();

    // The NormalizePath middleware will add a trailing slash at the end of the path, so we must remove it
    let path = strip_trailing_char(path.clone());
    let nas_file = NASFile::from_relative_path_str(&path, &username)?;

    let response_body = {
        match nas_file.category {
            NASFileCategory::Directory => {
                // For directories, render the file list page
                let contents = fs::read_dir(&nas_file).map_err(|_| NASError::PathReadError {
                    path: nas_file.absolute_path_str.to_string(),
                })?;
                let mut files = contents
                    .map(|f| -> Result<NASFile> {
                        let file = f?;
                        let file = NASFile::from_pathbuf(file.path(), &username)?;
                        Ok(file)
                    })
                    .collect::<Result<Vec<NASFile>>>()
                    .map_err(|_| NASError::PathReadError {
                        path: nas_file.relative_path_str.to_string(),
                    })?;
                files.sort();

                let breadcrumbs: PathBuf = PathBuf::new().join(&nas_file.relative_path_str);
                let breadcrumbs = breadcrumbs
                    .iter()
                    .map(|component| -> Result<_> {
                        let component =
                            component.to_str().ok_or(NASError::OsStrConversionError {
                                osstring: component.to_os_string(),
                            })?;
                        Ok(component.to_string())
                    })
                    .collect::<Result<Vec<String>>>()
                    .map_err(|_| NASError::BreadcrumbError {
                        pathbuf: nas_file.into(),
                    })?;

                let parent_href = {
                    if breadcrumbs.len() > 1 {
                        let mut b_iter = breadcrumbs.iter();
                        b_iter.next(); // Remove first segment, since it will always be `/`
                        let len = b_iter.len();
                        b_iter
                            .take(len - 1)
                            .map(|b| b.to_string())
                            .collect::<Vec<String>>()
                    } else {
                        // If there aren't at least 2 breadcrumbs, the parent will be this user's root path
                        vec![]
                    }
                };
                let parent_href = parent_href.join("/");

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
                    .map_err(|_| NASError::TemplateRenderError { template: "fs" })?
            }
            NASFileCategory::StreamPlaylist => templates
                .render(
                    "stream",
                    &StreamPageParams {
                        theme: CONFIG.theme.clone(),
                        src: format!("/stream/{}", path),
                        file_name: nas_file.name.to_string(),
                    },
                )
                .map_err(|_| NASError::TemplateRenderError { template: "stream" })?,
            _ => templates
                .render(
                    "400",
                    &BadRequestPageParams {
                        theme: CONFIG.theme.clone(),
                        title: "/fs".to_string(),
                        username,
                    },
                )
                .map_err(|_| NASError::TemplateRenderError { template: "400" })?,
        }
    };

    Ok(HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "text/html;charset=utf-8")
        .body(response_body))
}
