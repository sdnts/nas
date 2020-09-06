use anyhow::Result;
use serde_json::json;
use std::fs;

use crate::app_state::AppState;
use crate::error::NASError;
use crate::file::{NASFile, NASFileType};
use crate::templates::{BadRequestPage, FileListPage, StreamPage};

pub(crate) async fn get(req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    let templates = req.state().clone().templates;
    let path: String = req.param("path").unwrap_or_default();

    let nas_file = NASFile::from_relative_path_str(&path)?;
    let nas_path = &nas_file.path;

    let response_body = {
        if nas_path.is_dir() {
            // For directories, render the file list page

            let breadcrumbs = nas_file.to_relative_path_str()?;
            let breadcrumbs: Result<Vec<_>, _> = breadcrumbs
                .split("/")
                .map(|segment| -> Result<String> { Ok(segment.to_string()) })
                .collect();

            let contents =
                fs::read_dir(&nas_path).map_err(|e| NASError::FileNotFoundError(e.to_string()))?;
            let files: Result<Vec<_>> = contents
                .map(move |f| -> Result<NASFile> {
                    let file = f.map_err(|e| NASError::UnknownError(e.to_string()))?;
                    let path = file.path();
                    let path = path.to_str().ok_or(NASError::UnsupportedPathError)?;
                    let file = NASFile::from_absolute_path_str(path)?;
                    Ok(file)
                })
                .collect();
            let files = files.map_err(|e| NASError::UnknownError(e.to_string()))?;

            let file_names: Result<Vec<_>> = files
                .iter()
                .map(|f| -> Result<String> {
                    let file_name = f.path.file_name().ok_or(NASError::UnsupportedPathError)?;
                    let file_name = file_name.to_str().ok_or(NASError::UnsupportedPathError)?;
                    Ok(file_name.to_string())
                })
                .collect();
            let file_sizes: Result<Vec<_>> = files
                .iter()
                .map(|f| -> Result<u64> {
                    if f.path.is_dir() {
                        return Ok(0);
                    }
                    let metadata = f.path.metadata()?;
                    Ok(metadata.len())
                })
                .collect();
            let file_extensions: Result<Vec<_>> = files
                .iter()
                .map(|f| -> Result<String> {
                    if f.path.is_dir() {
                        return Ok("".to_string());
                    }
                    let extension = f.path.extension().ok_or(NASError::UnsupportedPathError)?;
                    let extension = extension.to_str().ok_or(NASError::UnsupportedPathError)?;
                    Ok(extension.to_string())
                })
                .collect();

            // let page = FileListPage {
            //     title: "/fs".to_string(),
            //     hostname: "0zark".to_string(),
            //     username: "0zark".to_string(),
            //     breadcrumbs: breadcrumbs.unwrap_or_default(),
            //     file_names: file_names.unwrap_or_default(),
            //     file_sizes: file_sizes.unwrap_or_default(),
            //     file_extensions: file_extensions.unwrap_or_default(),
            //     file_types: vec![],
            //     test: json!({
            //         "x": "test"
            //     }),
            // };
            templates.render(
                "fs",
                &json!({
                     "title": "/fs".to_string(),
                    "hostname": "0zark".to_string(),
                    "username": "0zark".to_string(),
                }),
            )?
        } else {
            // For files

            match nas_file.file_type {
                NASFileType::StreamPlaylist => templates.render(
                    "stream",
                    &json!({
                    "hostname": "0zark".to_string(),
                    "src": format!("/stream/{}", path),
                    "file_name": "S01E02".to_string(),
                    }),
                )?,
                _ => templates.render(
                    "stream",
                    &json!({
                        "title": "/400".to_string(),
                        "hostname": "0zark".to_string(),
                        "username": "0zark".to_string(),
                    }),
                )?,
            }
        }
    };

    let response = tide::Response::builder(200)
        .body(response_body)
        .content_type("text/html;charset=utf-8")
        .build();

    Ok(response)
}

pub(crate) async fn put(req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    dbg!(&req);
    use async_std::{fs::OpenOptions, io};
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("/home/ozark/nas_root/Movies/test.txt")
        .await?;

    let bytes_written = io::copy(req, file).await?;
    dbg!(bytes_written);

    Ok(tide::Response::builder(200).build())
}

// #[derive(Debug, Deserialize)]
// struct Test {
//     dirname: Option<String>,
//     file: Option<Vec<u8>>,
//     name: Option<String>,
//     data: Option<Vec<u8>>,
// }

pub(crate) async fn post(mut req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
    // println!("Here");
    // let path: String = req.param("path")?;
    // let body: String = req.body_string().await?;

    // println!("Starting, {:?}", body);

    // Ok(tide::Response::builder(200).build())

    // let mut file = std::fs::OpenOptions::new()
    //     .create(true)
    //     .write(true)
    //     .open(path);

    // let bytes_written = std::io::copy(&mut req, &mut file);

    // if let Some(dirname) = body.dirname {
    //     // Create a directory
    //     let nas_file = NASFile::from_relative_path_str(&path)?;
    //     let absolute_path = nas_file.path;
    //     let absolute_path = absolute_path.join(&dirname);
    //     fs::create_dir(&absolute_path)?;

    //     Ok(tide::Redirect::new(format!("/fs/{}", path)).into())
    // } else if let Some(file) = body.file {
    //     // Create a file
    //     dbg!(file);
    //     Ok(tide::Response::builder(200).build())
    // } else {
    //     Ok(tide::Response::builder(500).build())
    // }
}

pub(crate) async fn delete(_: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
}
