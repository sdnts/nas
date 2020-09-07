use anyhow::*;
use serde::Deserialize;
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
                let files = contents
                    .map(move |f| -> Result<NASFile> {
                        let file = f.context("[fs::get] Failed to get DirEntry ")?;
                        let file = NASFile::from_pathbuf(file.path())?;
                        Ok(file)
                    })
                    .collect::<Result<Vec<NASFile>>>()?;

                let breadcrumbs: PathBuf = nas_file.into();
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

                dbg!(&breadcrumbs);
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
                    file_name: "S01E02".to_string(),
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

    let response = tide::Response::builder(200)
        .body(response_body)
        .content_type("text/html;charset=utf-8")
        .build();

    Ok(response)
}

#[derive(Debug, Deserialize)]
struct FSPUTParams {
    // pub dir_name: String,
}
pub async fn put(_: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    Ok(tide::Response::builder(200).build())
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

    Ok(tide::Response::builder(200).build())
}

pub async fn delete(_: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
}
