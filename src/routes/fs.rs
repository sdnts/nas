use anyhow::*;
use std::fs;
use std::path::{Path, PathBuf};

use crate::app_state::AppState;
use crate::file::{NASFile, NASFileCategory};
use crate::templates::{BadRequestPageParams, FileListPageParams, StreamPageParams};

pub async fn get(req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    let templates = req.state().clone().templates;
    let path: String = req.param("path").unwrap_or_default();

    let nas_file = NASFile::from_relative_path_str(&path)?;

    let response_body = {
        match nas_file.category {
            NASFileCategory::Directory => {
                // For directories, render the file list page
                let contents = fs::read_dir(&nas_file)
                    .with_context(|| format!("[fs::get] Unable to read_dir: {:?}", nas_file))?;
                let mut files = contents
                    .map(move |f| -> Result<NASFile> {
                        let file = f.context("[fs::get] Failed to get DirEntry ")?;
                        let file = NASFile::from_pathbuf(file.path())?;
                        Ok(file)
                    })
                    .collect::<Result<Vec<NASFile>>>()?;
                files.sort();

                let breadcrumbs: PathBuf = PathBuf::new().join(nas_file.relative_path_str);
                let breadcrumbs = breadcrumbs
                    .iter()
                    .map(|component| -> Result<_> {
                        let component = component.to_str().with_context(|| {
                            format!(
                                "[fs::get] Failed to convert &OsStr to &str: {:?}",
                                component
                            )
                        })?;
                        Ok(component.to_string())
                    })
                    .collect::<Result<Vec<String>>>()?;

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

                templates.render(
                    "fs",
                    &FileListPageParams {
                        title: "/fs".to_string(),
                        hostname: "0zark".to_string(),
                        username: "0zark".to_string(),
                        breadcrumbs,
                        parent_href,
                        files,
                    },
                )?
            }
            NASFileCategory::StreamPlaylist => templates.render(
                "stream",
                &StreamPageParams {
                    hostname: "0zark".to_string(),
                    src: format!("/stream/{}", path),
                    file_name: nas_file.name.to_string(),
                },
            )?,
            _ => templates.render(
                "400",
                &BadRequestPageParams {
                    title: "/fs".to_string(),
                    hostname: "0zark".to_string(),
                    username: "0zark".to_string(),
                },
            )?,
        }
    };

    let response = tide::Response::builder(tide::StatusCode::Ok)
        .body(response_body)
        .content_type("text/html;charset=utf-8")
        .build();

    Ok(response)
}

pub async fn put(mut req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    let path: String = req.param("path").unwrap_or_default();
    let name = req.body_string().await?;

    let nas_file = NASFile::from_relative_path_str(&path)?;
    let renamed_path = NASFile::from_relative_path_str(&path)?;
    let renamed_path: PathBuf = renamed_path.into();
    let renamed_path = renamed_path
        .parent()
        .with_context(|| format!("[fs::put] Path to rename has no parent: {:?}", renamed_path))?
        .join(&name);

    if renamed_path.exists() {
        // Behaviour differs with platform, so exit early
        Err(tide::Error::new(
            tide::StatusCode::BadRequest,
            anyhow!("[fs::put] Target path already exists"),
        ))
    } else {
        fs::rename(nas_file, renamed_path)?;
        Ok(tide::Response::builder(tide::StatusCode::Ok).build())
    }
}

pub async fn post(req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    let path: String = req.param("path").unwrap_or_default();
    let path = NASFile::relative_to_absolute_str(&path)?;
    let path = Path::new(&path);

    let is_empty = req
        .is_empty()
        .with_context(|| format!("[fs::post] Unable to invoke is_empty for req: {:?}", req))?;

    if is_empty {
        // Create Dir at path
        fs::create_dir_all(path)?;
    } else {
        // Create file at path
        use async_std::{fs::OpenOptions, io};
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .await?;

        io::copy(req, file).await?;
    }

    Ok(tide::Response::builder(tide::StatusCode::Ok).build())
}

pub async fn delete(req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    let path: String = req.param("path").unwrap_or_default();

    let nas_file = NASFile::from_relative_path_str(&path)?;

    match nas_file.category {
        NASFileCategory::Directory => fs::remove_dir_all::<NASFile>(nas_file)?,
        _ => fs::remove_file::<NASFile>(nas_file.into())?,
    };

    Ok(tide::Response::builder(tide::StatusCode::Ok).build())
}
