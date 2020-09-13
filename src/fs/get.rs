use actix_web::{web, HttpResponse, Responder, Result};
use std::fs;
use std::path::PathBuf;

use crate::app_state::AppState;
use crate::error::NASError;
use crate::file::{NASFile, NASFileCategory};
use crate::templates::{BadRequestPageParams, FSPageParams, StreamPageParams};
use crate::utils::strip_trailing_char;

pub async fn get(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, NASError> {
    let templates = &app_state.templates;

    // The NormalizePath middleware will add a trailing slash at the end of the path, so w must remove it
    let path = strip_trailing_char(path.clone());
    let nas_file = NASFile::from_relative_path_str(&path)?;

    let response_body = {
        match nas_file.category {
            NASFileCategory::Directory => {
                // For directories, render the file list page
                let contents = fs::read_dir(&nas_file).map_err(|_| NASError::PathReadError {
                    path: nas_file.absolute_path_str.to_string(),
                })?;
                let mut files = contents
                    .map(move |f| -> Result<NASFile> {
                        let file = f?;
                        let file = NASFile::from_pathbuf(file.path())?;
                        Ok(file)
                    })
                    .collect::<Result<Vec<NASFile>>>()
                    .map_err(|_| NASError::PathReadError {
                        path: nas_file.absolute_path_str.to_string(),
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
                    if breadcrumbs.is_empty() {
                        vec![]
                    } else {
                        breadcrumbs
                            .iter()
                            .take(breadcrumbs.len() - 1)
                            .map(|b| b.to_string())
                            .collect::<Vec<String>>()
                    }
                };
                let parent_href = parent_href.join("/");

                templates
                    .render(
                        "fs",
                        &FSPageParams {
                            title: "/fs".to_string(),
                            hostname: "0zark".to_string(),
                            username: "0zark".to_string(),
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
                        hostname: "0zark".to_string(),
                        src: format!("/stream/{}", path),
                        file_name: nas_file.name.to_string(),
                    },
                )
                .map_err(|_| NASError::TemplateRenderError { template: "stream" })?,
            _ => templates
                .render(
                    "400",
                    &BadRequestPageParams {
                        title: "/fs".to_string(),
                        hostname: "0zark".to_string(),
                        username: "0zark".to_string(),
                    },
                )
                .map_err(|_| NASError::TemplateRenderError { template: "400" })?,
        }
    };

    Ok(HttpResponse::Ok()
        .body(response_body)
        .with_header("Content-Type", "text/html;charset=utf-8"))
}
