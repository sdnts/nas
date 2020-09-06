use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use crate::app_state::AppState;
use crate::error::NASError;
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
                    .map_err(|e| NASError::FileNotFoundError(e.to_string()))?;
                let files = contents
                    .map(move |f| -> Result<NASFile> {
                        let file = f.map_err(|e| NASError::UnknownError(e.to_string()))?;
                        let file = NASFile::from_pathbuf(file.path())?;
                        Ok(file)
                    })
                    .collect::<Result<Vec<NASFile>>>()?;

                let breadcrumbs: PathBuf = nas_file.into();
                let breadcrumbs = breadcrumbs
                    .iter()
                    .map(|component| -> Result<String> {
                        let component = component.to_str().ok_or(NASError::UnsupportedPathError)?;
                        Ok(component.to_string())
                    })
                    .collect::<Result<Vec<String>>>()?;

                templates.render(
                    "fs",
                    &FileListPageParams {
                        title: "/fs".to_string(),
                        hostname: "0zark".to_string(),
                        username: "0zark".to_string(),
                        files,
                        breadcrumbs,
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

pub async fn put(req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
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

pub async fn post(mut req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
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

pub async fn delete(_: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
}
