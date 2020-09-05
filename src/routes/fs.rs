use anyhow::Result;
use askama::Template;
use std::fs;

use crate::error::NASError;
use crate::file::{NASFile, NASFileType};
use crate::templates::{BadRequestPage, FileListPage, StreamPage};

pub(crate) async fn get(req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    let path: String = req.param("path").unwrap_or_default();

    let nas_file = NASFile::from_relative_path_str(&path)?;
    let nas_path = nas_file.path;

    let response_body = {
        if nas_path.is_dir() {
            // For directories, render the file list page

            let contents =
                fs::read_dir(&nas_path).map_err(|e| NASError::FileNotFoundError(e.to_string()))?;
            let file_list: Result<Vec<_>, _> = contents
                .map(move |f| -> Result<String> {
                    let file = f.map_err(|e| NASError::UnknownError(e.to_string()))?;
                    let path = file.path();
                    let path = path.to_str().ok_or(NASError::InvalidPathError(
                        "File path is not valid unicode".to_string(),
                    ))?;
                    let file = NASFile::from_absolute_path_str(path)?;
                    let file = file.to_relative_path_str()?;
                    Ok(file.to_string())
                })
                .collect();
            let file_list = file_list.map_err(|e| NASError::UnknownError(e.to_string()))?;

            let page = FileListPage {
                name: "0zark".to_string(),
                file_list,
            };
            page.render()?
        } else {
            // For files

            match nas_file.file_type {
                NASFileType::StreamPlaylist => {
                    let page = StreamPage {
                        name: "0zark".to_string(),
                        src: format!("/stream/{}", path),
                    };
                    page.render()?
                }
                _ => {
                    // For everything else, render the 400 page
                    let page = BadRequestPage {};
                    page.render()?
                }
            }
        }
    };

    let response = tide::Response::builder(200)
        .body(response_body)
        .content_type("text/html;charset=utf-8")
        .build();

    Ok(response)
}

pub(crate) async fn put(_: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
}

pub(crate) async fn post(_: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
}

pub(crate) async fn delete(_: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
}
